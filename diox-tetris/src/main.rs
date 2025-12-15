#![allow(non_snake_case)]

mod game;

use dioxus::prelude::*;
use game::{Game, GameState, MoveResult};
use gloo_net::http::Request;
use gloo_timers::future::TimeoutFuture;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

const CELL_SIZE: u32 = 30;
const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
// Shared API endpoint for all Tetris games
const API_BASE: &str = "https://tetris-api.deno.dev/api";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Score {
    pub name: String,
    pub score: u32,
    pub level: u32,
    pub lines: u32,
    #[serde(default)]
    pub timestamp: u64,
}

async fn fetch_high_scores() -> Vec<Score> {
    match Request::get(&format!("{}/scores", API_BASE)).send().await {
        Ok(resp) => resp.json().await.unwrap_or_default(),
        Err(_) => vec![],
    }
}

async fn submit_score(score: &Score) -> bool {
    match Request::post(&format!("{}/scores", API_BASE))
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(score).unwrap())
        .unwrap()
        .send()
        .await
    {
        Ok(resp) => resp.ok(),
        Err(_) => false,
    }
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut game = use_signal(|| Game::new());
    let mut state = use_signal(|| GameState::Idle);
    let mut high_scores = use_signal(Vec::<Score>::new);
    let player_name = use_signal(String::new);
    let mut show_name_input = use_signal(|| false);
    let mut score_submitted = use_signal(|| false);
    let mut game_loop_started = use_signal(|| false);
    let mut scores_fetched = use_signal(|| false);

    // Fetch high scores on mount (only once)
    use_effect(move || {
        if *scores_fetched.read() {
            return;
        }
        scores_fetched.set(true);
        spawn(async move {
            let scores = fetch_high_scores().await;
            high_scores.set(scores);
        });
    });

    // Game loop - spawn only once
    use_effect(move || {
        if *game_loop_started.read() {
            return;
        }
        game_loop_started.set(true);

        spawn(async move {
            loop {
                let current_state = *state.read();
                if current_state == GameState::Playing {
                    let interval = game.read().get_drop_interval();
                    TimeoutFuture::new(interval).await;

                    if *state.read() == GameState::Playing {
                        let result = game.write().tick();
                        if result == MoveResult::GameOver {
                            state.set(GameState::GameOver);
                            // Check if it's a high score
                            let final_score = game.read().score;
                            let scores = high_scores.read();
                            let is_high_score = scores.len() < 10
                                || scores.last().map(|s| final_score > s.score).unwrap_or(true);
                            if is_high_score && final_score > 0 {
                                show_name_input.set(true);
                                score_submitted.set(false);
                            }
                        }
                    }
                } else {
                    TimeoutFuture::new(100).await;
                }
            }
        });
    });

    // Keyboard handler
    let onkeydown = move |evt: KeyboardEvent| {
        let current_state = *state.read();
        let key = evt.key();

        // Don't handle keys when entering name
        if *show_name_input.read() {
            return;
        }

        // Start game
        if matches!(key, Key::Enter) || matches!(&key, Key::Character(c) if c == " ") {
            if current_state == GameState::Idle || current_state == GameState::GameOver {
                game.set(Game::new());
                state.set(GameState::Playing);
                score_submitted.set(false);
                return;
            }
        }

        // Hard drop (space only during play)
        if matches!(&key, Key::Character(c) if c == " ") && current_state == GameState::Playing {
            let result = game.write().hard_drop();
            if result == MoveResult::GameOver {
                state.set(GameState::GameOver);
            }
            return;
        }

        // Pause/unpause
        if matches!(key, Key::Escape) || matches!(&key, Key::Character(c) if c == "p" || c == "P") {
            if current_state == GameState::Playing {
                state.set(GameState::Paused);
            } else if current_state == GameState::Paused {
                state.set(GameState::Playing);
            }
            return;
        }

        // Restart
        if matches!(&key, Key::Character(c) if c == "r" || c == "R") {
            game.set(Game::new());
            state.set(GameState::Playing);
            score_submitted.set(false);
            return;
        }

        // Game controls (only when playing)
        if current_state != GameState::Playing {
            return;
        }

        match key {
            Key::ArrowLeft => { game.write().move_piece(-1, 0); }
            Key::ArrowRight => { game.write().move_piece(1, 0); }
            Key::ArrowDown => { game.write().soft_drop(); }
            Key::ArrowUp => { game.write().rotate(true); }
            Key::Character(ref c) if c == "x" || c == "X" => { game.write().rotate(true); }
            Key::Character(ref c) if c == "z" || c == "Z" => { game.write().rotate(false); }
            Key::Character(ref c) if c == "c" || c == "C" => { game.write().hold(); }
            _ => {}
        }
    };

    let current_state = *state.read();
    let game_data = game.read();
    let scores = high_scores.read();
    let showing_input = *show_name_input.read();
    let submitted = *score_submitted.read();

    rsx! {
        div {
            class: "container",
            tabindex: "0",
            autofocus: true,
            onkeydown: onkeydown,

            style { {include_str!("../assets/style.css")} }

            header {
                h1 { "DIOXUS TETRIS" }
                p { class: "subtitle", "Pure Rust + WebAssembly" }
            }

            div { class: "game-area",
                // Left panel
                div { class: "side-panel",
                    div { class: "panel-section",
                        div { class: "panel-label", "HOLD [C]" }
                        div { class: "preview-container",
                            PreviewPiece { piece_type: game_data.hold_piece }
                        }
                    }
                    div { class: "panel-section",
                        div { class: "panel-label", "HIGH SCORES" }
                        div { class: "scores-list",
                            if scores.is_empty() {
                                div { class: "no-scores", "-" }
                            } else {
                                for (i, score) in scores.iter().take(5).enumerate() {
                                    div { class: "score-row",
                                        span { class: "rank", "{i + 1}." }
                                        span { class: "name", "{score.name}" }
                                        span { class: "pts", "{score.score}" }
                                    }
                                }
                            }
                        }
                    }
                }

                // Game board
                div { class: "board-container",
                    svg {
                        width: "{BOARD_WIDTH as u32 * CELL_SIZE}",
                        height: "{BOARD_HEIGHT as u32 * CELL_SIZE}",
                        class: "tetris-board",

                        // Grid background
                        for y in 0..BOARD_HEIGHT {
                            for x in 0..BOARD_WIDTH {
                                rect {
                                    x: "{x as u32 * CELL_SIZE}",
                                    y: "{y as u32 * CELL_SIZE}",
                                    width: "{CELL_SIZE}",
                                    height: "{CELL_SIZE}",
                                    fill: "#1a1a2e",
                                    stroke: "#2a2a4a",
                                    stroke_width: "1",
                                }
                            }
                        }

                        // Board cells
                        BoardCells { game: Rc::new(game_data.clone()) }

                        // Ghost piece
                        GhostPiece { game: Rc::new(game_data.clone()) }

                        // Current piece
                        CurrentPiece { game: Rc::new(game_data.clone()) }

                        // Overlays
                        if current_state == GameState::Idle {
                            StartOverlay {}
                        }
                        if current_state == GameState::Paused {
                            PauseOverlay {}
                        }
                        if current_state == GameState::GameOver && !showing_input {
                            GameOverOverlay { score: game_data.score, submitted: submitted }
                        }
                    }

                    // Name input overlay (outside SVG)
                    if showing_input {
                        NameInputOverlay {
                            score: game_data.score,
                            level: game_data.level,
                            lines: game_data.lines_cleared,
                            player_name: player_name,
                            show_name_input: show_name_input,
                            score_submitted: score_submitted,
                            high_scores: high_scores,
                        }
                    }
                }

                // Right panel
                div { class: "side-panel",
                    div { class: "panel-section",
                        div { class: "panel-label", "NEXT" }
                        div { class: "preview-container",
                            PreviewPiece { piece_type: Some(game_data.next_piece) }
                        }
                    }
                    div { class: "panel-section",
                        div { class: "panel-label", "SCORE" }
                        div { class: "stat-value", "{game_data.score}" }
                    }
                    div { class: "panel-section",
                        div { class: "panel-label", "LEVEL" }
                        div { class: "stat-value", "{game_data.level}" }
                    }
                    div { class: "panel-section",
                        div { class: "panel-label", "LINES" }
                        div { class: "stat-value", "{game_data.lines_cleared}" }
                    }
                }
            }

            footer {
                p { "Built with Dioxus + Rust + WebAssembly" }
            }
        }
    }
}

