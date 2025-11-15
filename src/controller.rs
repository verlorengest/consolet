// controller.rs
use crate::{App, AppMode, PIXEL_WIDTH, execute_command, Config, file_browser};

use crate::keybindings::{Action, Keybinding, Keybindings};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, MouseEvent, MouseEventKind, MouseButton};
use std::io::Result;
use chrono::Local;
use crossterm::cursor::{Hide, Show, SetCursorStyle};
use crossterm::ExecutableCommand;
use std::io::stdout;
use std::time::Instant;
use strum::IntoEnumIterator;
use crate::config::ConfigSetting;
use unicode_segmentation::UnicodeSegmentation;


pub fn handle_events(app: &mut App) -> Result<()> {
    if event::poll(std::time::Duration::from_millis(20))? {
        match event::read()? {
            Event::Key(key) => handle_key_event(app, key)?,
            Event::Mouse(mouse_event) => {
                if app.mode == AppMode::FileBrowser {
                    file_browser::handle_browser_input(app, None, Some(mouse_event));
                } else if app.mouse_events_enabled {
                    handle_mouse_event(app, mouse_event);
                }
            },
            _ => {}
        }
    }
    Ok(())
}

pub fn handle_mouse_event(app: &mut App, mouse_event: MouseEvent) {



    if let Some(layer_area) = app.last_layer_area {
        if mouse_event.row >= layer_area.y && mouse_event.row < layer_area.bottom() && 
           mouse_event.column >= layer_area.x && mouse_event.column < layer_area.right() {
            match mouse_event.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                    let clicked_row = (mouse_event.row - layer_area.y) as usize;
                    let clicked_index = app.layer_scroll_state + clicked_row;
                    if clicked_index < app.layers.len() {
                        app.active_layer_index = clicked_index;
                        app.sync_canvas_from_layers();
                    }
                }
                MouseEventKind::ScrollUp => {
                    app.layer_scroll_state = app.layer_scroll_state.saturating_sub(1);
                }
                MouseEventKind::ScrollDown => {
                    let max_scroll = app.layers.len().saturating_sub(1);
                    app.layer_scroll_state = (app.layer_scroll_state + 1).min(max_scroll);
                }
                _ => {}
            }
            return;
        }
    }



    if let Some(palette_area) = app.last_palette_area {
        if mouse_event.row >= palette_area.y && mouse_event.row < palette_area.bottom() && mouse_event.column >= palette_area.x && mouse_event.column < palette_area.right() {
            let columns = (palette_area.width / 3).max(1) as usize;

            if app.mode != AppMode::Drawing && app.mode != AppMode::ColorPicker {
                return;
            }

            match mouse_event.kind {
                MouseEventKind::Down(MouseButton::Left) => {
                let columns = (palette_area.width / 3).max(1) as usize;
                let col = ((mouse_event.column - palette_area.x) / 3) as usize;
                let row = (mouse_event.row - palette_area.y) as usize;

                let clicked_index = app.palette_scroll_state + (row * columns) + col;

                    
                    if clicked_index < app.color_palette.len() {
                        app.palette_index = clicked_index;
                        app.select_color_entry();
                    }
                }
                MouseEventKind::ScrollUp => {
                    app.palette_scroll_state = app.palette_scroll_state.saturating_sub(columns);
                }
                MouseEventKind::ScrollDown => {
                    let total_rows = (app.color_palette.len() + columns - 1) / columns;
                    let visible_rows = palette_area.height as usize;
                    let max_scroll_row = total_rows.saturating_sub(visible_rows);
                    let max_scroll = max_scroll_row * columns;
                    app.palette_scroll_state = (app.palette_scroll_state + columns).min(max_scroll);
                }
                _ => {}
            }
            app.is_mouse_dragging = false; // Prevent canvas drawing
            return;
        }
    }

    if let AppMode::HelpScreen = app.mode {
        match mouse_event.kind {
            MouseEventKind::ScrollUp => app.help_scroll = app.help_scroll.saturating_sub(1),
            MouseEventKind::ScrollDown => app.help_scroll += 1,
            _ => {}
        }
        return; // Important: Do not process other mouse events
    }


    if let Some(tool_area) = app.last_tool_area {
        if mouse_event.row >= tool_area.y && mouse_event.row < tool_area.bottom() && mouse_event.column >= tool_area.x && mouse_event.column < tool_area.right() {
            if let MouseEventKind::Down(MouseButton::Left) = mouse_event.kind {
                let col = ((mouse_event.column - tool_area.x) / 3) as usize;
                if col < app.tool_palette.len() {
                    app.tool_index = col;
                    app.select_tool_entry();
                }
            }
            app.is_mouse_dragging = false;
            return;
        }
    }



