// VS Code glue: errors while you type in .cess files, completion and hover
// for the classes in pages and for the words of a .cess, links on @chiamm.
'use strict';

const fs = require('node:fs');
const path = require('node:path');
const vscode = require('vscode');
const wasm = require('./compiler/parocss_wasm.js');
const cess = require('./lib/cess.js');

const vocab = JSON.parse(wasm.vocabulary());
const names = JSON.parse(wasm.names());
const classItems = cess.classItems(names, wasm.examples());

/** Where the classes are written: pages, components, templates. */
const PAGES = ['htmlemmurt', 'html', 'fratmscript', 'javascriptreact', 'typescriptreact', 'vue', 'svelte', 'astro'];
const SELECTOR = ['cess', ...PAGES];

const norm = (file) => file.split(path.sep).join('/');

function activate(context) {
  const diagnostics = vscode.languages.createDiagnosticCollection('par-o-css');
  const timers = new Map();
  let sheets = [];

  const findSheets = async () => {
    sheets = (await vscode.workspace.findFiles('**/*.cess', '**/{node_modules,dist,target}/**')).map((u) => norm(u.fsPath));
  };

  // The text of a file: the open document when there is one, with its unsaved changes
  const textOf = (file) => {
    const open = vscode.workspace.textDocuments.find((d) => norm(d.uri.fsPath) === file);
    if (open) return open.getText();
    try {
      return fs.readFileSync(file, 'utf8');
    } catch {
      return null;
    }
  };

  // The .cess that pastes this one in, and the one above it…: a piece is
  // checked inside the stylesheet that calls it, with its @tema
  const topSheet = (file) => {
    let current = file;
    for (let depth = 0; depth < 8; depth++) {
      const parent = sheets.find((sheet) => {
        if (sheet === current) return false;
        const text = textOf(sheet);
        return text !== null && wasm.pieces(text, sheet).includes(current);
      });
      if (!parent) return current;
      current = parent;
    }
    return current;
  };

  const check = (doc) => {
    if (doc.languageId !== 'cess') return;
    if (!vscode.workspace.getConfiguration('parocss').get('diagnostics')) {
      diagnostics.delete(doc.uri);
      return;
    }
    const file = norm(doc.uri.fsPath);
    const top = topSheet(file);
    const source = top === file ? doc.getText() : textOf(top);
    const read = (p) => (p === file ? doc.getText() : textOf(p));
    const { errors } = wasm.compile(source ?? '', top, read, []);
    // Errors of the stylesheet above belong to it, not to this piece
    const mine = top === file ? errors : errors.filter((e) => norm(e.file) === file);
    diagnostics.set(
      doc.uri,
      mine.map((error) => {
        const place = cess.errorPlace(error, file);
        const start = new vscode.Position(place.line, place.column);
        const range = doc.getWordRangeAtPosition(start) ?? new vscode.Range(start, start.translate(0, 1));
        const diagnostic = new vscode.Diagnostic(range, place.message, vscode.DiagnosticSeverity.Error);
        diagnostic.source = 'par-o-css';
        return diagnostic;
      }),
    );
  };

  const later = (doc) => {
    clearTimeout(timers.get(doc.uri.toString()));
    timers.set(doc.uri.toString(), setTimeout(() => check(doc), 250));
  };

  const contextAt = (doc, position) => {
    const from = new vscode.Position(Math.max(0, position.line - 40), 0);
    return cess.context(doc.getText(new vscode.Range(from, position)), doc.languageId === 'cess');
  };

  const completion = {
    provideCompletionItems(doc, position) {
      const where = contextAt(doc, position);
      if (!where) return [];
      const range = new vscode.Range(position.translate(0, -where.word.length), position);
      if (where.kind === 'class') {
        return classItems.map((item) => {
          const kind = item.variant ? vscode.CompletionItemKind.Keyword : vscode.CompletionItemKind.Constant;
          const out = new vscode.CompletionItem(item.label, kind);
          out.detail = item.detail;
          out.range = range;
          if (item.variant) out.command = { command: 'editor.action.triggerSuggest', title: '' };
          return out;
        });
      }
      const kind = where.kind === 'property' ? vscode.CompletionItemKind.Property : vscode.CompletionItemKind.Value;
      return cess.wordItems(vocab, where.kind).map((item) => {
        const out = new vscode.CompletionItem(item.label, kind);
        out.detail = item.detail;
        out.range = range;
        return out;
      });
    },
    // The CSS of a class, only for the one in view
    resolveCompletionItem(item) {
      if (item.kind === vscode.CompletionItemKind.Constant) {
        item.documentation = new vscode.MarkdownString().appendCodeblock(wasm.classCss([item.label]), 'css');
      }
      return item;
    },
  };

  const hover = {
    provideHover(doc, position) {
      const range = doc.getWordRangeAtPosition(position, /[^\s"'`;{}<>=,]+/);
      if (!range) return null;
      const word = doc.getText(range);
      if (contextAt(doc, range.end)?.kind === 'class' && wasm.isClass(word)) {
        return new vscode.Hover(new vscode.MarkdownString().appendCodeblock(wasm.classCss([word]), 'css'), range);
      }
      const suggestion = contextAt(doc, range.end)?.kind === 'class' && wasm.suggestion(word);
      if (suggestion) return new vscode.Hover(`Vuoi dire \`${suggestion}\`?`, range);
      if (doc.languageId !== 'cess') return null;
      const lines = cess.meaning(vocab, word.replace(/^:+/, ''));
      return lines.length ? new vscode.Hover(new vscode.MarkdownString().appendCodeblock(lines.join('\n'), 'text'), range) : null;
    },
  };

  // The whole document, or nothing when it has a syntax error: the errors say why
  const formatter = {
    provideDocumentFormattingEdits(doc) {
      const text = wasm.format(doc.getText());
      if (text === undefined || text === doc.getText()) return [];
      return [vscode.TextEdit.replace(new vscode.Range(doc.positionAt(0), doc.positionAt(doc.getText().length)), text)];
    },
  };

  // Ctrl+click on @chiamm "x.cess" opens the file
  const linker = {
    provideDocumentLinks(doc) {
      return cess.links(doc.getText()).map((link) => {
        const range = new vscode.Range(doc.positionAt(link.start), doc.positionAt(link.end));
        return new vscode.DocumentLink(range, vscode.Uri.file(path.join(path.dirname(doc.uri.fsPath), link.path)));
      });
    },
  };

  const watcher = vscode.workspace.createFileSystemWatcher('**/*.cess');
  context.subscriptions.push(
    diagnostics,
    watcher,
    watcher.onDidCreate(findSheets),
    watcher.onDidDelete(findSheets),
    vscode.languages.registerDocumentLinkProvider('cess', linker),
    vscode.languages.registerDocumentFormattingEditProvider('cess', formatter),
    vscode.languages.registerCompletionItemProvider(SELECTOR, completion, '"', "'", '`', ' ', ':', '-'),
    vscode.languages.registerHoverProvider(SELECTOR, hover),
    vscode.workspace.onDidOpenTextDocument(check),
    vscode.workspace.onDidSaveTextDocument(check),
    vscode.workspace.onDidChangeTextDocument((e) => later(e.document)),
    vscode.workspace.onDidCloseTextDocument((doc) => diagnostics.delete(doc.uri)),
    vscode.workspace.onDidChangeConfiguration((e) => {
      if (e.affectsConfiguration('parocss')) vscode.workspace.textDocuments.forEach(check);
    }),
  );
  findSheets().then(() => vscode.workspace.textDocuments.forEach(check));
}

module.exports = { activate };
