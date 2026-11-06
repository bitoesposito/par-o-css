// Vite plugin: .cess files are stylesheets. Import one from a module
// (`import './stile.cess'`) or link it from a page (`<link rel="stylesheet"
// href="stile.cess">`, in htmlemmurt `<attacca rapporto="stile" addo="stile.cess">`):
// it is compiled with the classes of the project's page files, and again
// when a page gets a class it did not have.
import { existsSync, readFileSync } from 'node:fs';
import { dirname, isAbsolute, join, relative, resolve } from 'node:path';
import { compile, formatError, scan } from './index.js';
import { PAGES, fileClasses, pageFiles } from './lib/files.js';

// Vite's CSS pipeline takes an id ending in `lang.css`, as for Vue's <style>
const QUERY = '?parocss&lang.css';
const CESS = /\.cess(?:$|\?)/;

const clean = (id) => id.split('?', 1)[0];

export default function parocss() {
  let root = process.cwd();
  let classes = new Set();

  const scanAll = () => {
    classes = new Set();
    for (const file of pageFiles(root)) for (const word of fileClasses(file, scan)) classes.add(word);
  };

  return {
    name: 'par-o-css',
    enforce: 'pre',

    configResolved(config) {
      root = config.root;
    },

    buildStart() {
      scanAll();
    },

    transformIndexHtml: {
      order: 'pre',
      handler(html) {
        // A <link> to a .cess goes through the CSS pipeline like a .css
        return html.replace(/(<link\b[^>]*?\bhref=)(["'])([^"'?#]+\.cess)\2/gi, `$1$2$3${QUERY}$2`);
      },
    },

    resolveId(source, importer) {
      if (!CESS.test(source)) return null;
      const path = clean(source);
      let file;
      if (path.startsWith('/') && !existsSync(path)) file = join(root, path);
      else if (isAbsolute(path)) file = path;
      else file = resolve(importer ? dirname(clean(importer)) : root, path);
      return file + QUERY;
    },

    load(id) {
      if (!id.endsWith(QUERY)) return null;
      const file = clean(id);
      const source = readFileSync(file, 'utf8');
      const { css, dependencies, errors } = compile(source, { filename: file, root, classes });
      if (errors.length > 0) this.error(errorFor(errors[0], root));
      for (const dependency of dependencies) this.addWatchFile(resolve(root, dependency));
      return css;
    },

    handleHotUpdate({ file, modules, server }) {
      if (!PAGES.test(file)) return;
      let added = false;
      for (const word of fileClasses(file, scan)) {
        if (!classes.has(word)) {
          classes.add(word);
          added = true;
        }
      }
      if (!added) return;
      const sheets = [...server.moduleGraph.idToModuleMap.values()].filter((m) => m.id?.endsWith(QUERY));
      for (const sheet of sheets) server.moduleGraph.invalidateModule(sheet);
      return [...modules, ...sheets];
    },
  };
}

function errorFor(error, root) {
  return {
    message: formatError({ ...error, file: relative(root, error.file) }),
    id: error.file,
    loc: { file: error.file, line: error.line, column: error.column - 1 },
  };
}

export { parocss };
