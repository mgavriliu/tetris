# Tetris Collection

Three modern Tetris implementations built with Rust/WebAssembly, each using a different rendering approach.

| Project                         | Frontend      | Rendering  | Description                        |
| ------------------------------- | ------------- | ---------- | ---------------------------------- |
| [rusty-tetris](./rusty-tetris/) | TypeScript    | SVG        | Classic approach with JS game loop |
| [webgl-tetris](./webgl-tetris/) | TypeScript    | WebGL/GLSL | Hardware-accelerated rendering     |
| [diox-tetris](./diox-tetris/)   | Rust (Dioxus) | SVG        | Pure Rust frontend, no JS          |

**[Play on GitHub Pages](https://mgavriliu.github.io/tetris/)**

## Architecture

```
tetris/
├── server/                # Shared API server (Deno Deploy)
│   └── main.ts            # High scores API with Deno KV
├── rusty-tetris/          # SVG + TypeScript implementation
├── webgl-tetris/          # WebGL + TypeScript implementation
├── diox-tetris/           # Pure Rust + Dioxus implementation
└── docs/                  # GitHub Pages (game selector)
```

### Shared API

All three games share a common high scores API deployed on Deno Deploy:

- **Endpoint**: `https://tetris-api.mgavriliu.deno.net/api`
- **Storage**: Deno KV for persistent scores
- **CORS**: Enabled for cross-origin requests

```bash
# Deploy the shared API
cd server
deployctl deploy --project=tetris-api main.ts
```

## Comparison

| Aspect            | rusty-tetris   | webgl-tetris | diox-tetris           |
| ----------------- | -------------- | ------------ | --------------------- |
| **Frontend**      | TypeScript     | TypeScript   | Rust (Dioxus)         |
| **Rendering**     | SVG via DOM    | WebGL/GLSL   | Dioxus RSX → SVG      |
| **Game Loop**     | JavaScript RAF | Rust RAF     | Rust async            |
| **Frontend Size** | ~25KB JS       | ~9KB JS      | 0 JS (generated only) |
| **WASM Size**     | ~60KB          | ~80KB        | ~2MB (includes UI)    |

## Shared Features

All implementations share identical game mechanics:

- **Modern Tetris Mechanics**: 7-bag randomizer, SRS wall kicks, ghost piece, hold piece
- **NES-Style Speed Curve**: Progressive difficulty with level-based speed increases
- **High Score System**: Global leaderboard via shared API
- **DAS/ARR Input**: Delayed Auto Shift (167ms) and Auto Repeat Rate (33ms)

## Controls

| Key           | Action                   |
| ------------- | ------------------------ |
| `←` `→`       | Move left/right          |
| `↓`           | Soft drop                |
| `Space`       | Hard drop                |
| `↑` / `X`     | Rotate clockwise         |
| `Z` / `Ctrl`  | Rotate counter-clockwise |
| `C` / `Shift` | Hold piece               |
| `P` / `Esc`   | Pause                    |
| `R`           | Restart                  |

## Scoring

| Action    | Points      |
| --------- | ----------- |
| Soft drop | 1 per cell  |
| Hard drop | 2 per cell  |
| Single    | 100 × level |
| Double    | 300 × level |
| Triple    | 500 × level |
| Tetris    | 800 × level |

Level increases every 10 lines.

## Local Development

### Run the Game Selector

```bash
# From project root, start a static file server
python3 -m http.server 8080
# Open http://localhost:8080/docs/index.html
```

### Run Individual Games

```bash
# rusty-tetris or webgl-tetris (static files)
cd rusty-tetris
python3 -m http.server 8080 -d frontend
# Open http://localhost:8080

# diox-tetris (requires Dioxus CLI)
cd diox-tetris
dx serve
# Open http://localhost:8080
```

### Run the API Server Locally

```bash
cd server
deno task dev
# API available at http://localhost:8000
```

## Deployment

### GitHub Pages (Static Frontends)

All three games are static and can be hosted on GitHub Pages. Configure Pages to serve from the repo root.

### Deno Deploy (API Server)

```bash
cd server
deployctl deploy --project=tetris-api main.ts
```

After deployment, update `API_BASE` in each frontend if using a different project name.

## Prerequisites

- [Deno](https://deno.land/) 1.40+ (for API server)
- [Rust](https://rustup.rs/) 1.70+ (only for WASM modifications)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (only for WASM rebuild)
- [Dioxus CLI](https://dioxuslabs.com/) (only for diox-tetris development)

## License

MIT
