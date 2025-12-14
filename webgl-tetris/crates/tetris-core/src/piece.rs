use crate::board::{Cell, HEIGHT, WIDTH};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum PieceType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

impl PieceType {
    pub fn to_cell(self) -> Cell {
        match self {
            PieceType::I => Cell::I,
            PieceType::O => Cell::O,
            PieceType::T => Cell::T,
            PieceType::S => Cell::S,
            PieceType::Z => Cell::Z,
            PieceType::J => Cell::J,
            PieceType::L => Cell::L,
        }
    }

    pub fn all() -> [PieceType; 7] {
        [
            PieceType::I,
            PieceType::O,
            PieceType::T,
            PieceType::S,
            PieceType::Z,
            PieceType::J,
            PieceType::L,
        ]
    }

    /// Get the block offsets for this piece at rotation state 0
    /// Origin is the rotation center
    fn base_blocks(self) -> [(i32, i32); 4] {
        match self {
            // I piece (horizontal)
            PieceType::I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
            // O piece (doesn't rotate)
            PieceType::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            // T piece
            PieceType::T => [(-1, 0), (0, 0), (1, 0), (0, 1)],
            // S piece
            PieceType::S => [(0, 0), (1, 0), (-1, 1), (0, 1)],
            // Z piece
            PieceType::Z => [(-1, 0), (0, 0), (0, 1), (1, 1)],
            // J piece
            PieceType::J => [(-1, 1), (-1, 0), (0, 0), (1, 0)],
            // L piece
            PieceType::L => [(-1, 0), (0, 0), (1, 0), (1, 1)],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Rotation {
    R0 = 0,
    R90 = 1,
    R180 = 2,
    R270 = 3,
}

impl Rotation {
    pub fn clockwise(self) -> Rotation {
        match self {
            Rotation::R0 => Rotation::R90,
            Rotation::R90 => Rotation::R180,
            Rotation::R180 => Rotation::R270,
            Rotation::R270 => Rotation::R0,
        }
    }

    pub fn counter_clockwise(self) -> Rotation {
        match self {
            Rotation::R0 => Rotation::R270,
            Rotation::R90 => Rotation::R0,
            Rotation::R180 => Rotation::R90,
            Rotation::R270 => Rotation::R180,
        }
    }
}

/// SRS wall kick data for J, L, S, T, Z pieces
const SRS_KICKS_JLSTZ: [[(i32, i32); 5]; 8] = [
    // 0 -> R (clockwise from 0)
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    // R -> 0 (counter-clockwise from R)
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    // R -> 2 (clockwise from R)
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    // 2 -> R (counter-clockwise from 2)
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    // 2 -> L (clockwise from 2)
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
    // L -> 2 (counter-clockwise from L)
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    // L -> 0 (clockwise from L)
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    // 0 -> L (counter-clockwise from 0)
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
];

/// SRS wall kick data for I piece
const SRS_KICKS_I: [[(i32, i32); 5]; 8] = [
    // 0 -> R
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    // R -> 0
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    // R -> 2
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
    // 2 -> R
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    // 2 -> L
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    // L -> 2
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    // L -> 0
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    // 0 -> L
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
];

fn get_kick_index(from: Rotation, to: Rotation) -> usize {
    match (from, to) {
        (Rotation::R0, Rotation::R90) => 0,
        (Rotation::R90, Rotation::R0) => 1,
        (Rotation::R90, Rotation::R180) => 2,
        (Rotation::R180, Rotation::R90) => 3,
        (Rotation::R180, Rotation::R270) => 4,
        (Rotation::R270, Rotation::R180) => 5,
        (Rotation::R270, Rotation::R0) => 6,
        (Rotation::R0, Rotation::R270) => 7,
        _ => 0,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub x: i32,
    pub y: i32,
    pub rotation: Rotation,
}

impl Piece {
    pub fn new(piece_type: PieceType) -> Self {
        Self {
            piece_type,
            x: (WIDTH as i32) / 2 - 1,
            y: HEIGHT as i32, // Spawn in buffer zone
            rotation: Rotation::R0,
        }
    }

    /// Get the absolute positions of all blocks
    pub fn get_blocks(&self) -> [(i32, i32); 4] {
        let base = self.piece_type.base_blocks();
        let mut result = [(0, 0); 4];

        for (i, (bx, by)) in base.iter().enumerate() {
            let (rx, ry) = self.rotate_block(*bx, *by);
            result[i] = (self.x + rx, self.y + ry);
        }

        result
    }

    /// Rotate a single block offset by the current rotation
    fn rotate_block(&self, x: i32, y: i32) -> (i32, i32) {
        match self.rotation {
            Rotation::R0 => (x, y),
            Rotation::R90 => (y, -x),
            Rotation::R180 => (-x, -y),
            Rotation::R270 => (-y, x),
        }
    }

    /// Get SRS wall kicks for a rotation
    pub fn get_kicks(&self, clockwise: bool) -> &'static [(i32, i32); 5] {
        let new_rotation = if clockwise {
            self.rotation.clockwise()
        } else {
            self.rotation.counter_clockwise()
        };

        let kick_index = get_kick_index(self.rotation, new_rotation);

        match self.piece_type {
            PieceType::I => &SRS_KICKS_I[kick_index],
            PieceType::O => &[(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)], // O doesn't kick
            _ => &SRS_KICKS_JLSTZ[kick_index],
        }
    }

    /// Apply a rotation (without checking collision)
    pub fn rotate(&mut self, clockwise: bool) {
        self.rotation = if clockwise {
            self.rotation.clockwise()
        } else {
            self.rotation.counter_clockwise()
        };
    }

    /// Get blocks after applying a move
    pub fn get_blocks_after_move(&self, dx: i32, dy: i32) -> [(i32, i32); 4] {
        let blocks = self.get_blocks();
        let mut result = [(0, 0); 4];
        for (i, (x, y)) in blocks.iter().enumerate() {
            result[i] = (x + dx, y + dy);
        }
        result
    }

    /// Get blocks after applying rotation and kick
    pub fn get_blocks_after_rotation(&self, clockwise: bool, kick: (i32, i32)) -> [(i32, i32); 4] {
        let new_rotation = if clockwise {
            self.rotation.clockwise()
        } else {
            self.rotation.counter_clockwise()
        };

        let base = self.piece_type.base_blocks();
        let mut result = [(0, 0); 4];

        for (i, (bx, by)) in base.iter().enumerate() {
            let (rx, ry) = match new_rotation {
                Rotation::R0 => (*bx, *by),
                Rotation::R90 => (*by, -*bx),
                Rotation::R180 => (-*bx, -*by),
                Rotation::R270 => (-*by, *bx),
            };
            result[i] = (self.x + rx + kick.0, self.y + ry + kick.1);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_creation() {
        let piece = Piece::new(PieceType::T);
        assert_eq!(piece.piece_type, PieceType::T);
        assert_eq!(piece.rotation, Rotation::R0);
    }

    #[test]
    fn test_rotation_cycle() {
        let mut rot = Rotation::R0;
        rot = rot.clockwise();
        assert_eq!(rot, Rotation::R90);
        rot = rot.clockwise();
        assert_eq!(rot, Rotation::R180);
        rot = rot.clockwise();
        assert_eq!(rot, Rotation::R270);
        rot = rot.clockwise();
        assert_eq!(rot, Rotation::R0);
    }

    #[test]
    fn test_t_piece_blocks() {
        let mut piece = Piece::new(PieceType::T);
        piece.x = 5;
        piece.y = 10;

        let blocks = piece.get_blocks();
        // T at R0: [(-1, 0), (0, 0), (1, 0), (0, 1)]
        assert!(blocks.contains(&(4, 10)));
        assert!(blocks.contains(&(5, 10)));
        assert!(blocks.contains(&(6, 10)));
        assert!(blocks.contains(&(5, 11)));
    }
}