if let Some(canvas_rect) = app.last_centered_canvas_rect {
    if mouse_event.column >= canvas_rect.x && mouse_event.column < canvas_rect.right() &&
       mouse_event.row >= canvas_rect.y && mouse_event.row < canvas_rect.bottom() {
        
        match mouse_event.kind {
            MouseEventKind::ScrollUp => {
                match mouse_event.modifiers {
                    event::KeyModifiers::CONTROL => app.pan_view(0, -1),
                    event::KeyModifiers::SHIFT => app.pan_view(-2, 0),
                    event::KeyModifiers::ALT => app.zoom(2),
                    _ => match app.canvas_scroll_action {
                        crate::CanvasScrollAction::ChangePenSize => app.change_pen_size(1),
                        crate::CanvasScrollAction::ChangeOpacity => app.change_opacity(1.0),
                    },
                }
                return;
            },
            MouseEventKind::ScrollDown => {
                match mouse_event.modifiers {
                    event::KeyModifiers::CONTROL => app.pan_view(0, 1),
                    event::KeyModifiers::SHIFT => app.pan_view(2, 0),
                    event::KeyModifiers::ALT => app.zoom(-2),
                    _ => match app.canvas_scroll_action {
                        crate::CanvasScrollAction::ChangePenSize => app.change_pen_size(-1),
                        crate::CanvasScrollAction::ChangeOpacity => app.change_opacity(-1.0),
                    },
                }
                return;
            },
            _ => {}
        }

        let pixel_render_height = (app.zoom_level / PIXEL_WIDTH).max(1);
        let relative_x = (mouse_event.column - canvas_rect.x) / app.zoom_level;
        let relative_y = (mouse_event.row - canvas_rect.y) / pixel_render_height;

        let canvas_x_i32 = app.view_offset_x + relative_x as i32;
        let canvas_y_i32 = app.view_offset_y + relative_y as i32;

        if canvas_x_i32 < 0 || canvas_x_i32 >= app.canvas_width as i32 ||
           canvas_y_i32 < 0 || canvas_y_i32 >= app.canvas_height as i32 {
            if let MouseEventKind::Up(_) = mouse_event.kind {
                app.is_mouse_dragging = false;
                if app.protect_stroke { app.drawn_pixels_in_stroke.clear(); }
            }
            return;
        }
        let canvas_x = canvas_x_i32 as u16;
        let canvas_y = canvas_y_i32 as u16;

        app.cursor_pos = (canvas_x, canvas_y);

        match mouse_event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                app.is_mouse_dragging = true;
                if app.protect_stroke { app.drawn_pixels_in_stroke.clear(); }
                app.save_state_for_undo();
                app.apply_brush(canvas_x, canvas_y);
            },
            MouseEventKind::Drag(MouseButton::Left) => {
                if app.is_mouse_dragging {
                    app.apply_brush(canvas_x, canvas_y);
                }
            },
            MouseEventKind::Up(MouseButton::Left) => {
                app.is_mouse_dragging = false;
                if app.protect_stroke { app.drawn_pixels_in_stroke.clear(); }
            },
            MouseEventKind::Down(MouseButton::Right) => {
                app.is_mouse_dragging = true;
                if app.protect_stroke { app.drawn_pixels_in_stroke.clear(); }
                app.save_state_for_undo();
                app.erase_brush(canvas_x, canvas_y);
            },
            MouseEventKind::Drag(MouseButton::Right) => {
                if app.is_mouse_dragging {
                    app.erase_brush(canvas_x, canvas_y);
                }
            },
            MouseEventKind::Up(MouseButton::Right) => {
                app.is_mouse_dragging = false;
                if app.protect_stroke { app.drawn_pixels_in_stroke.clear(); }
            },
            _ => {}
        }
    }
}
}

