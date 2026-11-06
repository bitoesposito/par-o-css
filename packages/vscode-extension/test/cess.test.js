const { test } = require('node:test');
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const { join } = require('node:path');
const wasm = require('../compiler/parocss_wasm.js');
const cess = require('../lib/cess.js');

const vocab = JSON.parse(wasm.vocabulary());

test('the cursor in a list of classes', () => {
  assert.deepEqual(cess.context('<cascett classe="flessibile md:pass', false), {
    kind: 'class',
    typed: 'md:pass',
    variants: 'md:',
    word: 'pass',
  });
  assert.equal(cess.context('<div className={"spazio-4 ', false).word, '');
  assert.equal(cess.context('.carta {\n  @applica spazio-4 culore-r', true).word, 'culore-r');
  assert.equal(cess.context('larghezza-[1px:2', false), null);
  assert.equal(cess.context('<cascett chi="a', false), null);
});

test('the cursor on a property or a value of a .cess', () => {
  assert.deepEqual(cess.context('.a {\n  cul', true), { kind: 'property', word: 'cul' });
  assert.deepEqual(cess.context('.a {\n  culore: ru', true), { kind: 'value', property: 'culore', word: 'ru' });
  assert.equal(cess.context('cul', true), null);
  assert.equal(cess.context('.a {\n  cul', false), null);
});

test('items, meanings, links and errors', () => {
  const items = cess.classItems(JSON.parse(wasm.names()), wasm.examples());
  assert.ok(items.some((i) => i.label === 'passanno:' && i.variant && i.detail === 'hover:'));
  assert.ok(items.some((i) => i.label === 'flessibile' && i.detail === 'flex'));
  assert.ok(items.some((i) => i.label === 'culore-russo-500'));
  assert.ok(cess.wordItems(vocab, 'property').some((i) => i.label === 'culore' && i.detail === 'color'));
  assert.deepEqual(cess.meaning(vocab, 'passanno'), ['pseudo-classe: :hover']);
  assert.deepEqual(cess.meaning(vocab, 'cascett'), ['tag: <div>']);
  assert.deepEqual(cess.links('@chiamm "par-o-css"\n@chiamm "pezzi/carta.cess";'), [{ path: 'pezzi/carta.cess', start: 29, end: 45 }]);
  const error = { file: 'a.cess', line: 2, column: 3, message: "'color' nun è napoletano.", suggestion: "Vuoi dire 'culore'?" };
  assert.deepEqual(cess.errorPlace(error, 'a.cess'), { line: 1, column: 2, message: "'color' nun è napoletano.\nVuoi dire 'culore'?" });
  assert.equal(cess.errorPlace(error, 'b.cess').line, 0);
});

test('the grammar is the one of the current vocabulary', () => {
  const written = JSON.parse(readFileSync(join(__dirname, '..', 'syntaxes', 'napoletano.tmLanguage.json'), 'utf8'));
  assert.deepEqual(written, cess.grammar(vocab), 'run npm run compiler');
  // Every pattern is a valid regular expression (TextMate's Oniguruma is close enough here)
  for (const p of written.patterns) for (const re of [p.match, p.begin, p.end].filter(Boolean)) new RegExp(re, 'u');
});
