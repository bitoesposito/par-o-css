import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { classCss, compile, findClasses, formatError, isClass, names, scan, suggestion, version } from '../index.js';

test('compiles Neapolitan CSS', () => {
  const { css, errors } = compile('citofono:passanno { culore: russo }');
  assert.deepEqual(errors, []);
  assert.equal(css, 'button:hover { color: red }');
});

test('English is an error with the word to write', () => {
  const { css, errors } = compile('.a {\n  color: russo\n}', { filename: 'stile.cess' });
  assert.equal(css, null);
  assert.equal(errors[0].message, "'color' nun è napoletano.");
  assert.equal(formatError(errors[0]), "stile.cess:2:3: 'color' nun è napoletano.\n  Vuoi dire 'culore'?");
});

test('classes get a rule from @chiamm "par-o-css"', () => {
  const { css } = compile('@chiamm "par-o-css";', { classes: new Set(['flessibile', 'menu']) });
  assert.match(css, /\.flessibile \{\n {4}display: flex;/);
  assert.doesNotMatch(css, /\.menu/);
});

test('files named by @chiamm are read from disk, /… from root', () => {
  const root = mkdtempSync(join(tmpdir(), 'parocss-'));
  mkdirSync(join(root, 'pezzi'));
  writeFileSync(join(root, 'pezzi', 'carta.cess'), '.carta { culore: russo }');
  writeFileSync(join(root, 'index.emmurt'), '<cascett classe="spazio-4 carta">');
  const { css, dependencies } = compile('@chiamm "/pezzi/carta.cess";', { root });
  assert.equal(css, '.carta { color: red }');
  assert.deepEqual(dependencies, ['/pezzi/carta.cess']);
  assert.ok(findClasses(root).has('spazio-4'));
  rmSync(root, { recursive: true });
});

test('scan, suggestion, isClass, classCss, names, version', () => {
  assert.deepEqual(scan('<cascett classe="flessibile md:spazio-4">'), ['cascett', 'classe', 'flessibile', 'md:spazio-4']);
  assert.equal(suggestion('p-4'), 'spazio-4');
  assert.equal(suggestion('menu'), null);
  assert.equal(isClass('culore-russo-500'), true);
  assert.equal(isClass('culore-russo'), false);
  assert.match(classCss(['spazio-4']), /^:root \{\n {2}--spazio: 0\.25rem;\n\}\n@layer classi/);
  assert.ok(names().some((n) => n.nomme === 'passanno' && n.che === 'variante' && n.tailwind === 'hover'));
  assert.match(version(), /^\d+\.\d+\.\d+/);
});
