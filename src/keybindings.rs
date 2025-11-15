// keybindings.rs
use crossterm::event::{KeyCode, KeyModifiers};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use strum_macros::{Display,EnumIter};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug, Display, EnumIter)]
pub enum Action {
    Quit,
    MoveCursorUp,
    MoveCursorDown,
    MoveCursorLeft,
    MoveCursorRight,
    OpenCommandPrompt,
    OpenColorPicker,
    OpenToolPicker,
    PanViewUp,
    PanViewDown,
    PanViewLeft,
    PanViewRight,
    ZoomIn,
    ZoomOut,
    Undo,
    Redo,
    IncreasePenSize,
    DecreasePenSize,
    IncreaseOpacity,
    DecreaseOpacity,
    CycleSymmetry,
    PickColor,
    Fill,
    Draw,
    Erase,
    QuickSelectColorUp,
    QuickSelectColorDown,
    QuickSelectColorLeft,
    QuickSelectColorRight,
    QuickSelectToolLeft,
    QuickSelectToolRight,
    AdjustSymmetryNegative, // Represents 'j' key
    AdjustSymmetryPositive, // Represents 'k' key
    Spray,
    SelectLayerUp,
    SelectLayerDown,
    AddLayer,
    DeleteLayer,
    ToggleLayerVisibility,
    MoveLayerUp,
    MoveLayerDown,
    ToggleOnionSkin,
    IncreaseOnionOpacity,
    DecreaseOnionOpacity,
}


// 2. Define what a keybinding is.
#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Keybinding {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

// 3. The main struct that holds the mapping and handles load/save.
#[derive(Serialize, Deserialize, Clone)]
pub struct Keybindings {
    pub map: HashMap<Action, Keybinding>,
}

impl Keybindings {
    pub fn get_path() -> std::io::Result<PathBuf> {

        let app_dir = crate::utils::get_or_create_app_dir()?;
        Ok(app_dir.join("keybindings.json"))
    }

    pub fn save(&self) -> std::io::Result<()> {
        let path = Self::get_path()?;
        let json_data = serde_json::to_string_pretty(self).unwrap_or_default();
        std::fs::write(path, json_data)
    }

    pub fn load() -> Self {
        // Start with the complete set of default bindings.
        let mut bindings = Self::default();

        // If a saved file exists, load it and overwrite the defaults.
        if let Ok(path) = Self::get_path() {
            if let Ok(json_data) = std::fs::read_to_string(path) {
                if let Ok(saved_bindings) = serde_json::from_str::<Keybindings>(&json_data) {
                    // Layer the user's saved customizations on top of the defaults.
                    for (action, keybinding) in saved_bindings.map {
                        bindings.map.insert(action, keybinding);
                    }
                }
            }
        }
        // Return the merged result.
        bindings
    }
}

