use crate::App;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use std::io::Result;
use std::path::PathBuf;
use std::time::Instant;
use serde::Deserialize;





// A command can be either a simple string or a symmetry block
#[derive(Deserialize)]
#[serde(untagged)]
enum ScriptCommand {
    Simple(String),
    SymmetryBlock(SymmetryBlock),
}

#[derive(Deserialize)]
struct SymmetryBlock {
    symmetry: SymmetryInfo,
    commands: Vec<String>,
}

#[derive(Deserialize)]
struct SymmetryInfo {
    mode: String,
    coordinate: i32, // i32 to handle negative offsets for diagonals
}







// Helper to get the path to the script file
pub fn get_script_path() -> Result<PathBuf> {
    let app_dir = crate::utils::get_or_create_app_dir()?;
    Ok(app_dir.join("command_draw.json"))
}

// Loads the script from disk into the App state for editing
pub fn load_script_for_editing(app: &mut App) {
    let path = match get_script_path() {
        Ok(p) => p,
        Err(_) => {
            app.status_message = Some(("Could not access script path.".to_string(), Instant::now()));
            return;
        }
    };
    let content = if path.exists() {
        std::fs::read_to_string(path).unwrap_or_else(|_| "[\n\"apply_color:#RRGGBB X,Y\"\n]".to_string())
    } else {
        "[\n\"apply_color:#RRGGBB X,Y\"\n]".to_string()
    };
    app.script_content_lines = content.lines().map(String::from).collect();
    app.script_cursor_line = 0;
    app.script_scroll_state = 0;
    app.script_cursor_char_pos = 0;
    app.script_change_has_occured = false;
    app.mode = crate::AppMode::ScriptEditor;
}

// Saves the script from the App state back to disk
pub fn save_script(app: &mut App) {
    if let Ok(path) = get_script_path() {
        let content: String = app.script_content_lines.join("\n");
        if serde_json::from_str::<serde_json::Value>(&content).is_ok() {
            if std::fs::write(path, content).is_ok() {
                app.status_message = Some(("Script saved.".to_string(), Instant::now()));
            } else {
                app.status_message = Some(("Error saving script.".to_string(), Instant::now()));
            }
        } else {
            app.status_message = Some(("Invalid JSON. Could not save script.".to_string(), Instant::now()));
        }
    }
}

// Clears the script in the editor
pub fn clear_script(app: &mut App) {
    app.script_content_lines = vec![String::new()];
    app.script_cursor_line = 0;
    app.script_cursor_char_pos = 0;
    app.script_scroll_state = 0;
    app.status_message = Some(("Script cleared.".to_string(), Instant::now()));
}

// The core engine that parses and executes the drawing script
pub fn parse_and_execute_script(app: &mut App) {
    let path = match get_script_path() {
        Ok(p) => p,
        Err(_) => { app.status_message = Some(("Could not access script path.".to_string(), Instant::now())); return; }
    };
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => { app.status_message = Some(("command_draw.json not found.".to_string(), Instant::now())); return; }
    };
    let commands = match serde_json::from_str::<Vec<ScriptCommand>>(&content) {
        Ok(c) => c,
        Err(e) => { app.status_message = Some((format!("Invalid JSON in script: {}", e), Instant::now())); return; }
    };

    app.save_state_for_undo();
    let mut operations_performed = 0;
    let original_symmetry = app.symmetry_mode; // Save the user's current symmetry setting

    for command in commands {
        match command {
            ScriptCommand::Simple(cmd_str) => {
                // For simple commands, temporarily turn symmetry OFF
                app.symmetry_mode = crate::SymmetryMode::Off;
                execute_single_command_string(app, &cmd_str, &mut operations_performed);
            },
            ScriptCommand::SymmetryBlock(block) => {
                // For a symmetry block, set the specified symmetry mode
                let new_mode = match block.symmetry.mode.as_str() {
                    "vertical" => crate::SymmetryMode::Vertical(block.symmetry.coordinate as u16),
                    "horizontal" => crate::SymmetryMode::Horizontal(block.symmetry.coordinate as u16),
                    "diagonal_forward" => crate::SymmetryMode::DiagonalForward(block.symmetry.coordinate),
                    "diagonal_backward" => crate::SymmetryMode::DiagonalBackward(block.symmetry.coordinate),
                    _ => crate::SymmetryMode::Off,
                };
                app.symmetry_mode = new_mode;
                // Execute all commands within this block using that symmetry
                for cmd_str in &block.commands {
                    execute_single_command_string(app, cmd_str, &mut operations_performed);
                }
            }
        }
    }

    app.symmetry_mode = original_symmetry; // IMPORTANT: Restore the user's original symmetry setting
    app.status_message = Some((format!("Script executed. {} operations performed.", operations_performed), Instant::now()));
}

