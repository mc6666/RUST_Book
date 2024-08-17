import * as wasm from "./add_bg.wasm";
import { __wbg_set_wasm } from "./add_bg.js";
__wbg_set_wasm(wasm);
export * from "./add_bg.js";
