/* tslint:disable */
/* eslint-disable */

export class Tetris {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Get hold piece preview cells
   */
  get_hold_cells(): Uint8Array;
  /**
   * Get next piece preview cells
   */
  get_next_cells(): Uint8Array;
  /**
   * Get board cells (non-empty only)
   */
  get_board_cells(): Uint8Array;
  /**
   * Get ghost piece cells
   */
  get_ghost_cells(): Uint8Array;
  /**
   * Get current piece cells
   */
  get_piece_cells(): Uint8Array;
  /**
   * Check if hold is available
   */
  is_hold_available(): boolean;
  constructor();
  /**
   * Handle key up event
   */
  key_up(key: number): void;
  /**
   * Update game state. Call every frame with delta time in ms.
   * Returns true if render state changed.
   */
  update(delta_ms: number): boolean;
  /**
   * Called when window loses focus
   */
  on_blur(): void;
  /**
   * Handle key down event
   * key: 0=left, 1=right, 2=down, 3=space, 4=up/x, 5=z, 6=c/shift, 7=p/esc, 8=enter, 9=r
   */
  key_down(key: number): void;
  /**
   * Get level
   */
  get_level(): number;
  /**
   * Get lines cleared
   */
  get_lines(): number;
  /**
   * Get score
   */
  get_score(): number;
  /**
   * Get current game state: 0=idle, 1=playing, 2=paused, 3=gameOver
   */
  get_state(): number;
}

export class TetrisApp {
  free(): void;
  [Symbol.dispose](): void;
  render_initial(): void;
  start_game_loop(): void;
  set_on_score_update(callback: Function): void;
  set_on_state_change(callback: Function): void;
  constructor(board_canvas: HTMLCanvasElement, next_canvas: HTMLCanvasElement, hold_canvas: HTMLCanvasElement);
  key_up(code: string): void;
  on_blur(): void;
  key_down(code: string): void;
  get_level(): number;
  get_lines(): number;
  get_score(): number;
  get_state(): number;
}

export function get_color(cell_type: number): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_tetrisapp_free: (a: number, b: number) => void;
  readonly tetrisapp_get_level: (a: number) => number;
  readonly tetrisapp_get_lines: (a: number) => number;
  readonly tetrisapp_get_score: (a: number) => number;
  readonly tetrisapp_get_state: (a: number) => number;
  readonly tetrisapp_key_down: (a: number, b: number, c: number) => void;
  readonly tetrisapp_key_up: (a: number, b: number, c: number) => void;
  readonly tetrisapp_new: (a: any, b: any, c: any) => [number, number, number];
  readonly tetrisapp_on_blur: (a: number) => void;
  readonly tetrisapp_render_initial: (a: number) => void;
  readonly tetrisapp_set_on_score_update: (a: number, b: any) => void;
  readonly tetrisapp_set_on_state_change: (a: number, b: any) => void;
  readonly tetrisapp_start_game_loop: (a: number) => [number, number];
  readonly __wbg_tetris_free: (a: number, b: number) => void;
  readonly get_color: (a: number) => [number, number];
  readonly tetris_get_board_cells: (a: number) => [number, number];
  readonly tetris_get_ghost_cells: (a: number) => [number, number];
  readonly tetris_get_hold_cells: (a: number) => [number, number];
  readonly tetris_get_level: (a: number) => number;
  readonly tetris_get_lines: (a: number) => number;
  readonly tetris_get_next_cells: (a: number) => [number, number];
  readonly tetris_get_piece_cells: (a: number) => [number, number];
  readonly tetris_get_score: (a: number) => number;
  readonly tetris_get_state: (a: number) => number;
  readonly tetris_is_hold_available: (a: number) => number;
  readonly tetris_key_down: (a: number, b: number) => void;
  readonly tetris_key_up: (a: number, b: number) => void;
  readonly tetris_new: () => number;
  readonly tetris_on_blur: (a: number) => void;
  readonly tetris_update: (a: number, b: number) => number;
  readonly wasm_bindgen__convert__closures_____invoke__hc1ac6a06dab0941c: (a: number, b: number, c: number) => void;
  readonly wasm_bindgen__closure__destroy__h05f94b76025885e2: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
