export type { ParocssError, CompileOptions, CompileResult, Name, Vocabulary } from './index.js';
import type { CompileOptions, CompileResult, Name, ParocssError, Vocabulary } from './index.js';

/** Loads the WebAssembly compiler. Await it once before the other functions. */
export function init(input?: string | URL | Response | BufferSource | WebAssembly.Module): Promise<void>;
export function compile(source: string, options?: Omit<CompileOptions, 'root'>): CompileResult;
export function format(source: string): string | null;
export function vocabulary(): Vocabulary;
export function scan(text: string): string[];
export function suggestion(name: string): string | null;
export function isClass(name: string): boolean;
export function classCss(classes: Iterable<string>): string;
export function names(): Name[];
export function version(): string;
export function formatError(error: ParocssError): string;
