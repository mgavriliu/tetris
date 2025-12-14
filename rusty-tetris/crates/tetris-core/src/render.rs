use crate::board::{HEIGHT, WIDTH};
use crate::game::Game;
use crate::piece::Piece;
use serde::{Deserialize, Serialize};

/// A cell to render with position and color
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct RenderCell {
    pub x: u8,
    pub y: u8,
    pub color: u8,
    pub opacity: u8, // 0-255
}

/// Complete render state for one frame
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RenderState {
    /// Board cells (only non-empty)
    pub board_cells: Vec<RenderCell>,
    /// Current piece cells
    pub piece_cells: Vec<RenderCell>,
    /// Ghost piece cells
    pub ghost_cells: Vec<RenderCell>,
    /// Next piece preview cells (relative coords, centered)
    pub next_cells: Vec<RenderCell>,
    /// Hold piece preview cells (relative coords, centered)
    pub hold_cells: Vec<RenderCell>,
    /// Whether hold is available (affects opacity)
    pub hold_available: bool,
    /// Current score
    pub score: u32,
    /// Current level
    pub level: u32,
    /// Lines cleared
    pub lines: u32,
}

impl RenderState {
    pub fn from_game(game: &Game) -> Self {
        let mut state = RenderState::default();

        state.score = game.score;
        state.level = game.level;
        state.lines = game.lines_cleared;
        state.hold_available = game.can_hold;

        // Board cells (only non-empty)
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if let Some(cell) = game.board.get(x as i32, y as i32) {
                    if !cell.is_empty() {
                        state.board_cells.push(RenderCell {
                            x: x as u8,
                            y: (HEIGHT - 1 - y) as u8, // Flip for SVG
                            color: cell as u8,
                            opacity: 255,
                        });
                    }
                }
            }
        }

        // Current piece
        if let Some(piece) = &game.current_piece {
            let color = piece.piece_type.to_cell() as u8;
            for (px, py) in piece.get_blocks() {
                if py >= 0 && py < HEIGHT as i32 {
                    state.piece_cells.push(RenderCell {
                        x: px as u8,
                        y: (HEIGHT as i32 - 1 - py) as u8,
                        color,
                        opacity: 255,
                    });
                }
            }

            // Ghost piece
            if let Some(ghost_y) = game.get_ghost_y() {
                let dy = piece.y - ghost_y;
                for (px, py) in piece.get_blocks() {
                    let gy = py - dy;
                    if gy >= 0 && gy < HEIGHT as i32 {
                        state.ghost_cells.push(RenderCell {
                            x: px as u8,
                            y: (HEIGHT as i32 - 1 - gy) as u8,
                            color,
                            opacity: 77, // ~30%
                        });
                    }
                }
            }
        }

        // Next piece preview
        state.next_cells = Self::preview_cells(game.next_piece, 255);

        // Hold piece preview
        if let Some(hold_type) = game.hold_piece {
            let opacity = if game.can_hold { 255 } else { 102 }; // 40% if unavailable
            state.hold_cells = Self::preview_cells(hold_type, opacity);
        }

        state
    }

    fn preview_cells(piece_type: crate::piece::PieceType, opacity: u8) -> Vec<RenderCell> {
        let piece = Piece::new(piece_type);
        let blocks = piece.get_blocks();
        let color = piece_type.to_cell() as u8;

        // Find bounds for centering
        let min_x = blocks.iter().map(|(x, _)| *x).min().unwrap_or(0);
        let max_x = blocks.iter().map(|(x, _)| *x).max().unwrap_or(0);
        let min_y = blocks.iter().map(|(_, y)| *y).min().unwrap_or(0);
        let max_y = blocks.iter().map(|(_, y)| *y).max().unwrap_or(0);

        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;
        let offset_x = (4 - width) / 2 - min_x;
        let offset_y = (2 - height) / 2 - min_y;

        blocks
            .iter()
            .map(|(x, y)| RenderCell {
                x: (x + offset_x) as u8,
                y: (1 - (y + offset_y)) as u8, // Flip Y for SVG
                color,
                opacity,
            })
            .collect()
    }

    /// Serialize to flat arrays for efficient WASM transfer
    pub fn to_flat_arrays(&self) -> RenderArrays {
        RenderArrays {
            // Each cell: x, y, color, opacity (4 bytes)
            board: self.cells_to_bytes(&self.board_cells),
            piece: self.cells_to_bytes(&self.piece_cells),
            ghost: self.cells_to_bytes(&self.ghost_cells),
            next: self.cells_to_bytes(&self.next_cells),
            hold: self.cells_to_bytes(&self.hold_cells),
            score: self.score,
            level: self.level,
            lines: self.lines,
            hold_available: self.hold_available,
        }
    }

    fn cells_to_bytes(&self, cells: &[RenderCell]) -> Vec<u8> {
        cells
            .iter()
            .flat_map(|c| vec![c.x, c.y, c.color, c.opacity])
            .collect()
    }
}

#[derive(Clone, Debug, Default)]
pub struct RenderArrays {
    pub board: Vec<u8>,
    pub piece: Vec<u8>,
    pub ghost: Vec<u8>,
    pub next: Vec<u8>,
    pub hold: Vec<u8>,
    pub score: u32,
    pub level: u32,
    pub lines: u32,
    pub hold_available: bool,
}
