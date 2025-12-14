use crate::board::Board;
use crate::piece::{Piece, PieceType};
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum MoveResult {
    Success,
    Failed,
    Locked,
    GameOver,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Game {
    pub board: Board,
    pub current_piece: Option<Piece>,
    pub next_piece: PieceType,
    pub hold_piece: Option<PieceType>,
    pub can_hold: bool,
    pub score: u32,
    pub level: u32,
    pub lines_cleared: u32,
    pub game_over: bool,
    bag: Vec<PieceType>,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            board: Board::new(),
            current_piece: None,
            next_piece: PieceType::T, // Will be replaced
            hold_piece: None,
            can_hold: true,
            score: 0,
            level: 1,
            lines_cleared: 0,
            game_over: false,
            bag: Vec::new(),
        };
        game.refill_bag();
        game.next_piece = game.draw_from_bag();
        game.spawn_piece();
        game
    }

    fn refill_bag(&mut self) {
        let mut pieces = PieceType::all().to_vec();
        pieces.shuffle(&mut thread_rng());
        self.bag = pieces;
    }

    fn draw_from_bag(&mut self) -> PieceType {
        if self.bag.is_empty() {
            self.refill_bag();
        }
        self.bag.pop().unwrap()
    }

    pub fn spawn_piece(&mut self) -> bool {
        let piece_type = self.next_piece;
        self.next_piece = self.draw_from_bag();
        let piece = Piece::new(piece_type);

        // Check if spawn position is valid
        let blocks = piece.get_blocks();
        if self.board.check_collision(&blocks) {
            self.game_over = true;
            self.current_piece = None;
            return false;
        }

        self.current_piece = Some(piece);
        self.can_hold = true;
        true
    }

    pub fn move_piece(&mut self, dx: i32, dy: i32) -> MoveResult {
        if self.game_over {
            return MoveResult::GameOver;
        }

        let Some(piece) = &self.current_piece else {
            return MoveResult::Failed;
        };

        let new_blocks = piece.get_blocks_after_move(dx, dy);

        if self.board.check_collision(&new_blocks) {
            MoveResult::Failed
        } else {
            let piece = self.current_piece.as_mut().unwrap();
            piece.x += dx;
            piece.y += dy;
            MoveResult::Success
        }
    }

    pub fn rotate(&mut self, clockwise: bool) -> MoveResult {
        if self.game_over {
            return MoveResult::GameOver;
        }

        let Some(piece) = &self.current_piece else {
            return MoveResult::Failed;
        };

        // O piece doesn't rotate
        if piece.piece_type == PieceType::O {
            return MoveResult::Success;
        }

        let kicks = piece.get_kicks(clockwise);

        for &kick in kicks.iter() {
            let new_blocks = piece.get_blocks_after_rotation(clockwise, kick);
            if !self.board.check_collision(&new_blocks) {
                let piece = self.current_piece.as_mut().unwrap();
                piece.rotate(clockwise);
                piece.x += kick.0;
                piece.y += kick.1;
                return MoveResult::Success;
            }
        }

        MoveResult::Failed
    }

    pub fn soft_drop(&mut self) -> MoveResult {
        let result = self.move_piece(0, -1);
        if result == MoveResult::Success {
            self.score += 1; // Soft drop bonus
        }
        result
    }

    pub fn hard_drop(&mut self) -> MoveResult {
        if self.game_over {
            return MoveResult::GameOver;
        }

        let mut drop_distance = 0;
        while self.move_piece(0, -1) == MoveResult::Success {
            drop_distance += 1;
        }
        self.score += drop_distance * 2; // Hard drop bonus

        self.lock_piece()
    }

    pub fn tick(&mut self) -> MoveResult {
        if self.game_over {
            return MoveResult::GameOver;
        }

        if self.current_piece.is_none() {
            if !self.spawn_piece() {
                return MoveResult::GameOver;
            }
            return MoveResult::Success;
        }

        let move_result = self.move_piece(0, -1);

        if move_result == MoveResult::Failed {
            self.lock_piece()
        } else {
            move_result
        }
    }

    fn lock_piece(&mut self) -> MoveResult {
        let Some(piece) = self.current_piece.take() else {
            return MoveResult::Failed;
        };

        let blocks = piece.get_blocks();
        let cell = piece.piece_type.to_cell();
        self.board.lock_cells(&blocks, cell);

        // Clear lines and score
        let lines = self.board.clear_lines();
        if lines > 0 {
            self.lines_cleared += lines;
            self.score += self.calculate_line_score(lines);
            self.update_level();
        }

        // Check game over
        if self.board.is_topped_out() {
            self.game_over = true;
            return MoveResult::GameOver;
        }

        // Spawn next piece
        if !self.spawn_piece() {
            return MoveResult::GameOver;
        }

        MoveResult::Locked
    }

    fn calculate_line_score(&self, lines: u32) -> u32 {
        let base = match lines {
            1 => 100,
            2 => 300,
            3 => 500,
            4 => 800, // Tetris!
            _ => 0,
        };
        base * self.level
    }

    fn update_level(&mut self) {
        // Level up every 10 lines
        self.level = (self.lines_cleared / 10) + 1;
    }

    pub fn hold(&mut self) -> MoveResult {
        if self.game_over || !self.can_hold {
            return MoveResult::Failed;
        }

        let Some(current) = &self.current_piece else {
            return MoveResult::Failed;
        };

        let current_type = current.piece_type;

        if let Some(held) = self.hold_piece {
            // Swap with held piece
            self.hold_piece = Some(current_type);
            self.current_piece = Some(Piece::new(held));
        } else {
            // First hold
            self.hold_piece = Some(current_type);
            self.spawn_piece();
        }

        self.can_hold = false;
        MoveResult::Success
    }

    /// Get ghost piece Y position (where piece would land)
    pub fn get_ghost_y(&self) -> Option<i32> {
        let piece = self.current_piece.as_ref()?;
        let mut ghost_y = piece.y;

        loop {
            let test_blocks: Vec<(i32, i32)> = piece
                .get_blocks()
                .iter()
                .map(|&(x, y)| (x, y - (piece.y - ghost_y) - 1))
                .collect();

            if self.board.check_collision(&test_blocks) {
                break;
            }
            ghost_y -= 1;
        }

        Some(ghost_y)
    }

    /// Get drop speed in milliseconds based on level (NES-style curve)
    pub fn get_drop_interval(&self) -> u32 {
        // NES Tetris-inspired speed curve (frames at 60fps, converted to ms)
        let frames = match self.level {
            1 => 48,
            2 => 43,
            3 => 38,
            4 => 33,
            5 => 28,
            6 => 23,
            7 => 18,
            8 => 13,
            9 => 8,
            10..=12 => 6,
            13..=15 => 5,
            16..=18 => 4,
            19..=28 => 3,
            _ => 2, // Level 29+
        };
        // Convert frames to ms (assuming 60fps base)
        (frames * 1000) / 60
    }

    /// Get acceleration factor based on piece height (subtle: 1.0 to 1.15)
    pub fn get_height_acceleration(&self) -> f32 {
        if let Some(piece) = &self.current_piece {
            // Lower pieces fall slightly faster
            let height_ratio = 1.0 - (piece.y as f32 / 24.0);
            1.0 + (height_ratio * 0.15) // Max 15% faster at bottom
        } else {
            1.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new();
        assert!(!game.game_over);
        assert!(game.current_piece.is_some());
        assert_eq!(game.score, 0);
        assert_eq!(game.level, 1);
    }

    #[test]
    fn test_move_piece() {
        let mut game = Game::new();
        let initial_x = game.current_piece.as_ref().unwrap().x;

        assert_eq!(game.move_piece(1, 0), MoveResult::Success);
        assert_eq!(game.current_piece.as_ref().unwrap().x, initial_x + 1);

        assert_eq!(game.move_piece(-1, 0), MoveResult::Success);
        assert_eq!(game.current_piece.as_ref().unwrap().x, initial_x);
    }

    #[test]
    fn test_line_clear_scoring() {
        let game = Game::new();
        assert_eq!(game.calculate_line_score(1), 100);
        assert_eq!(game.calculate_line_score(4), 800);
    }
}
