use crate::logging;
use eframe::egui::{self, Context, Painter, Rect, Rgba, Pos2};
use rand::Rng;
use rand::prelude::SliceRandom; // Import the SliceRandom trait

// Define the set of Kanji characters to use
const KANJI_CHARACTERS: &[char] = &[
    '知', '慧', '悟', '愛', '和', '報', '済',
];

// Define the dimmed pride flag colors with fixed opacity
const DIMMED_PRIDE_COLORS: &[Rgba] = &[
    Rgba::from_rgba_premultiplied(0.5, 0.0, 0.0, 0.1), // Red
    Rgba::from_rgba_premultiplied(0.5, 0.25, 0.0, 0.1), // Orange
    Rgba::from_rgba_premultiplied(0.5, 0.5, 0.0, 0.1), // Yellow
    Rgba::from_rgba_premultiplied(0.0, 0.5, 0.0, 0.1), // Green
    Rgba::from_rgba_premultiplied(0.0, 0.0, 0.5, 0.1), // Blue
    Rgba::from_rgba_premultiplied(0.25, 0.0, 0.25, 0.1), // Purple
];

pub struct CodeChar {
    character: char,
    x: f32,
    y: f32,
    speed: f32,
    color: Rgba,
}

pub struct AnimationState {
    code_chars: Vec<CodeChar>,
}

impl AnimationState {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut code_chars = Vec::new();

        for _ in 0..100 {
            let color = *DIMMED_PRIDE_COLORS.choose(&mut rng).unwrap();
            code_chars.push(CodeChar {
                character: *KANJI_CHARACTERS.choose(&mut rng).unwrap(),
                x: rng.gen_range(0.0..1.0),
                y: rng.gen_range(0.0..1.0),
                speed: rng.gen_range(0.01..0.05), // Slower speed range
                color,
            });
        }

        logging::debug_info("AnimationState initialized with characters");

        Self { code_chars }
    }

    pub fn update(&mut self) {
        let mut rng = rand::thread_rng();
        for code_char in &mut self.code_chars {
            code_char.y += code_char.speed;
            if code_char.y > 1.0 {
                code_char.y = 0.0;
                code_char.character = *KANJI_CHARACTERS.choose(&mut rng).unwrap();
                code_char.x = rng.gen_range(0.0..1.0);
                code_char.speed = rng.gen_range(0.01..0.05); // Slower speed range
                code_char.color = *DIMMED_PRIDE_COLORS.choose(&mut rng).unwrap(); // Random dimmed pride color with fixed opacity
            }
        }
        logging::debug_info("AnimationState updated");
    }

    pub fn draw_background(&self, painter: &Painter, rect: Rect) {
        for code_char in &self.code_chars {
            let x = rect.width() * code_char.x;
            let y = rect.height() * code_char.y;
            painter.text(
                Pos2::new(rect.left() + x, y),
                egui::Align2::CENTER_TOP,
                code_char.character,
                egui::FontId::proportional(24.0),
                code_char.color.into(),
            );
            logging::debug_info(&format!("Drew character {} at ({}, {})", code_char.character, x, y));
        }
        logging::debug_info("Background drawn");
    }
}

pub fn init_module() -> Result<(), String> {
    let initialization_passed = true;
    if initialization_passed {
        logging::debug_info("animation module is online");
        Ok(())
    } else {
        Err("animation module initialization failed".to_string())
    }
}
