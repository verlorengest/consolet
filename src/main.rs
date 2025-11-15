// main.rs

use chrono::Local;
use crossterm::{
    event,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

#[cfg(not(windows))]
use crossterm::event::{Event, KeyCode};
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::fs::File;
use std::io::{Write, Read};
use image::{Rgba, RgbaImage};
mod palette;
mod commands;
mod keybindings;
mod controller;
mod config;
mod script_handler;
mod help_sheet;
mod utils;
mod file_browser;
use file_browser::BrowserMode;



use palette::{get_default_color_palette, get_default_tool_palette, PaletteEntry, Tool};
use commands::COMMANDS;
use commands::CommandType;
use ratatui::{
    prelude::*,
    text::{Line, Span},
    widgets::{block::Title, Block, Borders, Clear, ListState, Paragraph},
};
use std::io::{stdout, Result};
use std::time::Instant;
use std::path::PathBuf;
use std::collections::VecDeque;
use keybindings::{Action, Keybindings};
use strum::IntoEnumIterator;
use unicode_segmentation::UnicodeSegmentation;
use rand::Rng;


const PIXEL_WIDTH: u16 = 2;

const DEFAULT_SHADE_FACTOR: f32 = 0.03;


use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub struct SerializableColor(u8, u8, u8);

impl From<Color> for SerializableColor {
    fn from(color: Color) -> Self {
        let (r, g, b) = utils::to_rgb(color);
        SerializableColor(r, g, b)
    }
}

impl From<SerializableColor> for Color {
    fn from(sc: SerializableColor) -> Self {
        Color::Rgb(sc.0, sc.1, sc.2)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
struct Pixel {
    color: SerializableColor,
    alpha: f32,
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel {
            color: Color::Reset.into(),
            alpha: 0.0,
        }
    }
}


#[derive(Serialize, Deserialize)]
struct ProjectFile {
    width: usize,
    height: usize,
    canvas: Vec<Vec<Pixel>>,
    palette: Vec<SerializableColor>,
    layers: Option<Vec<Layer>>,
    active_layer_index: Option<usize>,
}


#[derive(Serialize, Deserialize)]
pub struct PaletteFile(Vec<SerializableColor>);

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    pen_size_sensitivity: u16,
    opacity_sensitivity: f32,
    pen_shape: PenShape,
    highlighter_enabled: bool,
    highlighter_value: f32,
    highlighter_mode: HighlighterMode,
    shade_factor: f32,
    protect_stroke: bool,
    apply_color_sec: f32,
    minimap_mode: MinimapMode,
    mouse_events_enabled: bool,
    color_mode: ColorMode,
    default_palette_name: String,
    canvas_scroll_action: CanvasScrollAction,
    spray_size: u16,
    spray_speed: u16,
    spray_intensity: f32,
    snap_to_palette: bool,
    snap_to_palette_mode: SnapToPaletteMode,
    protect_color_transitions: bool,
    palette_menu_position: PaletteMenuPosition,
    onion_skin_enabled: bool,
    onion_skin_opacity: f32,
    export_layer_mode: ExportLayerMode,


}

impl Default for Config {
    fn default() -> Self {
        Self {
            pen_size_sensitivity: 1,
            opacity_sensitivity: 0.05,
            pen_shape: PenShape::Circular,
            highlighter_enabled: true,
            highlighter_value: 0.5,
            highlighter_mode: HighlighterMode::Blend,
            shade_factor: DEFAULT_SHADE_FACTOR,
            protect_stroke: true,
            apply_color_sec: 0.2,
            minimap_mode: MinimapMode::Auto,
            mouse_events_enabled: true,
            color_mode: ColorMode::TrueColor,
            default_palette_name: "default".to_string(),
            canvas_scroll_action: CanvasScrollAction::ChangePenSize,
            spray_size: 5,
            spray_speed: 3,
            spray_intensity: 0.1,
            snap_to_palette: false,
            snap_to_palette_mode: SnapToPaletteMode::ClosestHue,
            protect_color_transitions: false,
            palette_menu_position: PaletteMenuPosition::Left,
            onion_skin_enabled: false,
            onion_skin_opacity: 0.3,
            export_layer_mode: ExportLayerMode::United,

        }
    }
}




#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
enum PenShape { Circular, Square }
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
enum HighlighterMode { Underscore, Blend }
#[derive(Clone, Copy, PartialEq, Debug)]
enum SymmetryMode {
    Off,
    Vertical(u16),
    DiagonalForward(i32),  // Represents y = x + c
    Horizontal(u16),
    DiagonalBackward(i32), // Represents y = -x + c
}
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
enum MinimapMode { Auto, On, Off }

#[derive(PartialEq)]
enum AppMode { Drawing, ColorPicker, ToolPicker, ResizingWidth, ResizingHeight, Command, HelpScreen, ConfirmOverwrite, Keybindings, ConfirmKeybindingSave, ConfigEditor, ConfirmConfigSave, ScriptEditor, ConfirmScriptSave, FileBrowser  }

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
enum ColorMode { TrueColor, Ansi256 }

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
enum SnapToPaletteMode { ClosestRgb, ClosestHue }


#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
enum CanvasScrollAction { ChangePenSize, ChangeOpacity }

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
pub enum PaletteMenuPosition { Left, Right }

impl Serialize for ExportLayerMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            ExportLayerMode::United => serializer.serialize_str("United"),
            ExportLayerMode::Separate => serializer.serialize_str("Separate"),
        }
    }
}

impl<'de> Deserialize<'de> for ExportLayerMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "United" => Ok(ExportLayerMode::United),
            "Separate" => Ok(ExportLayerMode::Separate),
            _ => Ok(ExportLayerMode::United),
        }
    }
}


#[derive(PartialEq)]
enum BrowserFocus {
    List,
    NameInput,
    ScaleInput,
}

#[derive(Clone, Serialize, Deserialize)]
struct Layer {
    name: String,
    canvas: Vec<Vec<Pixel>>,
    visible: bool,
    opacity: f32,
}

#[derive(PartialEq)]
enum LayerFocus {
    List,
    NameInput,
}

#[derive(PartialEq, Clone, Copy)]
enum ExportLayerMode {
    United,
    Separate,
}


struct App {
    canvas: Vec<Vec<Pixel>>,
    canvas_width: usize, canvas_height: usize,
    cursor_pos: (u16, u16),
    current_selection: PaletteEntry,
    color_palette: Vec<PaletteEntry>,
    palette_index: usize,
    tool_palette: Vec<PaletteEntry>,
    tool_index: usize,
    palette_scroll_state: usize,
    mode: AppMode,
    symmetry_mode: SymmetryMode,
    should_quit: bool,
    status_message: Option<(String, Instant)>,
    input_buffer: String,
    temp_width: usize,
    last_pixel_area: Option<Rect>,
    last_palette_area: Option<Rect>,
    last_tool_area: Option<Rect>,
    is_side_panel_visible: bool,
    pen_size: u16,
    opacity: f32,
    pen_size_sensitivity: u16,
    opacity_sensitivity: f32,
    pen_shape: PenShape,
    view_offset_x: i32,
    view_offset_y: i32,
    zoom_level: u16,
    suggestion_index: usize,
    undo_stack: VecDeque<Vec<Vec<Pixel>>>,
    redo_stack: VecDeque<Vec<Vec<Pixel>>>,
    is_mouse_dragging: bool,
    shade_factor: f32,
    highlighter_enabled: bool,
    highlighter_value: f32,
    highlighter_mode: HighlighterMode,
    protect_stroke: bool,
    is_space_held: bool,
    is_spraying: bool,
    last_apply_time: Option<chrono::DateTime<chrono::Local>>,
    apply_color_interval: chrono::Duration,
    drawn_pixels_in_stroke: std::collections::HashSet<(u16, u16)>,
    minimap_mode: MinimapMode,
    mouse_events_enabled: bool,
    color_mode: ColorMode,
    default_palette_name: String,
    command_history: Vec<String>,
    history_index: usize,
    command_input_before_history: String,
    command_cursor_pos: usize,
    suggestion_active: bool,
    project_path: Option<PathBuf>,
    autosave_interval: Option<std::time::Duration>,
    last_autosave_time: Instant,
    pending_save_path: Option<PathBuf>,
    help_scroll: u16,
    loaded_palettes: std::collections::HashMap<String, Vec<PaletteEntry>>,
    keybindings: Keybindings,
    keybindings_selection_index: usize,
    is_changing_keybinding: bool,
    keybinding_change_has_occured: bool,
    confirm_selection_yes: bool, // For the dialog
    keybindings_scroll_state: u16,
    selection_before_picker: Option<PaletteEntry>,
    config_selection_index: usize,
    config_change_has_occured: bool,
    script_content_lines: Vec<String>,
    script_cursor_line: usize,
    script_scroll_state: u16,
    script_cursor_char_pos: usize, // Tracks horizontal cursor position
    script_change_has_occured: bool,
    canvas_scroll_action: CanvasScrollAction,
    spray_size: u16,
    spray_speed: u16,
    spray_intensity: f32,
    snap_to_palette: bool,
    snap_to_palette_mode: SnapToPaletteMode,
    protect_color_transitions: bool,
    browser_mode: Option<BrowserMode>,
    browser_entries: Vec<PathBuf>,
    browser_list_state: ListState,
    browser_current_dir: PathBuf,
    browser_history_back: Vec<PathBuf>,
    browser_history_forward: Vec<PathBuf>,
    browser_error: Option<String>,
    browser_input_buffer: String,
    browser_scale_buffer: String,
    browser_focus: BrowserFocus,
    last_generated_palette: Option<Vec<PaletteEntry>>,
    last_image_palette_source: Option<String>,
    palette_menu_position: PaletteMenuPosition,
    last_centered_canvas_rect: Option<Rect>,
    layers: Vec<Layer>,
    active_layer_index: usize,
    onion_skin_enabled: bool,
    onion_skin_opacity: f32,
    layer_scroll_state: usize,
    last_layer_area: Option<Rect>,
    layer_input_buffer: String,
    layer_focus: LayerFocus,
    is_renaming_layer: bool,
    export_layer_mode: ExportLayerMode,

}

impl App {


    fn translate_color(&self, color: Color) -> Color {
        if self.color_mode == ColorMode::TrueColor {
            return color;
        }

        // ANSI 256 Color Mode Logic
        let (r, g, b) = utils::to_rgb(color);

        // Grayscale check
        if r == g && g == b {
            if r < 8 { return Color::Indexed(16); } // Black
            if r > 248 { return Color::Indexed(231); } // White
            let gray_index = 232 + ((r as u16 - 8) * 24 / 247) as u8;
            return Color::Indexed(gray_index);
        }

        // Color cube check
        let r_idx = (r as u16 * 6 / 256) as u8;
        let g_idx = (g as u16 * 6 / 256) as u8;
        let b_idx = (b as u16 * 6 / 256) as u8;
        let index = 16 + (r_idx * 36) + (g_idx * 6) + b_idx;
        Color::Indexed(index)
    }


