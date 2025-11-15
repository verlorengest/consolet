use crate::{App, AppMode, BrowserFocus};
use crossterm::event::{KeyCode, KeyEvent, MouseButton, MouseEvent, MouseEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::path::{Path, PathBuf};


// Defines the reason the browser was opened, to determine its behavior.
#[derive(Clone, Copy, Debug)]
pub enum BrowserMode {
    Load,
    Save,
    ImportPalette,
    Export,
    GeneratePaletteFromImage(bool),
}

// Entry point to open the browser.
pub fn open_browser(app: &mut App, mode: BrowserMode) {
    app.browser_mode = Some(mode);
    app.mode = AppMode::FileBrowser;
    app.browser_error = None;
    app.browser_focus = BrowserFocus::List;

    if matches!(mode, BrowserMode::Save | BrowserMode::Export) {
        let timestamp = chrono::Local::now().format("%Y-%m-%d_%H%M%S");
        app.browser_input_buffer = format!("project_{}", timestamp);
    } else {
        app.browser_input_buffer.clear();
    }

    let initial_path = std::env::current_dir()
        .and_then(std::fs::canonicalize)
        .unwrap_or_else(|_| PathBuf::from("/"));
    app.browser_history_back.clear();
    app.browser_history_forward.clear();

    read_directory(app, &initial_path);
}

// Reads the contents of a directory into the app's state.
fn read_directory(app: &mut App, path: &Path) {
    app.browser_current_dir = std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());

    app.browser_entries.clear();
    app.browser_list_state.select(Some(0));

    // Add ".." to go up a directory, if possible.
    if let Some(parent) = path.parent() {
        if parent != path {
            app.browser_entries.push(PathBuf::from(".."));
        }
    }

    match std::fs::read_dir(path) {
        Ok(entries) => {
            let mut dirs = Vec::new();
            let mut files = Vec::new();
            for entry in entries.filter_map(Result::ok) {
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    dirs.push(entry_path);
                } else {
                    files.push(entry_path);
                }
            }
            dirs.sort();
            files.sort();
            app.browser_entries.extend(dirs);
            app.browser_entries.extend(files);
        }
        Err(e) => {
            app.browser_error = Some(format!("Cannot read directory: {}", e));
        }
    }
}

// Handles all user input (keyboard and mouse) while the browser is active.
pub fn handle_browser_input(app: &mut App, key_event: Option<KeyEvent>, mouse_event: Option<MouseEvent>) {
    if let Some(key) = key_event {
        handle_browser_keyboard(app, key);
    }
    if let Some(mouse) = mouse_event {
        handle_browser_mouse(app, mouse);
    }
}

