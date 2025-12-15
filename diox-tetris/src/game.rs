use rand::seq::SliceRandom;
use rand::thread_rng;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;
pub const BUFFER_HEIGHT: usize = 4;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GameState {
  Idle,
  Playing,
  Paused,
  GameOver,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MoveResult {
  Success,
  Failed,
  Locked,
  GameOver,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

  fn base_blocks(self) -> [(i32, i32); 4] {
    match self {
      PieceType::I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
      PieceType::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
      PieceType::T => [(-1, 0), (0, 0), (1, 0), (0, 1)],
      PieceType::S => [(0, 0), (1, 0), (-1, 1), (0, 1)],
      PieceType::Z => [(-1, 0), (0, 0), (0, 1), (1, 1)],
      PieceType::J => [(-1, 1), (-1, 0), (0, 0), (1, 0)],
      PieceType::L => [(-1, 0), (0, 0), (1, 0), (1, 1)],
    }
  }

  pub fn preview_blocks(self) -> [(i32, i32); 4] {
    self.base_blocks()
  }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

const SRS_KICKS_JLSTZ: [[(i32, i32); 5]; 8] = [
  [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
  [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
  [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
  [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
  [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
  [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
  [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
  [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
];

const SRS_KICKS_I: [[(i32, i32); 5]; 8] = [
  [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
  [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
  [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
  [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
  [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
  [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
  [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
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

#[derive(Clone, Debug, PartialEq)]
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
      y: HEIGHT as i32,
      rotation: Rotation::R0,
    }
  }

  pub fn get_blocks(&self) -> [(i32, i32); 4] {
    let base = self.piece_type.base_blocks();
    let mut result = [(0, 0); 4];
    for (i, (bx, by)) in base.iter().enumerate() {
      let (rx, ry) = self.rotate_block(*bx, *by);
      result[i] = (self.x + rx, self.y + ry);
    }
    result
  }

  fn rotate_block(&self, x: i32, y: i32) -> (i32, i32) {
    match self.rotation {
      Rotation::R0 => (x, y),
      Rotation::R90 => (y, -x),
      Rotation::R180 => (-x, -y),
      Rotation::R270 => (-y, x),
    }
  }

  pub fn get_kicks(&self, clockwise: bool) -> &'static [(i32, i32); 5] {
    let new_rotation = if clockwise {
      self.rotation.clockwise()
    } else {
      self.rotation.counter_clockwise()
    };
    let kick_index = get_kick_index(self.rotation, new_rotation);
    match self.piece_type {
      PieceType::I => &SRS_KICKS_I[kick_index],
      PieceType::O => &[(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)],
      _ => &SRS_KICKS_JLSTZ[kick_index],
    }
  }

  pub fn rotate(&mut self, clockwise: bool) {
    self.rotation = if clockwise {
      self.rotation.clockwise()
    } else {
      self.rotation.counter_clockwise()
    };
  }

  pub fn get_blocks_after_move(&self, dx: i32, dy: i32) -> [(i32, i32); 4] {
    let blocks = self.get_blocks();
    let mut result = [(0, 0); 4];
    for (i, (x, y)) in blocks.iter().enumerate() {
      result[i] = (x + dx, y + dy);
    }
    result
  }

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

#[derive(Clone, Debug, PartialEq)]
pub struct Board {
  grid: [[Cell; WIDTH]; HEIGHT + BUFFER_HEIGHT],
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

  pub fn is_valid_position(&self, x: i32, y: i32) -> bool {
    if x < 0 || x >= WIDTH as i32 || y < 0 {
      return false;
    }
    if y >= (HEIGHT + BUFFER_HEIGHT) as i32 {
      return true;
    }
    self.grid[y as usize][x as usize].is_empty()
  }

  pub fn check_collision(&self, positions: &[(i32, i32)]) -> bool {
    positions
      .iter()
      .any(|&(x, y)| !self.is_valid_position(x, y))
  }

  pub fn lock_cells(&mut self, positions: &[(i32, i32)], cell: Cell) {
    for &(x, y) in positions {
      self.set(x, y, cell);
    }
  }

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

    while write_row < HEIGHT + BUFFER_HEIGHT {
      self.grid[write_row] = [Cell::Empty; WIDTH];
      write_row += 1;
    }

    lines_cleared
  }

  pub fn is_topped_out(&self) -> bool {
    for row in HEIGHT..(HEIGHT + BUFFER_HEIGHT) {
      if self.grid[row].iter().any(|&cell| !cell.is_empty()) {
        return true;
      }
    }
    false
  }
}

#[derive(Clone, Debug, PartialEq)]
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

impl Game {
  pub fn new() -> Self {
    let mut game = Self {
      board: Board::new(),
      current_piece: None,
      next_piece: PieceType::T,
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
      self.score += 1;
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
    self.score += drop_distance * 2;

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

    let lines = self.board.clear_lines();
    if lines > 0 {
      self.lines_cleared += lines;
      self.score += self.calculate_line_score(lines);
      self.update_level();
    }

    if self.board.is_topped_out() {
      self.game_over = true;
      return MoveResult::GameOver;
    }

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
      4 => 800,
      _ => 0,
    };
    base * self.level
  }

  fn update_level(&mut self) {
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
      self.hold_piece = Some(current_type);
      self.current_piece = Some(Piece::new(held));
    } else {
      self.hold_piece = Some(current_type);
      self.spawn_piece();
    }

    self.can_hold = false;
    MoveResult::Success
  }

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

  pub fn get_drop_interval(&self) -> u32 {
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
      _ => 2,
    };
    (frames * 1000) / 60
  }

  pub fn get_board_cells(&self) -> Vec<(i32, i32, u8)> {
    let mut cells = Vec::new();
    for y in 0..HEIGHT {
      for x in 0..WIDTH {
        let cell = self.board.get(x as i32, y as i32).unwrap_or(Cell::Empty);
        if !cell.is_empty() {
          cells.push((x as i32, y as i32, cell as u8));
        }
      }
    }
    cells
  }
}
