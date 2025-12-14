# Tetris

Two modern Tetris implementations built with Rust/WebAssembly and Deno, differing primarily in rendering approach.

| Project                         | Rendering         | Live Demo                                                                                                            |
| ------------------------------- | ----------------- | -------------------------------------------------------------------------------------------------------------------- |
| [rusty-tetris](./rusty-tetris/) | SVG (TypeScript)  | [Deno Deploy](https://rusty-tetris.mgavriliu.deno.net/) \| [GitHub Pages](https://mgavriliu.github.io/rusty-tetris/) |
| [webgl-tetris](./webgl-tetris/) | WebGL (Rust/GLSL) | [Deno Deploy](https://webgl-tetris.mgavriliu.deno.net/) \| [GitHub Pages](https://mgavriliu.github.io/webgl-tetris/) |

## Key Differences

| Aspect              | rusty-tetris                         | webgl-tetris                               |
| ------------------- | ------------------------------------ | ------------------------------------------ |
| **Rendering**       | SVG elements via TypeScript          | WebGL with GLSL shaders in Rust            |
| **Game Loop**       | JavaScript (`requestAnimationFrame`) | Rust (`requestAnimationFrame` via web-sys) |
| **Frontend Size**   | ~25KB JS                             | ~9KB JS                                    |
| **WASM Size**       | ~60KB                                | ~80KB (includes WebGL renderer)            |
| **TypeScript Role** | Full rendering + game loop           | DOM/overlay management only (~200 lines)   |

## Shared Features

Both implementations share identical game mechanics:

- **Rust/WASM Game Engine**: Core logic in Rust compiled to WebAssembly
- **Modern Tetris Mechanics**: 7-bag randomizer, SRS wall kicks, ghost piece, hold piece
- **NES-Style Speed Curve**: Progressive difficulty with level-based speed increases
- **High Score System**: Persistent global leaderboard via Deno KV
- **DAS/ARR Input**: Delayed Auto Shift (167ms) and Auto Repeat Rate (33ms)

## Architecture

Both projects follow the same structure:

```
project/
├── crates/tetris-core/    # Rust WASM game engine
├── frontend/              # TypeScript (UI, API client)
├── server/                # Deno HTTP server + KV storage
├── pkg/                   # WASM build output
└── deno.json              # Deno tasks
```

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

## Quick Start

```bash
cd rusty-tetris  # or webgl-tetris
deno task dev
# Open http://localhost:8000
```

### Prerequisites (for modifications)

- [Deno](https://deno.land/) 1.40+
- [Rust](https://rustup.rs/) 1.70+ (only for engine changes)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (only for WASM rebuild)

## License

MIT
