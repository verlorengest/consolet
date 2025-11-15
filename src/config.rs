use crate::{App, ColorMode, HighlighterMode, MinimapMode, PenShape, CanvasScrollAction};

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Clear, Paragraph},
};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone, Copy, EnumIter, Display, PartialEq)]
pub enum ConfigSetting {
    PenSizeSensitivity,
    OpacitySensitivity,
    PenShape,
    Highlighter,
    HighlighterValue,
    HighlighterMode,
    ShadeFactor,
    ProtectStroke,
    ApplyColorInterval,
    MinimapMode,
    MouseEvents,
    ColorMode,
    CanvasScrollAction,
    SpraySize,
    SpraySpeed,
    SprayIntensity,
    SnapToPalette,
    SnapToPaletteMode,
    ProtectColorTransitions,
    PaletteMenuPosition,





}

impl ConfigSetting {

    pub fn get_value_as_string(&self, app: &App) -> String {
        match self {
            Self::PenSizeSensitivity => app.pen_size_sensitivity.to_string(),
            Self::OpacitySensitivity => format!("{:.2}", app.opacity_sensitivity),
            Self::PenShape => format!("{:?}", app.pen_shape),
            Self::Highlighter => app.highlighter_enabled.to_string(),
            Self::HighlighterValue => format!("{:.2}", app.highlighter_value),
            Self::HighlighterMode => format!("{:?}", app.highlighter_mode),
            Self::ShadeFactor => format!("{:.3}", app.shade_factor),
            Self::ProtectStroke => app.protect_stroke.to_string(),
            Self::ApplyColorInterval => format!("{:.2}", app.apply_color_interval.num_milliseconds() as f32 / 1000.0),
            Self::MinimapMode => format!("{:?}", app.minimap_mode),
            Self::MouseEvents => app.mouse_events_enabled.to_string(),
            Self::ColorMode => format!("{:?}", app.color_mode),
            Self::CanvasScrollAction => format!("{:?}", app.canvas_scroll_action),
            Self::SpraySize => app.spray_size.to_string(),
            Self::SpraySpeed => app.spray_speed.to_string(),
            Self::SprayIntensity => format!("{:.2}", app.spray_intensity),
            Self::SnapToPalette => app.snap_to_palette.to_string(),
            Self::SnapToPaletteMode => format!("{:?}", app.snap_to_palette_mode),
            Self::ProtectColorTransitions => app.protect_color_transitions.to_string(),
            Self::PaletteMenuPosition => format!("{:?}", app.palette_menu_position),


        }
    }



    fn cycle_value(&self, app: &mut App) {
        match self {
            Self::PenShape => app.pen_shape = if app.pen_shape == PenShape::Circular { PenShape::Square } else { PenShape::Circular },
            Self::Highlighter => app.highlighter_enabled = !app.highlighter_enabled,
            Self::HighlighterMode => app.highlighter_mode = if app.highlighter_mode == HighlighterMode::Blend { HighlighterMode::Underscore } else { HighlighterMode::Blend },
            Self::ProtectStroke => app.protect_stroke = !app.protect_stroke,
            Self::MinimapMode => app.minimap_mode = match app.minimap_mode {
                MinimapMode::Auto => MinimapMode::On,
                MinimapMode::On => MinimapMode::Off,
                MinimapMode::Off => MinimapMode::Auto,
            },
            Self::MouseEvents => app.mouse_events_enabled = !app.mouse_events_enabled,
            Self::ColorMode => app.color_mode = if app.color_mode == ColorMode::TrueColor { ColorMode::Ansi256 } else { ColorMode::TrueColor },
            Self::CanvasScrollAction => app.canvas_scroll_action = if app.canvas_scroll_action == CanvasScrollAction::ChangePenSize { CanvasScrollAction::ChangeOpacity } else { CanvasScrollAction::ChangePenSize },
            Self::SnapToPalette => app.snap_to_palette = !app.snap_to_palette,
            Self::SnapToPaletteMode => app.snap_to_palette_mode = if app.snap_to_palette_mode == crate::SnapToPaletteMode::ClosestRgb { crate::SnapToPaletteMode::ClosestHue } else { crate::SnapToPaletteMode::ClosestRgb },
            Self::ProtectColorTransitions => app.protect_color_transitions = !app.protect_color_transitions,
            Self::PaletteMenuPosition => app.palette_menu_position = if app.palette_menu_position == crate::PaletteMenuPosition::Left { crate::PaletteMenuPosition::Right } else { crate::PaletteMenuPosition::Left },


            _ => {}
        }
    }





