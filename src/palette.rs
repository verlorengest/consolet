// palette.rs
use ratatui::prelude::Color;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Tool { Lighter, Darker, Blur }

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PaletteEntry {
    Color(Color),
    Tool(Tool),
}




pub fn get_default_tool_palette() -> Vec<PaletteEntry> {
    vec![
        PaletteEntry::Tool(Tool::Lighter),
        PaletteEntry::Tool(Tool::Darker),
        PaletteEntry::Tool(Tool::Blur),
    ]
}




fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = if h >= 0.0 && h < 60.0 {
        (c, x, 0.0)
    } else if h >= 60.0 && h < 120.0 {
        (x, c, 0.0)
    } else if h >= 120.0 && h < 180.0 {
        (0.0, c, x)
    } else if h >= 180.0 && h < 240.0 {
        (0.0, x, c)
    } else if h >= 240.0 && h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    (
        ((r_prime + m) * 255.0) as u8,
        ((g_prime + m) * 255.0) as u8,
        ((b_prime + m) * 255.0) as u8,
    )
}



pub fn get_default_color_palette() -> Vec<PaletteEntry> {
    vec![
            PaletteEntry::Color(Color::White),
            PaletteEntry::Color(Color::Rgb(245, 245, 245)),
            PaletteEntry::Color(Color::Rgb(220, 220, 220)),
            PaletteEntry::Color(Color::Rgb(192, 192, 192)),
            PaletteEntry::Color(Color::Rgb(169, 169, 169)),
            PaletteEntry::Color(Color::Gray),
            PaletteEntry::Color(Color::DarkGray),
            PaletteEntry::Color(Color::Rgb(64, 64, 64)),
            PaletteEntry::Color(Color::Rgb(40, 40, 40)),
            PaletteEntry::Color(Color::Rgb(20, 20, 20)),
            PaletteEntry::Color(Color::Black),
            PaletteEntry::Color(Color::Rgb(255, 240, 245)),
            PaletteEntry::Color(Color::Rgb(255, 192, 203)),
            PaletteEntry::Color(Color::Rgb(255, 182, 193)),
            PaletteEntry::Color(Color::Rgb(255, 105, 180)),
            PaletteEntry::Color(Color::Rgb(255, 20, 147)),
            PaletteEntry::Color(Color::Rgb(219, 112, 147)),
            PaletteEntry::Color(Color::Rgb(199, 21, 133)),
            PaletteEntry::Color(Color::Rgb(255, 228, 225)),
            PaletteEntry::Color(Color::Rgb(255, 218, 185)),
            PaletteEntry::Color(Color::Rgb(255, 160, 122)),
            PaletteEntry::Color(Color::LightRed),
            PaletteEntry::Color(Color::Rgb(255, 99, 71)),
            PaletteEntry::Color(Color::Red),
            PaletteEntry::Color(Color::Rgb(220, 20, 60)),
            PaletteEntry::Color(Color::Rgb(178, 34, 34)),
            PaletteEntry::Color(Color::Rgb(139, 0, 0)),
            PaletteEntry::Color(Color::Rgb(128, 0, 0)),
            PaletteEntry::Color(Color::Rgb(255, 250, 205)),
            PaletteEntry::Color(Color::Rgb(255, 239, 213)),
            PaletteEntry::Color(Color::Rgb(255, 228, 181)),
            PaletteEntry::Color(Color::Rgb(255, 218, 185)),
            PaletteEntry::Color(Color::Rgb(255, 165, 0)),
            PaletteEntry::Color(Color::Rgb(255, 140, 0)),
            PaletteEntry::Color(Color::Rgb(255, 127, 80)),
            PaletteEntry::Color(Color::Rgb(210, 105, 30)),
            PaletteEntry::Color(Color::Rgb(184, 134, 11)),
            PaletteEntry::Color(Color::Rgb(160, 82, 45)),
            PaletteEntry::Color(Color::Rgb(139, 69, 19)),
            PaletteEntry::Color(Color::Rgb(255, 255, 224)),
            PaletteEntry::Color(Color::LightYellow),
            PaletteEntry::Color(Color::Rgb(255, 250, 205)),
            PaletteEntry::Color(Color::Yellow),
            PaletteEntry::Color(Color::Rgb(255, 215, 0)),
            PaletteEntry::Color(Color::Rgb(218, 165, 32)),
            PaletteEntry::Color(Color::Rgb(189, 183, 107)),
            PaletteEntry::Color(Color::Rgb(154, 205, 50)),
            PaletteEntry::Color(Color::Rgb(128, 128, 0)),
            PaletteEntry::Color(Color::Rgb(245, 255, 250)),
            PaletteEntry::Color(Color::Rgb(240, 255, 240)),
            PaletteEntry::Color(Color::Rgb(144, 238, 144)),
            PaletteEntry::Color(Color::Rgb(152, 251, 152)),
            PaletteEntry::Color(Color::LightGreen),
            PaletteEntry::Color(Color::Rgb(124, 252, 0)),
            PaletteEntry::Color(Color::Rgb(0, 255, 127)),
            PaletteEntry::Color(Color::Rgb(0, 250, 154)),
            PaletteEntry::Color(Color::Rgb(50, 205, 50)),
            PaletteEntry::Color(Color::Rgb(34, 139, 34)),
            PaletteEntry::Color(Color::Green),
            PaletteEntry::Color(Color::Rgb(0, 128, 0)),
            PaletteEntry::Color(Color::Rgb(0, 100, 0)),
            PaletteEntry::Color(Color::Rgb(85, 107, 47)),
            PaletteEntry::Color(Color::Rgb(107, 142, 35)),
            PaletteEntry::Color(Color::Rgb(240, 255, 255)),
            PaletteEntry::Color(Color::Rgb(224, 255, 255)),
            PaletteEntry::Color(Color::Rgb(175, 238, 238)),
            PaletteEntry::Color(Color::Rgb(127, 255, 212)),
            PaletteEntry::Color(Color::Rgb(64, 224, 208)),
            PaletteEntry::Color(Color::LightCyan),
            PaletteEntry::Color(Color::Rgb(72, 209, 204)),
            PaletteEntry::Color(Color::Cyan),
            PaletteEntry::Color(Color::Rgb(0, 206, 209)),
            PaletteEntry::Color(Color::Rgb(0, 139, 139)),
            PaletteEntry::Color(Color::Rgb(0, 128, 128)),
            PaletteEntry::Color(Color::Rgb(240, 248, 255)),
            PaletteEntry::Color(Color::Rgb(230, 230, 250)),
            PaletteEntry::Color(Color::Rgb(176, 224, 230)),
            PaletteEntry::Color(Color::LightBlue),
            PaletteEntry::Color(Color::Rgb(135, 206, 250)),
            PaletteEntry::Color(Color::Rgb(135, 206, 235)),
            PaletteEntry::Color(Color::Rgb(0, 191, 255)),
            PaletteEntry::Color(Color::Rgb(30, 144, 255)),
            PaletteEntry::Color(Color::Blue),
            PaletteEntry::Color(Color::Rgb(0, 0, 205)),
            PaletteEntry::Color(Color::Rgb(0, 0, 139)),
            PaletteEntry::Color(Color::Rgb(25, 25, 112)),
            PaletteEntry::Color(Color::Rgb(0, 0, 128)),
            PaletteEntry::Color(Color::Rgb(230, 230, 250)),
            PaletteEntry::Color(Color::Rgb(221, 160, 221)),
            PaletteEntry::Color(Color::Rgb(238, 130, 238)),
            PaletteEntry::Color(Color::LightMagenta),
            PaletteEntry::Color(Color::Rgb(218, 112, 214)),
            PaletteEntry::Color(Color::Rgb(186, 85, 211)),
            PaletteEntry::Color(Color::Magenta),
            PaletteEntry::Color(Color::Rgb(147, 112, 219)),
            PaletteEntry::Color(Color::Rgb(138, 43, 226)),
            PaletteEntry::Color(Color::Rgb(148, 0, 211)),
            PaletteEntry::Color(Color::Rgb(128, 0, 128)),
            PaletteEntry::Color(Color::Rgb(75, 0, 130)),
            PaletteEntry::Color(Color::Rgb(106, 90, 205)),
            PaletteEntry::Color(Color::Rgb(72, 61, 139)),
            PaletteEntry::Color(Color::Rgb(245, 222, 179)),
            PaletteEntry::Color(Color::Rgb(222, 184, 135)),
            PaletteEntry::Color(Color::Rgb(210, 180, 140)),
            PaletteEntry::Color(Color::Rgb(188, 143, 143)),
            PaletteEntry::Color(Color::Rgb(244, 164, 96)),
            PaletteEntry::Color(Color::Rgb(205, 133, 63)),
    ]
}



