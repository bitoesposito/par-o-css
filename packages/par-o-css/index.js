// par-o-css compiler for Node.js (and bundlers that run in Node).
import { existsSync, readFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { join } from 'node:path';
import { createApi } from './lib/api.js';
import { fileClasses, pageFiles } from './lib/files.js';

const wasm = createRequire(import.meta.url)('./wasm/node/parocss_wasm.js');
const api = createApi(wasm);

export const { format, vocabulary, scan, suggestion, isClass, classCss, names, version } = api;
export { formatError } from './lib/api.js';

/**
 * Compiles a .cess file. Files named by `@chiamm` are read from disk: paths
 * starting with `/` that are not absolute paths on disk are taken from `root`.
 */
export function compile(source, options = {}) {
  const { root = process.cwd(), read = (path) => readFrom(root, path), ...rest } = options;
  return api.compile(source, { ...rest, read });
}

/** The words that may be classes in the page files under `root`. */
export function findClasses(root = process.cwd()) {
  const found = new Set();
  for (const file of pageFiles(root)) for (const word of fileClasses(file, scan)) found.add(word);
  return found;
}

function readFrom(root, path) {
  const file = path.startsWith('/') && !existsSync(path) ? join(root, path) : path;
  return existsSync(file) ? readFileSync(file, 'utf8') : null;
}