    fn load_palettes_from_disk() -> std::collections::HashMap<String, Vec<PaletteEntry>> {
        let mut palettes = std::collections::HashMap::new();
        let default_palette = get_default_color_palette();
        palettes.insert("default".to_string(), default_palette);

        if let Ok(app_dir) = utils::get_or_create_app_dir() {
            let palettes_dir = app_dir.join("palettes");
            if let Ok(entries) = std::fs::read_dir(palettes_dir) {
                for entry in entries.filter_map(Result::ok) {
                    let path = entry.path();
                    if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("consolet") {
                        if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                            if let Ok(json_data) = std::fs::read_to_string(&path) {
                                if let Ok(palette_file) = serde_json::from_str::<PaletteFile>(&json_data) {
                                    let entries = palette_file.0.into_iter().map(|sc| PaletteEntry::Color(sc.into())).collect();
                                    palettes.insert(name.to_string(), entries);
                                }
                            }
                        }
                    }
                }
            }
        }
        palettes
    }

    fn parse_hex_color(hex_str: &str) -> Option<Color> {
        let hex_str = hex_str.strip_prefix('#').unwrap_or(hex_str);
        if hex_str.len() != 6 { return None; }
        let r = u8::from_str_radix(&hex_str[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex_str[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex_str[4..6], 16).ok()?;
        Some(Color::Rgb(r, g, b))
    }

    fn load_and_store_palette(&mut self, path_str: &str) {
        let source_path = PathBuf::from(shellexpand::tilde(&path_str.replace("\"", "")).into_owned());

        if !source_path.exists() {
            self.status_message = Some((format!("Source file not found: {:?}", source_path), Instant::now()));
            return;
        }

        let palettes_dir = match utils::get_or_create_app_dir() {
            Ok(dir) => dir.join("palettes"),
            Err(_) => { self.status_message = Some(("Could not access app data directory.".to_string(), Instant::now())); return; }
        };

        let filename = match source_path.file_name() {
            Some(name) => name,
            None => { self.status_message = Some(("Invalid source file path.".to_string(), Instant::now())); return; }
        };

        let dest_path = palettes_dir.join(filename);

        if let Err(e) = std::fs::copy(&source_path, &dest_path) {
            self.status_message = Some((format!("Failed to copy palette to app data: {}", e), Instant::now()));
            return;
        }

        let palette_name = dest_path.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_string();
        if palette_name.is_empty() {
            self.status_message = Some(("Invalid palette file name.".to_string(), Instant::now()));
            return;
        }
        
        let json_data = match std::fs::read_to_string(&dest_path) {
            Ok(data) => data,
            Err(e) => { self.status_message = Some((format!("Error reading new palette file: {}", e), Instant::now())); return; }
        };

        let palette_file: PaletteFile = match serde_json::from_str(&json_data) {
            Ok(pf) => pf,
            Err(e) => { self.status_message = Some((format!("Error parsing palette: {}", e), Instant::now())); return; }
        };

        let entries = palette_file.0.into_iter().map(|sc| PaletteEntry::Color(sc.into())).collect();
        self.loaded_palettes.insert(palette_name.clone(), entries);
        self.status_message = Some((format!("Palette '{}' imported and saved.", palette_name), Instant::now()));
    }


    fn pick_color_at_cursor(&mut self) {
        let (x, y) = (self.cursor_pos.0 as usize, self.cursor_pos.1 as usize);
        if x >= self.canvas_width || y >= self.canvas_height { return; }

        let pixel = self.canvas[y][x];
        if pixel.alpha == 0.0 {
            self.status_message = Some(("Cannot pick color from a transparent pixel.".to_string(), Instant::now()));
            return;
        }

        let picked_color: Color = pixel.color.into();
        let picked_entry = PaletteEntry::Color(picked_color);

        if let Some(index) = self.color_palette.iter().position(|&entry| entry == picked_entry) {
            self.palette_index = index;
        } else {
            self.color_palette.push(picked_entry);
            self.palette_index = self.color_palette.len() - 1;
        }
        
        self.current_selection = picked_entry;
        let (r,g,b) = utils::to_rgb(picked_color);
        self.status_message = Some((format!("Color picked: ({}, {}, {})", r, g, b), Instant::now()));
    }


    fn get_suggestions(&self, input: &str) -> Vec<String> {
        if input.is_empty() {
            return Vec::new();
        }

        if let Some(prefix) = input.strip_prefix("load ") {
            if let Ok(app_dir) = utils::get_or_create_app_dir() {
                let projects_dir = app_dir.join("saved_projects");
                if let Ok(entries) = std::fs::read_dir(projects_dir) {
                    return entries
                        .filter_map(Result::ok)
                        .map(|entry| entry.file_name().into_string().unwrap_or_default())
                        .filter(|name| name.starts_with(prefix) && !name.starts_with('.'))
                        .collect();
                }
            }
        } else if let Some(prefix) = input.strip_prefix("colorpalette:") {
            return self.loaded_palettes.keys()
                .filter(|k| k.starts_with(prefix))
                .cloned()
                .collect();
        } else {
            // --- NEW: Handle colon-based commands and general commands ---
            return COMMANDS.iter()
                .map(|cmd| cmd.name.to_string())
                .filter(|name| name.starts_with(input))
                .map(|name| {
                    // If the command is a prefix type (like "savepalette:"),
                    // add the colon back for a better suggestion.
                    if name.ends_with(':') && input.contains(':') {
                        name
                    } else if name.ends_with(':') {
                        format!("{}:", name.strip_suffix(':').unwrap())
                    } else {
                        name
                    }
                })
                .collect();
        }
        Vec::new()
    }




    fn new() -> Self {
    let (width, height) = (30, 30);
    let loaded_palettes = App::load_palettes_from_disk();
    let default_palette = loaded_palettes.get("default").unwrap().clone();

        App {
            canvas: vec![vec![Pixel::default(); width]; height],
            layers: vec![Layer {
                name: "Layer 1".to_string(),
                canvas: vec![vec![Pixel::default(); width]; height],
                visible: true,
                opacity: 1.0,
            }],
            active_layer_index: 0,
            canvas_width: width, canvas_height: height,
            cursor_pos: (0, 0),
            current_selection: PaletteEntry::Color(Color::White),
            tool_palette: get_default_tool_palette(),
            color_palette: default_palette,
            loaded_palettes,
            palette_index: 0,
            tool_index: 0, 
            palette_scroll_state: 0,
            mode: AppMode::Drawing,
            symmetry_mode: SymmetryMode::Off,
            should_quit: false,
            status_message: None,
            input_buffer: String::new(),
            temp_width: 0,
            last_pixel_area: None,
            last_palette_area: None,
            last_tool_area: None,
            is_side_panel_visible: true,
            pen_size: 1,
            opacity: 1.0,
            pen_size_sensitivity: 1,
            opacity_sensitivity: 0.05,
            pen_shape: PenShape::Circular,
            view_offset_x: 0,
            view_offset_y: 0,
            zoom_level: PIXEL_WIDTH,
            suggestion_index: 0,
            undo_stack: VecDeque::new(),
            redo_stack: VecDeque::new(),
            is_mouse_dragging: false,
            shade_factor: DEFAULT_SHADE_FACTOR,
            highlighter_enabled: true,
            highlighter_value: 0.5,
            highlighter_mode: HighlighterMode::Blend,
            protect_stroke: true,
            is_space_held: false,
            is_spraying: false,
            last_apply_time: None,
            apply_color_interval: chrono::Duration::milliseconds(200),
            drawn_pixels_in_stroke: std::collections::HashSet::new(),
            minimap_mode: MinimapMode::Auto,
            mouse_events_enabled: true,
            color_mode: ColorMode::TrueColor,
            default_palette_name: "default".to_string(),
            command_history: Vec::new(),
            history_index: 0,
            command_input_before_history: String::new(),
            command_cursor_pos: 0,
            suggestion_active: false,        
            project_path: None,
            autosave_interval: None,
            last_autosave_time: Instant::now(),
            pending_save_path: None,
            help_scroll: 0,

            keybindings: Keybindings::load(),
            keybindings_selection_index: 0,
            is_changing_keybinding: false,
            keybinding_change_has_occured: false,
            confirm_selection_yes: true,
            keybindings_scroll_state: 0,
            selection_before_picker: None,
            config_selection_index: 0,
            config_change_has_occured: false,

            script_content_lines: Vec::new(),
            script_cursor_line: 0,
            script_scroll_state: 0,

            script_cursor_char_pos: 0,
            script_change_has_occured: false,
            canvas_scroll_action: CanvasScrollAction::ChangePenSize,
            spray_size: 5,
            spray_speed: 3,
            spray_intensity: 0.1,
            snap_to_palette: false,
            snap_to_palette_mode: SnapToPaletteMode::ClosestHue,
            protect_color_transitions: false,
            browser_mode: None,
            browser_entries: Vec::new(),
            browser_list_state: ListState::default(),
            browser_current_dir: PathBuf::new(),
            browser_history_back: Vec::new(),
            browser_history_forward: Vec::new(),
            browser_error: None,
            browser_input_buffer: String::new(),
            browser_scale_buffer: "1".to_string(), // Default scale is 1
            browser_focus: BrowserFocus::List,

            last_generated_palette: None,
            last_image_palette_source: None,
            palette_menu_position: PaletteMenuPosition::Left,
            last_centered_canvas_rect: None,
            onion_skin_enabled: false,
            onion_skin_opacity: 0.3,
            layer_scroll_state: 0,
            last_layer_area: None,
            layer_input_buffer: String::new(),
            layer_focus: LayerFocus::List,
            is_renaming_layer: false,
            export_layer_mode: ExportLayerMode::United,




    }
}



    fn get_active_canvas(&self) -> &Vec<Vec<Pixel>> {
        &self.layers[self.active_layer_index].canvas
    }

    fn get_active_canvas_mut(&mut self) -> &mut Vec<Vec<Pixel>> {
        &mut self.layers[self.active_layer_index].canvas
    }

    fn add_new_layer(&mut self) {
        let new_layer = Layer {
            name: format!("Layer {}", self.layers.len() + 1),
            canvas: vec![vec![Pixel::default(); self.canvas_width]; self.canvas_height],
            visible: true,
            opacity: 1.0,
        };
        self.layers.push(new_layer);
        self.active_layer_index = self.layers.len() - 1;
        self.sync_canvas_from_layers();
        self.status_message = Some((format!("Added {}", self.layers[self.active_layer_index].name), Instant::now()));
    }

    fn delete_active_layer(&mut self) {
        if self.layers.len() <= 1 {
            self.status_message = Some(("Cannot delete the only layer.".to_string(), Instant::now()));
            return;
        }
        self.layers.remove(self.active_layer_index);
        if self.active_layer_index >= self.layers.len() {
            self.active_layer_index = self.layers.len() - 1;
        }
        self.sync_canvas_from_layers();
        self.status_message = Some(("Layer deleted.".to_string(), Instant::now()));
    }

    fn toggle_layer_visibility(&mut self) {
        self.layers[self.active_layer_index].visible = !self.layers[self.active_layer_index].visible;
        self.sync_canvas_from_layers();
    }

    fn move_layer_up(&mut self) {
        if self.active_layer_index > 0 {
            self.layers.swap(self.active_layer_index, self.active_layer_index - 1);
            self.active_layer_index -= 1;
            self.sync_canvas_from_layers();
        }
    }

    fn move_layer_down(&mut self) {
        if self.active_layer_index < self.layers.len() - 1 {
            self.layers.swap(self.active_layer_index, self.active_layer_index + 1);
            self.active_layer_index += 1;
            self.sync_canvas_from_layers();
        }
    }

    fn sync_canvas_from_layers(&mut self) {
        self.canvas = vec![vec![Pixel::default(); self.canvas_width]; self.canvas_height];
        for layer in self.layers.iter().rev() {
            if !layer.visible {
                continue;
            }
            for y in 0..self.canvas_height {
                for x in 0..self.canvas_width {
                    let layer_pixel = layer.canvas[y][x];
                    if layer_pixel.alpha == 0.0 {
                        continue;
                    }
                    let dest_pixel = self.canvas[y][x];
                    let src_alpha = layer_pixel.alpha * layer.opacity;
                    if dest_pixel.alpha == 0.0 {
                        self.canvas[y][x] = Pixel {
                            color: layer_pixel.color,
                            alpha: src_alpha,
                        };
                    } else {
                        let final_alpha = src_alpha + dest_pixel.alpha * (1.0 - src_alpha);
                        let factor = src_alpha / final_alpha;
                        let final_color = utils::blend_colors(dest_pixel.color.into(), layer_pixel.color.into(), factor);
                        self.canvas[y][x] = Pixel {
                            color: final_color.into(),
                            alpha: final_alpha,
                        };
                    }
                }
            }
        }
    }

    fn sync_active_layer_from_canvas(&mut self) {
        self.layers[self.active_layer_index].canvas = self.canvas.clone();
    }

    fn change_layer_selection(&mut self, delta: i16) {
        let new_index = (self.active_layer_index as i16 + delta)
            .max(0)
            .min(self.layers.len() as i16 - 1) as usize;
        self.active_layer_index = new_index;
        self.sync_canvas_from_layers();
    }




    fn reset_keybindings(&mut self) {
        // 1. Delete the saved keybindings file.
        if let Ok(path) = keybindings::Keybindings::get_path() {
            // We ignore the result, it's okay if the file didn't exist.
            let _ = std::fs::remove_file(path);
        }

        // 2. Load the default bindings back into the current app state.
        self.keybindings = Keybindings::default();

        // 3. Inform the user.
        self.status_message = Some(("Keybindings have been reset to default.".to_string(), Instant::now()));
    }




fn rgb_to_hue(&self, r: u8, g: u8, b: u8) -> f32 {
    let r_norm = r as f32 / 255.0;
    let g_norm = g as f32 / 255.0;
    let b_norm = b as f32 / 255.0;
    let max = r_norm.max(g_norm).max(b_norm);
    let min = r_norm.min(g_norm).min(b_norm);
    let delta = max - min;
    if delta == 0.0 {
        return 0.0;
    }
    let hue = if max == r_norm {
        60.0 * (((g_norm - b_norm) / delta) % 6.0)
    } else if max == g_norm {
        60.0 * (((b_norm - r_norm) / delta) + 2.0)
    } else {
        60.0 * (((r_norm - g_norm) / delta) + 4.0)
    };
    if hue < 0.0 { hue + 360.0 } else { hue }
}
fn hue_distance(&self, h1: f32, h2: f32) -> f32 {
    let diff = (h1 - h2).abs();
    if diff > 180.0 { 360.0 - diff } else { diff }
}


fn find_closest_palette_color(&self, target: Color) -> Color {
    let (tr, tg, tb) = utils::to_rgb(target);
    let mut closest = target;
    let mut min_dist = f32::MAX;
    for entry in &self.color_palette {
        if let PaletteEntry::Color(c) = entry {
            let (r, g, b) = utils::to_rgb(*c);
            let dr = tr as f32 - r as f32;
            let dg = tg as f32 - g as f32;
            let db = tb as f32 - b as f32;
            let dist = (dr * dr + dg * dg + db * db).sqrt();
            if dist < min_dist {
                min_dist = dist;
                closest = *c;
            }
        }
    }
    closest
}








fn find_lighter_rgb(&self, current: Color) -> Color {
    let (cr, cg, cb) = utils::to_rgb(current);
    let current_brightness = cr as f32 + cg as f32 + cb as f32;
    if !self.protect_color_transitions {
        let total = cr.max(1) as f32;
        let ratio_g = cg as f32 / total;
        let ratio_b = cb as f32 / total;
        let mut closest = current;
        let mut min_score = f32::MAX;
        for entry in &self.color_palette {
            if let PaletteEntry::Color(c) = entry {
                let (r, g, b) = utils::to_rgb(*c);
                let brightness = r as f32 + g as f32 + b as f32;
                if brightness <= current_brightness { continue; }
                let cand_total = r.max(1) as f32;
                let cand_ratio_g = g as f32 / cand_total;
                let cand_ratio_b = b as f32 / cand_total;
                let ratio_diff = (ratio_g - cand_ratio_g).abs() + (ratio_b - cand_ratio_b).abs();
                let dr = cr as f32 - r as f32;
                let dg = cg as f32 - g as f32;
                let db = cb as f32 - b as f32;
                let rgb_dist = (dr * dr + dg * dg + db * db).sqrt();
                let score = rgb_dist + (ratio_diff * 500.0);
                if score < min_score {
                    min_score = score;
                    closest = *c;
                }
            }
        }
        return closest;
    }
    let current_hue = self.rgb_to_hue(cr, cg, cb);
    let mut closest = current;
    let mut min_score = f32::MAX;
    for entry in &self.color_palette {
        if let PaletteEntry::Color(c) = entry {
            let (r, g, b) = utils::to_rgb(*c);
            let brightness = r as f32 + g as f32 + b as f32;
            if brightness <= current_brightness { continue; }
            let cand_hue = self.rgb_to_hue(r, g, b);
            let hue_dist = self.hue_distance(current_hue, cand_hue);
            if hue_dist > 45.0 { continue; }
            let dr = cr as f32 - r as f32;
            let dg = cg as f32 - g as f32;
            let db = cb as f32 - b as f32;
            let rgb_dist = (dr * dr + dg * dg + db * db).sqrt();
            let score = rgb_dist + (hue_dist * 10.0);
            if score < min_score {
                min_score = score;
                closest = *c;
            }
        }
    }
    closest
}

fn find_darker_rgb(&self, current: Color) -> Color {
    let (cr, cg, cb) = utils::to_rgb(current);
    let current_brightness = cr as f32 + cg as f32 + cb as f32;
    if !self.protect_color_transitions {
        let total = cr.max(1) as f32;
        let ratio_g = cg as f32 / total;
        let ratio_b = cb as f32 / total;
        let mut closest = current;
        let mut min_score = f32::MAX;
        for entry in &self.color_palette {
            if let PaletteEntry::Color(c) = entry {
                let (r, g, b) = utils::to_rgb(*c);
                let brightness = r as f32 + g as f32 + b as f32;
                if brightness >= current_brightness { continue; }
                let cand_total = r.max(1) as f32;
                let cand_ratio_g = g as f32 / cand_total;
                let cand_ratio_b = b as f32 / cand_total;
                let ratio_diff = (ratio_g - cand_ratio_g).abs() + (ratio_b - cand_ratio_b).abs();
                let dr = cr as f32 - r as f32;
                let dg = cg as f32 - g as f32;
                let db = cb as f32 - b as f32;
                let rgb_dist = (dr * dr + dg * dg + db * db).sqrt();
                let score = rgb_dist + (ratio_diff * 500.0);
                if score < min_score {
                    min_score = score;
                    closest = *c;
                }
            }
        }
        return closest;
    }
    let current_hue = self.rgb_to_hue(cr, cg, cb);
    let mut closest = current;
    let mut min_score = f32::MAX;
    for entry in &self.color_palette {
        if let PaletteEntry::Color(c) = entry {
            let (r, g, b) = utils::to_rgb(*c);
            let brightness = r as f32 + g as f32 + b as f32;
            if brightness >= current_brightness { continue; }
            let cand_hue = self.rgb_to_hue(r, g, b);
            let hue_dist = self.hue_distance(current_hue, cand_hue);
            if hue_dist > 45.0 { continue; }
            let dr = cr as f32 - r as f32;
            let dg = cg as f32 - g as f32;
            let db = cb as f32 - b as f32;
            let rgb_dist = (dr * dr + dg * dg + db * db).sqrt();
            let score = rgb_dist + (hue_dist * 10.0);
            if score < min_score {
                min_score = score;
                closest = *c;
            }
        }
    }
    closest
}

fn find_lighter_palette_color(&self, current: Color) -> Color {
    let (cr, cg, cb) = utils::to_rgb(current);
    let current_brightness = cr as f32 + cg as f32 + cb as f32;
    let mut closest = current;
    let mut min_combined = f32::MAX;
    for entry in &self.color_palette {
        if let PaletteEntry::Color(c) = entry {
            let (r, g, b) = utils::to_rgb(*c);
            let brightness = r as f32 + g as f32 + b as f32;
            if brightness <= current_brightness {
                continue;
            }
            let dr = cr as f32 - r as f32;
            let dg = cg as f32 - g as f32;
            let db = cb as f32 - b as f32;
            let rgb_dist = (dr * dr + dg * dg + db * db).sqrt();
            let hue_diff = (dr.abs() + dg.abs() + db.abs()) / 3.0;
            let combined = rgb_dist + (hue_diff * 2.0);
            if combined < min_combined {
                min_combined = combined;
                closest = *c;
            }
        }
    }
    closest
}

fn find_darker_palette_color(&self, current: Color) -> Color {
    let (cr, cg, cb) = utils::to_rgb(current);
    let current_brightness = cr as f32 + cg as f32 + cb as f32;
    let mut closest = current;
    let mut min_combined = f32::MAX;
    for entry in &self.color_palette {
        if let PaletteEntry::Color(c) = entry {
            let (r, g, b) = utils::to_rgb(*c);
            let brightness = r as f32 + g as f32 + b as f32;
            if brightness >= current_brightness {
                continue;
            }
            let dr = cr as f32 - r as f32;
            let dg = cg as f32 - g as f32;
            let db = cb as f32 - b as f32;
            let rgb_dist = (dr * dr + dg * dg + db * db).sqrt();
            let hue_diff = (dr.abs() + dg.abs() + db.abs()) / 3.0;
            let combined = rgb_dist + (hue_diff * 2.0);
            if combined < min_combined {
                min_combined = combined;
                closest = *c;
            }
        }
    }
    closest
}


    fn apply_effect_with_stroke_tracking(&mut self, x: usize, y: usize) {
        if x >= self.canvas_width || y >= self.canvas_height { return; }

        if self.protect_stroke {
            let coord = (x as u16, y as u16);
            if !self.drawn_pixels_in_stroke.contains(&coord) {
                self.apply_effect_at_pixel(x, y);
                self.drawn_pixels_in_stroke.insert(coord);
            }
        } else {
            self.apply_effect_at_pixel(x, y);
        }
    }

    fn resize_canvas(&mut self, new_width: usize, new_height: usize) {
        self.canvas_width = new_width.max(1);
        self.canvas_height = new_height.max(1);
        self.canvas = vec![vec![Pixel::default(); self.canvas_width]; self.canvas_height];
        for layer in &mut self.layers {
            layer.canvas = vec![vec![Pixel::default(); self.canvas_width]; self.canvas_height];
        }
        self.sync_canvas_from_layers();

        self.cursor_pos.0 = self.cursor_pos.0.min(self.canvas_width.saturating_sub(1) as u16);
        self.cursor_pos.1 = self.cursor_pos.1.min(self.canvas_height.saturating_sub(1) as u16);

        // --- NEW: Auto-zoom to fit the new canvas to the screen ---
        if let Some(pixel_area) = self.last_pixel_area {
            if self.canvas_width > 0 && self.canvas_height > 0 {
                // Calculate the maximum possible zoom level based on width
                let max_zoom_x = pixel_area.width / self.canvas_width as u16;
                
                // Calculate the maximum possible zoom level based on height
                let max_zoom_y = (pixel_area.height * PIXEL_WIDTH) / self.canvas_height as u16;

                // The new zoom must respect both constraints, so we take the smaller of the two.
                let mut new_zoom = max_zoom_x.min(max_zoom_y);
                
                // Ensure zoom is at least 2 (for 1x) and is an even number to maintain the square aspect ratio.
                new_zoom = new_zoom.max(2);
                new_zoom = (new_zoom / 2) * 2;
                
                self.zoom_level = new_zoom;
            }
        }
        
        // --- NEW: Reset the camera pan to the top-left corner ---
        self.view_offset_x = 0;
        self.view_offset_y = 0;
    }
    fn clear_canvas(&mut self) {
        self.save_state_for_undo();
        self.layers[self.active_layer_index].canvas = vec![vec![Pixel::default(); self.canvas_width]; self.canvas_height];
        self.sync_canvas_from_layers();
        self.status_message = Some(("Active layer cleared.".to_string(), Instant::now()));
    }

    fn quit(&mut self) { self.should_quit = true; }

    fn move_cursor(&mut self, dx: i16, dy: i16) {
        if let AppMode::Drawing = self.mode {
            let (x, y) = self.cursor_pos;
            let new_x = (x as i16 + dx).max(0).min(self.canvas_width.saturating_sub(1) as i16);
            let new_y = (y as i16 + dy).max(0).min(self.canvas_height.saturating_sub(1) as i16);
            self.cursor_pos = (new_x as u16, new_y as u16);
        }
    }
    
    fn cycle_symmetry_mode(&mut self) {
        self.symmetry_mode = match self.symmetry_mode {
            SymmetryMode::Off => SymmetryMode::Vertical(self.canvas_width as u16 / 2),
            SymmetryMode::Vertical(_) => {
                let center_x = self.canvas_width as i32 / 2;
                let center_y = self.canvas_height as i32 / 2;
                SymmetryMode::DiagonalForward(center_y - center_x)
            }
            SymmetryMode::DiagonalForward(_) => SymmetryMode::Horizontal(self.canvas_height as u16 / 2),
            SymmetryMode::Horizontal(_) => {
                let center_x = self.canvas_width as i32 / 2;
                let center_y = self.canvas_height as i32 / 2;
                SymmetryMode::DiagonalBackward(center_y + center_x)
            }
            SymmetryMode::DiagonalBackward(_) => SymmetryMode::Off,
        };
    }

    fn change_pen_size(&mut self, delta: i16) {
        let change = self.pen_size_sensitivity as i16 * delta;
        let new_size = (self.pen_size as i16 + change).max(1);
        self.pen_size = new_size as u16;
        self.status_message = Some((format!("Pen size: {}", self.pen_size), Instant::now()));

    }

    fn change_opacity(&mut self, direction: f32) {
        let change = self.opacity_sensitivity * direction;
        self.opacity = (self.opacity + change).clamp(0.0, 1.0);
        self.status_message = Some((format!("Opacity: {:.0}%", self.opacity * 100.0), Instant::now()));

    }

    fn pan_view(&mut self, dx: i32, dy: i32) {
        self.view_offset_x += dx;
        self.view_offset_y += dy;
        // Clamping will be handled in the UI function to ensure it's always correct.
    }

    fn zoom(&mut self, delta: i16) {
        let new_zoom = self.zoom_level as i16 + delta;
        // Set zoom bounds (e.g., from 2 to 16)
        self.zoom_level = new_zoom.clamp(2, 16) as u16;
    }

    fn clamp_view_offsets(&mut self, visible_width: u16, visible_height: u16) {
        let pixel_render_height = (self.zoom_level / PIXEL_WIDTH).max(1);

        // --- FIX: Use ceiling division to correctly calculate how many pixels can fit ---
        // The formula (a + b - 1) / b correctly rounds up integer division.
        
        // Horizontal clamping in PIXELS
        let visible_pixels_x = (visible_width + self.zoom_level - 1) / self.zoom_level;
        let max_offset_x = self.canvas_width.saturating_sub(visible_pixels_x as usize) as i32;
        self.view_offset_x = self.view_offset_x.clamp(0, max_offset_x);

        // Vertical clamping in PIXELS
        let visible_pixels_y = (visible_height + pixel_render_height - 1) / pixel_render_height;
        let max_offset_y = self.canvas_height.saturating_sub(visible_pixels_y as usize) as i32;
        self.view_offset_y = self.view_offset_y.clamp(0, max_offset_y);
    }

    fn change_palette_selection_2d(&mut self, dx: i16, dy: i16) {
        if let Some(palette_area) = self.last_palette_area {
            let columns = (palette_area.width / 3).max(1) as usize;
            let visible_rows = palette_area.height as usize;

            // Calculate the proposed new index
            let current_col = (self.palette_index % columns) as i16;
            let current_row = (self.palette_index / columns) as i16;
            let new_col = (current_col + dx).clamp(0, columns as i16 - 1);
            let new_row = current_row + dy;
            let new_index = (new_row * columns as i16 + new_col)
                .clamp(0, self.color_palette.len() as i16 - 1) as usize;

            self.palette_index = new_index;

            // Now, adjust the scroll state to keep the new index visible
            let top_visible_row = self.palette_scroll_state / columns;
            let bottom_visible_row = top_visible_row + visible_rows - 1;
            let new_item_row = new_index / columns;

            if new_item_row < top_visible_row {
                // Scrolled up past the top
                self.palette_scroll_state = new_item_row * columns;
            } else if new_item_row > bottom_visible_row {
                // Scrolled down past the bottom
                let new_top_row = new_item_row - visible_rows + 1;
                self.palette_scroll_state = new_top_row * columns;
            }
        }
    }

    fn change_tool_selection(&mut self, delta: i16) {
        let new_index = self.tool_index as i16 + delta;
        self.tool_index = new_index.max(0).min(self.tool_palette.len() as i16 - 1) as usize;
    }

    fn select_color_entry(&mut self) {
        self.current_selection = self.color_palette[self.palette_index];
        self.mode = AppMode::Drawing;
        self.status_message = None;
    }

    fn select_tool_entry(&mut self) {
        self.current_selection = self.tool_palette[self.tool_index];
        self.mode = AppMode::Drawing;
        self.status_message = None;
    }

fn calculate_blur_at(&self, x: usize, y: usize, opacity: f32) -> Pixel {
        let active_canvas = &self.layers[self.active_layer_index].canvas;
        let original_pixel = active_canvas[y][x];
        let mut r_sum: u32 = 0;
        let mut g_sum: u32 = 0;
        let mut b_sum: u32 = 0;
        let mut a_sum: f32 = 0.0;
        let mut count: u32 = 0;
        let mut has_colored_neighbor = false;

        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < self.canvas_width as i32 && ny >= 0 && ny < self.canvas_height as i32 {
                    let neighbor = active_canvas[ny as usize][nx as usize];
                    if neighbor.alpha > 0.0 {
                        let (r, g, b) = utils::to_rgb(neighbor.color.into());
                        r_sum += r as u32;
                        g_sum += g as u32;
                        b_sum += b as u32;
                        if dx != 0 || dy != 0 {
                            has_colored_neighbor = true;
                        }
                    }
                    a_sum += neighbor.alpha;
                    count += 1;
                }
            }
        }

        if original_pixel.alpha == 0.0 && !has_colored_neighbor {
            return original_pixel;
        }

        if count > 0 {
            let blurred_color = Color::Rgb((r_sum / count) as u8, (g_sum / count) as u8, (b_sum / count) as u8);
            let blurred_alpha = a_sum / count as f32;

            let intermediate_color = utils::blend_colors(original_pixel.color.into(), blurred_color, opacity);
            let final_color = if self.snap_to_palette {
                self.find_closest_palette_color(intermediate_color)
            } else {
                intermediate_color
            };
            let final_alpha = original_pixel.alpha * (1.0 - opacity) + blurred_alpha * opacity;

            Pixel {
                color: final_color.into(),
                alpha: final_alpha,
            }

        } else {
            original_pixel
        }
    }

