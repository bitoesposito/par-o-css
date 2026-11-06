// The parts of the browser runtime that need no DOM, so tests can run them in Node.

/** `<style type="text/cess">` and `<link rel="stylesheet" type="text/cess">` */
export const STYLE_TYPE = 'text/cess';

/** What a page without any .cess gets: the theme, the reset and its classes. */
export const DEFAULT_SOURCE = '@chiamm "par-o-css";';

/**
 * Compiles a .cess, fetching the files it pastes in. The compiler reads files
 * synchronously, so each round compiles with the files fetched so far and
 * fetches the ones it was missing, until nothing is missing.
 *
 * @param {string} source
 * @param {string} filename  paths in the file are relative to it
 * @param {string[]} classes  the classes of the page
 * @param {(path: string) => Promise<string>} load  rejects when the file is missing
 * @param {(source: string, options: object) => any} compile  the par-o-css compile()
 */
export async function compileWithFiles(source, filename, classes, load, compile) {
  const files = new Map();
  for (;;) {
    const wanted = [];
    const result = compile(source, {
      filename,
      classes,
      read: (path) => {
        if (files.has(path)) return files.get(path);
        wanted.push(path);
        return null;
      },
    });
    if (wanted.length === 0) return result;
    await Promise.all(
      wanted.map(async (path) => {
        files.set(path, await load(path).catch(() => null));
      }),
    );
  }
}

/** Name of a stylesheet in errors: its `href`, or `stile-N.cess` when inline. */
export function styleName(href, index) {
  return href ? href.split(/[?#]/)[0] : `stile-${index + 1}.cess`;
}

/** The classes of elements, added to `into`; true when one is new. */
export function addClasses(elements, into) {
  let added = false;
  for (const element of elements) {
    for (const name of element.classList ?? []) {
      if (!into.has(name)) {
        into.add(name);
        added = true;
      }
    }
  }
  return added;
}
