// The comparison of lessons (src/confronta.fratm), without a browser: the
// module is compiled with FratmScript and the elements are fakes.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { compile } from 'fratmscript';

const source = readFileSync(new URL('../src/confronta.fratm', import.meta.url), 'utf8');
const { code, errors } = compile(source, { sourceMap: false });
assert.deepEqual(errors, []);
const { PROPRIETA, albero, differenza } = await import(`data:text/javascript,${encodeURIComponent(code)}`);

/** A fake element: its tag, the styles that differ from the defaults, its children. */
function elemento(tag, stile = {}, figli = []) {
  const el = {
    localName: tag,
    children: figli,
    ownerDocument: {
      defaultView: { getComputedStyle: () => ({ getPropertyValue: (p) => stile[p] ?? 'auto' }) },
    },
  };
  return el;
}

const pagina = (culore, figli = [elemento('span')]) =>
  albero(elemento('body', {}, [elemento('h1', { color: culore }), elemento('p', {}, figli), elemento('script')]));

test('the list covers colours, boxes, flex, borders and text', () => {
  for (const p of ['color', 'background-color', 'padding-top', 'margin-left', 'display', 'justify-content',
    'row-gap', 'width', 'border-top-width', 'border-top-left-radius', 'font-size', 'text-align', 'box-shadow', 'opacity']) {
    assert.ok(PROPRIETA.includes(p), p);
  }
});

test('the same styles are no difference', () => {
  assert.equal(differenza(pagina('rgb(220, 38, 38)'), pagina('rgb(220, 38, 38)')), null);
});

test('a different computed style is a difference, with where it is', () => {
  assert.equal(
    differenza(pagina('rgb(0, 0, 0)'), pagina('rgb(220, 38, 38)')),
    'body > h1:1: color rgb(0, 0, 0) ≠ rgb(220, 38, 38)',
  );
});

test('a different tree is a difference', () => {
  assert.match(differenza(pagina('red', []), pagina('red')), /body > p:2: 0 figli ≠ 1/);
  assert.match(differenza(pagina('red', [elemento('b')]), pagina('red')), /<b> ≠ <span>/);
});

test('scripts and styles are not part of the tree', () => {
  assert.equal(pagina('red').figli.length, 2);
});
