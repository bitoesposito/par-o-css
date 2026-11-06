// par-o-css compiler for browsers. Call `init()` once before the rest.
import initWasm, * as wasm from './wasm/web/parocss_wasm.js';
import { createApi } from './lib/api.js';

const api = createApi(wasm);
let ready = null;
let loaded = false;

export function init(input) {
  ready ??= initWasm(input === undefined ? undefined : { module_or_path: input }).then(() => {
    loaded = true;
  });
  return ready;
}

function loadedApi() {
  if (!loaded) throw new Error('par-o-css/web: await init() before using the compiler');
  return api;
}

export const compile = (source, options) => loadedApi().compile(source, options);
export const scan = (text) => loadedApi().scan(text);
export const suggestion = (name) => loadedApi().suggestion(name);
export const isClass = (name) => loadedApi().isClass(name);
export const classCss = (classes) => loadedApi().classCss(classes);
export const names = () => loadedApi().names();
export const version = () => loadedApi().version();
export { formatError } from './lib/api.js';