// Renders the file browser UI.
pub fn draw_browser(f: &mut Frame, app: &mut App) {
    let is_export_mode = matches!(app.browser_mode, Some(BrowserMode::Export));
    let is_save_or_export = is_export_mode || matches!(app.browser_mode, Some(BrowserMode::Save));

    // --- Layout ---
    let constraints = if is_save_or_export {
        vec![Constraint::Min(1), Constraint::Length(3), Constraint::Length(3)]
    } else {
        vec![Constraint::Min(1), Constraint::Length(3)]
    };
    let main_chunks = Layout::default().direction(Direction::Vertical).constraints(constraints).split(f.size());

    // --- List Rendering ---
    let list_chunk = main_chunks[0];
    let list_border_style = if app.browser_focus == BrowserFocus::List { Style::default().fg(Color::Yellow) } else { Style::default() };
    let items: Vec<ListItem> = app.browser_entries.iter().map(|path| {
        let name = if path.to_str() == Some("..") { "📁 ..".to_string() }
        else if path.is_dir() { format!("📁 {}", path.file_name().unwrap_or_default().to_string_lossy()) }
        else { format!("📄 {}", path.file_name().unwrap_or_default().to_string_lossy()) };
        ListItem::new(name)
    }).collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!(" 📂 {} ", app.browser_current_dir.display())).border_style(list_border_style))
        .highlight_style(Style::default().bg(Color::Blue).add_modifier(Modifier::BOLD))
        .highlight_symbol("▶ ");
    f.render_stateful_widget(list, list_chunk, &mut app.browser_list_state);

    // --- Input Boxes ---
    if is_save_or_export {
        let input_area = main_chunks[1];
        let input_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Min(10), Constraint::Length(10)]) // Flexible Name, Fixed Scale
            .split(input_area);
        
        let name_chunk = input_chunks[0];
        let scale_chunk = input_chunks[1];

        // RENDER FILENAME INPUT
        let name_border_style = if app.browser_focus == BrowserFocus::NameInput { Style::default().fg(Color::Yellow) } else { Style::default() };
        let name_input = Paragraph::new(app.browser_input_buffer.as_str())
            .block(Block::default().borders(Borders::ALL).title(" Filename ").border_style(name_border_style));
        f.render_widget(name_input, name_chunk);

        // RENDER SCALE INPUT (ONLY IN EXPORT MODE)
        if is_export_mode {
            let scale_border_style = if app.browser_focus == BrowserFocus::ScaleInput { Style::default().fg(Color::Yellow) } else { Style::default() };
            let scale_input = Paragraph::new(app.browser_scale_buffer.as_str())
                .block(Block::default().borders(Borders::ALL).title(" Scale ").border_style(scale_border_style));
            f.render_widget(scale_input, scale_chunk);
        }

        // --- Set Cursor ---
        match app.browser_focus {
            BrowserFocus::NameInput => f.set_cursor(name_chunk.x + app.browser_input_buffer.len() as u16 + 1, name_chunk.y + 1),
            BrowserFocus::ScaleInput if is_export_mode => {
                f.set_cursor(scale_chunk.x + app.browser_scale_buffer.len() as u16 + 1, scale_chunk.y + 1);
            }
            _ => {}
        }
    }
    
    // --- Help Text ---
    let help_chunk = *main_chunks.last().unwrap();
    let help = Paragraph::new("Tab: Cycle Focus | ↑/↓: Navigate | Enter: Select | Ctrl+S: Save Here | Esc: Cancel")
        .block(Block::default().borders(Borders::ALL).title(" Help "));
    f.render_widget(help, help_chunk);
}

// --- Internal Helper Functions ---

fn handle_browser_keyboard(app: &mut App, key: KeyEvent) {
    let is_save_or_export = matches!(app.browser_mode, Some(BrowserMode::Save | BrowserMode::Export));
    let is_export_mode = matches!(app.browser_mode, Some(BrowserMode::Export));

    // --- Tab Focus Cycling ---
    if key.code == KeyCode::Tab {
        if is_export_mode { // Cycle through all three: List -> Name -> Scale -> List
            app.browser_focus = match app.browser_focus {
                BrowserFocus::List => BrowserFocus::NameInput,
                BrowserFocus::NameInput => BrowserFocus::ScaleInput,
                BrowserFocus::ScaleInput => BrowserFocus::List,
            };
        } else if is_save_or_export { // Cycle through two: List -> Name -> List
            app.browser_focus = match app.browser_focus {
                BrowserFocus::List => BrowserFocus::NameInput,
                BrowserFocus::NameInput => BrowserFocus::List,
                BrowserFocus::ScaleInput => BrowserFocus::List, // Should not happen, but handle it
            };
        }
        return;
    }

    // --- Delegate Input Based on Focus ---
    match app.browser_focus {
        BrowserFocus::List => handle_list_input(app, key),
        BrowserFocus::NameInput => handle_name_input(app, key),
        BrowserFocus::ScaleInput if is_export_mode => handle_scale_input(app, key),
        _ => handle_list_input(app, key), // Default to list input if something is out of sync
    }
}

fn handle_browser_mouse(app: &mut App, mouse: MouseEvent) {
    if let Some(area) = app.last_pixel_area { // Reuse last_pixel_area as the browser's main rect
        match mouse.kind {
            MouseEventKind::ScrollUp => navigate_list(app, -1),
            MouseEventKind::ScrollDown => navigate_list(app, 1),
            MouseEventKind::Down(MouseButton::Left) => {
                if mouse.row >= area.y && mouse.row < area.bottom() {
                    let index = (mouse.row - area.y) as usize + app.browser_list_state.offset();
                    if index < app.browser_entries.len() {
                        app.browser_list_state.select(Some(index));
                        on_select(app);
                    }
                }
            }
            _ => {}
        }
    }
}

