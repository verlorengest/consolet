// commands.rs

use crate::App; // This allows us to use `App` in our function pointers
use std::time::Instant;
use crate::{Pixel, ExportLayerMode}; // Add Pixel and ExportLayerMode here


pub enum CommandType {
    Action(fn(&mut App)),
    SetterBool(fn(&mut App, bool)),
    SetterU16(fn(&mut App, u16), u16, u16), // fn, min, max
    SetterF32(fn(&mut App, f32), f32, f32), // fn, min, max
    SetterString(fn(&mut App, String)),
    Complex, // For commands like save, load, export that need custom parsing
}

pub struct Command {
    pub name: &'static str,
    pub description: &'static str,
    pub usage: &'static str,
    pub example: &'static str,
    pub command_type: CommandType,
}

pub const COMMANDS: &[Command] = &[
    // Simple Actions
    Command { name: "help", description: "Displays the keybindings cheatsheet.", usage: "help", example: "help", command_type: CommandType::Action(|app| { app.mode = crate::AppMode::HelpScreen; app.help_scroll = 0; })},
    Command { name: "quit", description: "Quits the application.", usage: "quit", example: "quit", command_type: CommandType::Action(|app| app.quit()) },
    Command { name: "q", description: "Alias for 'quit'.", usage: "q", example: "q", command_type: CommandType::Action(|app| app.quit()) },
    Command { name: "undo", description: "Undo the last action.", usage: "undo", example: "undo", command_type: CommandType::Action(|app| app.undo()) },
    Command { name: "redo", description: "Redo the last undone action.", usage: "redo", example: "redo", command_type: CommandType::Action(|app| app.redo()) },
    Command { name: "clear", description: "Clears the entire canvas.", usage: "clear", example: "clear", command_type: CommandType::Action(|app| app.clear_canvas()) },
    Command { name: "resize", description: "Begin resizing the canvas.", usage: "resize", example: "resize", command_type: CommandType::Action(|app| { app.mode = crate::AppMode::ResizingWidth; app.input_buffer.clear(); }) },
    Command { name: "keybindings:reset", description: "Resets all keybindings to their default values.", usage: "keybindings:reset", example: "keybindings:reset", command_type: CommandType::Action(|app| app.reset_keybindings()) },

    Command { name: "edit_script", description: "Opens the command drawing script editor.", usage: "edit_script", example: "edit_script", command_type: CommandType::Action(|app| { crate::script_handler::load_script_for_editing(app); })},
    Command { name: "draw_script", description: "Executes the command drawing script.", usage: "draw_script", example: "draw_script", command_type: CommandType::Action(|app| { crate::script_handler::parse_and_execute_script(app); })},

    // Boolean Setters
    Command { name: "minimap", description: "Toggles the minimap.", usage: "minimap={true|false}", example: "minimap=true", command_type: CommandType::SetterBool(|app, val| app.minimap_mode = if val { crate::MinimapMode::On } else { crate::MinimapMode::Off }) },
    Command { name: "highlighter", description: "Toggles the cursor highlighter.", usage: "highlighter={true|false}", example: "highlighter=false", command_type: CommandType::SetterBool(|app, val| app.highlighter_enabled = val) },
    Command { name: "protectStroke", description: "Prevents drawing over the same pixel in one stroke.", usage: "protectStroke={true|false}", example: "protectStroke=false", command_type: CommandType::SetterBool(|app, val| app.protect_stroke = val) },
    Command { name: "mouseEvents", description: "Enables or disables all mouse event handling.", usage: "mouseEvents={true|false}", example: "mouseEvents=false", command_type: CommandType::SetterBool(|app, val| app.mouse_events_enabled = val) },
    
    
    // U16 Setters
    Command { name: "penSizeSensitivity", description: "Sets pen size change sensitivity.", usage: "penSizeSensitivity={1-20}", example: "penSizeSensitivity=2", command_type: CommandType::SetterU16(|app, val| app.pen_size_sensitivity = val, 1, 20) },
    Command { name: "highlighterMode", description: "Sets highlighter mode (0=Underscore, 1=Blend).", usage: "highlighterMode={0|1}", example: "highlighterMode=1", command_type: CommandType::SetterU16(|app, val| app.highlighter_mode = if val == 0 { crate::HighlighterMode::Underscore } else { crate::HighlighterMode::Blend }, 0, 1) },
    Command { name: "spraySize", description: "Sets the size of the spray tool area.", usage: "spraySize={1-50}", example: "spraySize=10", command_type: CommandType::SetterU16(|app, val| app.spray_size = val, 1, 50) },
    Command { name: "spraySpeed", description: "Sets the density/speed of the spray tool.", usage: "spraySpeed={1-100}", example: "spraySpeed=5", command_type: CommandType::SetterU16(|app, val| app.spray_speed = val, 1, 100) },



    // F32 Setters
    Command { name: "opacitySensitivity", description: "Sets opacity change sensitivity.", usage: "opacitySensitivity={0.01-0.5}", example: "opacitySensitivity=0.1", command_type: CommandType::SetterF32(|app, val| app.opacity_sensitivity = val, 0.01, 0.5) },
    Command { name: "highlighterValue", description: "Sets highlighter strength.", usage: "highlighterValue={0.0-1.0}", example: "highlighterValue=0.5", command_type: CommandType::SetterF32(|app, val| app.highlighter_value = val, 0.0, 1.0) },
    Command { name: "pencilDensity", description: "Sets Lighter/Darker tool density.", usage: "pencilDensity={0.01-1.0}", example: "pencilDensity=0.05", command_type: CommandType::SetterF32(|app, val| app.shade_factor = val, 0.01, 1.0) },
    Command { name: "applyColorSec", description: "Sets auto-apply interval for holding Spacebar.", usage: "applyColorSec={0.05-2.0}", example: "applyColorSec=0.1", command_type: CommandType::SetterF32(|app, val| app.apply_color_interval = chrono::Duration::milliseconds((val * 1000.0) as i64), 0.05, 2.0) },
    Command { name: "sprayIntensity", description: "Sets the intensity/density of the spray tool.", usage: "sprayIntensity={0.01-1.0}", example: "sprayIntensity=0.5", command_type: CommandType::SetterF32(|app, val| app.spray_intensity = val, 0.01, 1.0) },
    
    
    
    
    // String Setters
    Command { name: "penShape", description: "Sets the brush shape.", usage: "penShape={circular|square}", example: "penShape=square", command_type: CommandType::SetterString(|app, val| if val == "circular" || val == "square" { app.pen_shape = if val == "circular" { crate::PenShape::Circular } else { crate::PenShape::Square }; }) },
    Command { name: "canvasScrollAction", description: "Sets mouse wheel action on canvas (ChangePenSize or ChangeOpacity).", usage: "canvasScrollAction={ChangePenSize|ChangeOpacity}", example: "canvasScrollAction=ChangeOpacity", command_type: CommandType::SetterString(|app, val| {
        if val == "ChangeOpacity" { app.canvas_scroll_action = crate::CanvasScrollAction::ChangeOpacity; }
        else if val == "ChangePenSize" { app.canvas_scroll_action = crate::CanvasScrollAction::ChangePenSize; }
    }) },
    // Complex Commands (handled separately)
    Command { name: "save", description: "Saves the project.", usage: "save <name.consolet> [-a mins] [-p path] [-f]", example: "save art.consolet -a 5", command_type: CommandType::Complex },
    Command { name: "load", description: "Loads a project.", usage: "load <name.consolet>", example: "load art.consolet", command_type: CommandType::Complex },
    Command { name: "export", description: "Exports canvas to PNG.", usage: "export [-o path] [-u scale] [-bg]", example: "export -o image.png -u 10", command_type: CommandType::Complex },
    Command { name: "import", description: "Imports an asset.", usage: "import palette <path>", example: "import palette my_palette.consolet", command_type: CommandType::Complex },
    Command { name: "colorpalette", description: "Switches to a loaded palette.", usage: "colorpalette:<name>", example: "colorpalette:default", command_type: CommandType::Complex },
    
    Command { name: "colorpalette:", description: "Switches to a loaded palette.", usage: "colorpalette:<name>", example: "colorpalette:default", command_type: CommandType::Complex },
    Command { name: "savepalette:", description: "Saves the current palette.", usage: "savepalette:<name>", example: "savepalette:my-palette", command_type: CommandType::Complex },
    Command { name: "colorpalette_image", description: "Generate a new palette from an image file.", usage: "colorpalette_image [--add]", example: "colorpalette_image", command_type: CommandType::Complex },   
    Command { name: "keybindings", description: "Opens the keybinding configuration panel.", usage: "keybindings", example: "keybindings", command_type: CommandType::Action(|app| { app.mode = crate::AppMode::Keybindings; })},
    Command { name: "config", description: "Opens the configuration editor panel.", usage: "config", example: "config", command_type: CommandType::Action(|app| { app.mode = crate::AppMode::ConfigEditor; })},

    Command { name: "colorMode", description: "Sets color mode (TrueColor or Ansi256).", usage: "colorMode={TrueColor|Ansi256}", example: "colorMode=Ansi256", command_type: CommandType::SetterString(|app, val| {
        if val.to_lowercase() == "ansi256" { app.color_mode = crate::ColorMode::Ansi256; }
        else if val.to_lowercase() == "truecolor" { app.color_mode = crate::ColorMode::TrueColor; }
    }) },

    Command {
        name: "layer_opacity",
        description: "Set active layer opacity (0.0 to 1.0)",
        usage: "layer_opacity=<value>",
        example: "layer_opacity=0.5",
        command_type: CommandType::SetterF32(
            |app, val| {
                if app.active_layer_index < app.layers.len() {
                    app.layers[app.active_layer_index].opacity = val;
                    app.sync_canvas_from_layers();
                }
            },
            0.0,
            1.0,
        ),
    },
    Command {
        name: "rename_layer",
        description: "Rename the active layer",
        usage: "rename_layer=<name>",
        example: "rename_layer=Background",
        command_type: CommandType::SetterString(|app, name| {
            if app.active_layer_index < app.layers.len() {
                app.layers[app.active_layer_index].name = name;
            }
        }),
    },
    Command {
        name: "export_mode",
        description: "Set export mode (united or separate)",
        usage: "export_mode=<mode>",
        example: "export_mode=separate",
        command_type: CommandType::SetterString(|app, mode| {
            match mode.to_lowercase().as_str() {
                "united" => app.export_layer_mode = crate::ExportLayerMode::United,
                "separate" => app.export_layer_mode = crate::ExportLayerMode::Separate,
                _ => app.status_message = Some(("Invalid mode. Use 'united' or 'separate'.".to_string(), Instant::now())),
            }
        }),
    },
    Command {
        name: "onion_opacity",
        description: "Set onion skin opacity (0.0 to 1.0)",
        usage: "onion_opacity=<value>",
        example: "onion_opacity=0.3",
        command_type: CommandType::SetterF32(|app, val| app.onion_skin_opacity = val, 0.0, 1.0),
    },
    Command {
        name: "onion_skin",
        description: "Toggle onion skinning on/off",
        usage: "onion_skin=<true|false>",
        example: "onion_skin=true",
        command_type: CommandType::SetterBool(|app, val| app.onion_skin_enabled = val),
    },
    Command {
        name: "add_layer",
        description: "Add a new layer",
        usage: "add_layer",
        example: "add_layer",
        command_type: CommandType::Action(|app| app.add_new_layer()),
    },
    Command {
        name: "delete_layer",
        description: "Delete the active layer",
        usage: "delete_layer",
        example: "delete_layer",
        command_type: CommandType::Action(|app| app.delete_active_layer()),
    },
    Command {
        name: "merge_down",
        description: "Merge active layer with the layer below",
        usage: "merge_down",
        example: "merge_down",
        command_type: CommandType::Action(|app| {
            if app.active_layer_index == 0 {
                app.status_message = Some(("Cannot merge bottom layer.".to_string(), Instant::now()));
                return;
            }
            let active_layer = app.layers[app.active_layer_index].clone();
            let below_layer = &mut app.layers[app.active_layer_index - 1];
            
            for y in 0..app.canvas_height {
                for x in 0..app.canvas_width {
                    let src_pixel = active_layer.canvas[y][x];
                    if src_pixel.alpha == 0.0 {
                        continue;
                    }
                    let dest_pixel = below_layer.canvas[y][x];
                    let src_alpha = src_pixel.alpha * active_layer.opacity;
                    
                    if dest_pixel.alpha == 0.0 {
                        below_layer.canvas[y][x] = Pixel {
                            color: src_pixel.color,
                            alpha: src_alpha,
                        };
                    } else {
                        let final_alpha = src_alpha + dest_pixel.alpha * (1.0 - src_alpha);
                        let factor = src_alpha / final_alpha;
                        let final_color = crate::utils::blend_colors(dest_pixel.color.into(), src_pixel.color.into(), factor);
                        below_layer.canvas[y][x] = Pixel {
                            color: final_color.into(),
                            alpha: final_alpha,
                        };
                    }
                }
            }
            
            app.layers.remove(app.active_layer_index);
            app.active_layer_index -= 1;
            app.sync_canvas_from_layers();
            app.status_message = Some(("Layer merged down.".to_string(), Instant::now()));
        }),
    },






    ];