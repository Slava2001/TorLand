/* tslint:disable */
/* eslint-disable */
/**
 * @param {string} input
 * @returns {string}
 */
export function compile(input: string): string;
/**
 * @param {string} input
 * @returns {string}
 */
export function decompile(input: string): string;
export class WorldWraper {
  free(): void;
  /**
   * @param {number} size
   * @param {string} bot
   * @returns {WorldWraper}
   */
  static new(size: number, bot: string): WorldWraper;
  update(): void;
  /**
   * @param {CanvasRenderingContext2D} ctx
   */
  draw(ctx: CanvasRenderingContext2D): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_worldwraper_free: (a: number, b: number) => void;
  readonly worldwraper_new: (a: number, b: number, c: number) => number;
  readonly worldwraper_update: (a: number) => void;
  readonly worldwraper_draw: (a: number, b: number) => void;
  readonly compile: (a: number, b: number, c: number) => void;
  readonly decompile: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