    pub fn increment_value(&self, app: &mut App) {
        match self {
            Self::PenSizeSensitivity => app.pen_size_sensitivity = app.pen_size_sensitivity.saturating_add(1).clamp(1, 20),
            Self::OpacitySensitivity => app.opacity_sensitivity = (app.opacity_sensitivity + 0.01).clamp(0.01, 0.5),
            Self::HighlighterValue => app.highlighter_value = (app.highlighter_value + 0.05).clamp(0.0, 1.0),
            Self::ShadeFactor => app.shade_factor = (app.shade_factor + 0.005).clamp(0.01, 1.0),
            Self::SpraySize => app.spray_size = app.spray_size.saturating_add(1).clamp(1, 50),
            Self::SpraySpeed => app.spray_speed = app.spray_speed.saturating_add(1).clamp(1, 100),
            Self::SprayIntensity => app.spray_intensity = (app.spray_intensity + 0.05).clamp(0.0, 1.0),
            Self::SnapToPalette => self.cycle_value(app),
            Self::SnapToPaletteMode => self.cycle_value(app),
            Self::ProtectColorTransitions => self.cycle_value(app),
            Self::PaletteMenuPosition => self.cycle_value(app),

            Self::ApplyColorInterval => {
                let current_ms = app.apply_color_interval.num_milliseconds() as f32;
                let new_ms = (current_ms + 10.0).clamp(50.0, 2000.0);
                app.apply_color_interval = chrono::Duration::milliseconds(new_ms as i64);
            }
            _ => self.cycle_value(app), // For toggles, incrementing just cycles
        }
    }

    pub fn decrement_value(&self, app: &mut App) {
        match self {
            Self::PenSizeSensitivity => app.pen_size_sensitivity = app.pen_size_sensitivity.saturating_sub(1).max(1),
            Self::OpacitySensitivity => app.opacity_sensitivity = (app.opacity_sensitivity - 0.01).clamp(0.01, 0.5),
            Self::HighlighterValue => app.highlighter_value = (app.highlighter_value - 0.05).clamp(0.0, 1.0),
            Self::ShadeFactor => app.shade_factor = (app.shade_factor - 0.005).clamp(0.01, 1.0),
            Self::SpraySize => app.spray_size = app.spray_size.saturating_sub(1).max(1),
            Self::SpraySpeed => app.spray_speed = app.spray_speed.saturating_sub(1).max(1),
            Self::SprayIntensity => app.spray_intensity = (app.spray_intensity - 0.05).clamp(0.0, 1.0),
            Self::SnapToPalette => self.cycle_value(app),
            Self::SnapToPaletteMode => self.cycle_value(app),
            Self::ProtectColorTransitions => self.cycle_value(app),
            Self::PaletteMenuPosition => self.cycle_value(app),


            Self::ApplyColorInterval => {
                let current_ms = app.apply_color_interval.num_milliseconds() as f32;
                let new_ms = (current_ms - 10.0).clamp(50.0, 2000.0);
                app.apply_color_interval = chrono::Duration::milliseconds(new_ms as i64);
            }
            _ => self.cycle_value(app), // For toggles, decrementing also just cycles
        }
    }


}

    pub fn draw_config_screen(frame: &mut Frame, app: &mut App) {
        let area = crate::utils::centered_rect(60, 80, frame.size());
        frame.render_widget(Clear, area);
        let block = Block::default().title(" Configuration (Arrows to Change, Esc to Exit) ").borders(Borders::ALL);
        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let mut items = vec![];
        for (i, setting) in ConfigSetting::iter().enumerate() {
            let is_selected = i == app.config_selection_index;
            let value_str = setting.get_value_as_string(app);

            let line = Line::from(vec![
                Span::styled(format!("{:<25}", setting.to_string()), Style::default()),
                Span::raw(value_str),
            ]);
            let style = if is_selected {
                Style::default().bg(Color::Yellow).fg(Color::Black)
            } else {
                Style::default()
            };
            items.push(line.style(style));
        }

        let list = Paragraph::new(items).block(Block::default());
        frame.render_widget(list, inner_area);
    }