pub fn get_ansi_color_palette() -> Vec<PaletteEntry> {
    vec![
        PaletteEntry::Color(Color::Black),
        PaletteEntry::Color(Color::DarkGray),
        PaletteEntry::Color(Color::Red),
        PaletteEntry::Color(Color::LightRed),
        PaletteEntry::Color(Color::Green),
        PaletteEntry::Color(Color::LightGreen),
        PaletteEntry::Color(Color::Yellow),
        PaletteEntry::Color(Color::LightYellow),
        PaletteEntry::Color(Color::Blue),
        PaletteEntry::Color(Color::LightBlue),
        PaletteEntry::Color(Color::Magenta),
        PaletteEntry::Color(Color::LightMagenta),
        PaletteEntry::Color(Color::Cyan),
        PaletteEntry::Color(Color::LightCyan),
        PaletteEntry::Color(Color::Gray),
        PaletteEntry::Color(Color::White),
    ]
}




pub fn get_xterm256_color_palette() -> Vec<PaletteEntry> {
    let mut entries = get_ansi_color_palette(); // Start with the basic 16

  
    let levels: [u8; 6] = [0, 95, 135, 175, 215, 255];
    for r in 0..6 {
        for g in 0..6 {
            for b in 0..6 {
                entries.push(PaletteEntry::Color(Color::Rgb(levels[r], levels[g], levels[b])));
            }
        }
    }

  
    for i in 0..24 {
        let l = 8 + i * 10;
        entries.push(PaletteEntry::Color(Color::Rgb(l, l, l)));
    }

    entries
}


