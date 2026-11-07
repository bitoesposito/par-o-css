// Turns the raw responses of the WASM bindings into the public API.

/** @param {any} wasm */
export function createApi(wasm) {
  function compile(source, options = {}) {
    const { filename = 'stile.cess', read, classes = [] } = options;
    // The bindings take a missing file as anything that is not a string
    const response = wasm.compile(
      String(source),
      filename,
      read ? (path) => read(path) : undefined,
      [...classes].map(String),
    );
    return { css: response.css, dependencies: response.dependencies, errors: response.errors };
  }

  return {
    compile,
    format: (source) => wasm.format(String(source)) ?? null,
    vocabulary: () => JSON.parse(wasm.vocabulary()),
    scan: (text) => wasm.scan(String(text)),
    suggestion: (name) => wasm.suggestion(String(name)) ?? null,
    isClass: (name) => wasm.isClass(String(name)),
    classCss: (classes) => wasm.classCss([...classes].map(String)),
    names: () => JSON.parse(wasm.names()),
    version: wasm.version,
  };
}

/** `file:line:column: message`, then the hint on its own line. */
export function formatError(error) {
  const hint = error.suggestion ? `\n  ${error.suggestion}` : '';
  return `${error.file}:${error.line}:${error.column}: ${error.message}${hint}`;
}
