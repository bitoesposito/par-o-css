export interface ParocssError {
  /** Error message, in Neapolitan */
  message: string;
  /** File the error is in: the .cess or a file it pastes in */
  file: string;
  /** 1-based line */
  line: number;
  /** 1-based column */
  column: number;
  /** A hint on how to fix it, when there is one */
  suggestion: string | null;
}

export interface CompileOptions {
  /** Name of the file, for errors and to resolve `@chiamm` paths (default: "stile.cess"). */
  filename?: string;
  /**
   * Reads a file named by `@chiamm`: its text, or null when it does not exist.
   * Default in Node: from disk, `/…` paths from `root`.
   */
  read?: (path: string) => string | null;
  /** Folder for paths that start with `/` (Node only, default: the working directory). */
  root?: string;
  /** The words of the pages that may be classes: `@chiamm "par-o-css"` writes a rule for each class. */
  classes?: Iterable<string>;
}

export interface CompileResult {
  /** The CSS, or null when there are errors */
  css: string | null;
  /** Files pasted in by `@chiamm` */
  dependencies: string[];
  errors: ParocssError[];
}

export interface Name {
  /** The alias, root or variant */
  nomme: string;
  /** "classe", "radice" or "variante" */
  che: string;
  /** Its name in Tailwind */
  tailwind: string;
}

/** Compiles a .cess file to CSS. Never throws on invalid source: check `errors`. */
export function compile(source: string, options?: CompileOptions): CompileResult;
/** The words of a text that may be classes. */
export function scan(text: string): string[];
/** The words that may be classes in the page files under `root` (Node only). */
export function findClasses(root?: string): Set<string>;
/** The par-o-css class for a Tailwind class (`p-4` → `spazio-4`), or null. */
export function suggestion(name: string): string | null;
/** Whether a word is a class of par-o-css, with the starting theme. */
export function isClass(name: string): boolean;
/** The CSS of some classes with the starting theme, and the theme properties they use. */
export function classCss(classes: Iterable<string>): string;
/** Every alias, root and variant. */
export function names(): Name[];
export function version(): string;
/** `file:line:column: message`, then the suggestion on its own line. */
export function formatError(error: ParocssError): string;