pub fn get_spectrum_color_palette() -> Vec<PaletteEntry> {
    let mut entries = Vec::new();

   
    for i in 0..32 {
        let l = (i * 8) as u8;
        entries.push(PaletteEntry::Color(Color::Rgb(l, l, l)));
    }


    let hue_steps = 32; 
    let sat_steps = 4; 
    let val_steps = 4;  

    for h in 0..hue_steps {
        for s in 1..=sat_steps {
            for v in 1..=val_steps {
                let hue = (h as f32 / hue_steps as f32) * 360.0;
                let saturation = s as f32 / sat_steps as f32;
                let value = v as f32 / val_steps as f32;

                let (r, g, b) = hsv_to_rgb(hue, saturation, value);
                entries.push(PaletteEntry::Color(Color::Rgb(r, g, b)));
            }
        }
    }
    entries
}

pub fn get_atari_color_palette() -> Vec<PaletteEntry> {
    vec![
        // Grays
        PaletteEntry::Color(Color::Rgb(0, 0, 0)),
        PaletteEntry::Color(Color::Rgb(64, 64, 64)),
        PaletteEntry::Color(Color::Rgb(112, 112, 112)),
        PaletteEntry::Color(Color::Rgb(152, 152, 152)),
        PaletteEntry::Color(Color::Rgb(184, 184, 184)),
        PaletteEntry::Color(Color::Rgb(208, 208, 208)),
        PaletteEntry::Color(Color::Rgb(228, 228, 228)),
        PaletteEntry::Color(Color::Rgb(252, 252, 252)),
        // Yellow/Browns
        PaletteEntry::Color(Color::Rgb(68, 40, 0)),
        PaletteEntry::Color(Color::Rgb(120, 74, 0)),
        PaletteEntry::Color(Color::Rgb(164, 104, 0)),
        PaletteEntry::Color(Color::Rgb(204, 132, 0)),
        PaletteEntry::Color(Color::Rgb(232, 154, 0)),
        PaletteEntry::Color(Color::Rgb(252, 174, 36)),
        PaletteEntry::Color(Color::Rgb(252, 194, 88)),
        PaletteEntry::Color(Color::Rgb(252, 218, 140)),
        // Oranges
        PaletteEntry::Color(Color::Rgb(112, 20, 0)),
        PaletteEntry::Color(Color::Rgb(168, 52, 0)),
        PaletteEntry::Color(Color::Rgb(208, 78, 0)),
        PaletteEntry::Color(Color::Rgb(244, 106, 0)),
        PaletteEntry::Color(Color::Rgb(252, 128, 0)),
        PaletteEntry::Color(Color::Rgb(252, 150, 48)),
        PaletteEntry::Color(Color::Rgb(252, 172, 92)),
        PaletteEntry::Color(Color::Rgb(252, 196, 140)),
        // Red/Oranges
        PaletteEntry::Color(Color::Rgb(132, 0, 0)),
        PaletteEntry::Color(Color::Rgb(192, 30, 0)),
        PaletteEntry::Color(Color::Rgb(232, 64, 0)),
        PaletteEntry::Color(Color::Rgb(252, 92, 20)),
        PaletteEntry::Color(Color::Rgb(252, 114, 68)),
        PaletteEntry::Color(Color::Rgb(252, 138, 108)),
        PaletteEntry::Color(Color::Rgb(252, 162, 148)),
        PaletteEntry::Color(Color::Rgb(252, 188, 188)),
        // Reds
        PaletteEntry::Color(Color::Rgb(140, 0, 0)),
        PaletteEntry::Color(Color::Rgb(204, 26, 26)),
        PaletteEntry::Color(Color::Rgb(232, 66, 66)),
        PaletteEntry::Color(Color::Rgb(252, 94, 94)),
        PaletteEntry::Color(Color::Rgb(252, 120, 120)),
        PaletteEntry::Color(Color::Rgb(252, 144, 144)),
        PaletteEntry::Color(Color::Rgb(252, 170, 170)),
        PaletteEntry::Color(Color::Rgb(252, 196, 196)),
        // Purples
        PaletteEntry::Color(Color::Rgb(124, 0, 64)),
        PaletteEntry::Color(Color::Rgb(184, 18, 108)),
        PaletteEntry::Color(Color::Rgb(216, 54, 144)),
        PaletteEntry::Color(Color::Rgb(244, 84, 172)),
        PaletteEntry::Color(Color::Rgb(252, 110, 196)),
        PaletteEntry::Color(Color::Rgb(252, 134, 212)),
        PaletteEntry::Color(Color::Rgb(252, 160, 228)),
        PaletteEntry::Color(Color::Rgb(252, 188, 244)),
        // Violets
        PaletteEntry::Color(Color::Rgb(84, 0, 116)),
        PaletteEntry::Color(Color::Rgb(136, 20, 168)),
        PaletteEntry::Color(Color::Rgb(172, 58, 208)),
        PaletteEntry::Color(Color::Rgb(204, 88, 236)),
        PaletteEntry::Color(Color::Rgb(224, 114, 252)),
        PaletteEntry::Color(Color::Rgb(236, 138, 252)),
        PaletteEntry::Color(Color::Rgb(248, 164, 252)),
        PaletteEntry::Color(Color::Rgb(252, 192, 252)),
        // Blue/Violets
        PaletteEntry::Color(Color::Rgb(36, 0, 140)),
        PaletteEntry::Color(Color::Rgb(80, 28, 192)),
        PaletteEntry::Color(Color::Rgb(116, 66, 224)),
        PaletteEntry::Color(Color::Rgb(148, 96, 248)),
        PaletteEntry::Color(Color::Rgb(172, 120, 252)),
        PaletteEntry::Color(Color::Rgb(192, 142, 252)),
        PaletteEntry::Color(Color::Rgb(212, 166, 252)),
        PaletteEntry::Color(Color::Rgb(232, 192, 252)),
        // Blues
        PaletteEntry::Color(Color::Rgb(0, 0, 148)),
        PaletteEntry::Color(Color::Rgb(26, 42, 200)),
        PaletteEntry::Color(Color::Rgb(62, 78, 228)),
        PaletteEntry::Color(Color::Rgb(92, 110, 252)),
        PaletteEntry::Color(Color::Rgb(118, 134, 252)),
        PaletteEntry::Color(Color::Rgb(140, 158, 252)),
        PaletteEntry::Color(Color::Rgb(166, 182, 252)),
        PaletteEntry::Color(Color::Rgb(192, 206, 252)),
        // Blue/Cyans
        PaletteEntry::Color(Color::Rgb(0, 48, 132)),
        PaletteEntry::Color(Color::Rgb(16, 84, 184)),
        PaletteEntry::Color(Color::Rgb(50, 118, 216)),
        PaletteEntry::Color(Color::Rgb(80, 146, 244)),
        PaletteEntry::Color(Color::Rgb(106, 168, 252)),
        PaletteEntry::Color(Color::Rgb(130, 188, 252)),
        PaletteEntry::Color(Color::Rgb(156, 208, 252)),
        PaletteEntry::Color(Color::Rgb(184, 228, 252)),
        // Cyans
        PaletteEntry::Color(Color::Rgb(0, 88, 88)),
        PaletteEntry::Color(Color::Rgb(0, 132, 132)),
        PaletteEntry::Color(Color::Rgb(38, 164, 164)),
        PaletteEntry::Color(Color::Rgb(68, 192, 192)),
        PaletteEntry::Color(Color::Rgb(94, 212, 212)),
        PaletteEntry::Color(Color::Rgb(118, 228, 228)),
        PaletteEntry::Color(Color::Rgb(146, 242, 242)),
        PaletteEntry::Color(Color::Rgb(176, 252, 252)),
        // Green/Cyans
        PaletteEntry::Color(Color::Rgb(0, 104, 52)),
        PaletteEntry::Color(Color::Rgb(0, 152, 84)),
        PaletteEntry::Color(Color::Rgb(22, 184, 116)),
        PaletteEntry::Color(Color::Rgb(56, 208, 144)),
        PaletteEntry::Color(Color::Rgb(84, 228, 164)),
        PaletteEntry::Color(Color::Rgb(110, 244, 184)),
        PaletteEntry::Color(Color::Rgb(138, 252, 204)),
        PaletteEntry::Color(Color::Rgb(170, 252, 224)),
        // Greens
        PaletteEntry::Color(Color::Rgb(0, 116, 0)),
        PaletteEntry::Color(Color::Rgb(20, 164, 20)),
        PaletteEntry::Color(Color::Rgb(58, 198, 58)),
        PaletteEntry::Color(Color::Rgb(88, 224, 88)),
        PaletteEntry::Color(Color::Rgb(114, 240, 114)),
        PaletteEntry::Color(Color::Rgb(138, 252, 138)),
        PaletteEntry::Color(Color::Rgb(164, 252, 164)),
        PaletteEntry::Color(Color::Rgb(192, 252, 192)),
        // Yellow/Greens
        PaletteEntry::Color(Color::Rgb(52, 108, 0)),
        PaletteEntry::Color(Color::Rgb(92, 152, 0)),
        PaletteEntry::Color(Color::Rgb(128, 184, 16)),
        PaletteEntry::Color(Color::Rgb(160, 212, 48)),
        PaletteEntry::Color(Color::Rgb(184, 232, 74)),
        PaletteEntry::Color(Color::Rgb(204, 244, 102)),
        PaletteEntry::Color(Color::Rgb(224, 252, 132)),
        PaletteEntry::Color(Color::Rgb(240, 252, 164)),
        // Orange/Greens
        PaletteEntry::Color(Color::Rgb(96, 88, 0)),
        PaletteEntry::Color(Color::Rgb(148, 126, 0)),
        PaletteEntry::Color(Color::Rgb(188, 160, 0)),
        PaletteEntry::Color(Color::Rgb(224, 188, 20)),
        PaletteEntry::Color(Color::Rgb(244, 208, 60)),
        PaletteEntry::Color(Color::Rgb(252, 224, 94)),
        PaletteEntry::Color(Color::Rgb(252, 238, 134)),
        PaletteEntry::Color(Color::Rgb(252, 252, 172)),
    ]
}