fn apply_effect_at_pixel(&mut self, x: usize, y: usize) {
    if x >= self.canvas_width || y >= self.canvas_height { return; }

    if let PaletteEntry::Tool(tool) = self.current_selection {
        let original_pixel = self.layers[self.active_layer_index].canvas[y][x];
        if original_pixel.alpha == 0.0 && tool != Tool::Blur { return; }

        let new_pixel = match tool {
            Tool::Lighter => {
                let new_color = if self.snap_to_palette {
                    match self.snap_to_palette_mode {
                        SnapToPaletteMode::ClosestRgb => self.find_lighter_rgb(original_pixel.color.into()),
                        SnapToPaletteMode::ClosestHue => self.find_lighter_palette_color(original_pixel.color.into()),
                    }
                } else {
                    utils::blend_colors(original_pixel.color.into(), Color::White, self.shade_factor)
                };
                Pixel { color: new_color.into(), ..original_pixel }
            }
            Tool::Darker => {
                let new_color = if self.snap_to_palette {
                    match self.snap_to_palette_mode {
                        SnapToPaletteMode::ClosestRgb => self.find_darker_rgb(original_pixel.color.into()),
                        SnapToPaletteMode::ClosestHue => self.find_darker_palette_color(original_pixel.color.into()),
                    }
                } else {
                    utils::blend_colors(original_pixel.color.into(), Color::Black, self.shade_factor)
                };
                Pixel { color: new_color.into(), ..original_pixel }
            }
            Tool::Blur => {
                self.calculate_blur_at(x, y, self.opacity)
            }
        };
        self.layers[self.active_layer_index].canvas[y][x] = new_pixel;
        self.sync_canvas_from_layers();
        return;
    }

    if let PaletteEntry::Color(src_color) = self.current_selection {
        let active_canvas = &mut self.layers[self.active_layer_index].canvas;
        let dest_pixel = active_canvas[y][x];
        let src_alpha = self.opacity;

        if dest_pixel.alpha == 0.0 {
            active_canvas[y][x] = Pixel { color: src_color.into(), alpha: src_alpha };
        } else {
            let final_alpha = src_alpha + dest_pixel.alpha * (1.0 - src_alpha);
            let factor = src_alpha / final_alpha;
            let final_color = utils::blend_colors(dest_pixel.color.into(), src_color, factor);
            active_canvas[y][x] = Pixel { color: final_color.into(), alpha: final_alpha };
        }
        self.sync_canvas_from_layers();
    }
}

