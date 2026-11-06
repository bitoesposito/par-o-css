import { dirname, resolve } from 'node:path';
import htmlemmurt from 'htmlemmurt/vite';
import fratmscript from 'fratmscript/vite';
import parocss from 'par-o-css/vite';

// par-o-css/vite takes every .cess id, `?raw` too, and compiles it to CSS.
// The examples are text for the editors: resolve them before it sees them.
const cessRaw = {
  name: 'cess-raw',
  enforce: 'pre',
  resolveId: (source, importer) => (/\.cess\?raw$/.test(source) && importer ? resolve(dirname(importer), source) : null),
};

export default {
  // Relative URLs, so the build works under github.io/par-o-css/
  base: './',
  plugins: [cessRaw, htmlemmurt(), fratmscript(), parocss()],
  // The examples in esempi/ are pages too: only index is the app
  // CodeMirror with the HTML and CSS languages is most of the bundle
  build: { rollupOptions: { input: 'index.html' }, chunkSizeWarningLimit: 700 },
  // The compilers find their .wasm next to their module: do not pre-bundle them
  optimizeDeps: { exclude: ['htmlemmurt', 'par-o-css'] },
  // The examples come from the repository's examples/ folder
  server: { fs: { allow: ['../..'] } },
};
