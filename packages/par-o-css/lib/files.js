// The files of a project whose words may be classes (Node only).
import { readdirSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

/** The extensions of the files the classes are looked for in. */
export const PAGES = /\.(emmurt|fratm|html?|jsx?|mjs|tsx?|vue|svelte|astro|md)$/;
const SKIP = new Set(['node_modules', 'dist', 'target']);

/** Every page file under `dir`, skipping hidden folders and the build ones. */
export function pageFiles(dir) {
  const files = [];
  let entries;
  try {
    entries = readdirSync(dir, { withFileTypes: true });
  } catch {
    return files;
  }
  for (const entry of entries) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) {
      if (!entry.name.startsWith('.') && !SKIP.has(entry.name)) files.push(...pageFiles(path));
    } else if (PAGES.test(entry.name)) {
      files.push(path);
    }
  }
  return files.sort();
}

/** The words of a file that may be classes, or none when it cannot be read. */
export function fileClasses(file, scan) {
  try {
    return scan(readFileSync(file, 'utf8'));
  } catch {
    return [];
  }
}
