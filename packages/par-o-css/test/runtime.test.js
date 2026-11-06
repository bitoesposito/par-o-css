import { test } from 'node:test';
import assert from 'node:assert/strict';
import { compile } from '../index.js';
import { addClasses, compileWithFiles, styleName } from '../lib/runtime.js';

const files = {
  'sito/pezzi/carta.cess': '.carta { @applica spazio-4 }\n@chiamm "bottone.cess"',
  'sito/pezzi/bottone.cess': 'citofono { culore: russo }',
};

function loader() {
  const fetched = [];
  const load = async (path) => {
    fetched.push(path);
    if (!(path in files)) throw new Error('404');
    return files[path];
  };
  return { load, fetched };
}

test('fetches the files pasted in until the stylesheet compiles', async () => {
  const { load, fetched } = loader();
  const source = '@chiamm "par-o-css"\n@chiamm "pezzi/carta.cess"';
  const { css, errors } = await compileWithFiles(source, 'sito/stile.cess', ['flessibile'], load, compile);
  assert.deepEqual(errors, []);
  assert.match(css, /\.flessibile \{/);
  assert.match(css, /\.carta \{ padding: calc\(var\(--spazio\) \* 4\); \}/);
  assert.match(css, /button \{ color: red \}/);
  assert.deepEqual(fetched, ['sito/pezzi/carta.cess', 'sito/pezzi/bottone.cess']);
});

test('a file that cannot be fetched is a compile error', async () => {
  const { load } = loader();
  const { errors } = await compileWithFiles('@chiamm "nada.cess"', 'sito/stile.cess', [], load, compile);
  assert.equal(errors[0].message, "Nun trovo 'nada.cess'.");
});

test('style names and new classes', () => {
  assert.equal(styleName('stile/stile.cess?v=2', 0), 'stile/stile.cess');
  assert.equal(styleName(null, 1), 'stile-2.cess');
  const seen = new Set(['a']);
  assert.equal(addClasses([{ classList: ['a'] }], seen), false);
  assert.equal(addClasses([{ classList: ['a', 'b'] }, {}], seen), true);
  assert.deepEqual([...seen], ['a', 'b']);
});