pub fn get_websafe_color_palette() -> Vec<PaletteEntry> {
    let mut entries = Vec::new();
    let levels: [u8; 6] = [0, 51, 102, 153, 204, 255];

    for r in levels {
        for g in levels {
            for b in levels {
                entries.push(PaletteEntry::Color(Color::Rgb(r, g, b)));
            }
        }
    }
    entries
}


pub fn get_websafe_extended_color_palette() -> Vec<PaletteEntry> {
    let mut entries = Vec::new();

    let levels: [u8; 11] = [0, 25, 51, 76, 102, 127, 153, 178, 204, 229, 255];

    for r in levels {
        for g in levels {
            for b in levels {
                entries.push(PaletteEntry::Color(Color::Rgb(r, g, b)));
            }
        }
    }
    entries
}





pub fn get_toned_color_palette() -> Vec<PaletteEntry> {
    let mut entries = Vec::new();

    // Define base colors by their Hue and Saturation.
    // (Hue from 0-360, Saturation from 0.0-1.0)
    let base_hues_sats = vec![
        (0.0, 0.0),      // Grayscale
        (0.0, 1.0),      // Red
        (30.0, 1.0),     // Orange
        (60.0, 1.0),     // Yellow
        (90.0, 1.0),     // Lime Green
        (120.0, 1.0),    // Green
        (150.0, 1.0),    // Sea Green
        (180.0, 1.0),    // Cyan
        (210.0, 1.0),    // Sky Blue
        (240.0, 1.0),    // Blue
        (270.0, 1.0),    // Purple
        (300.0, 1.0),    // Magenta
        (330.0, 1.0),    // Pink
        (30.0, 0.8),     // Brown (a desaturated orange)
    ];

    for (hue, saturation) in base_hues_sats {
        for j in 0..8 {
            // Create 8 steps of Value (brightness) from dark to light.
            // We use a range from 0.15 (dark) to 1.0 (full brightness).
            let value = 0.15 + (0.85 * (j as f32 / 7.0));

            let (r, g, b) = hsv_to_rgb(hue, saturation, value);
            entries.push(PaletteEntry::Color(Color::Rgb(r, g, b)));
        }
    }
    entries
}



