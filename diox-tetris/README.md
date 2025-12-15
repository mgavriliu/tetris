# Dioxus Tetris

A modern Tetris implementation written in **pure Rust** using the Dioxus framework, compiled to WebAssembly.

## Features

- **Pure Rust Frontend**: UI and game logic entirely in Rust using Dioxus reactive framework
- **No JavaScript**: Zero TypeScript/JavaScript code (only Dioxus-generated bindings)
- **Modern Tetris Mechanics**: 7-bag randomizer, wall kicks (SRS), ghost piece, hold piece, hard/soft drop
- **NES-Style Speed Curve**: Progressive difficulty with level-based speed increases
- **High Score System**: Global leaderboard via shared API
- **SVG Rendering**: Reactive SVG rendering via Dioxus RSX macros

## Architecture

```
diox-tetris/
├── src/
│   ├── main.rs           # Dioxus app, UI components, game loop
│   └── game.rs           # Core game logic (board, pieces, scoring)
├── assets/
│   └── style.css         # Styling
├── Cargo.toml            # Dependencies (dioxus, gloo-net, etc.)
├── Dioxus.toml           # Dioxus CLI configuration
└── target/dx/.../public/ # Build output (HTML, WASM, JS)
```

### Component Overview

#### Game Logic (`src/game.rs`)

Pure Rust implementation of Tetris mechanics:

- **`Game`**: Core game state including board, pieces, scoring, and level progression
- **`Board`**: 10x20 grid with collision detection and line clearing
- **`Piece`**: Seven tetromino types with SRS (Super Rotation System) wall kicks
- **`GameState`**: Enum for idle/playing/paused/game over states

#### Dioxus App (`src/main.rs`)

Reactive UI using Dioxus signals and components:

- **`App`**: Main component with game state signals and keyboard handling
- **`BoardCells`**, **`CurrentPiece`**, **`GhostPiece`**: SVG rendering components
- **`PreviewPiece`**: Next/hold piece preview
- **`NameInputOverlay`**: High score name entry
- Game loop via `use_effect` + `spawn` with async timers

## Building

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+)
- [Dioxus CLI](https://dioxuslabs.com/): `cargo install dioxus-cli`

### Development

```bash
dx serve
# Open http://localhost:8080
```

### Production Build

```bash
dx build --release
# Output in target/dx/diox-tetris/release/web/public/
```

## Controls

| Key               | Action                   |
| ----------------- | ------------------------ |
| `←` `→`           | Move left/right          |
| `↓`               | Soft drop                |
| `Space`           | Hard drop                |
| `↑` / `X`         | Rotate clockwise         |
| `Z`               | Rotate counter-clockwise |
| `C`               | Hold piece               |
| `P` / `Esc`       | Pause                    |
| `R`               | Restart                  |
| `Enter` / `Space` | Start game               |

## Scoring

| Action           | Points      |
| ---------------- | ----------- |
| Soft drop        | 1 per cell  |
| Hard drop        | 2 per cell  |
| Single line      | 100 × level |
| Double           | 300 × level |
| Triple           | 500 × level |
| Tetris (4 lines) | 800 × level |

Level increases every 10 lines cleared.

## Technical Details

### Dependencies

- **dioxus** (0.7): Reactive UI framework with web target
- **gloo-net**: HTTP client for high scores API
- **gloo-timers**: Async timers for game loop
- **rand**: Random number generation (with `js` feature for WASM)
- **serde**: JSON serialization for API

### Build Output

- WASM module: ~32MB (debug) / ~2MB (release with LTO)
- Generated JS bindings for WASM initialization

### Key Differences from Other Implementations

| Aspect            | diox-tetris      | rusty-tetris    | webgl-tetris |
| ----------------- | ---------------- | --------------- | ------------ |
| **Frontend Lang** | Rust (Dioxus)    | TypeScript      | TypeScript   |
| **Rendering**     | Dioxus RSX → SVG | SVG via DOM API | WebGL/GLSL   |
| **Game Loop**     | Rust async       | JavaScript RAF  | Rust RAF     |
| **State Mgmt**    | Dioxus Signals   | Manual          | Manual       |

## License

MIT