fn navigate_list(app: &mut App, delta: i32) {
    let current = app.browser_list_state.selected().unwrap_or(0);
    let next = (current as i32 + delta).max(0) as usize;
    if next < app.browser_entries.len() {
        app.browser_list_state.select(Some(next));
    }
}

fn on_select(app: &mut App) {
    if let Some(index) = app.browser_list_state.selected() {
        let selected_path = app.browser_entries[index].clone();

        // --- CORRECTED LOGIC ---
        // Handle "go up" case separately and reliably
        if selected_path.to_str() == Some("..") {
            if let Some(parent) = app.browser_current_dir.parent() {
                let parent_path = parent.to_path_buf(); // Clone the path to release the borrow
                app.browser_history_back.push(app.browser_current_dir.clone());
                app.browser_history_forward.clear();
                read_directory(app, &parent_path); // Use the cloned path
            }
            return;
        }

        if selected_path.is_dir() {
            app.browser_history_back.push(app.browser_current_dir.clone());
            app.browser_history_forward.clear();
            read_directory(app, &selected_path);
        } else {
            // It's a file, handle based on mode
            match app.browser_mode {
                Some(BrowserMode::Load) => app.load_project(&selected_path),
                Some(BrowserMode::ImportPalette) => app.load_and_store_palette(&selected_path.to_string_lossy()),
                Some(BrowserMode::GeneratePaletteFromImage(add)) => app.generate_palette_from_image(&selected_path, add),

                _ => return, // In Save/Export mode, selecting a file does nothing.
            }
            app.mode = AppMode::Drawing; // Close browser on successful action
        }
    }
}
fn go_back(app: &mut App) {
    if let Some(path) = app.browser_history_back.pop() {
        // Push the place we are leaving to the forward history.
        app.browser_history_forward.push(app.browser_current_dir.clone());
        read_directory(app, &path);
    }
}


fn go_forward(app: &mut App) {
    if let Some(path) = app.browser_history_forward.pop() {
        // Push the place we are leaving to the back history.
        app.browser_history_back.push(app.browser_current_dir.clone());
        read_directory(app, &path);
    }
}

fn on_confirm_directory(app: &mut App) {
    let mode = match app.browser_mode {
        Some(m) => m,
        None => return,
    };

    let mut filename = app.browser_input_buffer.clone();


    match mode {
        BrowserMode::Save => {
            if !filename.ends_with(".consolet") {
                filename.push_str(".consolet");
            }
            app.save_project(&app.browser_current_dir.join(filename), true);
        },
        BrowserMode::Export => {
            if !filename.ends_with(".png") {
                filename.push_str(".png");
            }
            let scale = app.browser_scale_buffer.parse::<u32>().unwrap_or(1);
app.export_to_png(Some(app.browser_current_dir.join(filename).to_string_lossy().to_string()), scale, true);
        },
        _ => return,
    }
    app.mode = AppMode::Drawing;
}


fn handle_list_input(app: &mut App, key: KeyEvent) {
    use crossterm::event::KeyModifiers;
    match key.code {
        KeyCode::Esc => app.mode = AppMode::Drawing,
        KeyCode::Up => navigate_list(app, -1),
        KeyCode::Down => navigate_list(app, 1),
        KeyCode::Enter => on_select(app),
        KeyCode::Backspace => go_back(app),
        KeyCode::Left if key.modifiers == KeyModifiers::ALT => go_back(app),
        KeyCode::Right if key.modifiers == KeyModifiers::ALT => go_forward(app),
        KeyCode::Char('s') if key.modifiers == KeyModifiers::CONTROL => on_confirm_directory(app),
        _ => {}
    }
}

fn handle_name_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char(c) => app.browser_input_buffer.push(c),
        KeyCode::Backspace => { app.browser_input_buffer.pop(); },
        KeyCode::Enter => on_confirm_directory(app),
        KeyCode::Esc => app.mode = AppMode::Drawing,
        // If another key is pressed, pass it to the main handler to allow focus change
        _ => handle_list_input(app, key),
    }
}

fn handle_scale_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char(c) if c.is_ascii_digit() => app.browser_scale_buffer.push(c),
        KeyCode::Backspace => { app.browser_scale_buffer.pop(); },
        KeyCode::Enter => on_confirm_directory(app),
        KeyCode::Esc => app.mode = AppMode::Drawing,
        // If another key is pressed, pass it to the main handler
        _ => handle_list_input(app, key),
    }
}