fn apply_brush(&mut self, center_x: u16, center_y: u16) {
    let radius = self.pen_size as i32 / 2;
    let start_x = center_x as i32 - radius;
    let start_y = center_y as i32 - radius;

    for y_offset in 0..self.pen_size as i32 {
        for x_offset in 0..self.pen_size as i32 {
            let mut should_draw = false;
            match self.pen_shape {
                PenShape::Square => should_draw = true,
                PenShape::Circular => {
                    let dx = x_offset - radius;
                    let dy = y_offset - radius;
                    if (dx * dx + dy * dy) <= (radius * radius) {
                        should_draw = true;
                    }
                }
            }
            if !should_draw { continue; }

            let canvas_x_i32 = start_x + x_offset;
            let canvas_y_i32 = start_y + y_offset;

            if canvas_x_i32 >= 0 && canvas_x_i32 < self.canvas_width as i32 &&
            canvas_y_i32 >= 0 && canvas_y_i32 < self.canvas_height as i32 {
                let canvas_x = canvas_x_i32 as usize;
                let canvas_y = canvas_y_i32 as usize;

                self.apply_effect_with_stroke_tracking(canvas_x, canvas_y);
                match self.symmetry_mode {
                    SymmetryMode::Vertical(line_x) => {
                        let mirrored_x = if self.canvas_width % 2 == 0 {
                            (2 * line_x as i32) - canvas_x_i32 - 1
                        } else {
                            (2 * line_x as i32) - canvas_x_i32
                        };
                        if mirrored_x >= 0 && mirrored_x < self.canvas_width as i32 {
                            self.apply_effect_with_stroke_tracking(mirrored_x as usize, canvas_y);
                        }
                    }
                    SymmetryMode::Horizontal(line_y) => {
                        let mirrored_y = if self.canvas_height % 2 == 0 {
                            (2 * line_y as i32) - canvas_y_i32 - 1
                        } else {
                            (2 * line_y as i32) - canvas_y_i32
                        };
                        if mirrored_y >= 0 && mirrored_y < self.canvas_height as i32 {
                            self.apply_effect_with_stroke_tracking(canvas_x, mirrored_y as usize);
                        }
                    }
                    SymmetryMode::DiagonalForward(c) => { // y = x + c
                        let mirrored_x = canvas_y_i32 - c;
                        let mirrored_y = canvas_x_i32 + c;
                        if mirrored_x >= 0 && mirrored_x < self.canvas_width as i32 && mirrored_y >= 0 && mirrored_y < self.canvas_height as i32 {
                            self.apply_effect_with_stroke_tracking(mirrored_x as usize, mirrored_y as usize);
                        }
                    }
                    SymmetryMode::DiagonalBackward(c) => { // y = -x + c
                        let mirrored_x = c - canvas_y_i32;
                        let mirrored_y = c - canvas_x_i32;
                        if mirrored_x >= 0 && mirrored_x < self.canvas_width as i32 && mirrored_y >= 0 && mirrored_y < self.canvas_height as i32 {
                            self.apply_effect_with_stroke_tracking(mirrored_x as usize, mirrored_y as usize);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
fn erase_brush(&mut self, center_x: u16, center_y: u16) {
    let radius = self.pen_size as i32 / 2;
    let start_x = center_x as i32 - radius;
    let start_y = center_y as i32 - radius;

    for y_offset in 0..self.pen_size as i32 {
        for x_offset in 0..self.pen_size as i32 {
            let mut should_erase = false;
            match self.pen_shape {
                PenShape::Square => should_erase = true,
                PenShape::Circular => {
                    let dx = x_offset - radius;
                    let dy = y_offset - radius;
                    if (dx * dx + dy * dy) <= (radius * radius) {
                        should_erase = true;
                    }
                }
            }
            if !should_erase { continue; }

            let canvas_x_i32 = start_x + x_offset;
            let canvas_y_i32 = start_y + y_offset;

            if canvas_x_i32 >= 0 && canvas_x_i32 < self.canvas_width as i32 &&
            canvas_y_i32 >= 0 && canvas_y_i32 < self.canvas_height as i32 {
                let canvas_x = canvas_x_i32 as usize;
                let canvas_y = canvas_y_i32 as usize;

                let apply_erase = |app: &mut App, x: usize, y: usize| {
                    app.layers[app.active_layer_index].canvas[y][x] = Pixel::default(); // This is correct
                    // The incorrect line that modified app.canvas is now gone.
                    if app.protect_stroke {
                        app.drawn_pixels_in_stroke.insert((x as u16, y as u16));
                    }
                };

                let coord = (canvas_x as u16, canvas_y as u16);
                if !self.protect_stroke || !self.drawn_pixels_in_stroke.contains(&coord) {
                    apply_erase(self, canvas_x, canvas_y);
                    match self.symmetry_mode {
                        SymmetryMode::Horizontal(line_y) => {
                            let mirrored_y = if self.canvas_height % 2 == 0 {
                                (2 * line_y as i32) - canvas_y_i32 - 1
                            } else {
                                (2 * line_y as i32) - canvas_y_i32
                            };
                            if mirrored_y >= 0 && mirrored_y < self.canvas_height as i32 {
                                apply_erase(self, canvas_x, mirrored_y as usize);
                            }
                        }
                        SymmetryMode::Vertical(line_x) => {
                            let mirrored_x = if self.canvas_width % 2 == 0 {
                                (2 * line_x as i32) - canvas_x_i32 - 1
                            } else {
                                (2 * line_x as i32) - canvas_x_i32
                            };
                            if mirrored_x >= 0 && mirrored_x < self.canvas_width as i32 {
                                apply_erase(self, mirrored_x as usize, canvas_y);
                            }
                        }
                        SymmetryMode::DiagonalForward(c) => {
                            let mirrored_x = canvas_y_i32 - c;
                            let mirrored_y = canvas_x_i32 + c;
                            if mirrored_x >= 0 && mirrored_x < self.canvas_width as i32 && mirrored_y >= 0 && mirrored_y < self.canvas_height as i32 {
                                apply_erase(self, mirrored_x as usize, mirrored_y as usize);
                            }
                        }
                        SymmetryMode::DiagonalBackward(c) => {
                            let mirrored_x = c - canvas_y_i32;
                            let mirrored_y = c - canvas_x_i32;
                            if mirrored_x >= 0 && mirrored_x < self.canvas_width as i32 && mirrored_y >= 0 && mirrored_y < self.canvas_height as i32 {
                                apply_erase(self, mirrored_x as usize, mirrored_y as usize);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    self.sync_canvas_from_layers();
}




fn apply_spray(&mut self) {
    if let PaletteEntry::Color(_) = self.current_selection {
        // Continue if a color is selected
    } else {
        self.status_message = Some(("Select a color to spray.".to_string(), Instant::now()));
        return;
    }

    let (center_x, center_y) = (self.cursor_pos.0 as i32, self.cursor_pos.1 as i32);
    let radius = self.spray_size as i32 / 2;
    let mut rng = rand::thread_rng();

    for _ in 0..self.spray_speed {
        let offset_x = rng.gen_range(-radius..=radius);
        let offset_y = rng.gen_range(-radius..=radius);

        let target_x = center_x + offset_x;
        let target_y = center_y + offset_y;

        // NEW: Use intensity to decide whether to draw
        if rng.gen::<f32>() < self.spray_intensity {
            if target_x >= 0 && target_x < self.canvas_width as i32 &&
               target_y >= 0 && target_y < self.canvas_height as i32 {
                self.apply_effect_at_pixel(target_x as usize, target_y as usize);
            }
        }
    }
}




    fn use_current_tool(&mut self) {
        self.save_state_for_undo();
        let (x, y) = self.cursor_pos;
        self.apply_brush(x, y);
    }

    fn erase_at_cursor(&mut self) {
        self.save_state_for_undo();
        let (x, y) = self.cursor_pos;
        self.erase_brush(x, y);
    }

fn fill_from_point(&mut self, start_x: usize, start_y: usize, fill_color: Color, fill_alpha: f32) {
    if start_x >= self.canvas_width || start_y >= self.canvas_height { return; }

    let target_pixel = self.layers[self.active_layer_index].canvas[start_y][start_x];
    let serializable_fill_color: SerializableColor = fill_color.into();

    if target_pixel.color == serializable_fill_color && target_pixel.alpha == fill_alpha {
        return;
    }

    self.save_state_for_undo(); // Save state BEFORE the mutable borrow below

    let active_canvas = &mut self.layers[self.active_layer_index].canvas;
    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));

    while let Some((x, y)) = queue.pop_front() {
        if x < self.canvas_width && y < self.canvas_height && active_canvas[y][x] == target_pixel {
            active_canvas[y][x].color = serializable_fill_color;
            active_canvas[y][x].alpha = fill_alpha;

            if x > 0 { queue.push_back((x - 1, y)); }
            if x + 1 < self.canvas_width { queue.push_back((x + 1, y)); }
            if y > 0 { queue.push_back((x, y - 1)); }
            if y + 1 < self.canvas_height { queue.push_back((x, y + 1)); }
        }
    }
    self.sync_canvas_from_layers();
}

    fn fill_area(&mut self) {
        let fill_color_entry = if let PaletteEntry::Color(c) = self.current_selection {
            c
        } else {
            self.status_message = Some(("Select a color to fill.".to_string(), Instant::now()));
            return;
        };
        let (start_x, start_y) = (self.cursor_pos.0 as usize, self.cursor_pos.1 as usize);
        self.fill_from_point(start_x, start_y, fill_color_entry, self.opacity);
    }

    fn save_state_for_undo(&mut self) {
        self.undo_stack.push_back(self.layers[self.active_layer_index].canvas.clone());
        if self.undo_stack.len() > 100 {
            self.undo_stack.pop_front();
        }
        self.redo_stack.clear();
    }

    fn undo(&mut self) {
        if !self.undo_stack.is_empty() {
            self.redo_stack.push_back(self.layers[self.active_layer_index].canvas.clone());
            self.layers[self.active_layer_index].canvas = self.undo_stack.pop_back().unwrap();
            self.sync_canvas_from_layers();
            self.status_message = Some(("Undo".to_string(), Instant::now()));
        } else {
            self.status_message = Some(("Nothing to undo".to_string(), Instant::now()));
        }
    }

    fn redo(&mut self) {
        if !self.redo_stack.is_empty() {
            self.undo_stack.push_back(self.layers[self.active_layer_index].canvas.clone());
            self.layers[self.active_layer_index].canvas = self.redo_stack.pop_back().unwrap();
            self.sync_canvas_from_layers();
            self.status_message = Some(("Redo".to_string(), Instant::now()));
        } else {
            self.status_message = Some(("Nothing to redo".to_string(), Instant::now()));
        }
    }

fn save_project(&mut self, path: &PathBuf, set_as_current: bool) {
    let current_palette: Vec<SerializableColor> = self.color_palette.iter().filter_map(|entry| {
        if let PaletteEntry::Color(c) = entry { Some((*c).into()) } else { None }
    }).collect();

    let project_file = ProjectFile {
        width: self.canvas_width,
        height: self.canvas_height,
        canvas: self.canvas.clone(),
        palette: current_palette,
        layers: Some(self.layers.clone()),
        active_layer_index: Some(self.active_layer_index),
    };

    if let Ok(json_data) = serde_json::to_string(&project_file) {
        if let Ok(file) = File::create(path) {
            let mut encoder = GzEncoder::new(file, Compression::default());
            if encoder.write_all(json_data.as_bytes()).is_ok() {
                if set_as_current { self.project_path = Some(path.clone()); }
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("file");
                self.status_message = Some((format!("Saved to {}", file_name), Instant::now()));
            } else {
                self.status_message = Some(("Error writing compressed data.".to_string(), Instant::now()));
            }
        } else {
            self.status_message = Some(("Error creating file.".to_string(), Instant::now()));
        }
    } else {
        self.status_message = Some(("Error serializing project.".to_string(), Instant::now()));
    }
}
fn load_project(&mut self, path: &PathBuf) {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => { self.status_message = Some((format!("Error reading file: {}", e), Instant::now())); return; }
    };

    let mut decoder = GzDecoder::new(file);
    let mut json_data = String::new();
    if decoder.read_to_string(&mut json_data).is_err() {
        self.status_message = Some(("File is not a valid compressed project.".to_string(), Instant::now()));
        return;
    }

    match serde_json::from_str::<ProjectFile>(&json_data) {
        Ok(project_file) => {
            self.canvas_width = project_file.width;
            self.canvas_height = project_file.height;
            self.canvas = project_file.canvas;
            
            if let Some(layers) = project_file.layers {
                self.layers = layers;
                self.active_layer_index = project_file.active_layer_index.unwrap_or(0);
                if self.active_layer_index >= self.layers.len() {
                    self.active_layer_index = 0;
                }
            } else {
                self.layers = vec![Layer {
                    name: "Layer 1".to_string(),
                    canvas: self.canvas.clone(),
                    visible: true,
                    opacity: 1.0,
                }];
                self.active_layer_index = 0;
            }
            self.sync_canvas_from_layers();
            let loaded_palette: Vec<PaletteEntry> = project_file.palette.into_iter()
                .map(|sc| PaletteEntry::Color(sc.into()))
                .collect();
            self.color_palette = loaded_palette;
            self.palette_index = 0;
            self.palette_scroll_state = 0;
            self.project_path = Some(path.clone());
            self.undo_stack.clear();
            self.redo_stack.clear();
            self.autosave_interval = None;
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("file");
            self.status_message = Some((format!("Loaded {}", file_name), Instant::now()));
        }
        Err(e) => { self.status_message = Some((format!("Error parsing project file: {}", e), Instant::now())); }
    }
}

    fn apply_config(&mut self, config: &Config) {
        self.pen_size_sensitivity = config.pen_size_sensitivity;
        self.opacity_sensitivity = config.opacity_sensitivity;
        self.pen_shape = config.pen_shape;
        self.highlighter_enabled = config.highlighter_enabled;
        self.highlighter_value = config.highlighter_value;
        self.highlighter_mode = config.highlighter_mode;
        self.shade_factor = config.shade_factor;
        self.protect_stroke = config.protect_stroke;
        self.apply_color_interval = chrono::Duration::milliseconds((config.apply_color_sec * 1000.0) as i64);
        self.minimap_mode = config.minimap_mode;
        self.mouse_events_enabled = config.mouse_events_enabled;
        self.color_mode = config.color_mode;
        self.default_palette_name = config.default_palette_name.clone();
        self.canvas_scroll_action = config.canvas_scroll_action;
        self.spray_size = config.spray_size;
        self.spray_speed = config.spray_speed;
        self.spray_intensity = config.spray_intensity;
        self.snap_to_palette = config.snap_to_palette;
        self.snap_to_palette_mode = config.snap_to_palette_mode;
        self.protect_color_transitions = config.protect_color_transitions;
        self.palette_menu_position = config.palette_menu_position;
        self.onion_skin_enabled = config.onion_skin_enabled;
        self.onion_skin_opacity = config.onion_skin_opacity;
        self.export_layer_mode = config.export_layer_mode;
    }

    fn save_current_config(&mut self) {
        let current_config = Config {
            pen_size_sensitivity: self.pen_size_sensitivity,
            opacity_sensitivity: self.opacity_sensitivity,
            pen_shape: self.pen_shape,
            highlighter_enabled: self.highlighter_enabled,
            highlighter_value: self.highlighter_value,
            highlighter_mode: self.highlighter_mode,
            shade_factor: self.shade_factor,
            protect_stroke: self.protect_stroke,
            apply_color_sec: self.apply_color_interval.num_milliseconds() as f32 / 1000.0,
            minimap_mode: self.minimap_mode,
            mouse_events_enabled: self.mouse_events_enabled,
            color_mode: self.color_mode,
            default_palette_name: self.default_palette_name.clone(),
            canvas_scroll_action: self.canvas_scroll_action,
            spray_size: self.spray_size,
            spray_speed: self.spray_speed,
            spray_intensity: self.spray_intensity,
            snap_to_palette: self.snap_to_palette,
            snap_to_palette_mode: self.snap_to_palette_mode,
            protect_color_transitions: self.protect_color_transitions,
            palette_menu_position: self.palette_menu_position,
            onion_skin_enabled: self.onion_skin_enabled,
            onion_skin_opacity: self.onion_skin_opacity,
            export_layer_mode: self.export_layer_mode,
        };

            if let Ok(path) = utils::get_config_path() {
                if let Ok(json_data) = serde_json::to_string_pretty(&current_config) {
                    if std::fs::write(path, json_data).is_ok() {
                        self.status_message = Some(("Configuration saved.".to_string(), Instant::now()));
                    } else {
                        self.status_message = Some(("Error: Could not write to config file.".to_string(), Instant::now()));
                    }
                }
            }
    }


fn generate_palette_from_image(&mut self, path: &PathBuf, add_to_current: bool) {
    let img = match image::open(path) {
        Ok(i) => i.into_rgb8(),
        Err(e) => {
            self.status_message = Some((format!("Error opening image: {}", e), Instant::now()));
            return;
        }
    };

    // --- NEW: K-Means Clustering Algorithm ---
    const TARGET_COLORS: usize = 16;
    const MAX_ITERATIONS: usize = 20;

    let mut color_counts = std::collections::HashMap::new();
    for pixel in img.pixels() {
        *color_counts.entry(pixel.0).or_insert(0) += 1;
    }
    let unique_colors: Vec<([u8; 3], u32)> = color_counts.into_iter().map(|(c, count)| (c, count as u32)).collect();

    if unique_colors.is_empty() {
        self.status_message = Some(("Image contains no colors.".to_string(), Instant::now()));
        return;
    }

    // K-Means++ Initialization: Intelligently select initial palette colors that are far apart.
    let mut palette: Vec<[f32; 3]> = Vec::with_capacity(TARGET_COLORS);
    let first_color = unique_colors[rand::thread_rng().gen_range(0..unique_colors.len())].0;
    palette.push([first_color[0] as f32, first_color[1] as f32, first_color[2] as f32]);

    while palette.len() < TARGET_COLORS {
        let mut max_dist = 0.0;
        let mut best_next_color = [0.0, 0.0, 0.0];
        for &(color, _) in &unique_colors {
            let color_f = [color[0] as f32, color[1] as f32, color[2] as f32];
            let dist_to_closest_center = palette.iter().map(|p| {
                (p[0] - color_f[0]).powi(2) + (p[1] - color_f[1]).powi(2) + (p[2] - color_f[2]).powi(2)
            }).fold(f32::INFINITY, f32::min);

            if dist_to_closest_center > max_dist {
                max_dist = dist_to_closest_center;
                best_next_color = color_f;
            }
        }
        palette.push(best_next_color);
    }
    
    // --- Iterative Refinement ---
    for _ in 0..MAX_ITERATIONS {
        let mut clusters = vec![(vec![], 0u32); TARGET_COLORS];
        
        for &(color, count) in &unique_colors {
            let color_f = [color[0] as f32, color[1] as f32, color[2] as f32];
            let closest_palette_index = palette.iter().enumerate().min_by(|(_, a), (_, b)| {
                let dist_a = (a[0] - color_f[0]).powi(2) + (a[1] - color_f[1]).powi(2) + (a[2] - color_f[2]).powi(2);
                let dist_b = (b[0] - color_f[0]).powi(2) + (b[1] - color_f[1]).powi(2) + (b[2] - color_f[2]).powi(2);
                dist_a.partial_cmp(&dist_b).unwrap()
            }).map(|(i, _)| i).unwrap_or(0);

            clusters[closest_palette_index].0.push((color, count));
        }

        for i in 0..TARGET_COLORS {
            if !clusters[i].0.is_empty() {
                let mut r_sum = 0.0;
                let mut g_sum = 0.0;
                let mut b_sum = 0.0;
                let mut total_weight = 0.0;
                for &(c, weight) in &clusters[i].0 {
                    r_sum += c[0] as f32 * weight as f32;
                    g_sum += c[1] as f32 * weight as f32;
                    b_sum += c[2] as f32 * weight as f32;
                    total_weight += weight as f32;
                }
                if total_weight > 0.0 {
                    palette[i] = [r_sum / total_weight, g_sum / total_weight, b_sum / total_weight];
                }
            }
        }
    }

    let new_palette: Vec<PaletteEntry> = palette.into_iter().map(|c| {
        PaletteEntry::Color(Color::Rgb(c[0] as u8, c[1] as u8, c[2] as u8))
    }).collect();

    self.last_generated_palette = Some(new_palette.clone());
    self.last_image_palette_source = path.file_stem().and_then(|s| s.to_str()).map(String::from);

    if add_to_current {
        self.add_palette_entries_uniquely(&new_palette);
    } else {
        self.color_palette = new_palette;
        self.palette_index = 0;
        self.palette_scroll_state = 0;
        self.status_message = Some(("Palette generated from image.".to_string(), Instant::now()));
    }
}
    fn save_last_generated_palette(&mut self, desired_name: Option<String>) {
        let Some(palette_entries) = self.last_generated_palette.as_ref() else {
            self.status_message = Some(("No image palette has been generated yet.".to_string(), Instant::now()));
            return;
        };

        let palette_name = desired_name.unwrap_or_else(|| {
            self.last_image_palette_source.as_ref().map_or_else(
                || "image_palette".to_string(),
                |name| format!("{}_palette", name)
            )
        });
        
        let palettes_dir = match utils::get_or_create_app_dir() {
            Ok(dir) => dir.join("palettes"),
            Err(_) => { self.status_message = Some(("Could not access palettes directory.".to_string(), Instant::now())); return; }
        };

        let file_path = palettes_dir.join(format!("{}.consolet", palette_name));
        let serializable_colors: Vec<SerializableColor> = palette_entries.iter().filter_map(|e| match e {
            PaletteEntry::Color(c) => Some((*c).into()),
            _ => None,
        }).collect();

        let palette_file = PaletteFile(serializable_colors);
        if let Ok(json_data) = serde_json::to_string_pretty(&palette_file) {
            if std::fs::write(&file_path, json_data).is_ok() {
                self.loaded_palettes.insert(palette_name.clone(), palette_entries.clone());
                self.status_message = Some((format!("Palette saved as '{}.consolet'", palette_name), Instant::now()));
            } else {
                self.status_message = Some(("Error writing palette file.".to_string(), Instant::now()));
            }
        }
    }


    fn save_current_palette(&mut self, palette_name: String) {
        if palette_name.is_empty() {
            self.status_message = Some(("Invalid palette name.".to_string(), Instant::now()));
            return;
        }

        let palettes_dir = match utils::get_or_create_app_dir() {
            Ok(dir) => dir.join("palettes"),
            Err(_) => { self.status_message = Some(("Could not access palettes directory.".to_string(), Instant::now())); return; }
        };

        let file_path = palettes_dir.join(format!("{}.consolet", palette_name));
        
        // Extract only the Color entries
        let serializable_colors: Vec<SerializableColor> = self.color_palette.iter().filter_map(|e| match e {
            PaletteEntry::Color(c) => Some((*c).into()),
            _ => None,
        }).collect();

        let palette_file = PaletteFile(serializable_colors);
        if let Ok(json_data) = serde_json::to_string_pretty(&palette_file) {
            if std::fs::write(&file_path, json_data).is_ok() {
                // Also update the in-memory loaded palettes
                self.loaded_palettes.insert(palette_name.clone(), self.color_palette.clone());
                self.status_message = Some((format!("Palette saved as '{}.consolet'", palette_name), Instant::now()));
            } else {
                self.status_message = Some(("Error writing palette file.".to_string(), Instant::now()));
            }
        }
    }




    fn add_palette_entries_uniquely(&mut self, entries_to_add: &[PaletteEntry]) {
        let mut new_colors_added = 0;
        for new_entry in entries_to_add {
            // Only consider colors for addition
            if let PaletteEntry::Color(new_color) = new_entry {
                let already_exists = self.color_palette.iter().any(|existing_entry| {
                    if let PaletteEntry::Color(existing_color) = existing_entry {
                        return existing_color == new_color;
                    }
                    false
                });

                if !already_exists {
                    self.color_palette.push(*new_entry);
                    new_colors_added += 1;
                }
            }
        }
        self.status_message = Some((format!("Added {} new colors to the palette.", new_colors_added), Instant::now()));
    }





fn export_to_png(&mut self, path: Option<String>, scale: u32, transparent: bool) {
        let Some(filename) = path else {
            self.status_message = Some(("Export failed: No filename provided.".to_string(), Instant::now()));
            return;
        };

        let scale = if scale == 0 { 1 } else { scale };
        
        match self.export_layer_mode {
            ExportLayerMode::United => {
                let img = RgbaImage::from_fn(self.canvas_width as u32 * scale, self.canvas_height as u32 * scale, |px, py| {
                    let x = (px / scale) as usize;
                    let y = (py / scale) as usize;
                    let pixel = self.layers[self.active_layer_index].canvas[y][x];


                    if transparent {
                        if pixel.alpha == 0.0 { return Rgba([0, 0, 0, 0]); }
                        let (r, g, b) = utils::to_rgb(pixel.color.into());
                        let alpha = (pixel.alpha * 255.0).round() as u8;
                        Rgba([r, g, b, alpha])
                    } else {
                        let bg_color = Color::Black;
                        let final_color = utils::blend_colors(bg_color, pixel.color.into(), pixel.alpha);
                        let (r, g, b) = utils::to_rgb(final_color);
                        Rgba([r, g, b, 255])
                    }
                });

                match img.save(&filename) {
                    Ok(_) => self.status_message = Some((format!("Exported to {}", filename), Instant::now())),
                    Err(e) => self.status_message = Some((format!("Error exporting file: {}", e), Instant::now())),
                }
            }
            ExportLayerMode::Separate => {
                let base_path = PathBuf::from(&filename);
                let parent = base_path.parent().unwrap_or(std::path::Path::new("."));
                let stem = base_path.file_stem().and_then(|s| s.to_str()).unwrap_or("export");
                
                for (idx, layer) in self.layers.iter().enumerate() {
                    if !layer.visible {
                        continue;
                    }
                    
                    let layer_filename = parent.join(format!("{}_{}.png", stem, idx + 1));
                    let img = RgbaImage::from_fn(self.canvas_width as u32 * scale, self.canvas_height as u32 * scale, |px, py| {
                        let x = (px / scale) as usize;
                        let y = (py / scale) as usize;
                        let pixel = layer.canvas[y][x];

                        if transparent {
                            if pixel.alpha == 0.0 { return Rgba([0, 0, 0, 0]); }
                            let (r, g, b) = utils::to_rgb(pixel.color.into());
                            let alpha = (pixel.alpha * layer.opacity * 255.0).round() as u8;
                            Rgba([r, g, b, alpha])
                        } else {
                            let bg_color = Color::Black;
                            let final_color = utils::blend_colors(bg_color, pixel.color.into(), pixel.alpha * layer.opacity);
                            let (r, g, b) = utils::to_rgb(final_color);
                            Rgba([r, g, b, 255])
                        }
                    });

                    if let Err(e) = img.save(&layer_filename) {
                        self.status_message = Some((format!("Error exporting layer {}: {}", idx + 1, e), Instant::now()));
                        return;
                    }
                }
                self.status_message = Some((format!("Exported {} layers", self.layers.iter().filter(|l| l.visible).count()), Instant::now()));
}
}
}


}














fn main() -> Result<()> {

    if !utils::check_terminal_support()? { return Ok(()); }
    let _ = utils::export_default_palettes_if_missing();
    let _ = script_handler::create_default_script_if_missing();

    stdout().execute(EnterAlternateScreen)?.execute(event::EnableMouseCapture)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let mut app = App::new();

    if let Ok(path) = keybindings::Keybindings::get_path() {
        if !path.exists() {
            // This is likely the first run, save the defaults.
            // We ignore the result, as it's not critical if this fails.
            let _ = app.keybindings.save();
        }
    }
    if let Ok(config_path) = utils::get_config_path() {
            if config_path.exists() {
                if let Ok(json_data) = std::fs::read_to_string(config_path) {
                    if let Ok(config) = serde_json::from_str::<Config>(&json_data) {
                        app.apply_config(&config);
                    }
                }
            }
        }

    if let Some(palette) = app.loaded_palettes.get(&app.default_palette_name).cloned() {
        app.color_palette = palette;
    }


    while !app.should_quit {
            if let Some(interval) = app.autosave_interval {
                if app.last_autosave_time.elapsed() >= interval {
                    if let Some(path) = app.project_path.clone() {
                        app.save_project(&path, false); // false = don't show status message
                        app.last_autosave_time = Instant::now();
                    }
                }
            }

            if app.is_space_held || app.is_spraying {
                if let Some(last_time) = app.last_apply_time {
                    if Local::now() > last_time + app.apply_color_interval {
                        if app.is_space_held {
                            let original_protection = app.protect_stroke;
                            app.protect_stroke = false;
                            app.use_current_tool();
                            app.protect_stroke = original_protection;
                        } else if app.is_spraying {
                            app.apply_spray();
                        }
                        app.last_apply_time = Some(Local::now());
                    }
                }
            }
            terminal.draw(|frame| ui(frame, &mut app))?;
            controller::handle_events(&mut app)?;
        }

        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?.execute(event::DisableMouseCapture)?;
        Ok(())
}

fn ui(frame: &mut Frame, app: &mut App) {
    if let AppMode::HelpScreen = app.mode {
        draw_help_screen(frame, app);
        return;
    }

    if let AppMode::Keybindings = app.mode {
        draw_keybindings_screen(frame, app);
        return;
    }


    if let AppMode::ConfigEditor = app.mode {
        config::draw_config_screen(frame, app);
        return;
    }

    if let AppMode::ScriptEditor = app.mode {
        script_handler::draw_script_editor(frame, app);
        return;
    }

    if let AppMode::FileBrowser = app.mode {
        file_browser::draw_browser(frame, app);
        return;
    }


    if let AppMode::ConfirmConfigSave = app.mode {
        draw_confirmation_dialog(frame, app, "Save configuration changes?");
        return;
    }
    if let AppMode::ConfirmScriptSave = app.mode {
        draw_confirmation_dialog(frame, app, "Save script changes?");
        return;
    }

    if let AppMode::ConfirmKeybindingSave = app.mode {
        // Draw the main UI first to have a background
        // ... (your existing main UI drawing logic) ...
        draw_confirmation_dialog(frame, app, "Save keybinding changes?");
        return;
    }


    const MIN_CANVAS_WIDTH: u16 = 20;
    const MIN_CANVAS_HEIGHT: u16 = 10;
    const SIDE_PANEL_WIDTH: u16 = 22;

app.is_side_panel_visible = frame.size().width > MIN_CANVAS_WIDTH + SIDE_PANEL_WIDTH && frame.size().height > MIN_CANVAS_HEIGHT;

let main_layout = Layout::default().direction(Direction::Vertical).constraints([Constraint::Min(0), Constraint::Length(3)]).split(frame.size());
let content_area = main_layout[0];
let bottom_bar_area = main_layout[1];

let (canvas_panel_area, palette_area_option) = if app.is_side_panel_visible {
    let constraints_left = [Constraint::Max(SIDE_PANEL_WIDTH), Constraint::Min(0)];
    let constraints_right = [Constraint::Min(0), Constraint::Max(SIDE_PANEL_WIDTH)];
    
    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(if app.palette_menu_position == PaletteMenuPosition::Left {
            constraints_left
        } else {
            constraints_right
        })
        .split(content_area);

    if app.palette_menu_position == PaletteMenuPosition::Left {
        (top_layout[1], Some(top_layout[0]))
    } else {
        (top_layout[0], Some(top_layout[1]))
    }
} else {
    (content_area, None)
};

let canvas_container_block = Block::default().borders(Borders::ALL).title(Title::from(" Canvas ").alignment(Alignment::Center));
let pixel_area = canvas_container_block.inner(canvas_panel_area);
frame.render_widget(canvas_container_block, canvas_panel_area);

if app.last_pixel_area.is_none() {
    app.last_pixel_area = Some(pixel_area);
}

if app.last_pixel_area.map_or(true, |last| last.width != pixel_area.width || last.height != pixel_area.height) {
    if app.canvas_width > 0 && app.canvas_height > 0 {
        let max_zoom_x = pixel_area.width / app.canvas_width as u16;
        let max_zoom_y = (pixel_area.height * PIXEL_WIDTH) / app.canvas_height as u16;
        let mut new_zoom = max_zoom_x.min(max_zoom_y);
        new_zoom = new_zoom.max(2);
        new_zoom = (new_zoom / 2) * 2;
        app.zoom_level = new_zoom;
        app.view_offset_x = 0;
        app.view_offset_y = 0;
    }
}
app.last_pixel_area = Some(pixel_area);

app.clamp_view_offsets(pixel_area.width, pixel_area.height);

let pixel_render_height = (app.zoom_level / PIXEL_WIDTH).max(1);
let canvas_screen_width = app.canvas_width as u16 * app.zoom_level;
let canvas_screen_height = app.canvas_height as u16 * pixel_render_height;
let canvas_area_x = pixel_area.x + pixel_area.width.saturating_sub(canvas_screen_width) / 2;
let canvas_area_y = pixel_area.y + pixel_area.height.saturating_sub(canvas_screen_height) / 2;
let centered_canvas_rect = Rect::new(canvas_area_x, canvas_area_y, canvas_screen_width, canvas_screen_height);
app.last_centered_canvas_rect = Some(centered_canvas_rect);

// --- Correct, Symmetrical Border Drawing ---
let border_rect = Rect {
    x: centered_canvas_rect.x.saturating_sub(1),
    y: centered_canvas_rect.y.saturating_sub(1),
    width: centered_canvas_rect.width + 2,
    height: centered_canvas_rect.height + 2,
};
let clipped_border_area = pixel_area.intersection(border_rect);
frame.render_widget(
    Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)),
    clipped_border_area,
);

// --- Canvas Content Drawing ---
let draw_area = pixel_area.intersection(centered_canvas_rect);
for screen_y in (draw_area.top()..draw_area.bottom()).step_by(pixel_render_height as usize) {
    for screen_x_start in (draw_area.left()..draw_area.right()).step_by(app.zoom_level as usize) {
        let canvas_x_i32 = app.view_offset_x + ((screen_x_start - centered_canvas_rect.x) / app.zoom_level) as i32;
        let canvas_y_i32 = app.view_offset_y + ((screen_y - centered_canvas_rect.y) / pixel_render_height) as i32;

        if canvas_x_i32 >= 0 && canvas_x_i32 < app.canvas_width as i32 && canvas_y_i32 >= 0 && canvas_y_i32 < app.canvas_height as i32 {
            let (canvas_x, canvas_y) = (canvas_x_i32 as usize, canvas_y_i32 as usize);
            let mut pixel = app.canvas[canvas_y][canvas_x];
            
            if app.onion_skin_enabled && app.active_layer_index > 0 {
                let prev_layer = &app.layers[app.active_layer_index - 1];
                if prev_layer.visible {
                    let prev_pixel = prev_layer.canvas[canvas_y][canvas_x];
                    if prev_pixel.alpha > 0.0 {
                        let onion_color = utils::blend_colors(Color::Black, prev_pixel.color.into(), prev_pixel.alpha);
                        if pixel.alpha == 0.0 {
                            pixel.color = onion_color.into();
                            pixel.alpha = app.onion_skin_opacity;
                        } else {
                            let blended = utils::blend_colors(pixel.color.into(), onion_color, app.onion_skin_opacity * 0.3);
                            pixel.color = blended.into();
                        }
                    }
                }
            }
            
            let mut final_color = if pixel.alpha > 0.0 { utils::blend_colors(Color::Black, pixel.color.into(), pixel.alpha) } else { Color::Reset };
            
            // For diagonal lines, we still blend the background
            match app.symmetry_mode {
                SymmetryMode::DiagonalForward(c) if canvas_y_i32 == canvas_x_i32 + c => { final_color = utils::blend_colors(final_color, Color::Yellow, 0.4); }
                SymmetryMode::DiagonalBackward(c) if canvas_y_i32 == -canvas_x_i32 + c => { final_color = utils::blend_colors(final_color, Color::Yellow, 0.4); }
                _ => {}
            }
            
            let block_width = app.zoom_level.min(draw_area.right() - screen_x_start);
            let block_height = pixel_render_height.min(draw_area.bottom() - screen_y);
            frame.render_widget(Block::default().bg(app.translate_color(final_color)), Rect::new(screen_x_start, screen_y, block_width, block_height));
        }
    }
}

// --- New, Thin Symmetry Line Overlay Drawing ---
match app.symmetry_mode {
    SymmetryMode::Vertical(line_x) => {
        let mut line_screen_x = centered_canvas_rect.x + (line_x * app.zoom_level);
        // For even-width canvases, the true center is between pixels. Shift the visual line left to appear on the boundary.
        if app.canvas_width % 2 == 0 {
            line_screen_x = line_screen_x.saturating_sub(1);
        }
        if line_screen_x >= draw_area.left() && line_screen_x < draw_area.right() {
            for y in draw_area.top()..draw_area.bottom() {
                frame.render_widget(Paragraph::new("┃").style(Style::default().fg(Color::Blue)), Rect::new(line_screen_x, y, 1, 1));
            }
        }
    }
    SymmetryMode::Horizontal(line_y) => {
        let mut line_screen_y = centered_canvas_rect.y + (line_y * pixel_render_height);
        // For even-height canvases, shift the visual line up to appear on the boundary.
        if app.canvas_height % 2 == 0 {
            line_screen_y = line_screen_y.saturating_sub(1);
        }
        if line_screen_y >= draw_area.top() && line_screen_y < draw_area.bottom() {
            for x in draw_area.left()..draw_area.right() {
                frame.render_widget(Paragraph::new("━").style(Style::default().fg(Color::Blue)), Rect::new(x, line_screen_y, 1, 1));
            }
        }
    }
    _ => {} // Diagonals are handled by blending above
}

let should_draw_minimap = match app.minimap_mode {
    MinimapMode::On => true,
    MinimapMode::Off => false,
    MinimapMode::Auto => app.canvas_width >= 100 && app.canvas_height >= 100,
};

if should_draw_minimap && pixel_area.width > 20 && pixel_area.height > 10 {
    let minimap_width = (pixel_area.width / 4).max(10);
    let minimap_height = (pixel_area.height / 3).max(5);
    let minimap_area = Rect::new(
        pixel_area.right() - minimap_width,
        pixel_area.bottom() - minimap_height,
        minimap_width,
        minimap_height,
    );
    frame.render_widget(Clear, minimap_area);
    draw_minimap(frame, app, minimap_area);
}
if let AppMode::Drawing = app.mode {
    let cursor_screen_x = ((app.cursor_pos.0 as i32 - app.view_offset_x) * app.zoom_level as i32) + centered_canvas_rect.x as i32;
    let cursor_screen_y = ((app.cursor_pos.1 as i32 - app.view_offset_y) * pixel_render_height as i32) + centered_canvas_rect.y as i32;
    if (app.cursor_pos.0 as usize) < app.canvas_width && (app.cursor_pos.1 as usize) < app.canvas_height {
        let offset = app.pen_size as i32 / 2;
        let brush_start_canvas_x = app.cursor_pos.0 as i32 - offset;
        let brush_start_canvas_y = app.cursor_pos.1 as i32 - offset;
        let brush_start_screen_x = ((brush_start_canvas_x - app.view_offset_x) * app.zoom_level as i32) + centered_canvas_rect.x as i32;
        let brush_start_screen_y = ((brush_start_canvas_y - app.view_offset_y) * pixel_render_height as i32) + centered_canvas_rect.y as i32;
        let brush_screen_width = app.pen_size * app.zoom_level;
        let brush_screen_height = app.pen_size * pixel_render_height;
        let brush_outline_rect = Rect::new(brush_start_screen_x as u16, brush_start_screen_y as u16, brush_screen_width, brush_screen_height);
        let brush_outline_block = Block::default().borders(Borders::ALL).border_style(Style::default().fg(app.translate_color(Color::Yellow)));
        if brush_outline_rect.intersects(pixel_area) { frame.render_widget(brush_outline_block, brush_outline_rect); }
        let center_cursor_rect = Rect::new(cursor_screen_x as u16, cursor_screen_y as u16, app.zoom_level, pixel_render_height);
        if center_cursor_rect.intersects(pixel_area) {
            match app.current_selection {
                PaletteEntry::Color(c) => {
                    let original_pixel = app.canvas[app.cursor_pos.1 as usize][app.cursor_pos.0 as usize];
                    let original_color: Color = original_pixel.color.into();
                    let display_color = utils::blend_colors(original_color, c, app.opacity);
                    frame.render_widget(Block::default().bg(app.translate_color(display_color)), center_cursor_rect);
                }
                PaletteEntry::Tool(tool) => {
                    let original_pixel = app.canvas[app.cursor_pos.1 as usize][app.cursor_pos.0 as usize];
                    let original_color: Color = original_pixel.color.into();
                    if original_pixel.alpha == 0.0 {
                        frame.render_widget(Block::default().bg(original_color), center_cursor_rect);
                        if app.highlighter_enabled && app.highlighter_mode == HighlighterMode::Underscore {
                            let underscore_rect = Rect::new(center_cursor_rect.x, center_cursor_rect.bottom().saturating_sub(1), center_cursor_rect.width, 1);
                            let p = Paragraph::new("_".repeat(app.zoom_level as usize)).style(Style::default().fg(app.translate_color(Color::Yellow)));
                            frame.render_widget(p, underscore_rect);
                        }
                    } else {
                        let final_color = match tool {
                            Tool::Lighter => utils::blend_colors(original_color, Color::White, app.shade_factor),
                            Tool::Darker => utils::blend_colors(original_color, Color::Black, app.shade_factor),
                            Tool::Blur => { let mut r_sum = 0u32; let mut g_sum = 0u32; let mut b_sum = 0u32; let mut count = 0u32; for dy in -1..=1 { for dx in -1..=1 { let nx = app.cursor_pos.0 as i32 + dx; let ny = app.cursor_pos.1 as i32 + dy; if nx >= 0 && nx < app.canvas_width as i32 && ny >= 0 && ny < app.canvas_height as i32 { let neighbor_pixel = app.canvas[ny as usize][nx as usize]; if neighbor_pixel.alpha > 0.0 { let (r, g, b) = utils::to_rgb(neighbor_pixel.color.into()); r_sum += r as u32; g_sum += g as u32; b_sum += b as u32; count += 1; } } } } if count > 0 { Color::Rgb((r_sum / count) as u8, (g_sum / count) as u8, (b_sum / count) as u8) } else { original_color } }
                        };
                        if app.highlighter_enabled {
                            match app.highlighter_mode {
                                HighlighterMode::Underscore => {
                                    frame.render_widget(Block::default().bg(original_color), center_cursor_rect);
                                    let underscore_rect = Rect::new(center_cursor_rect.x, center_cursor_rect.bottom().saturating_sub(1), center_cursor_rect.width, 1);
                                    let p = Paragraph::new("_".repeat(app.zoom_level as usize)).style(Style::default().fg(app.translate_color(Color::Yellow)).bg(app.translate_color(original_color)));
                                    frame.render_widget(p, underscore_rect);
                                }
                                HighlighterMode::Blend => {
                                    let display_color = utils::blend_colors(original_color, final_color, app.highlighter_value);
                                    frame.render_widget(Block::default().bg(app.translate_color(display_color)), center_cursor_rect);
                                }
                            }
                        } else { frame.render_widget(Block::default().bg(app.translate_color(final_color)), center_cursor_rect); }
                    }
                }
            }
        }
    }
}

if let Some(palette_area) = palette_area_option {
    let palette_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(8), Constraint::Length(8)])
        .split(palette_area);
    
    let tool_area = palette_layout[0];
    let color_area = palette_layout[1];
    let layer_area = palette_layout[2];
    
    let tool_block = Block::default().borders(Borders::ALL).title(Title::from(" Tools ").alignment(Alignment::Center)).border_style(match app.mode { AppMode::ToolPicker => Style::default().fg(app.translate_color(Color::Yellow)), _ => Style::default() });
    let actual_tool_area = tool_block.inner(tool_area);
    frame.render_widget(tool_block, tool_area);
    app.last_tool_area = Some(actual_tool_area);
    
    for (i, entry) in app.tool_palette.iter().enumerate() {
        let is_selected = i == app.tool_index;
        let symbol = if is_selected { ">" } else { " " };
        let item_text = match entry {
            PaletteEntry::Tool(Tool::Lighter) => Span::styled(format!("{}L", symbol), Style::default().bg(if is_selected { app.translate_color(Color::DarkGray) } else { Color::Reset })),
            PaletteEntry::Tool(Tool::Darker) => Span::styled(format!("{}D", symbol), Style::default().bg(if is_selected { app.translate_color(Color::DarkGray) } else { Color::Reset })),
            PaletteEntry::Tool(Tool::Blur) => Span::styled(format!("{}B", symbol), Style::default().bg(if is_selected { app.translate_color(Color::DarkGray) } else { Color::Reset })),
            _ => Span::raw(""),
        };
        let x = actual_tool_area.x + (i * 3) as u16;
        frame.render_widget(Paragraph::new(item_text), Rect::new(x, actual_tool_area.y, 3, 1));
    }

    let color_block = Block::default().borders(Borders::ALL).title(Title::from(" Colors ").alignment(Alignment::Center)).border_style(match app.mode { AppMode::ColorPicker => Style::default().fg(app.translate_color(Color::Yellow)), _ => Style::default() });
    let actual_color_area = color_block.inner(color_area);
    frame.render_widget(color_block, color_area);
    app.last_palette_area = Some(actual_color_area);
    
    let columns = (actual_color_area.width / 3).max(1) as usize;
    let rows = actual_color_area.height as usize;
    
    for i in app.palette_scroll_state..app.color_palette.len() {
        let entry = &app.color_palette[i];
        let row = (i - app.palette_scroll_state) / columns;
        let col = i % columns;
        if row >= rows { break; }
        let is_selected = i == app.palette_index;
        let symbol = if is_selected { ">" } else { " " };
        let item_text = match entry {
            PaletteEntry::Color(c) => Span::styled(
                format!("{}█", symbol),
                Style::default().fg(app.translate_color(*c)).bg(if is_selected { app.translate_color(Color::DarkGray) } else { Color::Reset }),
            ),
            _ => Span::raw(""),
        };
        let x = actual_color_area.x + (col * 3) as u16;
        let y = actual_color_area.y + row as u16;
        frame.render_widget(Paragraph::new(item_text), Rect::new(x, y, 3, 1));
    }



    let layer_block = Block::default()
        .borders(Borders::ALL)
        .title(Title::from(" Layers ").alignment(Alignment::Center));
    let actual_layer_area = layer_block.inner(layer_area);
    frame.render_widget(layer_block, layer_area);
    app.last_layer_area = Some(actual_layer_area);
    
    let visible_rows = actual_layer_area.height.saturating_sub(2) as usize;
    let start_idx = app.layer_scroll_state;
    let end_idx = (start_idx + visible_rows).min(app.layers.len());
    
    for (list_idx, layer_idx) in (start_idx..end_idx).enumerate() {
        let layer = &app.layers[layer_idx];
        let is_selected = layer_idx == app.active_layer_index;
        let symbol = if is_selected { ">" } else { " " };
        let visibility = if layer.visible { "â—" } else { "â—‹" };
        let text = format!("{}{} {}", symbol, visibility, layer.name);
        let style = if is_selected {
            Style::default().bg(app.translate_color(Color::DarkGray))
        } else {
            Style::default()
        };
        let y = actual_layer_area.y + list_idx as u16;
        if y < actual_layer_area.bottom() {
            frame.render_widget(
                Paragraph::new(text).style(style),
                Rect::new(actual_layer_area.x, y, actual_layer_area.width, 1)
            );
        }
    }
    
    if app.onion_skin_enabled {
        let onion_y = actual_layer_area.bottom().saturating_sub(2);
        if onion_y >= actual_layer_area.y {
            let onion_text = format!("Onion: {:.0}%", app.onion_skin_opacity * 100.0);
            frame.render_widget(
                Paragraph::new(onion_text).style(Style::default().fg(app.translate_color(Color::Cyan))),
                Rect::new(actual_layer_area.x, onion_y, actual_layer_area.width, 1)
            );
        }
    }





}
    if let AppMode::Command = app.mode {
        draw_command_screen(frame, app);
    } else {

        if let Some((_, timestamp)) = &app.status_message {
            if timestamp.elapsed() > std::time::Duration::from_secs(2) {
                app.status_message = None;
            }
        }

        let symmetry_text = match app.symmetry_mode {
            SymmetryMode::Off => "Off".to_string(),
            SymmetryMode::Horizontal(y) => format!("Horizontal @ Y={}", y),
            SymmetryMode::Vertical(x) => format!("Vertical @ X={}", x),
            SymmetryMode::DiagonalForward(c) => format!("Diag-Fwd @ c={}", c),
            SymmetryMode::DiagonalBackward(c) => format!("Diag-Bwd @ c={}", c),
        };
        let help_text = if let Some((msg, _)) = &app.status_message { msg.clone() } else {
            match app.mode {
                AppMode::Drawing => format!("({}, {}) | Pen: {} | Opacity: {:.0}% | Zoom: {}x | Symmetry:[{}]", app.cursor_pos.0, app.cursor_pos.1, app.pen_size, app.opacity * 100.0, app.zoom_level / 2, symmetry_text),
                AppMode::ResizingWidth => format!("New Width ({}x{}): {}", app.canvas_width, app.canvas_height, app.input_buffer),
                AppMode::ResizingHeight => format!("New Height ({}x{}): {}", app.temp_width, app.input_buffer, app.input_buffer),
                AppMode::ConfirmOverwrite => "File exists. Overwrite? (y/n)".to_string(),
                AppMode::ColorPicker => {
                    let key_str = app.keybindings.map.get(&Action::OpenColorPicker)
                        .map(utils::format_keybinding)
                        .unwrap_or_else(|| "N/A".to_string());
                    format!("Arrows: Navigate | Enter: Select | Esc/{}: Back", key_str)
                },
                AppMode::ToolPicker => {
                    let key_str = app.keybindings.map.get(&Action::OpenToolPicker)
                        .map(utils::format_keybinding)
                        .unwrap_or_else(|| "N/A".to_string());
                    format!("Arrows: Navigate | Enter: Select | Esc/{}: Back", key_str)
                },
                _ => "".to_string(),
            }
        };
        let help_block = Block::default().borders(Borders::ALL).title(Title::from(" Controls ").alignment(Alignment::Center));
        frame.render_widget(Paragraph::new(help_text).block(help_block), bottom_bar_area);
    }
}







// func


fn draw_command_screen(frame: &mut Frame, app: &App) {
    let input_bar_area = Rect {
        x: frame.size().x,
        y: frame.size().height.saturating_sub(3),
        width: frame.size().width,
        height: 3,
    };
    let input_text = vec![Line::from(vec![Span::raw("> "), Span::raw(app.input_buffer.as_str())])];
    let input_paragraph = Paragraph::new(input_text).block(Block::default().borders(Borders::ALL).title("Command Mode"));
    
    frame.render_widget(Clear, input_bar_area);
    frame.render_widget(input_paragraph, input_bar_area);
    let cursor_offset = app.input_buffer[..app.command_cursor_pos].graphemes(true).count() as u16;
    frame.set_cursor(input_bar_area.x + 2 + cursor_offset, input_bar_area.y + 1);

    let suggestions = app.get_suggestions(&app.input_buffer);


    if !suggestions.is_empty() {
        let max_suggestion_width = suggestions.iter().map(|s| s.len()).max().unwrap_or(0);
        let box_width = (max_suggestion_width + 4) as u16;
        let box_height = (suggestions.len() + 2) as u16;
        let suggestions_area = Rect {
            x: input_bar_area.x + 2,
            y: input_bar_area.y.saturating_sub(box_height),
            width: box_width,
            height: box_height,
        };

        let suggestion_items: Vec<Line> = suggestions.iter().enumerate()
            .map(|(i, s)| {

                let style = if app.suggestion_active && i == app.suggestion_index { 
                    Style::default().fg(app.translate_color(Color::Black)).bg(app.translate_color(Color::Yellow)) 
                } else { 
                    Style::default() 
                };

                Line::from(Span::styled(s, style))
            })
            .collect();
        
        let suggestions_paragraph = Paragraph::new(suggestion_items).block(Block::default().borders(Borders::ALL).title("Suggestions"));
        frame.render_widget(suggestions_paragraph, suggestions_area);

        
        let mut info_text: Option<Text> = None;
        let command_name_to_show = if app.suggestion_active && !suggestions.is_empty() {
            let s = &suggestions[app.suggestion_index];
            s.split_once(' ').map(|(c, _)| c).unwrap_or(s)
        } else {
            app.input_buffer.split_once('=').map(|(c, _)| c).unwrap_or(&app.input_buffer)
        };

        if let Some(cmd) = COMMANDS.iter().find(|c| c.name == command_name_to_show) {
            info_text = Some(Text::from(vec![
                Line::from(Span::styled(cmd.name, Style::default().bold())),
                Line::from(cmd.description),
                Line::from(Span::styled(format!("Usage: {}", cmd.usage), Style::default().fg(app.translate_color(Color::Yellow)))),
                Line::from(Span::styled(format!("Example: {}", cmd.example), Style::default().fg(app.translate_color(Color::Cyan)))),
            ]));
        }

        if let Some(text) = info_text {
            let box_height = 6;
            let info_area = Rect {
                x: input_bar_area.x,
                y: suggestions_area.y.saturating_sub(box_height),
                width: frame.size().width,
                height: box_height,
            };
            let info_paragraph = Paragraph::new(text)
                .block(Block::default().borders(Borders::ALL).title("Command Info"))
                .wrap(ratatui::widgets::Wrap { trim: true });
            frame.render_widget(info_paragraph, info_area);
        }

    }
}
fn draw_help_screen(frame: &mut Frame, app: &mut App) {
    let help_text = match utils::get_help_sheet_path() {
        Ok(path) => {
            match std::fs::read_to_string(&path) {
                Ok(content) => content, // File exists, use its content
                Err(_) => { // File doesn't exist or is unreadable
                    let default_content = help_sheet::get_default_help_text();
                    // Attempt to create it for next time
                    let _ = std::fs::write(path, default_content);
                    // Use the default content for this session
                    default_content.to_string()
                }
            }
        },
        Err(_) => "Error: Could not determine help sheet path.".to_string(),
    };

    let block = Block::default().title(" Help ").borders(Borders::ALL).border_style(Style::default().fg(app.translate_color(Color::Yellow)));
    let paragraph = Paragraph::new(help_text)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: false })
        .scroll((app.help_scroll, 0));

    let area = utils::centered_rect(80, 90, frame.size());
    frame.render_widget(Clear, area);
    frame.render_widget(paragraph, area);
}


fn draw_minimap(frame: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title("Minimap");
    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    if app.canvas_width == 0 || app.canvas_height == 0 || inner_area.width < 1 || inner_area.height < 1 {
        return;
    }

    let scale_x = app.canvas_width as f32 / inner_area.width as f32;
    let scale_y = app.canvas_height as f32 / (inner_area.height as f32 * 2.0);

    let Some(last_pixel_area) = app.last_pixel_area else { return };
    let pixel_render_height = (app.zoom_level / PIXEL_WIDTH).max(1);
    let visible_pixels_x = (last_pixel_area.width as f32 / app.zoom_level as f32) as i32;
    let visible_pixels_y = (last_pixel_area.height as f32 / pixel_render_height as f32) as i32;

    let get_color_for_region = |start_x: usize, end_x: usize, start_y: usize, end_y: usize| -> Option<Color> {
        for y in start_y..end_y.min(app.canvas_height) {
            for x in start_x..end_x.min(app.canvas_width) {
                if app.canvas[y][x].alpha > 0.0 {
                    let pixel = app.canvas[y][x];
                    return Some(utils::blend_colors(Color::Black, pixel.color.into(), pixel.alpha));
                }
            }
        }
        None
    };

    for my in 0..inner_area.height {
        for mx in 0..inner_area.width {
            let region_start_x = (mx as f32 * scale_x) as usize;
            let region_end_x = ((mx + 1) as f32 * scale_x) as usize;

            let region_start_y_top = (my as f32 * 2.0 * scale_y) as usize;
            let region_end_y_top = ((my as f32 * 2.0 + 1.0) * scale_y) as usize;
            let mut top_color = get_color_for_region(region_start_x, region_end_x, region_start_y_top, region_end_y_top)
                .unwrap_or(Color::Reset);

            let region_start_y_bot = ((my as f32 * 2.0 + 1.0) * scale_y) as usize;
            let region_end_y_bot = ((my as f32 * 2.0 + 2.0) * scale_y) as usize;
            let mut bottom_color = get_color_for_region(region_start_x, region_end_x, region_start_y_bot, region_end_y_bot)
                .unwrap_or(Color::Reset);

            // Efficient rectangle intersection instead of checking every pixel
            let viewport_left = app.view_offset_x;
            let viewport_right = app.view_offset_x + visible_pixels_x;
            let viewport_top = app.view_offset_y;
            let viewport_bottom = app.view_offset_y + visible_pixels_y;

            let region_left = region_start_x as i32;
            let region_right = region_end_x as i32;

            let is_top_in_view = region_start_y_top < viewport_bottom as usize 
                && region_end_y_top > viewport_top as usize
                && region_left < viewport_right
                && region_right > viewport_left;

            let is_bot_in_view = region_start_y_bot < viewport_bottom as usize 
                && region_end_y_bot > viewport_top as usize
                && region_left < viewport_right
                && region_right > viewport_left;

            if is_top_in_view { top_color = app.translate_color(utils::blend_colors(top_color, Color::Yellow, 0.4)); }
            if is_bot_in_view { bottom_color = app.translate_color(utils::blend_colors(bottom_color, Color::Yellow, 0.4)); }

            let style = Style::default().fg(app.translate_color(top_color)).bg(app.translate_color(bottom_color));
            frame.render_widget(Paragraph::new("▀").style(style), Rect::new(inner_area.x + mx, inner_area.y + my, 1, 1));
        }
    }
}


    fn parse_and_execute_save(app: &mut App, command: &str) {
        let parts: Vec<&str> = command.split_whitespace().collect();
        // NEW: Open explorer if no filename or --explorer is provided
        if parts.len() < 2 || parts.contains(&"--explorer") {
            file_browser::open_browser(app, file_browser::BrowserMode::Save);
            return;
        }
        
        let mut filename = parts[1].to_string();
        if !filename.ends_with(".consolet") {
            filename.push_str(".consolet");
        }
        let mut custom_path = None;
        let mut force_overwrite = false;
        let mut autosave_mins = None;

        let mut i = 2;
        while i < parts.len() {
            match parts[i] {
                "-p" => { i += 1; if i < parts.len() { custom_path = Some(parts[i].replace("\"", "")); } },
                "-f" => force_overwrite = true,
                "-a" => { i += 1; if i < parts.len() { autosave_mins = parts[i].parse::<u64>().ok(); } },
                _ => {}
            }
            i += 1;
        }

        let path = match custom_path {
            Some(p) => PathBuf::from(shellexpand::tilde(&p).into_owned()).join(&filename),
            None => utils::get_or_create_app_dir().unwrap().join("saved_projects").join(&filename),
        };

        if path.exists() && !force_overwrite {
            app.pending_save_path = Some(path);
            app.mode = AppMode::ConfirmOverwrite;
            return;
        }

        if let Some(mins) = autosave_mins {
            app.autosave_interval = Some(std::time::Duration::from_secs(mins * 60));
            app.last_autosave_time = Instant::now();
        }
        app.save_project(&path, true);
    }

fn parse_and_execute_load(app: &mut App, command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    // NEW: Open explorer if no filename or --explorer is provided
    if parts.len() < 2 || parts.contains(&"--explorer") {
        file_browser::open_browser(app, file_browser::BrowserMode::Load);
        return;
    }
    
    let filename = parts[1].replace("\"", "");
    let mut path = PathBuf::from(&filename);
    
    if !path.is_absolute() {
        let default_path = utils::get_or_create_app_dir().unwrap().join("saved_projects").join(&filename);
        if default_path.exists() {
            path = default_path;
        }
    }
    
    if path.exists() {
        app.load_project(&path);
    } else {
        app.status_message = Some((format!("File not found: {}", filename), Instant::now()));
    }
}


fn parse_and_execute_export(app: &mut App, command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let mut output_path_str: Option<String> = None;
    let mut upscale: u32 = 1;
    let mut with_background = false;

    // NEW: If "export" is typed alone or with --explorer, open the browser.
    if parts.len() == 1 || parts.contains(&"--explorer") {
        file_browser::open_browser(app, file_browser::BrowserMode::Export);
        return;
    }

    // --- Keep the existing argument parsing logic ---
    let mut i = 1;
    while i < parts.len() {
        match parts[i] {
            "-o" => {
                if i + 1 >= parts.len() { app.status_message = Some(("Error: -o requires a path.".to_string(), Instant::now())); return; }
                output_path_str = Some(parts[i + 1].to_string());
                i += 2;
            },
            "-u" => {
                if i + 1 >= parts.len() { app.status_message = Some(("Error: -u requires a number.".to_string(), Instant::now())); return; }
                upscale = parts[i + 1].parse::<u32>().unwrap_or(1).max(1);
                i += 2;
            },
            "-bg" => { with_background = true; i += 1; },
            // Ignore --explorer as it's already handled
            "--explorer" => { i += 1; }, 
            _ => { app.status_message = Some((format!("Error: Unknown argument for export: {}", parts[i]), Instant::now())); return; }
        }
    }
    
    // This part only runs if a path was provided via -o
    if let Some(path_str) = output_path_str {
        let final_path = shellexpand::tilde(&path_str.replace("\"", "")).into_owned();
        let path_buf = PathBuf::from(&final_path);
        if let Some(parent) = path_buf.parent() {
            if !parent.exists() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    app.status_message = Some((format!("Error creating directory: {}", e), Instant::now()));
                    return;
                }
            }
        }
        app.export_to_png(Some(final_path), upscale, !with_background);
    } else {
         // This case should now be rare, but we can keep a fallback
         // Or simply show a help message. Let's do that.
         app.status_message = Some(("Usage: export -o <path.png> or export --explorer".to_string(), Instant::now()));
    }
}

fn execute_command(app: &mut App, command: &str) {
    let command_to_run = command.trim();
    let parts: Vec<&str> = command_to_run.split_whitespace().collect();
    let should_save = parts.contains(&"--save");
    let mut status_update = None;

    // --- 1. Handle Complex Commands First ---
    let main_cmd = parts.get(0).unwrap_or(&"");
    if *main_cmd == "save" { parse_and_execute_save(app, command_to_run);
    } else if *main_cmd == "load" { parse_and_execute_load(app, command_to_run);
    } else if *main_cmd == "export" { parse_and_execute_export(app, command_to_run);
    } else if *main_cmd == "import" { if parts.get(1) == Some(&"palette") { parse_and_execute_import_palette(app, command_to_run); }
    } else if let Some(p) = main_cmd.strip_prefix("colorpalette:") {
        let n = p.strip_suffix(".consolet").unwrap_or(p);
        if let Some(pal) = app.loaded_palettes.get(n) {
            if parts.contains(&"--add") {
                let palette_to_add = pal.clone(); // Clone the data to release the borrow
                app.add_palette_entries_uniquely(&palette_to_add);
            } else {
                app.color_palette = pal.clone();
                app.palette_index = 0;
                status_update = Some(format!("Switched to palette '{}'", n));
            }
            if should_save {
                app.default_palette_name = n.to_string();
            }
        } else {
            status_update = Some(format!("Palette '{}' not found.", n));
        }


    } else if *main_cmd == "colorpalette_image" {

        let add_to_current = parts.contains(&"--add");
        file_browser::open_browser(app, file_browser::BrowserMode::GeneratePaletteFromImage(add_to_current));

    } else if *main_cmd == "colorpalette_image" {
        if parts.get(1) == Some(&"save") {
            let desired_name = parts.get(2).map(|s| s.replace("\"", ""));
            app.save_last_generated_palette(desired_name);
        } else {
            status_update = Some("Usage: colorpalette_image save [\"palette_name\"]".to_string());
        }
        } else if let Some(name) = main_cmd.strip_prefix("savepalette:") {
            app.save_current_palette(name.to_string());

    } else if let Some(c) = App::parse_hex_color(main_cmd) { app.current_selection = PaletteEntry::Color(c); if !app.color_palette.contains(&app.current_selection) { app.color_palette.push(app.current_selection); } app.palette_index = app.color_palette.iter().position(|&x| x == app.current_selection).unwrap_or(0); status_update = Some(format!("Color set to {}", main_cmd));
    } else {
        // --- 2. Handle Data-Driven Commands ---
        let mut command_found = false;
        let (cmd_name, value_str) = main_cmd.split_once('=').unwrap_or((main_cmd, ""));
        for cmd in COMMANDS.iter() {
            if cmd.name != cmd_name { continue; }
            command_found = true;
            match &cmd.command_type {
                CommandType::Action(action) => action(app),
                CommandType::SetterBool(action) => if let Ok(val) = value_str.parse::<bool>() { action(app, val); status_update = Some(format!("Set {} to {}", cmd.name, val)); } else { status_update = Some(format!("Invalid value. Usage: {}", cmd.usage)); },
                CommandType::SetterU16(action, min, max) => if let Ok(val) = value_str.parse::<u16>() { if val >= *min && val <= *max { action(app, val); status_update = Some(format!("Set {} to {}", cmd.name, val)); } else { status_update = Some(format!("Value out of range ({}-{}).", min, max)); } } else { status_update = Some(format!("Invalid value. Usage: {}", cmd.usage)); },
                CommandType::SetterF32(action, min, max) => if let Ok(val) = value_str.parse::<f32>() { if val >= *min && val <= *max { action(app, val); status_update = Some(format!("Set {} to {}", cmd.name, val)); } else { status_update = Some(format!("Value out of range ({}-{}).", min, max)); } } else { status_update = Some(format!("Invalid value. Usage: {}", cmd.usage)); },
                CommandType::SetterString(action) => { action(app, value_str.to_string()); status_update = Some(format!("Set {} to {}", cmd.name, value_str)); },
                _ => {}
            }
            break;
        }
        if !command_found && !command_to_run.is_empty() { status_update = Some(format!("Unknown command: {}", command_to_run)); }
    }

    if let Some(msg) = status_update { app.status_message = Some((msg, Instant::now())); }
    if should_save { app.save_current_config(); }
}

fn parse_and_execute_import_palette(app: &mut App, command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.contains(&"--explorer") {
        file_browser::open_browser(app, file_browser::BrowserMode::ImportPalette);
        return;
    }
    if let Some(path_str) = parts.get(2) {
        app.load_and_store_palette(path_str);
    } else {
        app.status_message = Some(("Usage: import palette <path>".to_string(), Instant::now()));
    }
}



fn draw_keybindings_screen(frame: &mut Frame, app: &mut App) {
    let area = utils::centered_rect(60, 80, frame.size());
    frame.render_widget(Clear, area);
    let block = Block::default().title(" Keybindings (Enter to Change, Esc to Exit) ").borders(Borders::ALL);
    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    if app.is_changing_keybinding {
        let waiting_area = utils::centered_rect(40, 20, frame.size());
        let text = Paragraph::new("Press any key combination...\n(Press Esc to cancel)")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Waiting for Input"));
        frame.render_widget(Clear, waiting_area);
        frame.render_widget(text, waiting_area);
        return;
    }

    let mut items = vec![];
    for (i, action) in Action::iter().enumerate() {
        let keybinding = app.keybindings.map.get(&action);
        let key_str = keybinding.map(utils::format_keybinding).unwrap_or_else(|| "Unbound".to_string());
        let line = Line::from(vec![
            Span::styled(format!("{:<25}", action.to_string()), Style::default()),
            Span::raw(key_str),
        ]);
        let style = if i == app.keybindings_selection_index {
            Style::default().bg(app.translate_color(Color::Yellow)).fg(app.translate_color(Color::Black))
        } else {
            Style::default()
        };
        items.push(line.style(style));
    }

    let list = Paragraph::new(items)
        .block(Block::default())
        .scroll((app.keybindings_scroll_state, 0));
    frame.render_widget(list, inner_area);
}

fn draw_confirmation_dialog(frame: &mut Frame, app: &mut App, message: &str) {
    let area = utils::centered_rect(30, 20, frame.size());
    frame.render_widget(Clear, area);
    let block = Block::default().title(" Confirmation ").borders(Borders::ALL);
    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let text = Paragraph::new(message).alignment(Alignment::Center);

    let yes_style = if app.confirm_selection_yes { Style::default().reversed() } else { Style::default() };
    let no_style = if !app.confirm_selection_yes { Style::default().reversed() } else { Style::default() };
    let buttons = Line::from(vec![
        Span::styled(" Yes ", yes_style),
        Span::raw(" / "),
        Span::styled(" No ", no_style),
    ]).alignment(Alignment::Center);
    
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(inner_area);

    frame.render_widget(text, layout[0]);
    frame.render_widget(buttons, layout[1]);
}