pub fn get_red_tones_palette() -> Vec<PaletteEntry> {
    let mut entries = Vec::new();
    for i in 0..50 {
        let brightness = i as f32 / 49.0;
        let (r, g, b) = hsv_to_rgb(0.0, 1.0 - (brightness * 0.3), 0.2 + (brightness * 0.8));
        entries.push(PaletteEntry::Color(Color::Rgb(r, g, b)));
    }
    entries
}

pub fn get_blue_tones_palette() -> Vec<PaletteEntry> {
    let mut entries = Vec::new();
    for i in 0..50 {
        let brightness = i as f32 / 49.0;
        let (r, g, b) = hsv_to_rgb(240.0, 1.0 - (brightness * 0.3), 0.2 + (brightness * 0.8));
        entries.push(PaletteEntry::Color(Color::Rgb(r, g, b)));
    }
    entries
}

pub fn get_green_tones_palette() -> Vec<PaletteEntry> {
    let mut entries = Vec::new();
    for i in 0..50 {
        let brightness = i as f32 / 49.0;
        let (r, g, b) = hsv_to_rgb(120.0, 1.0 - (brightness * 0.3), 0.2 + (brightness * 0.8));
        entries.push(PaletteEntry::Color(Color::Rgb(r, g, b)));
    }
    entries
}