#[component]
fn BoardCells(game: Rc<Game>) -> Element {
    let cells = game.get_board_cells();

    rsx! {
        for (x, y, cell) in cells {
            rect {
                x: "{x as u32 * CELL_SIZE + 1}",
                y: "{(BOARD_HEIGHT - 1 - y as usize) as u32 * CELL_SIZE + 1}",
                width: "{CELL_SIZE - 2}",
                height: "{CELL_SIZE - 2}",
                fill: "{cell_color(cell)}",
                rx: "3",
            }
        }
    }
}

#[component]
fn CurrentPiece(game: Rc<Game>) -> Element {
    let Some(piece) = &game.current_piece else {
        return rsx! {};
    };

    let blocks = piece.get_blocks();
    let color = piece_color(piece.piece_type);

    rsx! {
        for (x, y) in blocks {
            if y >= 0 && y < BOARD_HEIGHT as i32 {
                rect {
                    x: "{x as u32 * CELL_SIZE + 1}",
                    y: "{(BOARD_HEIGHT - 1 - y as usize) as u32 * CELL_SIZE + 1}",
                    width: "{CELL_SIZE - 2}",
                    height: "{CELL_SIZE - 2}",
                    fill: "{color}",
                    rx: "3",
                }
            }
        }
    }
}

