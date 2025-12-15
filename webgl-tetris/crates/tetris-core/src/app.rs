use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use crate::controller::{GameController, GameState};
use crate::webgl::{PreviewRenderer, WebGlRenderer};

const CELL_SIZE: f32 = 30.0;
const PREVIEW_CELL_SIZE: f32 = CELL_SIZE * 0.8;
const BOARD_WIDTH: u32 = 10;
const BOARD_HEIGHT: u32 = 20;

#[wasm_bindgen]
pub struct TetrisApp {
    inner: Rc<RefCell<TetrisAppInner>>,
}

struct TetrisAppInner {
    controller: GameController,
    board_renderer: WebGlRenderer,
    next_renderer: PreviewRenderer,
    hold_renderer: PreviewRenderer,
    last_time: f64,
    last_state: GameState,
    on_state_change: Option<js_sys::Function>,
    on_score_update: Option<js_sys::Function>,
    animation_id: Option<i32>,
}

#[wasm_bindgen]
impl TetrisApp {
    #[wasm_bindgen(constructor)]
    pub fn new(
        board_canvas: HtmlCanvasElement,
        next_canvas: HtmlCanvasElement,
        hold_canvas: HtmlCanvasElement,
    ) -> Result<TetrisApp, JsValue> {
        let mut board_renderer =
            WebGlRenderer::new(&board_canvas, BOARD_WIDTH, BOARD_HEIGHT, CELL_SIZE)?;
        board_renderer.set_grid_offset(3.0, 1.0);

        let next_renderer = PreviewRenderer::new(&next_canvas, PREVIEW_CELL_SIZE)?;
        let hold_renderer = PreviewRenderer::new(&hold_canvas, PREVIEW_CELL_SIZE)?;

        let inner = Rc::new(RefCell::new(TetrisAppInner {
            controller: GameController::new(),
            board_renderer,
            next_renderer,
            hold_renderer,
            last_time: 0.0,
            last_state: GameState::Idle,
            on_state_change: None,
            on_score_update: None,
            animation_id: None,
        }));

        Ok(TetrisApp { inner })
    }

    pub fn set_on_state_change(&mut self, callback: js_sys::Function) {
        self.inner.borrow_mut().on_state_change = Some(callback);
    }

    pub fn set_on_score_update(&mut self, callback: js_sys::Function) {
        self.inner.borrow_mut().on_score_update = Some(callback);
    }

    pub fn start_game_loop(&self) -> Result<(), JsValue> {
        let inner = self.inner.clone();

        // Initial render
        {
            let app = inner.borrow();
            app.board_renderer.clear();
            app.board_renderer.render_grid();
        }

        // Set up the game loop
        let f: Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>> = Rc::new(RefCell::new(None));
        let g = f.clone();
        let inner_clone = inner.clone();

        *g.borrow_mut() = Some(Closure::new(move |timestamp: f64| {
            let mut app = inner_clone.borrow_mut();

            // Calculate delta time
            let delta_ms = if app.last_time > 0.0 {
                timestamp - app.last_time
            } else {
                0.0
            };
            app.last_time = timestamp;

            // Update game
            app.controller.update(delta_ms);

            // Check for state changes and collect callback data
            let current_state = app.controller.state;
            let last_state = app.last_state;
            let state_changed = current_state != last_state;
            let state_callback = if state_changed {
                app.last_state = current_state;
                app.on_state_change.clone()
            } else {
                None
            };

            // Collect score callback data if playing
            let score_callback = if current_state == GameState::Playing {
                let (score, level, lines) = app.get_stats();
                app.on_score_update.clone().map(|cb| (cb, score, level, lines))
            } else {
                None
            };

            // Render if playing
            if current_state == GameState::Playing {
                app.render();
            }

            // Release borrow before calling callbacks (to avoid RefCell panic)
            drop(app);

            // Now call callbacks safely
            if let Some(callback) = state_callback {
                let _ = callback.call2(
                    &JsValue::NULL,
                    &JsValue::from(state_to_u8(current_state)),
                    &JsValue::from(state_to_u8(last_state)),
                );
            }

            if let Some((callback, score, level, lines)) = score_callback {
                let _ = callback.call3(
                    &JsValue::NULL,
                    &JsValue::from(score),
                    &JsValue::from(level),
                    &JsValue::from(lines),
                );
            }

            // Request next frame
            let window = web_sys::window().unwrap();
            let _ = window.request_animation_frame(
                f.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
            );
        }));

        // Start the loop
        let window = web_sys::window().unwrap();
        let id = window.request_animation_frame(
            g.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
        )?;

        self.inner.borrow_mut().animation_id = Some(id);

        // Keep the closure alive by leaking it (it runs for the lifetime of the app)
        std::mem::forget(g);

        Ok(())
    }

