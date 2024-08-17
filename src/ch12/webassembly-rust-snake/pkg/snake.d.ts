/* tslint:disable */
/* eslint-disable */
/**
*/
export class Game {
  free(): void;
/**
* @returns {Game}
*/
  static new(): Game;
/**
* @param {number} elapsed_milliseconds
*/
  tick(elapsed_milliseconds: number): void;
/**
* @param {CanvasRenderingContext2D} ctx
*/
  render(ctx: CanvasRenderingContext2D): void;
/**
* @returns {number}
*/
  width(): number;
/**
* @returns {number}
*/
  height(): number;
/**
* @param {number} x
* @param {number} y
*/
  click(x: number, y: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_game_free: (a: number) => void;
  readonly game_new: () => number;
  readonly game_tick: (a: number, b: number) => void;
  readonly game_render: (a: number, b: number) => void;
  readonly game_width: (a: number) => number;
  readonly game_height: (a: number) => number;
  readonly game_click: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
