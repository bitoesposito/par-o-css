// Runs par-o-css in a web page, no build step:
//
//   <script type="module" src="…/par-o-css/browser.js"></script>
//   <link rel="stylesheet" type="text/cess" href="stile.cess">
//   <style type="text/cess"> … </style>
//
// Each .cess (linked or inline) is compiled with the classes of the page
// and becomes a <style>. A page with no .cess gets `@chiamm "par-o-css"`:
// the theme, the reset and its classes. When code adds a class the page
// did not have, the CSS is compiled again. Errors are shown in the page and
// in the console, in Neapolitan. `ready` resolves when the CSS is in.
import { init, compile, formatError } from './web.js';
import { DEFAULT_SOURCE, STYLE_TYPE, addClasses, compileWithFiles, styleName } from './lib/runtime.js';

async function load(path) {
  const response = await fetch(new URL(path, document.baseURI));
  if (!response.ok) throw new Error(`${response.status} ${response.statusText}`);
  return response.text();
}

function domReady() {
  if (document.readyState !== 'loading') return Promise.resolve();
  return new Promise((resolve) => document.addEventListener('DOMContentLoaded', resolve, { once: true }));
}

let errorBox = null;

function report(message) {
  console.error(`🤌 par-o-css ${message}`);
  errorBox ??= document.createElement('pre');
  errorBox.style.cssText =
    'position:fixed;inset:auto 1rem 1rem 1rem;z-index:2147483647;margin:0;color:#7a1f1f;background:#fff4f4;padding:1rem;border:1px solid #e3b4ae;border-radius:6px;white-space:pre-wrap;font:14px/1.5 ui-monospace,monospace';
  errorBox.textContent = `Uh, mannaggia! ${message}`;
  document.body.append(errorBox);
}

/** A stylesheet of the page: its source and the <style> it becomes. */
async function sheets() {
  const tags = [...document.querySelectorAll(`style[type="${STYLE_TYPE}"], link[type="${STYLE_TYPE}"]`)];
  if (tags.length === 0) {
    const style = document.createElement('style');
    document.head.append(style);
    return [{ source: DEFAULT_SOURCE, filename: 'par-o-css.cess', style }];
  }
  const found = [];
  for (const [index, tag] of tags.entries()) {
    const href = tag.getAttribute('href');
    const filename = styleName(href, index);
    let source;
    try {
      source = href ? await load(href) : tag.textContent;
    } catch (error) {
      report(`nun riesco a caricà ${filename} (${error.message})`);
      continue;
    }
    const style = document.createElement('style');
    style.dataset.parocss = filename;
    tag.replaceWith(style);
    found.push({ source, filename, style });
  }
  return found;
}

export const ready = Promise.all([init(), domReady()]).then(async () => {
  const classes = new Set();
  addClasses(document.querySelectorAll('[class]'), classes);
  const all = await sheets();

  const render = async () => {
    let failed = false;
    for (const sheet of all) {
      const { css, errors } = await compileWithFiles(sheet.source, sheet.filename, [...classes], load, compile);
      if (errors.length > 0) {
        report(formatError(errors[0]));
        failed = true;
      } else {
        sheet.style.textContent = css;
      }
    }
    if (!failed && errorBox) {
      errorBox.remove();
      errorBox = null;
    }
  };
  await render();

  // Classes added by code: compile again once per frame at most
  let pending = false;
  new MutationObserver((records) => {
    const touched = [];
    for (const record of records) {
      if (record.type === 'attributes') touched.push(record.target);
      for (const node of record.addedNodes) {
        if (node.nodeType !== 1) continue;
        touched.push(node, ...node.querySelectorAll('[class]'));
      }
    }
    if (!addClasses(touched, classes) || pending) return;
    pending = true;
    requestAnimationFrame(() => {
      pending = false;
      render();
    });
  }).observe(document.documentElement, {
    subtree: true,
    childList: true,
    attributes: true,
    attributeFilter: ['class'],
  });
});