#[component]
fn GhostPiece(game: Rc<Game>) -> Element {
    let Some(ghost_y) = game.get_ghost_y() else {
        return rsx! {};
    };

    let Some(piece) = &game.current_piece else {
        return rsx! {};
    };

    let blocks = piece.get_blocks();
    let dy = piece.y - ghost_y;
    let color = piece_color(piece.piece_type);

    rsx! {
        for (x, y) in blocks {
            {
                let gy = y - dy;
                if gy >= 0 && gy < BOARD_HEIGHT as i32 {
                    rsx! {
                        rect {
                            x: "{x as u32 * CELL_SIZE + 1}",
                            y: "{(BOARD_HEIGHT - 1 - gy as usize) as u32 * CELL_SIZE + 1}",
                            width: "{CELL_SIZE - 2}",
                            height: "{CELL_SIZE - 2}",
                            fill: "{color}",
                            opacity: "0.3",
                            rx: "3",
                        }
                    }
                } else {
                    rsx! {}
                }
            }
        }
    }
}

#[component]
fn PreviewPiece(piece_type: Option<game::PieceType>) -> Element {
    let Some(pt) = piece_type else {
        return rsx! {
            svg { width: "96", height: "48", class: "preview-svg" }
        };
    };

    let blocks = pt.preview_blocks();
    let color = piece_color(pt);
    let cell_size = 24;

    rsx! {
        svg {
            width: "96",
            height: "48",
            class: "preview-svg",

            for (x, y) in blocks {
                rect {
                    x: "{(x + 1) as u32 * cell_size + 1}",
                    y: "{(1 - y) as u32 * cell_size + 1}",
                    width: "{cell_size - 2}",
                    height: "{cell_size - 2}",
                    fill: "{color}",
                    rx: "2",
                }
            }
        }
    }
}

#[component]
fn StartOverlay() -> Element {
    rsx! {
        g {
            rect {
                width: "{BOARD_WIDTH as u32 * CELL_SIZE}",
                height: "{BOARD_HEIGHT as u32 * CELL_SIZE}",
                fill: "rgba(0,0,0,0.8)",
            }
            text {
                x: "{BOARD_WIDTH as u32 * CELL_SIZE / 2}",
                y: "120",
                text_anchor: "middle",
                fill: "#00f5ff",
                font_size: "28",
                font_family: "monospace",
                font_weight: "bold",
                "TETRIS"
            }
            text {
                x: "{BOARD_WIDTH as u32 * CELL_SIZE / 2}",
                y: "180",
                text_anchor: "middle",
                fill: "#fff",
                font_size: "12",
                font_family: "monospace",
                "CONTROLS"
            }
            text {
                x: "{BOARD_WIDTH as u32 * CELL_SIZE / 2}",
                y: "210",
                text_anchor: "middle",
                fill: "#888",
                font_size: "11",
                font_family: "monospace",
                "Arrow Keys - Move"
            }
            text {
                x: "{BOARD_WIDTH as u32 * CELL_SIZE / 2}",
                y: "230",
                text_anchor: "middle",
                fill: "#888",
                font_size: "11",
                font_family: "monospace",
                "Space - Hard Drop"
            }
            text {
                x: "{BOARD_WIDTH as u32 * CELL_SIZE / 2}",
                y: "250",
                text_anchor: "middle",
                fill: "#888",
                font_size: "11",
                font_family: "monospace",
                "Z/X - Rotate"
            }
            text {
                x: "{BOARD_WIDTH as u32 * CELL_SIZE / 2}",
                y: "270",
                text_anchor: "middle",
                fill: "#888",
                font_size: "11",
                font_family: "monospace",
                "C - Hold Piece"
            }
            text {
                x: "{BOARD_WIDTH as u32 * CELL_SIZE / 2}",
                y: "320",
                text_anchor: "middle",
                fill: "#00f5ff",
                font_size: "14",
                font_family: "monospace",
                "Press SPACE to start"
            }
        }
    }
}

