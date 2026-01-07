use crate::{Block, Borders, Clear, PaletteFile, Paragraph, SerializableColor, palette, stdout};

use ratatui::prelude::*;
use std::io::Result;
use std::path::PathBuf;

#[cfg(not(windows))]
use crossterm::event::{Event, KeyCode};
#[cfg(not(windows))]
use ratatui::{Terminal, backend::CrosstermBackend};
#[cfg(not(windows))]
use crossterm::{
    event,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};


pub fn get_or_create_app_dir() -> Result<PathBuf> {
    let proj_dirs = dirs::data_local_dir().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Could not find local data directory"))?;
    let app_dir = proj_dirs.join("consolet");
    let projects_dir = app_dir.join("saved_projects");
    let palettes_dir = app_dir.join("palettes");
    std::fs::create_dir_all(&projects_dir)?;
    std::fs::create_dir_all(&palettes_dir)?;
    Ok(app_dir)
}

pub fn blend_colors(c1: Color, c2: Color, factor: f32) -> Color {
    let (r1, g1, b1) = to_rgb(c1);
    let (r2, g2, b2) = to_rgb(c2);
    let r = (r1 as f32 * (1.0 - factor) + r2 as f32 * factor).round() as u8;
    let g = (g1 as f32 * (1.0 - factor) + g2 as f32 * factor).round() as u8;
    let b = (b1 as f32 * (1.0 - factor) + b2 as f32 * factor).round() as u8;
    Color::Rgb(r, g, b)
}

pub fn to_rgb(c: Color) -> (u8, u8, u8) {
    match c {
        Color::Rgb(r, g, b) => (r, g, b),
        Color::Black => (0, 0, 0), Color::Red => (255, 0, 0), Color::Green => (0, 255, 0),
        Color::Yellow => (255, 255, 0), Color::Blue => (0, 0, 255), Color::Magenta => (255, 0, 255),
        Color::Cyan => (0, 255, 255), Color::Gray => (128, 128, 128), Color::DarkGray => (64, 64, 64),
        Color::LightRed => (255, 128, 128), Color::LightGreen => (128, 255, 128), Color::LightYellow => (255, 255, 128),
        Color::LightBlue => (128, 128, 255), Color::LightMagenta => (255, 128, 255), Color::LightCyan => (128, 255, 255),
        Color::White => (255, 255, 255),
        _ => (0, 0, 0),
    }
}

pub fn export_default_palettes_if_missing() -> std::io::Result<()> {
    let palettes_dir = get_or_create_app_dir()?.join("palettes");
    for (name, generator) in palette::get_built_in_palettes() {
        let palette_path = palettes_dir.join(format!("{}.consolet", name));
        if !palette_path.exists() {
            let entries = generator();
            let serializable_colors: Vec<SerializableColor> = entries
                .into_iter()
                .filter_map(|entry| match entry {
                    crate::palette::PaletteEntry::Color(c) => Some(c.into()),
                    _ => None,
                })
                .collect();
            let palette_file = PaletteFile(serializable_colors);
            if let Ok(json_data) = serde_json::to_string_pretty(&palette_file) {
                let _ = std::fs::write(palette_path, json_data);
            }
        }
    }
    Ok(())
}

#[cfg(not(windows))]
fn draw_compatibility_dialog(frame: &mut Frame, selection_yes: bool) {
    let message = "Warning: Your terminal may not fully support 24-bit color,\nwhich can lead to a degraded experience.\n\nContinue anyway?";
    let area = centered_rect(50, 30, frame.size());
    let block = Block::default().title(" Terminal Compatibility ").borders(Borders::ALL);
    let inner_area = block.inner(area);
    frame.render_widget(Clear, area);
    frame.render_widget(block, area);
    let text = Paragraph::new(message).alignment(Alignment::Center);
    let yes_style = if selection_yes { Style::default().reversed() } else { Style::default() };
    let no_style = if !selection_yes { Style::default().reversed() } else { Style::default() };
    let buttons = text::Line::from(vec![
        text::Span::styled(" Yes ", yes_style), text::Span::raw(" / "), text::Span::styled(" No ", no_style),
    ]).alignment(Alignment::Center);
    let layout = Layout::default().direction(Direction::Vertical).constraints([Constraint::Min(1), Constraint::Length(1)]).split(inner_area);
    frame.render_widget(text, layout[0]);
    frame.render_widget(buttons, layout[1]);
}

#[cfg(not(windows))]
pub fn check_terminal_support() -> Result<bool> {
    if let Ok(val) = std::env::var("COLORTERM") {
        if val.to_lowercase() == "truecolor" || val.to_lowercase() == "24bit" {
            return Ok(true);
        }
    }
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut continue_app = true;
    loop {
        terminal.draw(|frame| draw_compatibility_dialog(frame, continue_app))?;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => break,
                        KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                            continue_app = false;
                            break;
                        }
                        KeyCode::Left | KeyCode::Right => continue_app = !continue_app,
                        KeyCode::Enter => break,
                        _ => {}
                    }
                }
            }
        }
    }
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(continue_app)
}

#[cfg(windows)]
pub fn check_terminal_support() -> Result<bool> {
    Ok(true)
}

pub fn get_help_sheet_path() -> Result<PathBuf> {
    let app_dir = get_or_create_app_dir()?;
    Ok(app_dir.join("help_sheet.txt"))
}

pub fn get_config_path() -> Result<PathBuf> {
    let app_dir = get_or_create_app_dir()?;
    Ok(app_dir.join("config.consolet"))
}

pub fn format_keybinding(kb: &crate::keybindings::Keybinding) -> String {
    let mut parts = vec![];
    if kb.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) { parts.push("Ctrl"); }
    if kb.modifiers.contains(crossterm::event::KeyModifiers::ALT) { parts.push("Alt"); }
    if kb.modifiers.contains(crossterm::event::KeyModifiers::SHIFT) { parts.push("Shift"); }
    let key_str = match kb.code {
        crossterm::event::KeyCode::Char(c) => c.to_string(),
        _ => format!("{:?}", kb.code),
    };
    parts.push(&key_str);
    parts.join(" + ")
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
