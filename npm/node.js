// Node.js entry for @ruvnet/bmssp.
//
// The wasm-pack `web` glue loads the module with fetch(new URL(..., import.meta.url)),
// and Node's fetch() rejects file: URLs. This entry reads the .wasm from disk instead,
// so `await init()` works the same way in Node as it does in a browser.
import __wbg_init, { initSync } from './bmssp_rust.js';

export * from './bmssp_rust.js';
export { initSync };

const WASM_URL = new URL('./bmssp_rust_bg.wasm', import.meta.url);

function isFileUrl(value) {
  if (value instanceof URL) return value.protocol === 'file:';
  return typeof value === 'string' && value.startsWith('file:');
}

export default async function init(input) {
  let source = input;
  if (source !== null && typeof source === 'object' && Object.getPrototypeOf(source) === Object.prototype) {
    source = source.module_or_path;
  }
  if (source === undefined || isFileUrl(source)) {
    const { readFile } = await import('node:fs/promises');
    const bytes = await readFile(source === undefined ? WASM_URL : new URL(source));
    return __wbg_init({ module_or_path: bytes });
  }
  return __wbg_init(input);
}