#[component]
fn PauseOverlay() -> Element {
    rsx! {
        g {
            rect {
                width: "{BOARD_WIDTH as u32 * CELL_SIZE}",
                height: "{BOARD_HEIGHT as u32 * CELL_SIZE}",
                fill: "rgba(0,0,0,0.7)",
            }
            text {
                x: "{BOARD_WIDTH as u32 * CELL_SIZE / 2}",
                y: "{BOARD_HEIGHT as u32 * CELL_SIZE / 2}",
                text_anchor: "middle",
                fill: "#fff",
                font_size: "24",
                font_family: "monospace",
                "PAUSED"
            }
            text {
                x: "{BOARD_WIDTH as u32 * CELL_SIZE / 2}",
                y: "{BOARD_HEIGHT as u32 * CELL_SIZE / 2 + 30}",
                text_anchor: "middle",
                fill: "#888",
                font_size: "14",
                font_family: "monospace",
                "Press P or ESC to resume"
            }
        }
    }
}

#[component]
fn NameInputOverlay(
    score: u32,
    level: u32,
    lines: u32,
    mut player_name: Signal<String>,
    mut show_name_input: Signal<bool>,
    mut score_submitted: Signal<bool>,
    mut high_scores: Signal<Vec<Score>>,
) -> Element {
    rsx! {
        div { class: "name-input-overlay",
            div { class: "name-input-box",
                h2 { "HIGH SCORE!" }
                p { "Score: {score}" }
                input {
                    r#type: "text",
                    placeholder: "Enter your name",
                    maxlength: "20",
                    value: "{player_name}",
                    oninput: move |evt| player_name.set(evt.value()),
                    onkeydown: move |evt| {
                        if evt.key() == Key::Enter {
                            let name = player_name.read().trim().to_string();
                            if !name.is_empty() {
                                let new_score = Score {
                                    name: name.clone(),
                                    score,
                                    level,
                                    lines,
                                    timestamp: 0,
                                };
                                spawn(async move {
                                    submit_score(&new_score).await;
                                    let scores = fetch_high_scores().await;
                                    high_scores.set(scores);
                                });
                                show_name_input.set(false);
                                score_submitted.set(true);
                                player_name.set(String::new());
                            }
                        } else if evt.key() == Key::Escape {
                            show_name_input.set(false);
                        }
                    }
                }
                p { class: "hint", "Press ENTER to submit, ESC to skip" }
            }
        }
    }
}

#[component]
fn GameOverOverlay(score: u32, submitted: bool) -> Element {
    rsx! {
        g {
            rect {
                width: "{BOARD_WIDTH as u32 * CELL_SIZE}",
                height: "{BOARD_HEIGHT as u32 * CELL_SIZE}",
                fill: "rgba(0,0,0,0.7)",
            }
            text {
                x: "{BOARD_WIDTH as u32 * CELL_SIZE / 2}",
                y: "{BOARD_HEIGHT as u32 * CELL_SIZE / 2 - 20}",
                text_anchor: "middle",
                fill: "#ff6b6b",
                font_size: "24",
                font_family: "monospace",
                "GAME OVER"
            }
            text {
                x: "{BOARD_WIDTH as u32 * CELL_SIZE / 2}",
                y: "{BOARD_HEIGHT as u32 * CELL_SIZE / 2 + 10}",
                text_anchor: "middle",
                fill: "#fff",
                font_size: "16",
                font_family: "monospace",
                "Score: {score}"
            }
            if submitted {
                text {
                    x: "{BOARD_WIDTH as u32 * CELL_SIZE / 2}",
                    y: "{BOARD_HEIGHT as u32 * CELL_SIZE / 2 + 35}",
                    text_anchor: "middle",
                    fill: "#00f5ff",
                    font_size: "12",
                    font_family: "monospace",
                    "Score submitted!"
                }
            }
            text {
                x: "{BOARD_WIDTH as u32 * CELL_SIZE / 2}",
                y: "{BOARD_HEIGHT as u32 * CELL_SIZE / 2 + 60}",
                text_anchor: "middle",
                fill: "#888",
                font_size: "14",
                font_family: "monospace",
                "Press R to restart"
            }
        }
    }
}

fn cell_color(cell: u8) -> &'static str {
    match cell {
        0 => "#1a1a2e",
        1 => "#00f5ff",
        2 => "#ffd700",
        3 => "#9d4edd",
        4 => "#00ff7f",
        5 => "#ff6b6b",
        6 => "#4169e1",
        7 => "#ff8c00",
        _ => "#1a1a2e",
    }
}

fn piece_color(piece_type: game::PieceType) -> &'static str {
    match piece_type {
        game::PieceType::I => "#00f5ff",
        game::PieceType::O => "#ffd700",
        game::PieceType::T => "#9d4edd",
        game::PieceType::S => "#00ff7f",
        game::PieceType::Z => "#ff6b6b",
        game::PieceType::J => "#4169e1",
        game::PieceType::L => "#ff8c00",
    }
}