// 4. Define the default keybindings.
impl Default for Keybindings {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(Action::MoveCursorUp, Keybinding { code: KeyCode::Up, modifiers: KeyModifiers::NONE });
        map.insert(Action::MoveCursorDown, Keybinding { code: KeyCode::Down, modifiers: KeyModifiers::NONE });
        map.insert(Action::MoveCursorLeft, Keybinding { code: KeyCode::Left, modifiers: KeyModifiers::NONE });
        map.insert(Action::MoveCursorRight, Keybinding { code: KeyCode::Right, modifiers: KeyModifiers::NONE });
        map.insert(Action::PanViewUp, Keybinding { code: KeyCode::Char('k'), modifiers: KeyModifiers::NONE });
        map.insert(Action::PanViewDown, Keybinding { code: KeyCode::Char('j'), modifiers: KeyModifiers::NONE });
        map.insert(Action::PanViewLeft, Keybinding { code: KeyCode::Char('h'), modifiers: KeyModifiers::NONE });
        map.insert(Action::PanViewRight, Keybinding { code: KeyCode::Char('l'), modifiers: KeyModifiers::NONE });
        map.insert(Action::ZoomIn, Keybinding { code: KeyCode::Char('='), modifiers: KeyModifiers::NONE });
        map.insert(Action::ZoomOut, Keybinding { code: KeyCode::Char('-'), modifiers: KeyModifiers::NONE });
        map.insert(Action::OpenCommandPrompt, Keybinding { code: KeyCode::Esc, modifiers: KeyModifiers::NONE });
        map.insert(Action::OpenColorPicker, Keybinding { code: KeyCode::Char('c'), modifiers: KeyModifiers::NONE });
        map.insert(Action::OpenToolPicker, Keybinding { code: KeyCode::Char('t'), modifiers: KeyModifiers::NONE });
        map.insert(Action::IncreasePenSize, Keybinding { code: KeyCode::Char(']'), modifiers: KeyModifiers::NONE });
        map.insert(Action::DecreasePenSize, Keybinding { code: KeyCode::Char('['), modifiers: KeyModifiers::NONE });
        map.insert(Action::IncreaseOpacity, Keybinding { code: KeyCode::Char('p'), modifiers: KeyModifiers::NONE });
        map.insert(Action::DecreaseOpacity, Keybinding { code: KeyCode::Char('o'), modifiers: KeyModifiers::NONE });
        map.insert(Action::Undo, Keybinding { code: KeyCode::Char('z'), modifiers: KeyModifiers::CONTROL });
        map.insert(Action::Redo, Keybinding { code: KeyCode::Char('y'), modifiers: KeyModifiers::CONTROL });
        map.insert(Action::CycleSymmetry, Keybinding { code: KeyCode::Char('s'), modifiers: KeyModifiers::NONE });
        map.insert(Action::PickColor, Keybinding { code: KeyCode::Char('r'), modifiers: KeyModifiers::NONE });
        map.insert(Action::Fill, Keybinding { code: KeyCode::Char('f'), modifiers: KeyModifiers::NONE });
        map.insert(Action::Draw, Keybinding { code: KeyCode::Char(' '), modifiers: KeyModifiers::NONE });
        map.insert(Action::Erase, Keybinding { code: KeyCode::Char('e'), modifiers: KeyModifiers::NONE });
        map.insert(Action::QuickSelectColorUp, Keybinding { code: KeyCode::Up, modifiers: KeyModifiers::CONTROL });
        map.insert(Action::QuickSelectColorDown, Keybinding { code: KeyCode::Down, modifiers: KeyModifiers::CONTROL });
        map.insert(Action::QuickSelectColorLeft, Keybinding { code: KeyCode::Left, modifiers: KeyModifiers::CONTROL });
        map.insert(Action::QuickSelectColorRight, Keybinding { code: KeyCode::Right, modifiers: KeyModifiers::CONTROL });
        map.insert(Action::QuickSelectToolLeft, Keybinding { code: KeyCode::Left, modifiers: KeyModifiers::SHIFT });
        map.insert(Action::QuickSelectToolRight, Keybinding { code: KeyCode::Right, modifiers: KeyModifiers::SHIFT });
        map.insert(Action::AdjustSymmetryNegative, Keybinding { code: KeyCode::Char('m'), modifiers: KeyModifiers::NONE });
        map.insert(Action::AdjustSymmetryPositive, Keybinding { code: KeyCode::Char('n'), modifiers: KeyModifiers::NONE });
        map.insert(Action::SelectLayerUp, Keybinding { code: KeyCode::Up, modifiers: KeyModifiers::ALT });
        map.insert(Action::SelectLayerDown, Keybinding { code: KeyCode::Down, modifiers: KeyModifiers::ALT });
        map.insert(Action::AddLayer, Keybinding { code: KeyCode::Char('a'), modifiers: KeyModifiers::ALT });
        map.insert(Action::DeleteLayer, Keybinding { code: KeyCode::Char('d'), modifiers: KeyModifiers::ALT });
        map.insert(Action::ToggleLayerVisibility, Keybinding { code: KeyCode::Char('v'), modifiers: KeyModifiers::ALT });
        map.insert(Action::MoveLayerUp, Keybinding { code: KeyCode::Char('k'), modifiers: KeyModifiers::ALT });
        map.insert(Action::MoveLayerDown, Keybinding { code: KeyCode::Char('j'), modifiers: KeyModifiers::ALT });
        map.insert(Action::ToggleOnionSkin, Keybinding { code: KeyCode::Char('i'), modifiers: KeyModifiers::NONE });
        map.insert(Action::IncreaseOnionOpacity, Keybinding { code: KeyCode::Char('u'), modifiers: KeyModifiers::NONE });
        map.insert(Action::DecreaseOnionOpacity, Keybinding { code: KeyCode::Char('y'), modifiers: KeyModifiers::NONE });
    Self { map }
    }
}