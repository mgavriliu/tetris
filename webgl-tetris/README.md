# WebGL Tetris

A modern Tetris implementation with a Rust/WebAssembly core featuring **WebGL rendering** with custom GLSL shaders.

**[Play on GitHub Pages](https://mgavriliu.github.io/tetris/docs/)**

## Features

- **Rust/WASM Game Engine**: Core game logic and WebGL rendering written in Rust
- **WebGL Rendering**: Hardware-accelerated graphics with custom GLSL shaders
- **Modern Tetris Mechanics**: 7-bag randomizer, wall kicks (SRS), ghost piece, hold piece, hard/soft drop
- **NES-Style Speed Curve**: Progressive difficulty with level-based speed increases
- **High Score System**: Global leaderboard via shared API
- **Responsive Controls**: Keyboard input with DAS (Delayed Auto Shift) support

## Architecture

```
webgl-tetris/
├── crates/
│   └── tetris-core/          # Rust WASM game engine
│       ├── src/
│       │   ├── lib.rs        # WASM bindings & public API
│       │   ├── app.rs        # Main app with game loop & callbacks
│       │   ├── webgl.rs      # WebGL renderer with GLSL shaders
│       │   ├── game.rs       # Core game state & logic
│       │   ├── board.rs      # 10x20 playfield management
│       │   ├── piece.rs      # Tetromino definitions & SRS rotation
│       │   ├── controller.rs # State machine (idle/playing/paused/gameover)
│       │   ├── input.rs      # Input handling with DAS/ARR
│       │   └── render.rs     # Render state extraction
│       └── Cargo.toml
├── frontend/
│   ├── index.html            # Game UI with embedded styles
│   ├── main.ts               # DOM setup & overlay management
│   ├── api.ts                # High score API client
│   └── dist/                 # Bundled output
├── pkg/                      # WASM build output
└── deno.json                 # Build tasks
```

### Component Overview

#### Rust Core (`crates/tetris-core`)

The game engine and renderer are written in Rust and compiled to WebAssembly:

- **`TetrisApp`**: Main WASM interface managing game loop, rendering, and input
- **`WebGlRenderer`**: Custom WebGL renderer with GLSL shaders for cells and grid
- **`Game`**: Core game state including board, pieces, scoring, and level progression
- **`Board`**: 10x20 grid with collision detection and line clearing
- **`Piece`**: Seven tetromino types with SRS (Super Rotation System) wall kicks
- **`GameController`**: Finite state machine managing game states and timing
- **`InputHandler`**: Keyboard input with configurable DAS (167ms) and ARR (33ms)

The game loop runs entirely in Rust via `requestAnimationFrame`, with callbacks to JavaScript for state changes and score updates.

#### Frontend (`frontend/`)

Minimal TypeScript (~200 lines) handling only:

- DOM setup (canvas elements, panels)
- Overlay management (start screen, pause, game over)
- High scores API calls
- Input event forwarding to Rust

## Local Development

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+) - only needed to modify game engine
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) - only needed to rebuild WASM

### Quick Start

The repository includes pre-built WASM and frontend bundles:

```bash
# Serve static files
python3 -m http.server 8080 -d frontend
# Open http://localhost:8080
```

### Full Build (if modifying Rust code)

```bash
# Build WASM module
deno task build:wasm

# Bundle frontend
deno task build:frontend

# Or build both
deno task build
```

## Controls

| Key | Action |
|-----|--------|
| `←` `→` | Move left/right |
| `↓` | Soft drop |
| `Space` | Hard drop |
| `↑` / `X` | Rotate clockwise |
| `Z` / `Ctrl` | Rotate counter-clockwise |
| `C` / `Shift` | Hold piece |
| `P` / `Esc` | Pause |
| `R` | Restart (when paused/game over) |
| `Enter` / `Space` | Start game |

## Scoring

| Action | Points |
|--------|--------|
| Soft drop | 1 per cell |
| Hard drop | 2 per cell |
| Single line | 100 × level |
| Double | 300 × level |
| Triple | 500 × level |
| Tetris (4 lines) | 800 × level |

Level increases every 10 lines cleared.

## API

High scores are managed by a shared API server. See the [root README](../README.md) for deployment instructions.

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/api/scores` | GET | Get top 10 high scores |
| `/api/scores` | POST | Submit a new score |

## Technical Details

### WebGL Rendering

The renderer uses two shader programs:

1. **Cell Shader**: Renders filled tetromino cells with per-cell colors
2. **Grid Shader**: Renders the background grid lines

All rendering state is computed in Rust and passed to WebGL via typed arrays.

### Build Outputs

- `pkg/`: Contains `tetris_core.js`, `tetris_core_bg.wasm` (~80KB)
- `frontend/dist/`: Bundled JavaScript (~9KB)

### Key Differences from rusty-tetris

| Aspect | webgl-tetris | rusty-tetris |
|--------|--------------|--------------|
| **Rendering** | WebGL/GLSL in Rust | SVG via TypeScript |
| **Game Loop** | Rust (`requestAnimationFrame`) | JavaScript |
| **Frontend Size** | ~9KB | ~25KB |
| **WASM Size** | ~80KB | ~60KB |

## License

MIT