pub fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<()> {
    if app.is_changing_keybinding {
        if key.kind == KeyEventKind::Press {
            let new_binding = Keybinding { code: key.code, modifiers: key.modifiers };
            let action_to_change = Action::iter().nth(app.keybindings_selection_index).unwrap();

            app.keybindings.map.insert(action_to_change, new_binding);
            app.is_changing_keybinding = false;
            app.keybinding_change_has_occured = true;
        }
        return Ok(());
    }

    if key.kind == KeyEventKind::Release {
        if let Some(binding) = app.keybindings.map.get(&Action::Draw) {
            if key.code == binding.code && key.modifiers == binding.modifiers {
                app.is_space_held = false;
                app.last_apply_time = None;
                if app.protect_stroke {
                    app.drawn_pixels_in_stroke.clear();
                }
            }
        }

        if let Some(binding) = app.keybindings.map.get(&Action::Spray) {
            if key.code == binding.code && key.modifiers == binding.modifiers {
                app.is_spraying = false;
                app.last_apply_time = None;
            }
        }

        return Ok(());
    }

    if key.kind != KeyEventKind::Press {
        return Ok(());
    }

    match app.mode {

        AppMode::FileBrowser => {
            file_browser::handle_browser_input(app, Some(key), None);
        },

        AppMode::Drawing => {
            // Only proceed if a non-modifier key was pressed.
            // This prevents Ctrl/Shift alone from triggering actions.
            if !matches!(key.code, KeyCode::Modifier(_)) {
            if let Some((action, _)) = app.keybindings.map.iter().find(|(_, &binding)| {
                binding.code == key.code && binding.modifiers == key.modifiers
            }) {
                    match action {
                        Action::MoveCursorUp => app.move_cursor(0, -1),
                        Action::MoveCursorDown => app.move_cursor(0, 1),
                        Action::MoveCursorLeft => app.move_cursor(-1, 0),
                        Action::MoveCursorRight => app.move_cursor(1, 0),
                        Action::PanViewUp => app.pan_view(0, -1),
                        Action::PanViewDown => app.pan_view(0, 1),
                        Action::PanViewLeft => app.pan_view(-1, 0),
                        Action::PanViewRight => app.pan_view(1, 0),
                        Action::ZoomIn => app.zoom(2),
                        Action::ZoomOut => app.zoom(-2),
                        Action::OpenCommandPrompt => { stdout().execute(Show)?.execute(SetCursorStyle::SteadyBlock)?; app.mode = AppMode::Command; app.input_buffer.clear(); app.command_cursor_pos = 0; },
                        Action::OpenColorPicker => {
                            app.selection_before_picker = Some(app.current_selection);
                            app.mode = AppMode::ColorPicker;
                        },
                        Action::OpenToolPicker => {
                            app.selection_before_picker = Some(app.current_selection);
                            app.mode = AppMode::ToolPicker;
                        },
                        Action::IncreasePenSize => app.change_pen_size(1),
                        Action::DecreasePenSize => app.change_pen_size(-1),
                        Action::IncreaseOpacity => app.change_opacity(1.0),
                        Action::DecreaseOpacity => app.change_opacity(-1.0),
                        Action::Undo => app.undo(),
                        Action::Redo => app.redo(),
                        Action::CycleSymmetry => app.cycle_symmetry_mode(),
                        Action::PickColor => app.pick_color_at_cursor(),
                        Action::Fill => app.fill_area(),
                        Action::Erase => app.erase_at_cursor(),
                        Action::Spray => {
                            if !app.is_spraying {
                                app.is_spraying = true;
                                app.save_state_for_undo();
                                app.apply_spray();
                                app.last_apply_time = Some(Local::now());
                            }
                        }


                        Action::SelectLayerUp => app.change_layer_selection(-1),
                        Action::SelectLayerDown => app.change_layer_selection(1),
                        Action::AddLayer => app.add_new_layer(),
                        Action::DeleteLayer => app.delete_active_layer(),
                        Action::ToggleLayerVisibility => app.toggle_layer_visibility(),
                        Action::MoveLayerUp => app.move_layer_up(),
                        Action::MoveLayerDown => app.move_layer_down(),
                        Action::ToggleOnionSkin => {
                            app.onion_skin_enabled = !app.onion_skin_enabled;
                            app.status_message = Some((format!("Onion Skin: {}", if app.onion_skin_enabled { "ON" } else { "OFF" }), Instant::now()));
                        },
                        Action::IncreaseOnionOpacity => {
                            app.onion_skin_opacity = (app.onion_skin_opacity + 0.1).min(1.0);
                            app.status_message = Some((format!("Onion Opacity: {:.0}%", app.onion_skin_opacity * 100.0), Instant::now()));
                        },
                        Action::DecreaseOnionOpacity => {
                            app.onion_skin_opacity = (app.onion_skin_opacity - 0.1).max(0.0);
                            app.status_message = Some((format!("Onion Opacity: {:.0}%", app.onion_skin_opacity * 100.0), Instant::now()));
                        },


                        Action::QuickSelectColorUp => { app.change_palette_selection_2d(0, -1); app.select_color_entry(); },
                        Action::QuickSelectColorDown => { app.change_palette_selection_2d(0, 1); app.select_color_entry(); },
                        Action::QuickSelectColorLeft => { app.change_palette_selection_2d(-1, 0); app.select_color_entry(); },
                        Action::QuickSelectColorRight => { app.change_palette_selection_2d(1, 0); app.select_color_entry(); },
                        Action::QuickSelectToolLeft => { app.change_tool_selection(-1); app.select_tool_entry(); },
                        Action::QuickSelectToolRight => { app.change_tool_selection(1); app.select_tool_entry(); },
                        Action::AdjustSymmetryNegative => match &mut app.symmetry_mode {
                            crate::SymmetryMode::Vertical(x) => *x = x.saturating_sub(1),
                            crate::SymmetryMode::Horizontal(y) => *y = y.saturating_add(1).min(app.canvas_height.saturating_sub(1) as u16),
                            crate::SymmetryMode::DiagonalForward(c) => *c -= 1,
                            crate::SymmetryMode::DiagonalBackward(c) => *c -= 1,
                            _ => {}
                        },
                        Action::AdjustSymmetryPositive => match &mut app.symmetry_mode {
                            crate::SymmetryMode::Vertical(x) => *x = x.saturating_add(1).min(app.canvas_width.saturating_sub(1) as u16),
                            crate::SymmetryMode::Horizontal(y) => *y = y.saturating_sub(1),
                            crate::SymmetryMode::DiagonalForward(c) => *c += 1,
                            crate::SymmetryMode::DiagonalBackward(c) => *c += 1,
                            _ => {}
                        },
                            Action::Draw => {
                                if !app.is_space_held {
                                    app.is_space_held = true;
                                    if app.protect_stroke {
                                        app.drawn_pixels_in_stroke.clear();
                                    }
                                    app.use_current_tool();
                                    app.last_apply_time = Some(Local::now());
                                }
                            },
                
                        Action::Quit => app.quit(),
                    }
                }
            }
        },
        AppMode::Keybindings => match key.code {
            KeyCode::Esc => {
                if app.keybinding_change_has_occured {
                    app.mode = AppMode::ConfirmKeybindingSave;
                } else {
                    app.mode = AppMode::Drawing;
                }
            },
            KeyCode::Up => {
                app.keybindings_selection_index = app.keybindings_selection_index.saturating_sub(1);
                // Adjust scroll if selection goes above the current view
                if app.keybindings_selection_index < app.keybindings_scroll_state as usize {
                    app.keybindings_scroll_state = app.keybindings_selection_index as u16;
                }
            },
            KeyCode::Down => {
                let total_actions = Action::iter().count();
                if app.keybindings_selection_index < total_actions - 1 {
                    app.keybindings_selection_index += 1;
                    // Adjust scroll if selection goes below the current view
                    // This assumes a certain height, but works well enough. A more robust solution
                    // would need the view height from the UI function.
                    if app.keybindings_selection_index > app.keybindings_scroll_state as usize + 15 {
                         app.keybindings_scroll_state = (app.keybindings_selection_index - 15) as u16;
                    }
                }
            },
            KeyCode::Enter => app.is_changing_keybinding = true,
            _ => {}
        },


        AppMode::ConfigEditor => {
            let setting = ConfigSetting::iter().nth(app.config_selection_index).unwrap();
            let total_settings = ConfigSetting::iter().count();

            match key.code {
                KeyCode::Esc => {
                    if app.config_change_has_occured { app.mode = AppMode::ConfirmConfigSave; }
                    else { app.mode = AppMode::Drawing; }
                },
                KeyCode::Up => app.config_selection_index = app.config_selection_index.saturating_sub(1),
                KeyCode::Down => if app.config_selection_index < total_settings - 1 { app.config_selection_index += 1; },
                KeyCode::Left => {
                    setting.decrement_value(app);
                    app.config_change_has_occured = true;
                },
                KeyCode::Right => {
                    setting.increment_value(app);
                    app.config_change_has_occured = true;
                },
                _ => {}
            }
        },
        AppMode::ConfirmConfigSave => match key.code {
            KeyCode::Left | KeyCode::Right => app.confirm_selection_yes = !app.confirm_selection_yes,
            KeyCode::Enter => {
                if app.confirm_selection_yes {
                    app.save_current_config();
                } else {
                   if let Ok(path) = crate::utils::get_config_path() {
                        if let Ok(json) = std::fs::read_to_string(path) {
                            if let Ok(cfg) = serde_json::from_str::<Config>(&json) { app.apply_config(&cfg); }
                        }
                    }
                }
                app.config_change_has_occured = false;
                app.mode = AppMode::Drawing;
            },
            KeyCode::Esc => app.mode = AppMode::ConfigEditor,
            _ => {}
        },


AppMode::ScriptEditor => {
    // Handle Ctrl shortcuts first, as they don't involve text manipulation
    if key.modifiers == crossterm::event::KeyModifiers::CONTROL {
        match key.code {
            KeyCode::Char('s') => {
                crate::script_handler::save_script(app);
            }
            KeyCode::Char('n') => {
                crate::script_handler::clear_script(app);
                app.script_change_has_occured = true;
            }
            _ => {} // Other Ctrl combinations do nothing
        }
        return Ok(()); // Event handled, no further processing needed
    }
    
    // --- Standard Text Editing Logic ---
    app.script_change_has_occured = true; // Assume change on any other key press
    let line_count = app.script_content_lines.len();
    let current_line_len = app.script_content_lines[app.script_cursor_line].graphemes(true).count();

    match key.code {
        KeyCode::Esc => {
            if app.script_change_has_occured {
                app.mode = AppMode::ConfirmScriptSave;
            } else {
                app.mode = AppMode::Drawing;
            }
        },
        KeyCode::Up => {
            app.script_cursor_line = app.script_cursor_line.saturating_sub(1);
            let new_line_len = app.script_content_lines[app.script_cursor_line].graphemes(true).count();
            app.script_cursor_char_pos = app.script_cursor_char_pos.min(new_line_len);
        },
        KeyCode::Down => {
            if app.script_cursor_line < line_count - 1 {
                app.script_cursor_line += 1;
                let new_line_len = app.script_content_lines[app.script_cursor_line].graphemes(true).count();
                app.script_cursor_char_pos = app.script_cursor_char_pos.min(new_line_len);
            }
        },
        KeyCode::Left => app.script_cursor_char_pos = app.script_cursor_char_pos.saturating_sub(1),
        KeyCode::Right => app.script_cursor_char_pos = (app.script_cursor_char_pos + 1).min(current_line_len),
        KeyCode::Char(c) => {
            let line = &mut app.script_content_lines[app.script_cursor_line];
            let byte_index = line.grapheme_indices(true).nth(app.script_cursor_char_pos).map_or(line.len(), |(i, _)| i);
            line.insert(byte_index, c);
            app.script_cursor_char_pos += 1;
        },
        KeyCode::Backspace => {
            if app.script_cursor_char_pos > 0 {
                let line = &mut app.script_content_lines[app.script_cursor_line];
                let byte_index_end = line.grapheme_indices(true).nth(app.script_cursor_char_pos).map_or(line.len(), |(i, _)| i);
                let byte_index_start = line.grapheme_indices(true).nth(app.script_cursor_char_pos - 1).map_or(0, |(i, _)| i);
                line.replace_range(byte_index_start..byte_index_end, "");
                app.script_cursor_char_pos -= 1;
            } else if app.script_cursor_line > 0 {
                let current_line = app.script_content_lines.remove(app.script_cursor_line);
                app.script_cursor_line -= 1;
                let prev_line = &mut app.script_content_lines[app.script_cursor_line];
                app.script_cursor_char_pos = prev_line.graphemes(true).count();
                prev_line.push_str(&current_line);
            }
        },
        KeyCode::Enter => {
            let line = &mut app.script_content_lines[app.script_cursor_line];
            let byte_index = line.grapheme_indices(true).nth(app.script_cursor_char_pos).map_or(line.len(), |(i, _)| i);
            let new_line_content = line.split_off(byte_index);
            app.script_cursor_line += 1;
            app.script_content_lines.insert(app.script_cursor_line, new_line_content);
            app.script_cursor_char_pos = 0;
        },
        KeyCode::Tab => { // Insert 2 spaces
            let line = &mut app.script_content_lines[app.script_cursor_line];
            let byte_index = line.grapheme_indices(true).nth(app.script_cursor_char_pos).map_or(line.len(), |(i, _)| i);
            line.insert_str(byte_index, "  ");
            app.script_cursor_char_pos += 2;
        },
        _ => app.script_change_has_occured = false, // No change for unhandled keys
    }
    // Keep cursor in view (scrolling logic)
    if let Some(area) = app.last_pixel_area {
         let view_height = area.height.saturating_sub(2) as usize; // A reasonable guess for editor height
         if app.script_cursor_line < app.script_scroll_state as usize {
            app.script_scroll_state = app.script_cursor_line as u16;
         } else if app.script_cursor_line >= app.script_scroll_state as usize + view_height {
            app.script_scroll_state = (app.script_cursor_line - view_height + 1) as u16;
         }
    }
},

        AppMode::ConfirmScriptSave => match key.code {
            KeyCode::Left | KeyCode::Right => app.confirm_selection_yes = !app.confirm_selection_yes,
            KeyCode::Enter => {
                if app.confirm_selection_yes {
                    crate::script_handler::save_script(app);
                }
                app.mode = AppMode::Drawing;
            },
            KeyCode::Esc => app.mode = AppMode::ScriptEditor,
            _ => {}
        },
        AppMode::ConfirmKeybindingSave => match key.code {
            KeyCode::Left | KeyCode::Right => app.confirm_selection_yes = !app.confirm_selection_yes,
            KeyCode::Enter => {
                if app.confirm_selection_yes {
                    app.keybindings.save().unwrap_or_default();
                    app.status_message = Some(("Keybindings saved.".to_string(), Instant::now()));
                } else {
                    app.keybindings = Keybindings::load();
                    app.status_message = Some(("Keybinding changes discarded.".to_string(), Instant::now()));
                }
                app.keybinding_change_has_occured = false;
                app.mode = AppMode::Drawing;
            },
            KeyCode::Esc => app.mode = AppMode::Keybindings,
            _ => {}
        },

        AppMode::ConfirmOverwrite => match key.code {
            KeyCode::Char('y') => {
                if let Some(path) = app.pending_save_path.take() {
                    app.save_project(&path, true);
                }
                app.mode = AppMode::Drawing;
            }
            KeyCode::Char('n') | KeyCode::Esc => {
                app.pending_save_path = None;
                app.status_message = Some(("Save cancelled.".to_string(), Instant::now()));
                app.mode = AppMode::Drawing;
            }
            _ => {}
        },

        AppMode::HelpScreen => match key.code {
            KeyCode::Esc => app.mode = AppMode::Drawing,
            KeyCode::Up => app.help_scroll = app.help_scroll.saturating_sub(1),
            KeyCode::Down => app.help_scroll += 1,
            _ => {}
        },

    AppMode::Command => {
            match key.code {
                KeyCode::Enter => {
                    stdout().execute(Hide)?;
                    let command_to_run = app.input_buffer.trim().to_string();
                    if !command_to_run.is_empty() && app.command_history.get(0) != Some(&command_to_run) {
                        app.command_history.insert(0, command_to_run.clone());
                    }
                    app.mode = AppMode::Drawing;
                    app.input_buffer.clear();
                    app.command_cursor_pos = 0;
                    app.suggestion_index = 0;
                    app.history_index = 0;
                    app.suggestion_active = false;
                    execute_command(app, &command_to_run);
                },
                KeyCode::Char(c) => {
                    app.input_buffer.insert(app.command_cursor_pos, c);
                    app.command_cursor_pos += c.len_utf8();
                    app.suggestion_index = 0;
                    app.suggestion_active = false; // NEW: Typing deactivates suggestion selection
                    app.history_index = 0;
                },
                KeyCode::Backspace => {
                    if app.command_cursor_pos > 0 {
                        let current_pos = app.command_cursor_pos;
                        let prev_pos = app.input_buffer[..current_pos].grapheme_indices(true).last().map_or(0, |(i, _)| i);
                        app.input_buffer.drain(prev_pos..current_pos);
                        app.command_cursor_pos = prev_pos;
                        app.suggestion_active = false; // This deactivates selection mode
                        app.suggestion_index = 0;      // This resets the selection to the top
                        app.history_index = 0;
                    }
                },
                KeyCode::Left => {
                    let current_pos = app.command_cursor_pos;
                    if let Some((prev_pos, _)) = app.input_buffer.grapheme_indices(true).rev().find(|(i, _)| *i < current_pos) {
                        app.command_cursor_pos = prev_pos;
                    }
                },
                KeyCode::Right => {
                    let current_pos = app.command_cursor_pos;
                    if let Some((grapheme_pos, grapheme)) = app.input_buffer.grapheme_indices(true).find(|(i, _)| *i == current_pos) {
                        app.command_cursor_pos = grapheme_pos + grapheme.len();
                    }
                },
                KeyCode::Up => {
                    let suggestions = app.get_suggestions(&app.input_buffer);
                    if !suggestions.is_empty() {
                        app.suggestion_active = true;
                        let new_index = if app.suggestion_index == 0 { suggestions.len() - 1 } else { app.suggestion_index - 1 };
                        app.suggestion_index = new_index;
                    } else {
                        if app.history_index == 0 { app.command_input_before_history = app.input_buffer.clone(); }
                        if app.history_index < app.command_history.len() {
                            app.history_index += 1;
                            app.input_buffer = app.command_history[app.history_index - 1].clone();
                            app.command_cursor_pos = app.input_buffer.len();
                        }
                    }
                },
                KeyCode::Down => {
                    let suggestions = app.get_suggestions(&app.input_buffer);
                    if !suggestions.is_empty() {
                        app.suggestion_active = true;
                        app.suggestion_index = (app.suggestion_index + 1) % suggestions.len();
                    } else {
                        if app.history_index > 1 {
                            app.history_index -= 1;
                            app.input_buffer = app.command_history[app.history_index - 1].clone();
                            app.command_cursor_pos = app.input_buffer.len();
                        } else if app.history_index == 1 {
                            app.history_index = 0;
                            app.input_buffer = app.command_input_before_history.clone();
                            app.command_cursor_pos = app.input_buffer.len();
                        }
                    }
                },
                KeyCode::Tab => {
                    let suggestions = app.get_suggestions(&app.input_buffer);
                    if app.suggestion_active && !suggestions.is_empty() {
                        let selected_suggestion = &suggestions[app.suggestion_index];
                        
                        let new_input = if app.input_buffer.starts_with("load ") {
                            format!("load {}", selected_suggestion)
                        } else if app.input_buffer.starts_with("colorpalette:") {
                            format!("colorpalette:{}", selected_suggestion)
                        } else {
                            selected_suggestion.clone()
                        };
                        app.input_buffer = new_input;
                        app.command_cursor_pos = app.input_buffer.len();
                        app.suggestion_active = false;
                        app.suggestion_index = 0;
                    }
                },
                KeyCode::Esc => {
                    stdout().execute(Hide)?;
                    app.mode = AppMode::Drawing;
                    app.input_buffer.clear();
                    app.command_cursor_pos = 0;
                    app.suggestion_index = 0;
                    app.history_index = 0;
                    app.suggestion_active = false;
                },
                _ => {}
            }
        },

    AppMode::ColorPicker => {
        let current_keybinding = Keybinding { code: key.code, modifiers: key.modifiers };
        if Some(&current_keybinding) == app.keybindings.map.get(&Action::OpenColorPicker) {
            app.current_selection = app.color_palette[app.palette_index];
            app.mode = AppMode::Drawing;
        } else {
            match key.code {
                KeyCode::Esc => {
                    if let Some(old_selection) = app.selection_before_picker {
                        // Find the old selection in the palettes to reset the index
                        if let Some(index) = app.color_palette.iter().position(|&e| e == old_selection) {
                            app.palette_index = index;
                        }
                        app.current_selection = old_selection;
                    }
                    app.mode = AppMode::Drawing;
                },
                KeyCode::Up => app.change_palette_selection_2d(0, -1),
                KeyCode::Down => app.change_palette_selection_2d(0, 1),
                KeyCode::Left => app.change_palette_selection_2d(-1, 0),
                KeyCode::Right => app.change_palette_selection_2d(1, 0),
                KeyCode::Enter => app.select_color_entry(),
                _ => {}
            }
        }
    },

    AppMode::ToolPicker => {
        let current_keybinding = Keybinding { code: key.code, modifiers: key.modifiers };
        if Some(&current_keybinding) == app.keybindings.map.get(&Action::OpenToolPicker) {
            app.current_selection = app.tool_palette[app.tool_index];
            app.mode = AppMode::Drawing;
        } else {
            match key.code {
                KeyCode::Esc => {
                    if let Some(old_selection) = app.selection_before_picker {
                        // Find the old selection in the palettes to reset the index
                        if let Some(index) = app.tool_palette.iter().position(|&e| e == old_selection) {
                            app.tool_index = index;
                        }
                        app.current_selection = old_selection;
                    }
                    app.mode = AppMode::Drawing;
                },
                KeyCode::Left => app.change_tool_selection(-1),
                KeyCode::Right => app.change_tool_selection(1),
                KeyCode::Enter => app.select_tool_entry(),
                _ => {}
            }
        }
    },

    AppMode::ResizingWidth | AppMode::ResizingHeight => {
        match key.code {
            KeyCode::Enter => match app.mode {
                AppMode::ResizingWidth => { if let Ok(width) = app.input_buffer.parse::<usize>() { if width > 0 { app.temp_width = width; app.mode = AppMode::ResizingHeight; app.input_buffer.clear(); } } },
                AppMode::ResizingHeight => { if let Ok(height) = app.input_buffer.parse::<usize>() { if height > 0 { app.resize_canvas(app.temp_width, height); app.mode = AppMode::Drawing; } } },
                _ => {}
            },
            KeyCode::Esc => app.mode = AppMode::Drawing,
            KeyCode::Char(c) if c.is_ascii_digit() => app.input_buffer.push(c),
            KeyCode::Backspace => { app.input_buffer.pop(); },
            _ => {}
        }
    }



    }
    Ok(())
}