pub fn get_pink_tones_palette() -> Vec<PaletteEntry> {
    let mut entries = Vec::new();
    for i in 0..50 {
        let brightness = i as f32 / 49.0;
        let (r, g, b) = hsv_to_rgb(330.0, 1.0 - (brightness * 0.3), 0.2 + (brightness * 0.8));
        entries.push(PaletteEntry::Color(Color::Rgb(r, g, b)));
    }
    entries
}

pub fn get_brown_tones_palette() -> Vec<PaletteEntry> {
    let mut entries = Vec::new();
    for i in 0..50 {
        let brightness = i as f32 / 49.0;
        let (r, g, b) = hsv_to_rgb(30.0, 0.8 - (brightness * 0.3), 0.2 + (brightness * 0.6));
        entries.push(PaletteEntry::Color(Color::Rgb(r, g, b)));
    }
    entries
}

pub fn get_cyan_tones_palette() -> Vec<PaletteEntry> {
    let mut entries = Vec::new();
    for i in 0..50 {
        let brightness = i as f32 / 49.0;
        let (r, g, b) = hsv_to_rgb(180.0, 1.0 - (brightness * 0.3), 0.2 + (brightness * 0.8));
        entries.push(PaletteEntry::Color(Color::Rgb(r, g, b)));
    }
    entries
}




