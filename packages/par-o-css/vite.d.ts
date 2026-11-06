import type { Plugin } from 'vite';

/**
 * Compiles the `.cess` files a module imports or a page links to, with the
 * classes found in the project's page files (`.emmurt`, `.fratm`, `.html`,
 * `.js`, `.jsx`, `.ts`, `.tsx`, `.vue`, `.svelte`, `.astro`, `.md`), and
 * again when a page gets a new class.
 */
export default function parocss(): Plugin;
export { parocss };
