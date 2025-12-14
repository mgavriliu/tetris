use crate::game::{Game, MoveResult};
use crate::input::{Action, InputState};
use crate::render::RenderState;
use serde::{Deserialize, Serialize};

/// Game states
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum GameState {
    Idle,
    Playing,
    Paused,
    GameOver,
}

/// Main game controller - combines state machine, input, and game logic
#[derive(Clone, Debug)]
pub struct GameController {
    pub state: GameState,
    pub game: Option<Game>,
    pub input: InputState,
    drop_accumulator: f64,
}

impl Default for GameController {
    fn default() -> Self {
        Self::new()
    }
}

impl GameController {
    pub fn new() -> Self {
        Self {
            state: GameState::Idle,
            game: None,
            input: InputState::new(),
            drop_accumulator: 0.0,
        }
    }

    /// Start a new game
    pub fn start(&mut self) {
        self.game = Some(Game::new());
        self.state = GameState::Playing;
        self.input.reset();
        self.drop_accumulator = 0.0;
    }

    /// Pause the game
    pub fn pause(&mut self) {
        if self.state == GameState::Playing {
            self.state = GameState::Paused;
            self.input.reset();
        }
    }

    /// Resume from pause
    pub fn resume(&mut self) {
        if self.state == GameState::Paused {
            self.state = GameState::Playing;
        }
    }

    /// Restart (from game over or pause)
    pub fn restart(&mut self) {
        self.start();
    }

    /// Handle key down event
    /// key: 0=left, 1=right, 2=down, 3=space, 4=up/x, 5=z, 6=c/shift, 7=p/esc, 8=enter/space(start), 9=r
    pub fn key_down(&mut self, key: u8) {
        if let Some(action) = self.input.key_down(key) {
            self.handle_action(action);
        }
    }

    /// Handle key up event
    pub fn key_up(&mut self, key: u8) {
        self.input.key_up(key);
    }

    /// Handle an action
    fn handle_action(&mut self, action: Action) {
        // Handle state transitions first (without borrowing game)
        match (self.state, action) {
            (GameState::Idle, Action::Start) => {
                self.start();
                return;
            }
            (GameState::Playing, Action::Pause) => {
                self.pause();
                return;
            }
            (GameState::Playing, Action::Restart)
            | (GameState::Paused, Action::Restart)
            | (GameState::GameOver, Action::Restart)
            | (GameState::GameOver, Action::Start) => {
                self.restart();
                return;
            }
            (GameState::Paused, Action::Pause) => {
                self.resume();
                return;
            }
            _ => {}
        }

        // Handle game actions
        if self.state == GameState::Playing {
            if let Some(game) = &mut self.game {
                match action {
                    Action::MoveLeft => {
                        game.move_piece(-1, 0);
                    }
                    Action::MoveRight => {
                        game.move_piece(1, 0);
                    }
                    Action::SoftDrop => {
                        game.soft_drop();
                    }
                    Action::HardDrop => {
                        game.hard_drop();
                    }
                    Action::RotateCW => {
                        game.rotate(true);
                    }
                    Action::RotateCCW => {
                        game.rotate(false);
                    }
                    Action::Hold => {
                        game.hold();
                    }
                    _ => {}
                }
                if game.game_over {
                    self.state = GameState::GameOver;
                }
            }
        }
    }

    /// Update game state (call every frame)
    /// delta_ms: time since last frame in milliseconds
    /// Returns true if render state changed
    pub fn update(&mut self, delta_ms: f64) -> bool {
        if self.state != GameState::Playing {
            return false;
        }

        if self.game.is_none() {
            return false;
        }

        // Process DAS/ARR input (collect actions first to avoid borrow issues)
        let actions: Vec<Action> = self.input.update(delta_ms);
        for action in actions {
            self.handle_action(action);
        }

        // Handle gravity with acceleration
        let (base_interval, acceleration) = {
            let game = self.game.as_ref().unwrap();
            (
                game.get_drop_interval() as f64,
                game.get_height_acceleration() as f64,
            )
        };
        let effective_interval = base_interval / acceleration;

        self.drop_accumulator += delta_ms;
        if self.drop_accumulator >= effective_interval {
            self.drop_accumulator = 0.0;
            if let Some(game) = &mut self.game {
                let result = game.tick();
                if result == MoveResult::GameOver {
                    self.state = GameState::GameOver;
                }
            }
        }

        true
    }

    /// Get current render state
    pub fn get_render_state(&self) -> RenderState {
        match &self.game {
            Some(game) => RenderState::from_game(game),
            None => RenderState::default(),
        }
    }

    /// Get current game state as u8
    pub fn get_state(&self) -> u8 {
        match self.state {
            GameState::Idle => 0,
            GameState::Playing => 1,
            GameState::Paused => 2,
            GameState::GameOver => 3,
        }
    }

    /// Get score (for high score check)
    pub fn get_score(&self) -> u32 {
        self.game.as_ref().map(|g| g.score).unwrap_or(0)
    }

    /// Get level
    pub fn get_level(&self) -> u32 {
        self.game.as_ref().map(|g| g.level).unwrap_or(1)
    }

    /// Get lines
    pub fn get_lines(&self) -> u32 {
        self.game.as_ref().map(|g| g.lines_cleared).unwrap_or(0)
    }

    /// Called when window loses focus
    pub fn on_blur(&mut self) {
        if self.state == GameState::Playing {
            self.pause();
        }
    }
}
