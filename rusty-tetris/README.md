# Rusty Tetris

A modern Tetris implementation with a Rust/WebAssembly core and TypeScript frontend.

## Features

- **Rust/WASM Game Engine**: Core game logic written in Rust, compiled to WebAssembly
- **Modern Tetris Mechanics**: 7-bag randomizer, wall kicks (SRS), ghost piece, hold piece, hard/soft drop
- **NES-Style Speed Curve**: Progressive difficulty with level-based speed increases
- **High Score System**: Global leaderboard via shared API
- **SVG Rendering**: Smooth, scalable graphics using SVG elements
- **Responsive Controls**: Keyboard input with DAS (Delayed Auto Shift) support

## Architecture

```
rusty-tetris/
├── crates/
│   └── tetris-core/          # Rust WASM game engine
│       ├── src/
│       │   ├── lib.rs        # WASM bindings & public API
│       │   ├── game.rs       # Core game state & logic
│       │   ├── board.rs      # 10x20 playfield management
│       │   ├── piece.rs      # Tetromino definitions & SRS rotation
│       │   ├── controller.rs # State machine (idle/playing/paused/gameover)
│       │   ├── input.rs      # Input handling with DAS/ARR
│       │   └── render.rs     # Render state extraction
│       └── Cargo.toml
├── frontend/
│   ├── index.html            # Game UI with embedded styles
│   ├── main.ts               # Game loop & SVG rendering
│   ├── api.ts                # High score API client
│   └── dist/                 # Bundled output
├── pkg/                      # WASM build output
└── deno.json                 # Build tasks
```

### Component Overview

#### Rust Core (`crates/tetris-core`)

The game engine is written in pure Rust and exposes a minimal API via `wasm-bindgen`:

- **`Tetris`**: Main WASM interface wrapping the game controller
- **`Game`**: Core game state including board, pieces, scoring, and level progression
- **`Board`**: 10x20 grid with collision detection and line clearing
- **`Piece`**: Seven tetromino types with SRS (Super Rotation System) wall kicks
- **`GameController`**: Finite state machine managing game states and timing
- **`InputHandler`**: Keyboard input with configurable DAS (167ms) and ARR (33ms)

Data flows from Rust to JavaScript as flat `Uint8Array` buffers for efficient rendering:
- Each cell is encoded as 4 bytes: `[x, y, color, opacity]`

#### Frontend (`frontend/`)

TypeScript application bundled with esbuild:

- Initializes WASM module and creates game instance
- Runs 60fps game loop via `requestAnimationFrame`
- Renders game state to SVG elements
- Handles keyboard input and maps to Rust key codes
- Manages UI overlays (start screen, pause, game over)

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

### Submit Score Request
```json
{
  "name": "Player",
  "score": 12500,
  "level": 5,
  "lines": 42
}
```

## Technical Details

### Rust Crates

- **tetris-core**: Main game library compiled to WASM
  - Dependencies: `wasm-bindgen`, `js-sys`, `web-sys`, `rand`, `serde`
  - Optimized for size (`opt-level = "s"`, LTO enabled)
  - Final WASM size: ~60KB

### Build Outputs

- `pkg/`: Contains `tetris_core.js`, `tetris_core_bg.wasm`, and TypeScript definitions
- `frontend/dist/`: Bundled JavaScript (~25KB)

### Implementation Notes

- The WASM module uses `getrandom` with the `js` feature for random number generation
- SRS (Super Rotation System) wall kicks are implemented for all pieces except O
- Ghost piece rendering shows where the current piece will land
- The 7-bag randomizer ensures fair piece distribution

## License

MIT
