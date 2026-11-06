// Copies the Node build of the compiler (packages/par-o-css, `npm run build`
// there first) into compiler/, so the extension ships it, and writes the
// grammar of the Neapolitan words from its vocabulary.
import { copyFileSync, mkdirSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

const root = dirname(dirname(fileURLToPath(import.meta.url)));
const from = join(root, '..', 'par-o-css', 'wasm', 'node');
mkdirSync(join(root, 'compiler'), { recursive: true });
for (const file of ['parocss_wasm.js', 'parocss_wasm_bg.wasm']) {
  copyFileSync(join(from, file), join(root, 'compiler', file));
}
copyFileSync(join(root, '..', '..', 'LICENSE'), join(root, 'LICENSE'));

const require = createRequire(import.meta.url);
const wasm = require('../compiler/parocss_wasm.js');
const { grammar } = require('../lib/cess.js');
writeFileSync(
  join(root, 'syntaxes', 'napoletano.tmLanguage.json'),
  `${JSON.stringify(grammar(JSON.parse(wasm.vocabulary())), null, 2)}\n`,
);