// Renders the UI for the script editor
pub fn draw_script_editor(frame: &mut Frame, app: &mut App) {
    let area = crate::utils::centered_rect(80, 90, frame.size());
    frame.render_widget(Clear, area);
    let block = Block::default()
        .title(" Script Editor (Esc to Exit) ")
        .borders(Borders::ALL);
    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let items: Vec<Line> = app.script_content_lines.iter()
        .map(|line| Line::from(line.as_str()))
        .collect();

    let paragraph = Paragraph::new(items)
        .block(Block::default())
        .scroll((app.script_scroll_state, 0));

    frame.render_widget(paragraph, inner_area);

    if app.script_cursor_line >= app.script_scroll_state as usize {
        let cursor_x = inner_area.x + app.script_cursor_char_pos as u16;
        let cursor_y = inner_area.y + (app.script_cursor_line - app.script_scroll_state as usize) as u16;
        if cursor_y < inner_area.bottom() {
            frame.set_cursor(cursor_x, cursor_y);
        }
    }
}



pub fn create_default_script_if_missing() -> std::io::Result<()> {
    let script_path = get_script_path()?;
    if !script_path.exists() {
        let default_content = r#"[
        "apply_color:#FF0000 10,10",
        {
            "symmetry": { "mode": "vertical", "coordinate": 15 },
            "commands": [
            "apply_color:#00FF00 10,12-12,12"
            ]
        }
        ]"#;
        std::fs::write(script_path, default_content)?;
    }
    Ok(())
}




fn execute_single_command_string(app: &mut App, cmd_str: &str, operations_performed: &mut i32) {
    let parse_coord = |s: &str| -> Option<(u16, u16)> {
        s.split_once(',')
         .and_then(|(x_str, y_str)| {
            x_str.parse::<u16>().ok()
                .and_then(|x| y_str.parse::<u16>().ok().map(|y| (x, y)))
         })
    };

    let parts: Vec<&str> = cmd_str.split_whitespace().collect();
    if parts.len() < 2 { return; }

    let command_part = parts[0];
    let coordinate_parts = &parts[1..];

    // --- Corrected if/else Structure ---

    if let Some((cmd, value)) = command_part.split_once(':') {
        // This block handles commands WITH a color value, like "apply_color:" or "fill:"
        if cmd == "apply_color" {
            if let Some(color) = App::parse_hex_color(value) {
                let original_selection = app.current_selection;
                let original_opacity = app.opacity;
                app.current_selection = crate::palette::PaletteEntry::Color(color);
                app.opacity = 1.0; // Scripts should always draw at full opacity

                for coord_str in coordinate_parts {
                    if let Some((start_str, end_str)) = coord_str.split_once('-') {
                        if let (Some((x1, y1)), Some((x2, y2))) = (parse_coord(start_str), parse_coord(end_str)) {
                            for y in y1.min(y2)..=y1.max(y2) {
                                for x in x1.min(x2)..=x1.max(x2) {
                                    app.apply_brush(x, y);
                                    *operations_performed += 1;
                                }
                            }
                        }
                    } else if let Some((x, y)) = parse_coord(coord_str) {
                        app.apply_brush(x, y);
                        *operations_performed += 1;
                    }
                }
                app.current_selection = original_selection;
                app.opacity = original_opacity;
            }
        } else if cmd == "fill" && !coordinate_parts.is_empty() {
            if let Some((x, y)) = parse_coord(coordinate_parts[0]) {
                if let Some(color) = App::parse_hex_color(value) {
                    app.fill_from_point(x as usize, y as usize, color, 1.0);
                    *operations_performed += 1;
                }
            }
        }
    } else if command_part == "erase" {
        // This block handles commands WITHOUT a color value
        for coord_str in coordinate_parts {
            if let Some((start_str, end_str)) = coord_str.split_once('-') {
                if let (Some((x1, y1)), Some((x2, y2))) = (parse_coord(start_str), parse_coord(end_str)) {
                    for y in y1.min(y2)..=y1.max(y2) {
                        for x in x1.min(x2)..=x1.max(x2) {
                            app.erase_brush(x, y);
                            *operations_performed += 1;
                        }
                    }
                }
            } else if let Some((x, y)) = parse_coord(coord_str) {
                app.erase_brush(x, y);
                *operations_performed += 1;
            }
        }
    }
}




