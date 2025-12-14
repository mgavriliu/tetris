use serde::{Deserialize, Serialize};

/// Input actions
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Action {
    MoveLeft,
    MoveRight,
    SoftDrop,
    HardDrop,
    RotateCW,
    RotateCCW,
    Hold,
    Pause,
    Start,
    Restart,
}

/// DAS/ARR input handler
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputState {
    // DAS = Delayed Auto Shift (ms before repeat starts)
    das_delay: f64,
    // ARR = Auto Repeat Rate (ms between repeats)
    arr_rate: f64,

    // Current key states
    left_held: bool,
    right_held: bool,
    down_held: bool,

    // DAS timers (time held in ms)
    left_time: f64,
    right_time: f64,
    down_time: f64,

    // Last ARR fire time
    left_arr_time: f64,
    right_arr_time: f64,
    down_arr_time: f64,
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

impl InputState {
    pub fn new() -> Self {
        Self {
            das_delay: 170.0,
            arr_rate: 50.0,
            left_held: false,
            right_held: false,
            down_held: false,
            left_time: 0.0,
            right_time: 0.0,
            down_time: 0.0,
            left_arr_time: 0.0,
            right_arr_time: 0.0,
            down_arr_time: 0.0,
        }
    }

    pub fn key_down(&mut self, key: u8) -> Option<Action> {
        match key {
            0 => { // Left
                if !self.left_held {
                    self.left_held = true;
                    self.left_time = 0.0;
                    self.left_arr_time = 0.0;
                    return Some(Action::MoveLeft);
                }
            }
            1 => { // Right
                if !self.right_held {
                    self.right_held = true;
                    self.right_time = 0.0;
                    self.right_arr_time = 0.0;
                    return Some(Action::MoveRight);
                }
            }
            2 => { // Down
                if !self.down_held {
                    self.down_held = true;
                    self.down_time = 0.0;
                    self.down_arr_time = 0.0;
                    return Some(Action::SoftDrop);
                }
            }
            3 => return Some(Action::HardDrop),
            4 => return Some(Action::RotateCW),
            5 => return Some(Action::RotateCCW),
            6 => return Some(Action::Hold),
            7 => return Some(Action::Pause),
            8 => return Some(Action::Start),
            9 => return Some(Action::Restart),
            _ => {}
        }
        None
    }

    pub fn key_up(&mut self, key: u8) {
        match key {
            0 => self.left_held = false,
            1 => self.right_held = false,
            2 => self.down_held = false,
            _ => {}
        }
    }

    /// Update DAS/ARR timers, returns actions to execute
    pub fn update(&mut self, delta_ms: f64) -> Vec<Action> {
        let mut actions = Vec::new();

        // Left DAS/ARR
        if self.left_held {
            self.left_time += delta_ms;
            if self.left_time >= self.das_delay {
                self.left_arr_time += delta_ms;
                if self.left_arr_time >= self.arr_rate {
                    self.left_arr_time -= self.arr_rate;
                    actions.push(Action::MoveLeft);
                }
            }
        }

        // Right DAS/ARR
        if self.right_held {
            self.right_time += delta_ms;
            if self.right_time >= self.das_delay {
                self.right_arr_time += delta_ms;
                if self.right_arr_time >= self.arr_rate {
                    self.right_arr_time -= self.arr_rate;
                    actions.push(Action::MoveRight);
                }
            }
        }

        // Down DAS/ARR (faster)
        if self.down_held {
            self.down_time += delta_ms;
            if self.down_time >= self.das_delay * 0.5 {
                self.down_arr_time += delta_ms;
                if self.down_arr_time >= self.arr_rate * 0.5 {
                    self.down_arr_time -= self.arr_rate * 0.5;
                    actions.push(Action::SoftDrop);
                }
            }
        }

        actions
    }

    pub fn reset(&mut self) {
        self.left_held = false;
        self.right_held = false;
        self.down_held = false;
        self.left_time = 0.0;
        self.right_time = 0.0;
        self.down_time = 0.0;
    }
}
