use crate::logging;
use eframe::egui::{self, Painter, Rect, Rgba, Pos2};
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

use std::time::Instant;

pub struct AnimationState {
    code_chars: Vec<CodeChar>,
    speed_factor: f32,
    last_update: Instant,
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

        Self { 
            code_chars, 
            speed_factor: 1.0,
            last_update: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update).as_secs_f32();
        self.last_update = now;

        let mut rng = rand::thread_rng();
        for code_char in &mut self.code_chars {
            code_char.y += code_char.speed * self.speed_factor * delta_time;
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
                egui::FontId::proportional(24.0), // Use FontId::proportional directly
                code_char.color.into(),
            );
            // logging::debug_info(&format!("Drew character {} at ({}, {})", code_char.character, x, y));
        }
        // logging::debug_info("Background drawn");
    }

    pub fn set_speed_factor(&mut self, speed_factor: f32) {
        self.speed_factor = speed_factor;
        logging::debug_info(&format!("Speed factor set to: {}", self.speed_factor));
    }
}

pub fn speedometer(speed: u8) -> f32 {
    // Convert the speed to a factor (e.g., 01-99 maps to 0.01-0.99)
    let factor = speed as f32 / 100.0;
    factor
}

pub fn init_module() -> Result<(), String> {
    let initialization_passed = true;
    if initialization_passed {
        logging::debug_info("gui_engine_animation module is online");
        Ok(())
    } else {
        Err("gui_engine_animation module initialization failed".to_string())
    }
}