    pub fn key_down(&self, code: &str) {
        if let Some(key) = key_code_to_u8(code) {
            self.inner.borrow_mut().controller.key_down(key);
        }
    }

    pub fn key_up(&self, code: &str) {
        if let Some(key) = key_code_to_u8(code) {
            self.inner.borrow_mut().controller.key_up(key);
        }
    }

    pub fn on_blur(&self) {
        self.inner.borrow_mut().controller.on_blur();
    }

    pub fn get_state(&self) -> u8 {
        state_to_u8(self.inner.borrow().controller.state)
    }

    pub fn get_score(&self) -> u32 {
        self.inner
            .borrow()
            .controller
            .game
            .as_ref()
            .map(|g| g.score)
            .unwrap_or(0)
    }

    pub fn get_level(&self) -> u32 {
        self.inner
            .borrow()
            .controller
            .game
            .as_ref()
            .map(|g| g.level)
            .unwrap_or(1)
    }

    pub fn get_lines(&self) -> u32 {
        self.inner
            .borrow()
            .controller
            .game
            .as_ref()
            .map(|g| g.lines_cleared)
            .unwrap_or(0)
    }

    pub fn render_initial(&self) {
        let app = self.inner.borrow();
        app.board_renderer.clear();
        app.board_renderer.render_grid();
    }
}

impl TetrisAppInner {
    fn get_stats(&self) -> (u32, u32, u32) {
        self.controller
            .game
            .as_ref()
            .map(|g| (g.score, g.level, g.lines_cleared))
            .unwrap_or((0, 1, 0))
    }

    fn render(&self) {
        // Clear and render grid
        self.board_renderer.clear();
        self.board_renderer.render_grid();

        // Get render state
        let render_state = self.controller.get_render_state();
        let arrays = render_state.to_flat_arrays();

        // Render board cells
        self.board_renderer.render_cells(&arrays.board, 3.0);

        // Render ghost
        self.board_renderer.render_cells(&arrays.ghost, 3.0);

        // Render piece
        self.board_renderer.render_cells(&arrays.piece, 3.0);

        // Render previews
        self.next_renderer.render_cells(&arrays.next, 2.0);
        self.hold_renderer.render_cells(&arrays.hold, 2.0);
    }
}

fn key_code_to_u8(code: &str) -> Option<u8> {
    match code {
        "ArrowLeft" => Some(0),
        "ArrowRight" => Some(1),
        "ArrowDown" => Some(2),
        "Space" => Some(3),
        "ArrowUp" | "KeyX" => Some(4),
        "KeyZ" | "ControlLeft" | "ControlRight" => Some(5),
        "KeyC" | "ShiftLeft" | "ShiftRight" => Some(6),
        "KeyP" | "Escape" => Some(7),
        "Enter" => Some(8),
        "KeyR" => Some(9),
        _ => None,
    }
}

fn state_to_u8(state: GameState) -> u8 {
    match state {
        GameState::Idle => 0,
        GameState::Playing => 1,
        GameState::Paused => 2,
        GameState::GameOver => 3,
    }
}
