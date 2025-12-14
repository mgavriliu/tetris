// pkg/tetris_core.js
var wasm;
function addToExternrefTable0(obj) {
  const idx = wasm.__externref_table_alloc();
  wasm.__wbindgen_externrefs.set(idx, obj);
  return idx;
}
function getArrayU8FromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}
function getStringFromWasm0(ptr, len) {
  ptr = ptr >>> 0;
  return decodeText(ptr, len);
}
var cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
  if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
    cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
  }
  return cachedUint8ArrayMemory0;
}
function handleError(f, args) {
  try {
    return f.apply(this, args);
  } catch (e) {
    const idx = addToExternrefTable0(e);
    wasm.__wbindgen_exn_store(idx);
  }
}
function isLikeNone(x) {
  return x === void 0 || x === null;
}
var cachedTextDecoder = new TextDecoder("utf-8", { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
var MAX_SAFARI_DECODE_BYTES = 2146435072;
var numBytesDecoded = 0;
function decodeText(ptr, len) {
  numBytesDecoded += len;
  if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
    cachedTextDecoder = new TextDecoder("utf-8", { ignoreBOM: true, fatal: true });
    cachedTextDecoder.decode();
    numBytesDecoded = len;
  }
  return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}
var TetrisFinalization = typeof FinalizationRegistry === "undefined" ? { register: () => {
}, unregister: () => {
} } : new FinalizationRegistry((ptr) => wasm.__wbg_tetris_free(ptr >>> 0, 1));
var Tetris = class {
  __destroy_into_raw() {
    const ptr = this.__wbg_ptr;
    this.__wbg_ptr = 0;
    TetrisFinalization.unregister(this);
    return ptr;
  }
  free() {
    const ptr = this.__destroy_into_raw();
    wasm.__wbg_tetris_free(ptr, 0);
  }
  /**
   * Get hold piece preview cells
   * @returns {Uint8Array}
   */
  get_hold_cells() {
    const ret = wasm.tetris_get_hold_cells(this.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * Get next piece preview cells
   * @returns {Uint8Array}
   */
  get_next_cells() {
    const ret = wasm.tetris_get_next_cells(this.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * Get board cells (non-empty only)
   * @returns {Uint8Array}
   */
  get_board_cells() {
    const ret = wasm.tetris_get_board_cells(this.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * Get ghost piece cells
   * @returns {Uint8Array}
   */
  get_ghost_cells() {
    const ret = wasm.tetris_get_ghost_cells(this.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * Get current piece cells
   * @returns {Uint8Array}
   */
  get_piece_cells() {
    const ret = wasm.tetris_get_piece_cells(this.__wbg_ptr);
    var v1 = getArrayU8FromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 1, 1);
    return v1;
  }
  /**
   * Check if hold is available
   * @returns {boolean}
   */
  is_hold_available() {
    const ret = wasm.tetris_is_hold_available(this.__wbg_ptr);
    return ret !== 0;
  }
  constructor() {
    const ret = wasm.tetris_new();
    this.__wbg_ptr = ret >>> 0;
    TetrisFinalization.register(this, this.__wbg_ptr, this);
    return this;
  }
  /**
   * Handle key up event
   * @param {number} key
   */
  key_up(key) {
    wasm.tetris_key_up(this.__wbg_ptr, key);
  }
  /**
   * Update game state. Call every frame with delta time in ms.
   * Returns true if render state changed.
   * @param {number} delta_ms
   * @returns {boolean}
   */
  update(delta_ms) {
    const ret = wasm.tetris_update(this.__wbg_ptr, delta_ms);
    return ret !== 0;
  }
  /**
   * Called when window loses focus
   */
  on_blur() {
    wasm.tetris_on_blur(this.__wbg_ptr);
  }
  /**
   * Handle key down event
   * key: 0=left, 1=right, 2=down, 3=space, 4=up/x, 5=z, 6=c/shift, 7=p/esc, 8=enter, 9=r
   * @param {number} key
   */
  key_down(key) {
    wasm.tetris_key_down(this.__wbg_ptr, key);
  }
  /**
   * Get level
   * @returns {number}
   */
  get_level() {
    const ret = wasm.tetris_get_level(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
   * Get lines cleared
   * @returns {number}
   */
  get_lines() {
    const ret = wasm.tetris_get_lines(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
   * Get score
   * @returns {number}
   */
  get_score() {
    const ret = wasm.tetris_get_score(this.__wbg_ptr);
    return ret >>> 0;
  }
  /**
   * Get current game state: 0=idle, 1=playing, 2=paused, 3=gameOver
   * @returns {number}
   */
  get_state() {
    const ret = wasm.tetris_get_state(this.__wbg_ptr);
    return ret;
  }
};
if (Symbol.dispose)
  Tetris.prototype[Symbol.dispose] = Tetris.prototype.free;
function get_color(cell_type) {
  let deferred1_0;
  let deferred1_1;
  try {
    const ret = wasm.get_color(cell_type);
    deferred1_0 = ret[0];
    deferred1_1 = ret[1];
    return getStringFromWasm0(ret[0], ret[1]);
  } finally {
    wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
  }
}
var EXPECTED_RESPONSE_TYPES = /* @__PURE__ */ new Set(["basic", "cors", "default"]);
async function __wbg_load(module2, imports) {
  if (typeof Response === "function" && module2 instanceof Response) {
    if (typeof WebAssembly.instantiateStreaming === "function") {
      try {
        return await WebAssembly.instantiateStreaming(module2, imports);
      } catch (e) {
        const validResponse = module2.ok && EXPECTED_RESPONSE_TYPES.has(module2.type);
        if (validResponse && module2.headers.get("Content-Type") !== "application/wasm") {
          console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
        } else {
          throw e;
        }
      }
    }
    const bytes = await module2.arrayBuffer();
    return await WebAssembly.instantiate(bytes, imports);
  } else {
    const instance = await WebAssembly.instantiate(module2, imports);
    if (instance instanceof WebAssembly.Instance) {
      return { instance, module: module2 };
    } else {
      return instance;
    }
  }
}
function __wbg_get_imports() {
  const imports = {};
  imports.wbg = {};
  imports.wbg.__wbg___wbindgen_is_function_8d400b8b1af978cd = function(arg0) {
    const ret = typeof arg0 === "function";
    return ret;
  };
  imports.wbg.__wbg___wbindgen_is_object_ce774f3490692386 = function(arg0) {
    const val = arg0;
    const ret = typeof val === "object" && val !== null;
    return ret;
  };
  imports.wbg.__wbg___wbindgen_is_string_704ef9c8fc131030 = function(arg0) {
    const ret = typeof arg0 === "string";
    return ret;
  };
  imports.wbg.__wbg___wbindgen_is_undefined_f6b95eab589e0269 = function(arg0) {
    const ret = arg0 === void 0;
    return ret;
  };
  imports.wbg.__wbg___wbindgen_throw_dd24417ed36fc46e = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
  };
  imports.wbg.__wbg_call_3020136f7a2d6e44 = function() {
    return handleError(function(arg0, arg1, arg2) {
      const ret = arg0.call(arg1, arg2);
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_call_abb4ff46ce38be40 = function() {
    return handleError(function(arg0, arg1) {
      const ret = arg0.call(arg1);
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_crypto_574e78ad8b13b65f = function(arg0) {
    const ret = arg0.crypto;
    return ret;
  };
  imports.wbg.__wbg_getRandomValues_b8f5dbd5f3995a9e = function() {
    return handleError(function(arg0, arg1) {
      arg0.getRandomValues(arg1);
    }, arguments);
  };
  imports.wbg.__wbg_length_22ac23eaec9d8053 = function(arg0) {
    const ret = arg0.length;
    return ret;
  };
  imports.wbg.__wbg_msCrypto_a61aeb35a24c1329 = function(arg0) {
    const ret = arg0.msCrypto;
    return ret;
  };
  imports.wbg.__wbg_new_no_args_cb138f77cf6151ee = function(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return ret;
  };
  imports.wbg.__wbg_new_with_length_aa5eaf41d35235e5 = function(arg0) {
    const ret = new Uint8Array(arg0 >>> 0);
    return ret;
  };
  imports.wbg.__wbg_node_905d3e251edff8a2 = function(arg0) {
    const ret = arg0.node;
    return ret;
  };
  imports.wbg.__wbg_process_dc0fbacc7c1c06f7 = function(arg0) {
    const ret = arg0.process;
    return ret;
  };
  imports.wbg.__wbg_prototypesetcall_dfe9b766cdc1f1fd = function(arg0, arg1, arg2) {
    Uint8Array.prototype.set.call(getArrayU8FromWasm0(arg0, arg1), arg2);
  };
  imports.wbg.__wbg_randomFillSync_ac0988aba3254290 = function() {
    return handleError(function(arg0, arg1) {
      arg0.randomFillSync(arg1);
    }, arguments);
  };
  imports.wbg.__wbg_require_60cc747a6bc5215a = function() {
    return handleError(function() {
      const ret = module.require;
      return ret;
    }, arguments);
  };
  imports.wbg.__wbg_static_accessor_GLOBAL_769e6b65d6557335 = function() {
    const ret = typeof global === "undefined" ? null : global;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
  };
  imports.wbg.__wbg_static_accessor_GLOBAL_THIS_60cf02db4de8e1c1 = function() {
    const ret = typeof globalThis === "undefined" ? null : globalThis;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
  };
  imports.wbg.__wbg_static_accessor_SELF_08f5a74c69739274 = function() {
    const ret = typeof self === "undefined" ? null : self;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
  };
  imports.wbg.__wbg_static_accessor_WINDOW_a8924b26aa92d024 = function() {
    const ret = typeof window === "undefined" ? null : window;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
  };
  imports.wbg.__wbg_subarray_845f2f5bce7d061a = function(arg0, arg1, arg2) {
    const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
    return ret;
  };
  imports.wbg.__wbg_versions_c01dfd4722a88165 = function(arg0) {
    const ret = arg0.versions;
    return ret;
  };
  imports.wbg.__wbindgen_cast_2241b6af4c4b2941 = function(arg0, arg1) {
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
  };
  imports.wbg.__wbindgen_cast_cb9088102bce6b30 = function(arg0, arg1) {
    const ret = getArrayU8FromWasm0(arg0, arg1);
    return ret;
  };
  imports.wbg.__wbindgen_init_externref_table = function() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, void 0);
    table.set(offset + 0, void 0);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
  };
  return imports;
}
function __wbg_finalize_init(instance, module2) {
  wasm = instance.exports;
  __wbg_init.__wbindgen_wasm_module = module2;
  cachedUint8ArrayMemory0 = null;
  wasm.__wbindgen_start();
  return wasm;
}
async function __wbg_init(module_or_path) {
  if (wasm !== void 0)
    return wasm;
  if (typeof module_or_path !== "undefined") {
    if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
      ({ module_or_path } = module_or_path);
    } else {
      console.warn("using deprecated parameters for the initialization function; pass a single object instead");
    }
  }
  if (typeof module_or_path === "undefined") {
    module_or_path = new URL("tetris_core_bg.wasm", import.meta.url);
  }
  const imports = __wbg_get_imports();
  if (typeof module_or_path === "string" || typeof Request === "function" && module_or_path instanceof Request || typeof URL === "function" && module_or_path instanceof URL) {
    module_or_path = fetch(module_or_path);
  }
  const { instance, module: module2 } = await __wbg_load(await module_or_path, imports);
  return __wbg_finalize_init(instance, module2);
}
var tetris_core_default = __wbg_init;

// frontend/api.ts
var API_BASE = "/api";
async function getHighScores() {
  try {
    const response = await fetch(`${API_BASE}/scores`);
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}`);
    }
    return await response.json();
  } catch (error) {
    console.error("Failed to fetch high scores:", error);
    return [];
  }
}
async function submitScore(score) {
  try {
    const response = await fetch(`${API_BASE}/scores`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify(score)
    });
    return response.ok;
  } catch (error) {
    console.error("Failed to submit score:", error);
    return false;
  }
}

// frontend/main.ts
var CELL_SIZE = 30;
var BOARD_WIDTH = 10;
var BOARD_HEIGHT = 20;
var KEY_MAP = {
  ArrowLeft: 0,
  ArrowRight: 1,
  ArrowDown: 2,
  Space: 3,
  ArrowUp: 4,
  KeyX: 4,
  KeyZ: 5,
  ControlLeft: 5,
  ControlRight: 5,
  KeyC: 6,
  ShiftLeft: 6,
  ShiftRight: 6,
  KeyP: 7,
  Escape: 7,
  Enter: 8,
  KeyR: 9
};
var STATE_IDLE = 0;
var STATE_PLAYING = 1;
var STATE_PAUSED = 2;
var STATE_GAME_OVER = 3;
var svg;
var boardGroup;
var pieceGroup;
var ghostGroup;
var overlayGroup;
var nextSvg;
var holdSvg;
var scoreEl;
var levelEl;
var linesEl;
var scoresEl;
var tetris;
var lastTime = 0;
var lastState = STATE_IDLE;
async function main() {
  await tetris_core_default("/pkg/tetris_core_bg.wasm");
  tetris = new Tetris();
  createUI();
  setupInputHandlers();
  await loadHighScores();
  showStartScreen();
  requestAnimationFrame(gameLoop);
}
function createUI() {
  const container = document.getElementById("game-container");
  svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  svg.setAttribute("width", String(BOARD_WIDTH * CELL_SIZE + 6));
  svg.setAttribute("height", String(BOARD_HEIGHT * CELL_SIZE + 2));
  svg.setAttribute("class", "tetris-board");
  boardGroup = document.createElementNS("http://www.w3.org/2000/svg", "g");
  ghostGroup = document.createElementNS("http://www.w3.org/2000/svg", "g");
  pieceGroup = document.createElementNS("http://www.w3.org/2000/svg", "g");
  overlayGroup = document.createElementNS("http://www.w3.org/2000/svg", "g");
  svg.appendChild(boardGroup);
  svg.appendChild(ghostGroup);
  svg.appendChild(pieceGroup);
  svg.appendChild(overlayGroup);
  for (let y = 0; y < BOARD_HEIGHT; y++) {
    for (let x = 0; x < BOARD_WIDTH; x++) {
      const rect = document.createElementNS("http://www.w3.org/2000/svg", "rect");
      rect.setAttribute("x", String(x * CELL_SIZE));
      rect.setAttribute("y", String(y * CELL_SIZE));
      rect.setAttribute("width", String(CELL_SIZE));
      rect.setAttribute("height", String(CELL_SIZE));
      rect.setAttribute("fill", get_color(0));
      rect.setAttribute("stroke", "#2a2a4a");
      rect.setAttribute("stroke-width", "1");
      boardGroup.appendChild(rect);
    }
  }
  nextSvg = createPreviewSvg();
  holdSvg = createPreviewSvg();
  const gameArea = document.createElement("div");
  gameArea.className = "game-area";
  const leftPanel = document.createElement("div");
  leftPanel.className = "side-panel";
  leftPanel.innerHTML = `
    <div class="panel-section">
      <div class="panel-label">HOLD</div>
      <div class="preview-container" id="hold-container"></div>
    </div>
    <div class="panel-section">
      <div class="panel-label">HIGH SCORES</div>
      <div id="ingame-scores" class="ingame-scores"></div>
    </div>
  `;
  const rightPanel = document.createElement("div");
  rightPanel.className = "side-panel";
  rightPanel.innerHTML = `
    <div class="panel-section">
      <div class="panel-label">NEXT</div>
      <div class="preview-container" id="next-container"></div>
    </div>
    <div class="panel-section">
      <div class="panel-label">SCORE</div>
      <div class="stat-value" id="score">0</div>
    </div>
    <div class="panel-section">
      <div class="panel-label">LEVEL</div>
      <div class="stat-value" id="level">1</div>
    </div>
    <div class="panel-section">
      <div class="panel-label">LINES</div>
      <div class="stat-value" id="lines">0</div>
    </div>
  `;
  gameArea.appendChild(leftPanel);
  gameArea.appendChild(svg);
  gameArea.appendChild(rightPanel);
  container.appendChild(gameArea);
  document.getElementById("hold-container").appendChild(holdSvg);
  document.getElementById("next-container").appendChild(nextSvg);
  scoreEl = document.getElementById("score");
  levelEl = document.getElementById("level");
  linesEl = document.getElementById("lines");
  scoresEl = document.getElementById("ingame-scores");
}
function createPreviewSvg() {
  const s = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  s.setAttribute("width", String(4 * CELL_SIZE * 0.8));
  s.setAttribute("height", String(2 * CELL_SIZE * 0.8));
  s.setAttribute("class", "preview-svg");
  return s;
}
function setupInputHandlers() {
  window.addEventListener("keydown", (e) => {
    const key = KEY_MAP[e.code];
    if (key !== void 0) {
      e.preventDefault();
      tetris.key_down(key);
    }
  });
  window.addEventListener("keyup", (e) => {
    const key = KEY_MAP[e.code];
    if (key !== void 0) {
      tetris.key_up(key);
    }
  });
  window.addEventListener("blur", () => {
    tetris.on_blur();
  });
}
function gameLoop(currentTime) {
  const deltaMs = lastTime ? currentTime - lastTime : 0;
  lastTime = currentTime;
  tetris.update(deltaMs);
  const state = tetris.get_state();
  if (state !== lastState) {
    if (state === STATE_GAME_OVER) {
      showGameOver();
      handleGameOver();
    } else if (state === STATE_PAUSED) {
      showPauseOverlay();
    } else if (state === STATE_PLAYING && lastState !== STATE_PLAYING) {
      clearOverlay();
    } else if (state === STATE_IDLE) {
      showStartScreen();
    }
    lastState = state;
  }
  if (state === STATE_PLAYING) {
    render();
  }
  requestAnimationFrame(gameLoop);
}
function render() {
  scoreEl.textContent = String(tetris.get_score());
  levelEl.textContent = String(tetris.get_level());
  linesEl.textContent = String(tetris.get_lines());
  ghostGroup.innerHTML = "";
  pieceGroup.innerHTML = "";
  const boardRects = boardGroup.querySelectorAll("rect");
  boardRects.forEach((rect) => rect.setAttribute("fill", get_color(0)));
  const boardCells = tetris.get_board_cells();
  for (let i = 0; i < boardCells.length; i += 4) {
    const x = boardCells[i];
    const y = boardCells[i + 1];
    const color = boardCells[i + 2];
    const idx = y * BOARD_WIDTH + x;
    boardRects[idx]?.setAttribute("fill", get_color(color));
  }
  const ghostCells = tetris.get_ghost_cells();
  renderCells(ghostGroup, ghostCells, CELL_SIZE);
  const pieceCells = tetris.get_piece_cells();
  renderCells(pieceGroup, pieceCells, CELL_SIZE);
  renderPreview(nextSvg, tetris.get_next_cells());
  renderPreview(holdSvg, tetris.get_hold_cells());
}
function renderCells(group, cells, cellSize) {
  for (let i = 0; i < cells.length; i += 4) {
    const x = cells[i];
    const y = cells[i + 1];
    const color = cells[i + 2];
    const opacity = cells[i + 3] / 255;
    const rect = document.createElementNS("http://www.w3.org/2000/svg", "rect");
    rect.setAttribute("x", String(x * cellSize + 1));
    rect.setAttribute("y", String(y * cellSize + 1));
    rect.setAttribute("width", String(cellSize - 2));
    rect.setAttribute("height", String(cellSize - 2));
    rect.setAttribute("fill", get_color(color));
    rect.setAttribute("opacity", String(opacity));
    rect.setAttribute("rx", "3");
    group.appendChild(rect);
  }
}
function renderPreview(svg2, cells) {
  svg2.innerHTML = "";
  const cellSize = CELL_SIZE * 0.8;
  for (let i = 0; i < cells.length; i += 4) {
    const x = cells[i];
    const y = cells[i + 1];
    const color = cells[i + 2];
    const opacity = cells[i + 3] / 255;
    const rect = document.createElementNS("http://www.w3.org/2000/svg", "rect");
    rect.setAttribute("x", String(x * cellSize + 1));
    rect.setAttribute("y", String(y * cellSize + 1));
    rect.setAttribute("width", String(cellSize - 2));
    rect.setAttribute("height", String(cellSize - 2));
    rect.setAttribute("fill", get_color(color));
    rect.setAttribute("opacity", String(opacity));
    rect.setAttribute("rx", "2");
    svg2.appendChild(rect);
  }
}
function showStartScreen() {
  clearOverlay();
  let screen = document.getElementById("start-screen");
  if (!screen) {
    screen = document.createElement("div");
    screen.id = "start-screen";
    screen.innerHTML = `
      <h1>TETRIS</h1>
      <p class="subtitle">Press SPACE or ENTER to start</p>
      <div class="controls">
        <h3>Controls</h3>
        <ul>
          <li><kbd>&larr;</kbd> <kbd>&rarr;</kbd> Move</li>
          <li><kbd>&darr;</kbd> Soft drop</li>
          <li><kbd>Space</kbd> Hard drop</li>
          <li><kbd>&uarr;</kbd> <kbd>X</kbd> Rotate CW</li>
          <li><kbd>Z</kbd> Rotate CCW</li>
          <li><kbd>C</kbd> <kbd>Shift</kbd> Hold</li>
          <li><kbd>P</kbd> <kbd>Esc</kbd> Pause</li>
        </ul>
      </div>
      <div id="high-scores"></div>
    `;
    document.getElementById("game-container")?.appendChild(screen);
    loadHighScores();
  }
  screen.style.display = "flex";
}
function showPauseOverlay() {
  clearOverlay();
  let overlay = document.getElementById("pause-overlay");
  if (!overlay) {
    overlay = document.createElement("div");
    overlay.id = "pause-overlay";
    overlay.innerHTML = `
      <h2>PAUSED</h2>
      <p>Press P or ESC to resume</p>
      <p>Press R to restart</p>
    `;
    document.getElementById("game-container")?.appendChild(overlay);
  }
  overlay.style.display = "flex";
}
function showGameOver() {
  const rect = document.createElementNS("http://www.w3.org/2000/svg", "rect");
  rect.setAttribute("width", String(BOARD_WIDTH * CELL_SIZE));
  rect.setAttribute("height", String(BOARD_HEIGHT * CELL_SIZE));
  rect.setAttribute("fill", "rgba(0,0,0,0.7)");
  overlayGroup.appendChild(rect);
  const text = document.createElementNS("http://www.w3.org/2000/svg", "text");
  text.setAttribute("x", String(BOARD_WIDTH * CELL_SIZE / 2));
  text.setAttribute("y", String(BOARD_HEIGHT * CELL_SIZE / 2));
  text.setAttribute("text-anchor", "middle");
  text.setAttribute("fill", "#fff");
  text.setAttribute("font-size", "24");
  text.setAttribute("font-family", "monospace");
  text.textContent = "GAME OVER";
  overlayGroup.appendChild(text);
  const subtext = document.createElementNS("http://www.w3.org/2000/svg", "text");
  subtext.setAttribute("x", String(BOARD_WIDTH * CELL_SIZE / 2));
  subtext.setAttribute("y", String(BOARD_HEIGHT * CELL_SIZE / 2 + 30));
  subtext.setAttribute("text-anchor", "middle");
  subtext.setAttribute("fill", "#888");
  subtext.setAttribute("font-size", "14");
  subtext.setAttribute("font-family", "monospace");
  subtext.textContent = "Press R to restart";
  overlayGroup.appendChild(subtext);
}
function clearOverlay() {
  overlayGroup.innerHTML = "";
  document.getElementById("start-screen")?.style.setProperty("display", "none");
  document.getElementById("pause-overlay")?.style.setProperty("display", "none");
}
async function loadHighScores() {
  const scores = await getHighScores();
  displayHighScores(scores);
  displayInGameScores(scores);
}
function displayHighScores(scores) {
  const container = document.getElementById("high-scores");
  if (!container)
    return;
  if (scores.length === 0) {
    container.innerHTML = "<p class='no-scores'>No high scores yet!</p>";
    return;
  }
  container.innerHTML = `
    <h3>High Scores</h3>
    <ol class="scores-list">
      ${scores.slice(0, 10).map((s) => `
        <li><span class="score-name">${escapeHtml(s.name)}</span>
        <span class="score-value">${s.score.toLocaleString()}</span></li>
      `).join("")}
    </ol>
  `;
}
function displayInGameScores(scores) {
  if (!scoresEl)
    return;
  if (scores.length === 0) {
    scoresEl.innerHTML = '<div class="no-scores">-</div>';
    return;
  }
  scoresEl.innerHTML = scores.slice(0, 5).map(
    (s, i) => `<div class="score-row"><span class="rank">${i + 1}.</span><span class="name">${escapeHtml(s.name)}</span><span class="pts">${s.score.toLocaleString()}</span></div>`
  ).join("");
}
function escapeHtml(text) {
  const div = document.createElement("div");
  div.textContent = text;
  return div.innerHTML;
}
async function handleGameOver() {
  const score = tetris.get_score();
  const level = tetris.get_level();
  const lines = tetris.get_lines();
  const scores = await getHighScores();
  const isHighScore = scores.length < 10 || score > (scores[9]?.score ?? 0);
  if (isHighScore && score > 0) {
    const name = prompt(`High Score! Enter your name (Score: ${score.toLocaleString()})`);
    if (name?.trim()) {
      await submitScore({ name: name.trim().substring(0, 20), score, level, lines });
      await loadHighScores();
    }
  }
}
main().catch(console.error);
