let wasm;

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => state.dtor(state.a, state.b));

function getArrayF32FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getFloat32ArrayMemory0().subarray(ptr / 4, ptr / 4 + len);
}

function getArrayU8FromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return getUint8ArrayMemory0().subarray(ptr / 1, ptr / 1 + len);
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

let cachedFloat32ArrayMemory0 = null;
function getFloat32ArrayMemory0() {
    if (cachedFloat32ArrayMemory0 === null || cachedFloat32ArrayMemory0.byteLength === 0) {
        cachedFloat32ArrayMemory0 = new Float32Array(wasm.memory.buffer);
    }
    return cachedFloat32ArrayMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
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
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            state.dtor(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    }
}

let WASM_VECTOR_LEN = 0;

function wasm_bindgen__convert__closures_____invoke__hc1ac6a06dab0941c(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__hc1ac6a06dab0941c(arg0, arg1, arg2);
}

const TetrisFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_tetris_free(ptr >>> 0, 1));

const TetrisAppFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_tetrisapp_free(ptr >>> 0, 1));

/**
 * Main WASM interface - a thin wrapper around GameController
 */
export class Tetris {
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
}
if (Symbol.dispose) Tetris.prototype[Symbol.dispose] = Tetris.prototype.free;

export class TetrisApp {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TetrisAppFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_tetrisapp_free(ptr, 0);
    }
    render_initial() {
        wasm.tetrisapp_render_initial(this.__wbg_ptr);
    }
    start_game_loop() {
        const ret = wasm.tetrisapp_start_game_loop(this.__wbg_ptr);
        if (ret[1]) {
            throw takeFromExternrefTable0(ret[0]);
        }
    }
    /**
     * @param {Function} callback
     */
    set_on_score_update(callback) {
        wasm.tetrisapp_set_on_score_update(this.__wbg_ptr, callback);
    }
    /**
     * @param {Function} callback
     */
    set_on_state_change(callback) {
        wasm.tetrisapp_set_on_state_change(this.__wbg_ptr, callback);
    }
    /**
     * @param {HTMLCanvasElement} board_canvas
     * @param {HTMLCanvasElement} next_canvas
     * @param {HTMLCanvasElement} hold_canvas
     */
    constructor(board_canvas, next_canvas, hold_canvas) {
        const ret = wasm.tetrisapp_new(board_canvas, next_canvas, hold_canvas);
        if (ret[2]) {
            throw takeFromExternrefTable0(ret[1]);
        }
        this.__wbg_ptr = ret[0] >>> 0;
        TetrisAppFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * @param {string} code
     */
    key_up(code) {
        const ptr0 = passStringToWasm0(code, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.tetrisapp_key_up(this.__wbg_ptr, ptr0, len0);
    }
    on_blur() {
        wasm.tetrisapp_on_blur(this.__wbg_ptr);
    }
    /**
     * @param {string} code
     */
    key_down(code) {
        const ptr0 = passStringToWasm0(code, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.tetrisapp_key_down(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {number}
     */
    get_level() {
        const ret = wasm.tetrisapp_get_level(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get_lines() {
        const ret = wasm.tetrisapp_get_lines(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get_score() {
        const ret = wasm.tetrisapp_get_score(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {number}
     */
    get_state() {
        const ret = wasm.tetrisapp_get_state(this.__wbg_ptr);
        return ret;
    }
}
if (Symbol.dispose) TetrisApp.prototype[Symbol.dispose] = TetrisApp.prototype.free;

/**
 * @param {number} cell_type
 * @returns {string}
 */
export function get_color(cell_type) {
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

const EXPECTED_RESPONSE_TYPES = new Set(['basic', 'cors', 'default']);

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && EXPECTED_RESPONSE_TYPES.has(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }
}

function __wbg_get_imports() {
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbg___wbindgen_boolean_get_dea25b33882b895b = function(arg0) {
        const v = arg0;
        const ret = typeof(v) === 'boolean' ? v : undefined;
        return isLikeNone(ret) ? 0xFFFFFF : ret ? 1 : 0;
    };
    imports.wbg.__wbg___wbindgen_is_function_8d400b8b1af978cd = function(arg0) {
        const ret = typeof(arg0) === 'function';
        return ret;
    };
    imports.wbg.__wbg___wbindgen_is_object_ce774f3490692386 = function(arg0) {
        const val = arg0;
        const ret = typeof(val) === 'object' && val !== null;
        return ret;
    };
    imports.wbg.__wbg___wbindgen_is_string_704ef9c8fc131030 = function(arg0) {
        const ret = typeof(arg0) === 'string';
        return ret;
    };
    imports.wbg.__wbg___wbindgen_is_undefined_f6b95eab589e0269 = function(arg0) {
        const ret = arg0 === undefined;
        return ret;
    };
    imports.wbg.__wbg___wbindgen_throw_dd24417ed36fc46e = function(arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbg__wbg_cb_unref_87dfb5aaa0cbcea7 = function(arg0) {
        arg0._wbg_cb_unref();
    };
    imports.wbg.__wbg_attachShader_bc2b53790fd12d3a = function(arg0, arg1, arg2) {
        arg0.attachShader(arg1, arg2);
    };
    imports.wbg.__wbg_bindBuffer_110b128c65a97376 = function(arg0, arg1, arg2) {
        arg0.bindBuffer(arg1 >>> 0, arg2);
    };
    imports.wbg.__wbg_blendFunc_5eed6dc03a180da2 = function(arg0, arg1, arg2) {
        arg0.blendFunc(arg1 >>> 0, arg2 >>> 0);
    };
    imports.wbg.__wbg_bufferData_ac5c7900b06f1517 = function(arg0, arg1, arg2, arg3) {
        arg0.bufferData(arg1 >>> 0, arg2, arg3 >>> 0);
    };
    imports.wbg.__wbg_call_3020136f7a2d6e44 = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.call(arg1, arg2);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_call_78f94eb02ec7f9b2 = function() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
        const ret = arg0.call(arg1, arg2, arg3, arg4);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_call_abb4ff46ce38be40 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.call(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_call_c8baa5c5e72d274e = function() { return handleError(function (arg0, arg1, arg2, arg3) {
        const ret = arg0.call(arg1, arg2, arg3);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_clearColor_fe8de7e582b77e40 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.clearColor(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_clear_52caf9271911674b = function(arg0, arg1) {
        arg0.clear(arg1 >>> 0);
    };
    imports.wbg.__wbg_compileShader_ac0bf6f0837881c3 = function(arg0, arg1) {
        arg0.compileShader(arg1);
    };
    imports.wbg.__wbg_createBuffer_8601b8ec330ab49d = function(arg0) {
        const ret = arg0.createBuffer();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createProgram_023ba0fc6ff6efd6 = function(arg0) {
        const ret = arg0.createProgram();
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_createShader_4626088b63c33727 = function(arg0, arg1) {
        const ret = arg0.createShader(arg1 >>> 0);
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_crypto_574e78ad8b13b65f = function(arg0) {
        const ret = arg0.crypto;
        return ret;
    };
    imports.wbg.__wbg_deleteShader_c686dd351de5a068 = function(arg0, arg1) {
        arg0.deleteShader(arg1);
    };
    imports.wbg.__wbg_drawArrays_c106ebe0234971d4 = function(arg0, arg1, arg2, arg3) {
        arg0.drawArrays(arg1 >>> 0, arg2, arg3);
    };
    imports.wbg.__wbg_enableVertexAttribArray_def9952d8426be95 = function(arg0, arg1) {
        arg0.enableVertexAttribArray(arg1 >>> 0);
    };
    imports.wbg.__wbg_enable_52598759008d46ee = function(arg0, arg1) {
        arg0.enable(arg1 >>> 0);
    };
    imports.wbg.__wbg_getAttribLocation_5917620b3c9497e6 = function(arg0, arg1, arg2, arg3) {
        const ret = arg0.getAttribLocation(arg1, getStringFromWasm0(arg2, arg3));
        return ret;
    };
    imports.wbg.__wbg_getContext_01f42b234e833f0a = function() { return handleError(function (arg0, arg1, arg2) {
        const ret = arg0.getContext(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    }, arguments) };
    imports.wbg.__wbg_getProgramInfoLog_ea3064b153e4542a = function(arg0, arg1, arg2) {
        const ret = arg1.getProgramInfoLog(arg2);
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_getProgramParameter_ff1aee3815d6a8f9 = function(arg0, arg1, arg2) {
        const ret = arg0.getProgramParameter(arg1, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_getRandomValues_b8f5dbd5f3995a9e = function() { return handleError(function (arg0, arg1) {
        arg0.getRandomValues(arg1);
    }, arguments) };
    imports.wbg.__wbg_getShaderInfoLog_1affea8c74bd191c = function(arg0, arg1, arg2) {
        const ret = arg1.getShaderInfoLog(arg2);
        var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len1 = WASM_VECTOR_LEN;
        getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
        getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
    };
    imports.wbg.__wbg_getShaderParameter_1f86483b99db3dcc = function(arg0, arg1, arg2) {
        const ret = arg0.getShaderParameter(arg1, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_getUniformLocation_2a4ddf8dd8285373 = function(arg0, arg1, arg2, arg3) {
        const ret = arg0.getUniformLocation(arg1, getStringFromWasm0(arg2, arg3));
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_height_a07787f693c253d2 = function(arg0) {
        const ret = arg0.height;
        return ret;
    };
    imports.wbg.__wbg_instanceof_WebGlRenderingContext_f2a36a2d31de185f = function(arg0) {
        let result;
        try {
            result = arg0 instanceof WebGLRenderingContext;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_instanceof_Window_b5cf7783caa68180 = function(arg0) {
        let result;
        try {
            result = arg0 instanceof Window;
        } catch (_) {
            result = false;
        }
        const ret = result;
        return ret;
    };
    imports.wbg.__wbg_length_22ac23eaec9d8053 = function(arg0) {
        const ret = arg0.length;
        return ret;
    };
    imports.wbg.__wbg_linkProgram_2f770464e69099dc = function(arg0, arg1) {
        arg0.linkProgram(arg1);
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
    imports.wbg.__wbg_randomFillSync_ac0988aba3254290 = function() { return handleError(function (arg0, arg1) {
        arg0.randomFillSync(arg1);
    }, arguments) };
    imports.wbg.__wbg_requestAnimationFrame_994dc4ebde22b8d9 = function() { return handleError(function (arg0, arg1) {
        const ret = arg0.requestAnimationFrame(arg1);
        return ret;
    }, arguments) };
    imports.wbg.__wbg_require_60cc747a6bc5215a = function() { return handleError(function () {
        const ret = module.require;
        return ret;
    }, arguments) };
    imports.wbg.__wbg_shaderSource_8a7a30baeaf655d5 = function(arg0, arg1, arg2, arg3) {
        arg0.shaderSource(arg1, getStringFromWasm0(arg2, arg3));
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_769e6b65d6557335 = function() {
        const ret = typeof global === 'undefined' ? null : global;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_GLOBAL_THIS_60cf02db4de8e1c1 = function() {
        const ret = typeof globalThis === 'undefined' ? null : globalThis;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_SELF_08f5a74c69739274 = function() {
        const ret = typeof self === 'undefined' ? null : self;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_static_accessor_WINDOW_a8924b26aa92d024 = function() {
        const ret = typeof window === 'undefined' ? null : window;
        return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
    };
    imports.wbg.__wbg_subarray_845f2f5bce7d061a = function(arg0, arg1, arg2) {
        const ret = arg0.subarray(arg1 >>> 0, arg2 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_uniform1f_058417475b9966c8 = function(arg0, arg1, arg2) {
        arg0.uniform1f(arg1, arg2);
    };
    imports.wbg.__wbg_uniform2f_d0f7977f54aed068 = function(arg0, arg1, arg2, arg3) {
        arg0.uniform2f(arg1, arg2, arg3);
    };
    imports.wbg.__wbg_uniform3f_dd2e42055b857ce7 = function(arg0, arg1, arg2, arg3, arg4) {
        arg0.uniform3f(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_useProgram_142dd02d095f80f1 = function(arg0, arg1) {
        arg0.useProgram(arg1);
    };
    imports.wbg.__wbg_versions_c01dfd4722a88165 = function(arg0) {
        const ret = arg0.versions;
        return ret;
    };
    imports.wbg.__wbg_vertexAttribPointer_5c516f4c675103bf = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        arg0.vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
    };
    imports.wbg.__wbg_width_dd0cfe94d42f5143 = function(arg0) {
        const ret = arg0.width;
        return ret;
    };
    imports.wbg.__wbindgen_cast_2241b6af4c4b2941 = function(arg0, arg1) {
        // Cast intrinsic for `Ref(String) -> Externref`.
        const ret = getStringFromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_cast_675167efb57f5149 = function(arg0, arg1) {
        // Cast intrinsic for `Closure(Closure { dtor_idx: 5, function: Function { arguments: [F64], shim_idx: 6, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
        const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h05f94b76025885e2, wasm_bindgen__convert__closures_____invoke__hc1ac6a06dab0941c);
        return ret;
    };
    imports.wbg.__wbindgen_cast_cb9088102bce6b30 = function(arg0, arg1) {
        // Cast intrinsic for `Ref(Slice(U8)) -> NamedExternref("Uint8Array")`.
        const ret = getArrayU8FromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_cast_cd07b1914aa3d62c = function(arg0, arg1) {
        // Cast intrinsic for `Ref(Slice(F32)) -> NamedExternref("Float32Array")`.
        const ret = getArrayF32FromWasm0(arg0, arg1);
        return ret;
    };
    imports.wbg.__wbindgen_cast_d6cd19b81560fd6e = function(arg0) {
        // Cast intrinsic for `F64 -> Externref`.
        const ret = arg0;
        return ret;
    };
    imports.wbg.__wbindgen_init_externref_table = function() {
        const table = wasm.__wbindgen_externrefs;
        const offset = table.grow(4);
        table.set(0, undefined);
        table.set(offset + 0, undefined);
        table.set(offset + 1, null);
        table.set(offset + 2, true);
        table.set(offset + 3, false);
    };

    return imports;
}

function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    __wbg_init.__wbindgen_wasm_module = module;
    cachedDataViewMemory0 = null;
    cachedFloat32ArrayMemory0 = null;
    cachedUint8ArrayMemory0 = null;


    wasm.__wbindgen_start();
    return wasm;
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (typeof module !== 'undefined') {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (typeof module_or_path !== 'undefined') {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (typeof module_or_path === 'undefined') {
        module_or_path = new URL('tetris_core_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync };
export default __wbg_init;
