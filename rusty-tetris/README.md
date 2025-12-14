# Rusty Tetris

A modern Tetris implementation with a Rust/WebAssembly core and TypeScript frontend, served by a Deno backend.

**[Play Live on Deno Deploy](https://rusty-tetris.mgavriliu.deno.net/)** | **[GitHub Pages](https://mgavriliu.github.io/tetris/)**

## Features

- **Rust/WASM Game Engine**: Core game logic written in Rust, compiled to WebAssembly for near-native performance in the browser
- **Modern Tetris Mechanics**: 7-bag randomizer, wall kicks (SRS), ghost piece, hold piece, hard/soft drop
- **NES-Style Speed Curve**: Progressive difficulty with level-based speed increases
- **High Score System**: Persistent global leaderboard using Deno KV
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
├── server/
│   └── main.ts               # Deno/Oak HTTP server with KV storage
├── pkg/                      # WASM build output
├── deno.json                 # Deno tasks & config
└── Cargo.toml                # Rust workspace
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

#### Server (`server/`)

Deno application using Oak framework:

- Serves static files (HTML, JS, WASM)
- REST API for high scores (`GET/POST /api/scores`)
- Scores persisted using Deno KV (works locally and on Deno Deploy)

## Deployment

### Deno Deploy (Recommended)

The easiest way to deploy is via [Deno Deploy](https://deno.com/deploy):

1. Go to [dash.deno.com](https://dash.deno.com)
2. Click "New Project"
3. Connect your GitHub repository
4. Set the entrypoint to `server/main.ts`
5. Deploy!

The app uses Deno KV for persistent high score storage, which is automatically provisioned on Deno Deploy.

### Manual Deployment

For other platforms, ensure you have:
- Deno runtime installed
- The `pkg/` and `frontend/dist/` directories (pre-built and included in repo)

```bash
deno run --allow-net --allow-read --allow-env --unstable-kv server/main.ts
```

## Local Development

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+) - only needed to modify game engine
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) - only needed to rebuild WASM
- [Deno](https://deno.land/) (1.40+)

### Quick Start

The repository includes pre-built WASM and frontend bundles, so you can run immediately:

```bash
deno task dev
```

Then open http://localhost:8000

### Full Build (if modifying Rust code)

```bash
# Install build tools (first time only)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack

# Build everything and start server
deno task start
```

### Manual Build Steps

```bash
# Build WASM module
cd crates/tetris-core && wasm-pack build --target web --out-dir ../../pkg

# Bundle frontend
deno run --allow-read --allow-write --allow-env npm:esbuild frontend/main.ts --bundle --format=esm --outdir=frontend/dist

# Start server
deno task dev
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

## API Endpoints

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
- Deno KV provides persistent storage that works both locally and on Deno Deploy

## License

MIT
