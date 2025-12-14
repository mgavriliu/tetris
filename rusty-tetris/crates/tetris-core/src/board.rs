use serde::{Deserialize, Serialize};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;
pub const BUFFER_HEIGHT: usize = 4; // Hidden rows above visible area

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum Cell {
    Empty = 0,
    I = 1,
    O = 2,
    T = 3,
    S = 4,
    Z = 5,
    J = 6,
    L = 7,
}

impl Cell {
    pub fn is_empty(self) -> bool {
        matches!(self, Cell::Empty)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Board {
    // Grid stored as row-major, bottom row is index 0
    grid: [[Cell; WIDTH]; HEIGHT + BUFFER_HEIGHT],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[Cell::Empty; WIDTH]; HEIGHT + BUFFER_HEIGHT],
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<Cell> {
        if x < 0 || x >= WIDTH as i32 || y < 0 || y >= (HEIGHT + BUFFER_HEIGHT) as i32 {
            None
        } else {
            Some(self.grid[y as usize][x as usize])
        }
    }

    pub fn set(&mut self, x: i32, y: i32, cell: Cell) {
        if x >= 0 && x < WIDTH as i32 && y >= 0 && y < (HEIGHT + BUFFER_HEIGHT) as i32 {
            self.grid[y as usize][x as usize] = cell;
        }
    }

    /// Check if a position is valid (in bounds and empty)
    pub fn is_valid_position(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= WIDTH as i32 || y < 0 {
            return false;
        }
        if y >= (HEIGHT + BUFFER_HEIGHT) as i32 {
            return true; // Above the board is valid
        }
        self.grid[y as usize][x as usize].is_empty()
    }

    /// Check if blocks at given positions would collide
    pub fn check_collision(&self, positions: &[(i32, i32)]) -> bool {
        positions.iter().any(|&(x, y)| !self.is_valid_position(x, y))
    }

    /// Lock a piece onto the board
    pub fn lock_cells(&mut self, positions: &[(i32, i32)], cell: Cell) {
        for &(x, y) in positions {
            self.set(x, y, cell);
        }
    }

    /// Clear completed lines and return count
    pub fn clear_lines(&mut self) -> u32 {
        let mut lines_cleared = 0;
        let mut write_row = 0;

        for read_row in 0..(HEIGHT + BUFFER_HEIGHT) {
            let is_full = self.grid[read_row].iter().all(|&cell| !cell.is_empty());

            if is_full {
                lines_cleared += 1;
            } else {
                if write_row != read_row {
                    self.grid[write_row] = self.grid[read_row];
                }
                write_row += 1;
            }
        }

        // Fill remaining rows with empty
        while write_row < HEIGHT + BUFFER_HEIGHT {
            self.grid[write_row] = [Cell::Empty; WIDTH];
            write_row += 1;
        }

        lines_cleared
    }

    /// Check if the game is over (blocks in buffer zone after lock)
    pub fn is_topped_out(&self) -> bool {
        for row in HEIGHT..(HEIGHT + BUFFER_HEIGHT) {
            if self.grid[row].iter().any(|&cell| !cell.is_empty()) {
                return true;
            }
        }
        false
    }

    /// Get visible grid as flat array for JS (top-to-bottom for SVG rendering)
    pub fn get_visible_grid(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(WIDTH * HEIGHT);
        // Output from top row (HEIGHT-1) to bottom row (0) for SVG coordinate system
        for row in (0..HEIGHT).rev() {
            for col in 0..WIDTH {
                result.push(self.grid[row][col] as u8);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                assert!(board.get(x as i32, y as i32).unwrap().is_empty());
            }
        }
    }

    #[test]
    fn test_line_clear() {
        let mut board = Board::new();
        // Fill bottom row
        for x in 0..WIDTH {
            board.set(x as i32, 0, Cell::I);
        }
        assert_eq!(board.clear_lines(), 1);
        // Bottom row should now be empty
        for x in 0..WIDTH {
            assert!(board.get(x as i32, 0).unwrap().is_empty());
        }
    }

    #[test]
    fn test_collision_detection() {
        let mut board = Board::new();
        board.set(5, 0, Cell::T);

        assert!(board.check_collision(&[(5, 0)]));
        assert!(!board.check_collision(&[(4, 0)]));
        assert!(board.check_collision(&[(-1, 0)])); // Out of bounds
        assert!(board.check_collision(&[(0, -1)])); // Below board
    }
}
