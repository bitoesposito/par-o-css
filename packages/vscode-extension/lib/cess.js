// The parts of the extension that need no VS Code, so tests can run them in Node.
'use strict';

// The value of a class attribute up to the cursor: `classe="…`, `class="…`,
// `className="…`, `className={"…`, `classe={`…`
const CLASS_ATTRIBUTE = /\b(?:classe|class|className)\s*=\s*\{?\s*["'`]([^"'`]*)$/;
// An @applica in a .cess, up to the cursor
const APPLICA = /@applica\s+([^;{}]*)$/;

/**
 * Where the cursor is, from the text before it: in a list of classes (an
 * attribute, or `@applica` in a .cess) with the class being typed, or after
 * `proprietà:` in a .cess, or at the start of a declaration.
 */
function context(before, isCess) {
  const list = (isCess && APPLICA.exec(before)) || CLASS_ATTRIBUTE.exec(before);
  if (list) {
    const typed = list[1].split(/\s+/).pop();
    // The variants already written stay: complete after the last `:`
    const colon = lastColon(typed);
    return { kind: 'class', typed, variants: typed.slice(0, colon + 1), word: typed.slice(colon + 1) };
  }
  if (!isCess) return null;
  const line = before.slice(before.lastIndexOf('\n') + 1);
  const value = /^\s*([\p{L}-]+)\s*:\s*[^;{}]*?([\p{L}-]*)$/u.exec(line);
  if (value && insideBlock(before)) return { kind: 'value', property: value[1], word: value[2] };
  const property = /^\s*([\p{L}-]*)$/u.exec(line);
  if (property && insideBlock(before)) return { kind: 'property', word: property[1] };
  return null;
}

/** Index of the last `:` outside brackets, or -1. */
function lastColon(text) {
  let depth = 0;
  for (let i = text.length - 1; i >= 0; i--) {
    if (text[i] === ']') depth++;
    else if (text[i] === '[') depth--;
    else if (text[i] === ':' && depth === 0) return i;
  }
  return -1;
}

function insideBlock(text) {
  let depth = 0;
  for (const c of text) {
    if (c === '{') depth++;
    else if (c === '}') depth = Math.max(0, depth - 1);
  }
  return depth > 0;
}

/** Completion items for a class: the variants, then the classes. */
function classItems(names, examples) {
  const variants = names
    .filter((n) => n.che === 'variante')
    .map((n) => ({ label: `${n.nomme}:`, detail: `${n.tailwind}:`, variant: true }));
  const tailwind = new Map(names.filter((n) => n.che === 'classe').map((n) => [n.nomme, n.tailwind]));
  const classes = examples.map((c) => ({ label: c, detail: tailwind.get(c) ?? '' }));
  return [...variants, ...classes];
}

/** Completion items for a property or a value of a .cess. */
function wordItems(vocab, kind) {
  const list = kind === 'property' ? vocab.properties : vocab.values;
  const seen = new Set();
  return list
    .filter(([nap]) => !seen.has(nap) && seen.add(nap))
    .map(([nap, css]) => ({ label: nap, detail: css }));
}

/** What a Neapolitan word of a .cess is in CSS, one line per meaning. */
function meaning(vocab, word) {
  const lines = [];
  const add = (table, what, format = (css) => css) => {
    for (const [nap, css] of vocab[table]) if (nap === word) lines.push(`${what}: ${format(css)}`);
  };
  add('properties', 'proprietà');
  add('values', 'valore');
  add('pseudo', 'pseudo-classe', (css) => `:${css}`);
  add('tags', 'tag', (css) => `<${css}>`);
  return lines;
}

/** The `"…"` of each `@chiamm "x.cess"`, with its offsets. */
function links(text) {
  const out = [];
  for (const match of text.matchAll(/@chiamm\s+(["'])([^"']+\.cess)\1/g)) {
    const start = match.index + match[0].indexOf(match[2]);
    out.push({ path: match[2], start, end: start + match[2].length });
  }
  return out;
}

/** Line and column (from 0) and message of an error, in the file `file`. */
function errorPlace(error, file) {
  const same = error.file.replace(/\\/g, '/') === file.replace(/\\/g, '/');
  const text = error.suggestion ? `${error.message}\n${error.suggestion}` : error.message;
  if (same) return { line: error.line - 1, column: error.column - 1, message: text };
  // An error in a file pasted in: show it at the top, naming the file
  return { line: 0, column: 0, message: `${error.file}:${error.line}:${error.column}: ${text}` };
}

const escape = (word) => word.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
const alternatives = (words) =>
  [...new Set(words)].sort((a, b) => b.length - a.length || a.localeCompare(b)).map(escape).join('|');

/**
 * The injection grammar that colours the Neapolitan words of a .cess (and of
 * `<cess>` in htmlemmurt pages) as the CSS grammar colours the English ones.
 */
function grammar(vocab) {
  const word = (words) => `(?<![\\w.#-])(?:${alternatives(words)})(?![\\w-])`;
  return {
    $schema: 'https://raw.githubusercontent.com/martinring/tmlanguage/master/tmlanguage.json',
    scopeName: 'cess.napoletano.injection',
    injectionSelector:
      'L:source.cess -comment -string, L:text.html.emmurt meta.embedded.block.css -comment -string',
    patterns: [
      {
        begin: '(@applica)\\b',
        beginCaptures: { 1: { name: 'keyword.control.at-rule.cess' } },
        end: '(?=[;{}\\n])',
        patterns: [{ match: '[^\\s;{}]+', name: 'entity.other.attribute-name.class.css' }],
      },
      { match: '@(?:chiamm|tema|quanno)\\b', name: 'keyword.control.at-rule.cess' },
      { match: '!importante\\b', name: 'keyword.other.important.css' },
      {
        match: `(?<=:)${alternatives(vocab.pseudo.map(([nap]) => nap))}(?![\\w-])`,
        name: 'entity.other.attribute-name.pseudo-class.css',
      },
      {
        match: `${word(vocab.properties.map(([nap]) => nap))}(?=\\s*:)`,
        name: 'support.type.property-name.css',
      },
      {
        match: word(vocab.values.map(([nap]) => nap)),
        name: 'support.constant.property-value.css',
      },
      {
        match: `${word(vocab.tags.map(([nap]) => nap))}(?=[^;{}]*\\{)`,
        name: 'entity.name.tag.css',
      },
    ],
  };
}

module.exports = { context, classItems, wordItems, meaning, links, errorPlace, grammar };