pub fn get_built_in_palettes() -> std::collections::HashMap<&'static str, fn() -> Vec<PaletteEntry>> {
    let mut palettes = std::collections::HashMap::new();
    palettes.insert("default", get_default_color_palette as fn() -> Vec<PaletteEntry>);
    palettes.insert("ansi", get_ansi_color_palette as fn() -> Vec<PaletteEntry>);
    palettes.insert("xterm256", get_xterm256_color_palette as fn() -> Vec<PaletteEntry>);
    palettes.insert("spectrum", get_spectrum_color_palette as fn() -> Vec<PaletteEntry>);
    palettes.insert("atari", get_atari_color_palette as fn() -> Vec<PaletteEntry>);
    palettes.insert("websafe", get_websafe_color_palette as fn() -> Vec<PaletteEntry>);
    palettes.insert("websafe_2", get_websafe_extended_color_palette as fn() -> Vec<PaletteEntry>);
    palettes.insert("toned", get_toned_color_palette as fn() -> Vec<PaletteEntry>);
    palettes.insert("red_tones", get_red_tones_palette as fn() -> Vec<PaletteEntry>);
    palettes.insert("blue_tones", get_blue_tones_palette as fn() -> Vec<PaletteEntry>);
    palettes.insert("green_tones", get_green_tones_palette as fn() -> Vec<PaletteEntry>);
    palettes.insert("pink_tones", get_pink_tones_palette as fn() -> Vec<PaletteEntry>);
    palettes.insert("brown_tones", get_brown_tones_palette as fn() -> Vec<PaletteEntry>);
    palettes.insert("cyan_tones", get_cyan_tones_palette as fn() -> Vec<PaletteEntry>);


    palettes
}

