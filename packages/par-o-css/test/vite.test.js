import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import parocss from '../vite.js';

function project() {
  const root = mkdtempSync(join(tmpdir(), 'parocss-vite-'));
  writeFileSync(join(root, 'stile.cess'), '@chiamm "par-o-css"\n.carta { culore: russo }');
  writeFileSync(join(root, 'index.emmurt'), '<cascett classe="flessibile carta">');
  const plugin = parocss();
  plugin.configResolved({ root });
  plugin.buildStart();
  return { root, plugin };
}

const context = () => {
  const watched = [];
  return {
    watched,
    addWatchFile: (file) => watched.push(file),
    error: (e) => {
      throw new Error(e.message);
    },
  };
};

test('a linked .cess goes through the CSS pipeline, with the classes of the pages', () => {
  const { root, plugin } = project();
  const html = plugin.transformIndexHtml.handler('<link rel="stylesheet" href="stile.cess"><link href="a.css">');
  assert.equal(html, '<link rel="stylesheet" href="stile.cess?parocss&lang.css"><link href="a.css">');
  const id = plugin.resolveId('/stile.cess?parocss&lang.css', undefined);
  assert.equal(id, join(root, 'stile.cess') + '?parocss&lang.css');
  assert.equal(plugin.resolveId('./stile.cess', join(root, 'main.js')), id);
  assert.equal(plugin.resolveId('./a.css', join(root, 'main.js')), null);
  // `?raw` and `?url` are the text and the URL of the file, as for any file
  assert.equal(plugin.resolveId('./stile.cess?raw', join(root, 'main.js')), null);
  assert.equal(plugin.resolveId('./stile.cess?url', join(root, 'main.js')), null);
  const css = plugin.load.call(context(), id);
  assert.match(css, /\.flessibile \{/);
  assert.match(css, /\.carta \{ color: red \}/);
  rmSync(root, { recursive: true });
});

test('errors carry their place', () => {
  const { root, plugin } = project();
  writeFileSync(join(root, 'stile.cess'), '.carta {\n  color: russo\n}');
  assert.throws(() => plugin.load.call(context(), join(root, 'stile.cess') + '?parocss&lang.css'), /stile\.cess:2:3: 'color' nun è napoletano\.\n {2}Vuoi dire 'culore'\?/);
  rmSync(root, { recursive: true });
});

test('a page with a new class updates the stylesheets', () => {
  const { root, plugin } = project();
  const sheet = { id: join(root, 'stile.cess') + '?parocss&lang.css' };
  const invalidated = [];
  const server = { moduleGraph: { idToModuleMap: new Map([[sheet.id, sheet], ['x', { id: 'x.js' }]]), invalidateModule: (m) => invalidated.push(m) } };
  const page = join(root, 'index.emmurt');
  assert.equal(plugin.handleHotUpdate({ file: page, modules: [], server }), undefined);
  writeFileSync(page, '<cascett classe="flessibile griglia carta">');
  assert.deepEqual(plugin.handleHotUpdate({ file: page, modules: [], server }), [sheet]);
  assert.deepEqual(invalidated, [sheet]);
  assert.equal(plugin.handleHotUpdate({ file: join(root, 'stile.css'), modules: [], server }), undefined);
  rmSync(root, { recursive: true });
});
