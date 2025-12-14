pub mod board;
pub mod controller;
pub mod game;
pub mod input;
pub mod piece;
pub mod render;

use controller::GameController;
use wasm_bindgen::prelude::*;

/// Main WASM interface - a thin wrapper around GameController
#[wasm_bindgen]
pub struct Tetris {
    controller: GameController,
}

#[wasm_bindgen]
impl Tetris {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            controller: GameController::new(),
        }
    }

    /// Update game state. Call every frame with delta time in ms.
    /// Returns true if render state changed.
    pub fn update(&mut self, delta_ms: f64) -> bool {
        self.controller.update(delta_ms)
    }

    /// Handle key down event
    /// key: 0=left, 1=right, 2=down, 3=space, 4=up/x, 5=z, 6=c/shift, 7=p/esc, 8=enter, 9=r
    pub fn key_down(&mut self, key: u8) {
        self.controller.key_down(key);
    }

    /// Handle key up event
    pub fn key_up(&mut self, key: u8) {
        self.controller.key_up(key);
    }

    /// Called when window loses focus
    pub fn on_blur(&mut self) {
        self.controller.on_blur();
    }

    /// Get current game state: 0=idle, 1=playing, 2=paused, 3=gameOver
    pub fn get_state(&self) -> u8 {
        self.controller.get_state()
    }

    /// Get score
    pub fn get_score(&self) -> u32 {
        self.controller.get_score()
    }

    /// Get level
    pub fn get_level(&self) -> u32 {
        self.controller.get_level()
    }

    /// Get lines cleared
    pub fn get_lines(&self) -> u32 {
        self.controller.get_lines()
    }

    // ===== Render data methods =====
    // Returns flat arrays: each cell is 4 bytes [x, y, color, opacity]

    /// Get board cells (non-empty only)
    pub fn get_board_cells(&self) -> Vec<u8> {
        self.controller.get_render_state().to_flat_arrays().board
    }

    /// Get current piece cells
    pub fn get_piece_cells(&self) -> Vec<u8> {
        self.controller.get_render_state().to_flat_arrays().piece
    }

    /// Get ghost piece cells
    pub fn get_ghost_cells(&self) -> Vec<u8> {
        self.controller.get_render_state().to_flat_arrays().ghost
    }

    /// Get next piece preview cells
    pub fn get_next_cells(&self) -> Vec<u8> {
        self.controller.get_render_state().to_flat_arrays().next
    }

    /// Get hold piece preview cells
    pub fn get_hold_cells(&self) -> Vec<u8> {
        self.controller.get_render_state().to_flat_arrays().hold
    }

    /// Check if hold is available
    pub fn is_hold_available(&self) -> bool {
        self.controller
            .game
            .as_ref()
            .map(|g| g.can_hold)
            .unwrap_or(true)
    }
}

impl Default for Tetris {
    fn default() -> Self {
        Self::new()
    }
}

// Key code constants for JS
pub const KEY_LEFT: u8 = 0;
pub const KEY_RIGHT: u8 = 1;
pub const KEY_DOWN: u8 = 2;
pub const KEY_SPACE: u8 = 3;
pub const KEY_ROTATE_CW: u8 = 4;
pub const KEY_ROTATE_CCW: u8 = 5;
pub const KEY_HOLD: u8 = 6;
pub const KEY_PAUSE: u8 = 7;
pub const KEY_START: u8 = 8;
pub const KEY_RESTART: u8 = 9;

// Game state constants for JS
pub const STATE_IDLE: u8 = 0;
pub const STATE_PLAYING: u8 = 1;
pub const STATE_PAUSED: u8 = 2;
pub const STATE_GAME_OVER: u8 = 3;

// Cell colors for rendering
#[wasm_bindgen]
pub fn get_color(cell_type: u8) -> String {
    match cell_type {
        0 => "#1a1a2e".to_string(), // Empty
        1 => "#00f5ff".to_string(), // I - Cyan
        2 => "#ffd700".to_string(), // O - Yellow
        3 => "#9d4edd".to_string(), // T - Purple
        4 => "#00ff7f".to_string(), // S - Green
        5 => "#ff6b6b".to_string(), // Z - Red
        6 => "#4169e1".to_string(), // J - Blue
        7 => "#ff8c00".to_string(), // L - Orange
        _ => "#1a1a2e".to_string(),
    }
}
