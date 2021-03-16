import * as wasm from './synth_bg.wasm';

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

const lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;

let cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetFloat64Memory0 = null;
function getFloat64Memory0() {
    if (cachegetFloat64Memory0 === null || cachegetFloat64Memory0.buffer !== wasm.memory.buffer) {
        cachegetFloat64Memory0 = new Float64Array(wasm.memory.buffer);
    }
    return cachegetFloat64Memory0;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return  `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for(let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

const lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;

let cachedTextEncoder = new lTextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
    return cachedTextEncoder.encodeInto(arg, view);
}
    : function (arg, view) {
    const buf = cachedTextEncoder.encode(arg);
    view.set(buf);
    return {
        read: arg.length,
        written: buf.length
    };
});

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

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
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
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
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_24(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h8f3865a04294e471(arg0, arg1);
}

function __wbg_adapter_27(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1cb401a3ba7a90a6(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_30(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6def8933b38c890f(arg0, arg1);
}

function __wbg_adapter_33(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1cb401a3ba7a90a6(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_36(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1cb401a3ba7a90a6(arg0, arg1, addHeapObject(arg2));
}

function __wbg_adapter_39(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1cb401a3ba7a90a6(arg0, arg1, addHeapObject(arg2));
}

/**
* @param {string} canvas_id
*/
export function start(canvas_id) {
    var ptr0 = passStringToWasm0(canvas_id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.start(ptr0, len0);
}

function getArrayU8FromWasm0(ptr, len) {
    return getUint8Memory0().subarray(ptr / 1, ptr / 1 + len);
}

function handleError(f) {
    return function () {
        try {
            return f.apply(this, arguments);

        } catch (e) {
            wasm.__wbindgen_exn_store(addHeapObject(e));
        }
    };
}

export const __wbindgen_object_drop_ref = function(arg0) {
    takeObject(arg0);
};

export const __wbindgen_string_new = function(arg0, arg1) {
    var ret = getStringFromWasm0(arg0, arg1);
    return addHeapObject(ret);
};

export const __wbindgen_cb_drop = function(arg0) {
    const obj = takeObject(arg0).original;
    if (obj.cnt-- == 1) {
        obj.a = 0;
        return true;
    }
    var ret = false;
    return ret;
};

export const __wbg_instanceof_WebGl2RenderingContext_acac10ed74c696cb = function(arg0) {
    var ret = getObject(arg0) instanceof WebGL2RenderingContext;
    return ret;
};

export const __wbg_drawingBufferWidth_88ef8b0a2794d701 = function(arg0) {
    var ret = getObject(arg0).drawingBufferWidth;
    return ret;
};

export const __wbg_drawingBufferHeight_14333bdf86372817 = function(arg0) {
    var ret = getObject(arg0).drawingBufferHeight;
    return ret;
};

export const __wbg_bufferData_80963d2bd1ecb1bc = function(arg0, arg1, arg2, arg3) {
    getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
};

export const __wbg_texImage2D_a5dad82b8f689bbd = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
    getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9 === 0 ? undefined : getArrayU8FromWasm0(arg9, arg10));
});

export const __wbg_activeTexture_32edab6336bd38a9 = function(arg0, arg1) {
    getObject(arg0).activeTexture(arg1 >>> 0);
};

export const __wbg_attachShader_5d53b7b00823cafb = function(arg0, arg1, arg2) {
    getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
};

export const __wbg_bindBuffer_4a7874f09df12419 = function(arg0, arg1, arg2) {
    getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
};

export const __wbg_bindTexture_d659843380f373b5 = function(arg0, arg1, arg2) {
    getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
};

export const __wbg_blendFunc_8bd5998b54c12fd3 = function(arg0, arg1, arg2) {
    getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);
};

export const __wbg_clear_25e035ed3961f1c6 = function(arg0, arg1) {
    getObject(arg0).clear(arg1 >>> 0);
};

export const __wbg_clearColor_fc22409197a5bd68 = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
};

export const __wbg_compileShader_f7e245515fa1405d = function(arg0, arg1) {
    getObject(arg0).compileShader(getObject(arg1));
};

export const __wbg_createBuffer_4302ddbcbfc99048 = function(arg0) {
    var ret = getObject(arg0).createBuffer();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_createProgram_128698dd90ec070d = function(arg0) {
    var ret = getObject(arg0).createProgram();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_createShader_26e4f959d5d64d80 = function(arg0, arg1) {
    var ret = getObject(arg0).createShader(arg1 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_createTexture_8ba2e566eb313fcf = function(arg0) {
    var ret = getObject(arg0).createTexture();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_disable_827be6d0f77447e1 = function(arg0, arg1) {
    getObject(arg0).disable(arg1 >>> 0);
};

export const __wbg_drawElements_c109bfea7998fd99 = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).drawElements(arg1 >>> 0, arg2, arg3 >>> 0, arg4);
};

export const __wbg_enable_65590f4951fd0112 = function(arg0, arg1) {
    getObject(arg0).enable(arg1 >>> 0);
};

export const __wbg_enableVertexAttribArray_413ef49912a23f9e = function(arg0, arg1) {
    getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
};

export const __wbg_getAttribLocation_422da253e02c78ac = function(arg0, arg1, arg2, arg3) {
    var ret = getObject(arg0).getAttribLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
    return ret;
};

export const __wbg_getProgramInfoLog_f8f65be65281f691 = function(arg0, arg1, arg2) {
    var ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbg_getProgramParameter_b949ba1d9662f6a2 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
};

export const __wbg_getShaderInfoLog_5412e8bc642139e8 = function(arg0, arg1, arg2) {
    var ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbg_getShaderParameter_cced0ff8ba83f3e7 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
};

export const __wbg_getUniformLocation_472b7459010900a5 = function(arg0, arg1, arg2, arg3) {
    var ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_linkProgram_370ed11b34456c89 = function(arg0, arg1) {
    getObject(arg0).linkProgram(getObject(arg1));
};

export const __wbg_pixelStorei_d2b5d30ea97fc3ba = function(arg0, arg1, arg2) {
    getObject(arg0).pixelStorei(arg1 >>> 0, arg2);
};

export const __wbg_scissor_1f78ef0050a93516 = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).scissor(arg1, arg2, arg3, arg4);
};

export const __wbg_shaderSource_96ace5133c032f2f = function(arg0, arg1, arg2, arg3) {
    getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
};

export const __wbg_texParameteri_c0b2b665319f6a16 = function(arg0, arg1, arg2, arg3) {
    getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
};

export const __wbg_uniform1i_a1e8f5ad954fa6b5 = function(arg0, arg1, arg2) {
    getObject(arg0).uniform1i(getObject(arg1), arg2);
};

export const __wbg_uniform2f_ab7c909be2949448 = function(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform2f(getObject(arg1), arg2, arg3);
};

export const __wbg_useProgram_b1cc885b00b8f52c = function(arg0, arg1) {
    getObject(arg0).useProgram(getObject(arg1));
};

export const __wbg_vertexAttribPointer_3bb013e284cd07bf = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
};

export const __wbg_viewport_86b156d5858adab9 = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).viewport(arg1, arg2, arg3, arg4);
};

export const __wbg_instanceof_Window_fa4595281eb5ba83 = function(arg0) {
    var ret = getObject(arg0) instanceof Window;
    return ret;
};

export const __wbg_document_d8cce4c1031c64eb = function(arg0) {
    var ret = getObject(arg0).document;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_location_f8de588551329bf4 = function(arg0) {
    var ret = getObject(arg0).location;
    return addHeapObject(ret);
};

export const __wbg_innerWidth_aab6ec3242dff39e = handleError(function(arg0) {
    var ret = getObject(arg0).innerWidth;
    return addHeapObject(ret);
});

export const __wbg_innerHeight_7e514d9823f7864e = handleError(function(arg0) {
    var ret = getObject(arg0).innerHeight;
    return addHeapObject(ret);
});

export const __wbg_devicePixelRatio_dc4405584b5cb63f = function(arg0) {
    var ret = getObject(arg0).devicePixelRatio;
    return ret;
};

export const __wbg_performance_800ff37c906b5f3b = function(arg0) {
    var ret = getObject(arg0).performance;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_localStorage_a79a5d8ee7487fcb = handleError(function(arg0) {
    var ret = getObject(arg0).localStorage;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

export const __wbg_open_c8c0fa97b521eb3d = handleError(function(arg0, arg1, arg2, arg3, arg4) {
    var ret = getObject(arg0).open(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

export const __wbg_requestAnimationFrame_ef037dc409649fbf = handleError(function(arg0, arg1) {
    var ret = getObject(arg0).requestAnimationFrame(getObject(arg1));
    return ret;
});

export const __wbg_setInterval_f3122aab24298341 = handleError(function(arg0, arg1, arg2) {
    var ret = getObject(arg0).setInterval(getObject(arg1), arg2);
    return ret;
});

export const __wbg_top_8ad808e9af99b597 = function(arg0) {
    var ret = getObject(arg0).top;
    return ret;
};

export const __wbg_left_1761db11f4e18020 = function(arg0) {
    var ret = getObject(arg0).left;
    return ret;
};

export const __wbg_getItem_3fc9a85a5c86c097 = handleError(function(arg0, arg1, arg2, arg3) {
    var ret = getObject(arg1).getItem(getStringFromWasm0(arg2, arg3));
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
});

export const __wbg_setItem_99592651ffc703f6 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).setItem(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
});

export const __wbg_setProperty_881bd3ab228526b3 = handleError(function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).setProperty(getStringFromWasm0(arg1, arg2), getStringFromWasm0(arg3, arg4));
});

export const __wbg_addEventListener_9b66d58c2a9ba39a = handleError(function(arg0, arg1, arg2, arg3) {
    getObject(arg0).addEventListener(getStringFromWasm0(arg1, arg2), getObject(arg3));
});

export const __wbg_hash_f1a1e37355e44338 = handleError(function(arg0, arg1) {
    var ret = getObject(arg1).hash;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
});

export const __wbg_clientX_9708d75fb355c54c = function(arg0) {
    var ret = getObject(arg0).clientX;
    return ret;
};

export const __wbg_clientY_73c641398c4e5a64 = function(arg0) {
    var ret = getObject(arg0).clientY;
    return ret;
};

export const __wbg_button_75c49405361e7da4 = function(arg0) {
    var ret = getObject(arg0).button;
    return ret;
};

export const __wbg_deltaX_1eb557c7e0056588 = function(arg0) {
    var ret = getObject(arg0).deltaX;
    return ret;
};

export const __wbg_deltaY_1ab9240217b48aa3 = function(arg0) {
    var ret = getObject(arg0).deltaY;
    return ret;
};

export const __wbg_now_9f22124bc74da886 = function(arg0) {
    var ret = getObject(arg0).now();
    return ret;
};

export const __wbg_body_52b3f453148fd124 = function(arg0) {
    var ret = getObject(arg0).body;
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_getElementById_aeb1b7331ed88a97 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_getBoundingClientRect_6c4d67366fb08e31 = function(arg0) {
    var ret = getObject(arg0).getBoundingClientRect();
    return addHeapObject(ret);
};

export const __wbg_instanceof_WebGlRenderingContext_bf4c1c161cce63d8 = function(arg0) {
    var ret = getObject(arg0) instanceof WebGLRenderingContext;
    return ret;
};

export const __wbg_drawingBufferWidth_b31cf22e5817f6db = function(arg0) {
    var ret = getObject(arg0).drawingBufferWidth;
    return ret;
};

export const __wbg_drawingBufferHeight_985e8a982ad94907 = function(arg0) {
    var ret = getObject(arg0).drawingBufferHeight;
    return ret;
};

export const __wbg_bufferData_1e028cc0639f0264 = function(arg0, arg1, arg2, arg3) {
    getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
};

export const __wbg_texImage2D_b1d95ccb3f8fd616 = handleError(function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10) {
    getObject(arg0).texImage2D(arg1 >>> 0, arg2, arg3, arg4, arg5, arg6, arg7 >>> 0, arg8 >>> 0, arg9 === 0 ? undefined : getArrayU8FromWasm0(arg9, arg10));
});

export const __wbg_activeTexture_ccd864030355beba = function(arg0, arg1) {
    getObject(arg0).activeTexture(arg1 >>> 0);
};

export const __wbg_attachShader_176dfde48c626eb8 = function(arg0, arg1, arg2) {
    getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
};

export const __wbg_bindBuffer_aff83e0a72ebe9c6 = function(arg0, arg1, arg2) {
    getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
};

export const __wbg_bindTexture_3c4cdd29edc870f9 = function(arg0, arg1, arg2) {
    getObject(arg0).bindTexture(arg1 >>> 0, getObject(arg2));
};

export const __wbg_blendFunc_ae81cbf4e0169885 = function(arg0, arg1, arg2) {
    getObject(arg0).blendFunc(arg1 >>> 0, arg2 >>> 0);
};

export const __wbg_clear_4026459dc218d806 = function(arg0, arg1) {
    getObject(arg0).clear(arg1 >>> 0);
};

export const __wbg_clearColor_3feff7be5983725c = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
};

export const __wbg_compileShader_b154f866a37ef240 = function(arg0, arg1) {
    getObject(arg0).compileShader(getObject(arg1));
};

export const __wbg_createBuffer_9cd00017c8012ded = function(arg0) {
    var ret = getObject(arg0).createBuffer();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_createProgram_1dc1d5b4f815c74e = function(arg0) {
    var ret = getObject(arg0).createProgram();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_createShader_a568ae9716cf79bd = function(arg0, arg1) {
    var ret = getObject(arg0).createShader(arg1 >>> 0);
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_createTexture_9165d6614a3f8c26 = function(arg0) {
    var ret = getObject(arg0).createTexture();
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_disable_71a7779d266ab83f = function(arg0, arg1) {
    getObject(arg0).disable(arg1 >>> 0);
};

export const __wbg_drawElements_567b7125cfe5debf = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).drawElements(arg1 >>> 0, arg2, arg3 >>> 0, arg4);
};

export const __wbg_enable_8c08778f17ea82d3 = function(arg0, arg1) {
    getObject(arg0).enable(arg1 >>> 0);
};

export const __wbg_enableVertexAttribArray_19841ca8c10ee785 = function(arg0, arg1) {
    getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
};

export const __wbg_getAttribLocation_3cbba362123e3451 = function(arg0, arg1, arg2, arg3) {
    var ret = getObject(arg0).getAttribLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
    return ret;
};

export const __wbg_getProgramInfoLog_b3af1c1f2f050ac5 = function(arg0, arg1, arg2) {
    var ret = getObject(arg1).getProgramInfoLog(getObject(arg2));
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbg_getProgramParameter_15c77e6ded344978 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
};

export const __wbg_getShaderInfoLog_62bc93f21372bbdb = function(arg0, arg1, arg2) {
    var ret = getObject(arg1).getShaderInfoLog(getObject(arg2));
    var ptr0 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbg_getShaderParameter_b652420e47ea83c3 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0);
    return addHeapObject(ret);
};

export const __wbg_getUniformLocation_0e74513fa8e0fcef = function(arg0, arg1, arg2, arg3) {
    var ret = getObject(arg0).getUniformLocation(getObject(arg1), getStringFromWasm0(arg2, arg3));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_linkProgram_0a51f6ca8e067ba7 = function(arg0, arg1) {
    getObject(arg0).linkProgram(getObject(arg1));
};

export const __wbg_scissor_53f081ef55d3977d = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).scissor(arg1, arg2, arg3, arg4);
};

export const __wbg_shaderSource_9f03812e74c7504e = function(arg0, arg1, arg2, arg3) {
    getObject(arg0).shaderSource(getObject(arg1), getStringFromWasm0(arg2, arg3));
};

export const __wbg_texParameteri_26de60b40766928f = function(arg0, arg1, arg2, arg3) {
    getObject(arg0).texParameteri(arg1 >>> 0, arg2 >>> 0, arg3);
};

export const __wbg_uniform1i_6a282c117216b6ef = function(arg0, arg1, arg2) {
    getObject(arg0).uniform1i(getObject(arg1), arg2);
};

export const __wbg_uniform2f_2c62de1e5acc87da = function(arg0, arg1, arg2, arg3) {
    getObject(arg0).uniform2f(getObject(arg1), arg2, arg3);
};

export const __wbg_useProgram_9174cae30cc67e4d = function(arg0, arg1) {
    getObject(arg0).useProgram(getObject(arg1));
};

export const __wbg_vertexAttribPointer_f1d73baac9e3b6e9 = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
};

export const __wbg_viewport_f89fe7da7b1e24e2 = function(arg0, arg1, arg2, arg3, arg4) {
    getObject(arg0).viewport(arg1, arg2, arg3, arg4);
};

export const __wbg_log_8485ead621ceded9 = function(arg0) {
    console.log(getObject(arg0));
};

export const __wbg_warn_eb158fa0859088bf = function(arg0) {
    console.warn(getObject(arg0));
};

export const __wbg_style_6840ca6f1c83499f = function(arg0) {
    var ret = getObject(arg0).style;
    return addHeapObject(ret);
};

export const __wbg_pageX_01c8418913761f98 = function(arg0) {
    var ret = getObject(arg0).pageX;
    return ret;
};

export const __wbg_pageY_74d7bf8fe7866455 = function(arg0) {
    var ret = getObject(arg0).pageY;
    return ret;
};

export const __wbg_get_5fbce9875ca09f4a = function(arg0, arg1) {
    var ret = getObject(arg0)[arg1 >>> 0];
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
};

export const __wbg_instanceof_HtmlCanvasElement_c9f334afe4eed430 = function(arg0) {
    var ret = getObject(arg0) instanceof HTMLCanvasElement;
    return ret;
};

export const __wbg_width_726d17d6876631b4 = function(arg0) {
    var ret = getObject(arg0).width;
    return ret;
};

export const __wbg_setwidth_41b2497107faaff7 = function(arg0, arg1) {
    getObject(arg0).width = arg1 >>> 0;
};

export const __wbg_height_5fd8d13e879338d0 = function(arg0) {
    var ret = getObject(arg0).height;
    return ret;
};

export const __wbg_setheight_e15cb9243262e701 = function(arg0, arg1) {
    getObject(arg0).height = arg1 >>> 0;
};

export const __wbg_getContext_d277f710e8035242 = handleError(function(arg0, arg1, arg2) {
    var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addHeapObject(ret);
});

export const __wbg_keyCode_4390925021a31546 = function(arg0) {
    var ret = getObject(arg0).keyCode;
    return ret;
};

export const __wbg_altKey_9ead66021848aa74 = function(arg0) {
    var ret = getObject(arg0).altKey;
    return ret;
};

export const __wbg_ctrlKey_1e3e28894d756523 = function(arg0) {
    var ret = getObject(arg0).ctrlKey;
    return ret;
};

export const __wbg_shiftKey_bf56a659d464f031 = function(arg0) {
    var ret = getObject(arg0).shiftKey;
    return ret;
};

export const __wbg_metaKey_ad3c08fec6e4d71d = function(arg0) {
    var ret = getObject(arg0).metaKey;
    return ret;
};

export const __wbg_isComposing_bb034058b069282b = function(arg0) {
    var ret = getObject(arg0).isComposing;
    return ret;
};

export const __wbg_key_8642816c7d0fb736 = function(arg0, arg1) {
    var ret = getObject(arg1).key;
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbg_preventDefault_2a53c6dce5093030 = function(arg0) {
    getObject(arg0).preventDefault();
};

export const __wbg_stopPropagation_6177617793da16e6 = function(arg0) {
    getObject(arg0).stopPropagation();
};

export const __wbg_touches_335f913c6a1e3a84 = function(arg0) {
    var ret = getObject(arg0).touches;
    return addHeapObject(ret);
};

export const __wbg_call_8487a9f580e47219 = handleError(function(arg0, arg1) {
    var ret = getObject(arg0).call(getObject(arg1));
    return addHeapObject(ret);
});

export const __wbindgen_object_clone_ref = function(arg0) {
    var ret = getObject(arg0);
    return addHeapObject(ret);
};

export const __wbg_newnoargs_179d393e4626fcf7 = function(arg0, arg1) {
    var ret = new Function(getStringFromWasm0(arg0, arg1));
    return addHeapObject(ret);
};

export const __wbg_getHours_1af63180ed11b49d = function(arg0) {
    var ret = getObject(arg0).getHours();
    return ret;
};

export const __wbg_getMilliseconds_bde52242127a5c03 = function(arg0) {
    var ret = getObject(arg0).getMilliseconds();
    return ret;
};

export const __wbg_getMinutes_9c21a27a8849d22a = function(arg0) {
    var ret = getObject(arg0).getMinutes();
    return ret;
};

export const __wbg_getSeconds_c5431d6c48f9ed3c = function(arg0) {
    var ret = getObject(arg0).getSeconds();
    return ret;
};

export const __wbg_new0_8c7faee4e4e8144d = function() {
    var ret = new Date();
    return addHeapObject(ret);
};

export const __wbg_self_eeabd9085c04fc17 = handleError(function() {
    var ret = self.self;
    return addHeapObject(ret);
});

export const __wbg_window_f110c13310da2c8f = handleError(function() {
    var ret = window.window;
    return addHeapObject(ret);
});

export const __wbg_globalThis_a2669bee93faee43 = handleError(function() {
    var ret = globalThis.globalThis;
    return addHeapObject(ret);
});

export const __wbg_global_a5584d717f4d6761 = handleError(function() {
    var ret = global.global;
    return addHeapObject(ret);
});

export const __wbindgen_is_undefined = function(arg0) {
    var ret = getObject(arg0) === undefined;
    return ret;
};

export const __wbg_buffer_e35e010c3ba9f945 = function(arg0) {
    var ret = getObject(arg0).buffer;
    return addHeapObject(ret);
};

export const __wbg_new_5b74a8dd0c5b71ac = function(arg0) {
    var ret = new Int16Array(getObject(arg0));
    return addHeapObject(ret);
};

export const __wbg_new_139e70222494b1ff = function(arg0) {
    var ret = new Uint8Array(getObject(arg0));
    return addHeapObject(ret);
};

export const __wbg_new_fe24eae01e10f223 = function(arg0) {
    var ret = new Float32Array(getObject(arg0));
    return addHeapObject(ret);
};

export const __wbg_subarray_7b811f5c80cb90d3 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};

export const __wbg_subarray_8a52f1c1a11c02a8 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};

export const __wbg_subarray_3c6f7cfb4edcc351 = function(arg0, arg1, arg2) {
    var ret = getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0);
    return addHeapObject(ret);
};

export const __wbg_instanceof_Memory_7ec765ef65ff7aaa = function(arg0) {
    var ret = getObject(arg0) instanceof WebAssembly.Memory;
    return ret;
};

export const __wbindgen_number_get = function(arg0, arg1) {
    const obj = getObject(arg1);
    var ret = typeof(obj) === 'number' ? obj : undefined;
    getFloat64Memory0()[arg0 / 8 + 1] = isLikeNone(ret) ? 0 : ret;
    getInt32Memory0()[arg0 / 4 + 0] = !isLikeNone(ret);
};

export const __wbindgen_boolean_get = function(arg0) {
    const v = getObject(arg0);
    var ret = typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
    return ret;
};

export const __wbindgen_debug_string = function(arg0, arg1) {
    var ret = debugString(getObject(arg1));
    var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    getInt32Memory0()[arg0 / 4 + 1] = len0;
    getInt32Memory0()[arg0 / 4 + 0] = ptr0;
};

export const __wbindgen_throw = function(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export const __wbindgen_rethrow = function(arg0) {
    throw takeObject(arg0);
};

export const __wbindgen_memory = function() {
    var ret = wasm.memory;
    return addHeapObject(ret);
};

export const __wbindgen_closure_wrapper1367 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 734, __wbg_adapter_24);
    return addHeapObject(ret);
};

export const __wbindgen_closure_wrapper1368 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 734, __wbg_adapter_27);
    return addHeapObject(ret);
};

export const __wbindgen_closure_wrapper1370 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 734, __wbg_adapter_30);
    return addHeapObject(ret);
};

export const __wbindgen_closure_wrapper1373 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 734, __wbg_adapter_33);
    return addHeapObject(ret);
};

export const __wbindgen_closure_wrapper1375 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 734, __wbg_adapter_36);
    return addHeapObject(ret);
};

export const __wbindgen_closure_wrapper1377 = function(arg0, arg1, arg2) {
    var ret = makeMutClosure(arg0, arg1, 734, __wbg_adapter_39);
    return addHeapObject(ret);
};

