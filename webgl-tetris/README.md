# WebGL Tetris

A modern Tetris implementation with a Rust/WebAssembly core featuring **WebGL rendering**, served by a Deno backend.

**[Play Live on Deno Deploy](https://webgl-tetris--master.mgavriliu.deno.net/)** | **[GitHub Pages](https://mgavriliu.github.io/webgl-tetris/)**

## Features

- **Rust/WASM Game Engine**: Core game logic and WebGL rendering written in Rust
- **WebGL Rendering**: Hardware-accelerated graphics with custom GLSL shaders
- **Modern Tetris Mechanics**: 7-bag randomizer, wall kicks (SRS), ghost piece, hold piece, hard/soft drop
- **NES-Style Speed Curve**: Progressive difficulty with level-based speed increases
- **High Score System**: Persistent global leaderboard using Deno KV
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
├── server/
│   └── main.ts               # Deno HTTP server with KV storage
├── pkg/                      # WASM build output
├── deno.json                 # Deno tasks & config
└── Cargo.toml                # Rust workspace
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

#### Server (`server/`)

Deno application:

- Serves static files (HTML, JS, WASM)
- REST API for high scores (`GET/POST /api/scores`)
- Scores persisted using Deno KV

## Deployment

### Deno Deploy (Recommended)

1. Go to [dash.deno.com](https://dash.deno.com)
2. Click "New Project"
3. Connect your GitHub repository
4. Set the entrypoint to `server/main.ts`
5. Deploy!

### Manual Deployment

```bash
deno run --allow-net --allow-read --allow-env --unstable-kv server/main.ts
```

## Local Development

### Prerequisites

- [Rust](https://rustup.rs/) (1.70+) - only needed to modify game engine
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) - only needed to rebuild WASM
- [Deno](https://deno.land/) (1.40+)

### Quick Start

The repository includes pre-built WASM and frontend bundles:

```bash
deno task dev
```

Then open http://localhost:8000

### Full Build (if modifying Rust code)

```bash
# Build everything and start server
deno task start
```

### Manual Build Steps

```bash
# Build WASM module
cd crates/tetris-core && wasm-pack build --target web --out-dir ../../pkg

# Bundle frontend
npx esbuild frontend/main.ts --bundle --format=esm --outdir=frontend/dist '--external:../pkg/*'

# Start server
deno task dev
```

## Controls

| Key               | Action                          |
| ----------------- | ------------------------------- |
| `←` `→`           | Move left/right                 |
| `↓`               | Soft drop                       |
| `Space`           | Hard drop                       |
| `↑` / `X`         | Rotate clockwise                |
| `Z` / `Ctrl`      | Rotate counter-clockwise        |
| `C` / `Shift`     | Hold piece                      |
| `P` / `Esc`       | Pause                           |
| `R`               | Restart (when paused/game over) |
| `Enter` / `Space` | Start game                      |

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

### WebGL Rendering

Custom GLSL shaders handle:

- **Cell Shader**: Renders tetromino blocks with rounded corners and opacity
- **Grid Shader**: Renders the background grid with cell borders

All rendering runs in Rust via `web-sys` WebGL bindings.

### Build Outputs

- `pkg/`: Contains `tetris_core.js`, `tetris_core_bg.wasm`, and TypeScript definitions
- `frontend/dist/`: Bundled JavaScript (~9KB)
- WASM size: ~80KB (includes WebGL renderer)

### Implementation Notes

- Game loop runs in Rust using `requestAnimationFrame`
- Input is processed entirely in Rust with DAS/ARR support
- State changes trigger JavaScript callbacks for DOM updates
- The 7-bag randomizer ensures fair piece distribution

## License

MIT
