//! # par-o-css
//!
//! CSS in Neapolitan, with classes ready to use like Tailwind's. A `.cess`
//! file is CSS with the words of FratmScript's `stile`:
//!
//! ```rust
//! use parocss_core::{compile, Options};
//!
//! let out = compile("cascett:passanno { culore: russo }", &Options::default()).unwrap();
//! assert_eq!(out.css, "div:hover { color: red }");
//! ```
//!
//! It is strict like htmlemmurt: an English name that has a Neapolitan word
//! (`color`, `:hover`, `@media`, `div`) is an error that says which word
//! to write. Three at-rules are par-o-css's own:
//!
//! - `@chiamm "par-o-css"` puts in the theme, the base reset and the rules
//!   of the classes the pages use (`Options::classes`, found with [`scan`]);
//!   `@chiamm "pezzi/carta.cess"` pastes another file in.
//! - `@tema { --culore-pummarola-500: #e33 }` changes or adds to the theme:
//!   that colour is now `culore-pummarola-500`, `sfunno-pummarola-500`…
//! - `@applica spazio-4 passanno:sfunno-russo-600` inside a rule writes the
//!   declarations of those classes there.

mod classi;
mod format;
pub mod tema;

use std::collections::BTreeSet;
use std::fmt;

use fratm_core::stile;
use serde::Serialize;

pub use classi::{examples, generate, is_class, names, scan, suggestion, Theme};
pub use format::format;

pub struct Options<'a> {
    /// Name of the file, used in errors and to resolve `@chiamm` paths.
    pub filename: &'a str,
    /// Reads a file named by `@chiamm`, or None when it cannot. Gets the
    /// path joined to the folder of the file that names it, or as written
    /// when it starts with `/`.
    pub read: &'a dyn Fn(&str) -> Option<String>,
    /// The words that may be classes in the pages, from [`scan`]: the ones
    /// that are classes of par-o-css get a rule from `@chiamm "par-o-css"`.
    pub classes: &'a BTreeSet<String>,
}

static NO_CLASSES: BTreeSet<String> = BTreeSet::new();

impl Default for Options<'_> {
    fn default() -> Self {
        Options {
            filename: "stile.cess",
            read: &|_| None,
            classes: &NO_CLASSES,
        }
    }
}

#[derive(Debug)]
pub struct Output {
    pub css: String,
    /// Files pasted in by `@chiamm`.
    pub dependencies: Vec<String>,
}

/// A compile error. The message is in Neapolitan; `suggestion` says how to fix it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Error {
    pub message: String,
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub suggestion: Option<String>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}: {}",
            self.file, self.line, self.column, self.message
        )?;
        if let Some(s) = &self.suggestion {
            write!(f, "\n  {s}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Error {}

/// What `@chiamm "…"` names to get the classes.
const PAROCSS: &str = "par-o-css";

/// Compiles a `.cess` file to CSS.
pub fn compile(source: &str, options: &Options) -> Result<Output, Error> {
    let mut compiler = Compiler {
        options,
        chain: vec![options.filename.replace('\\', "/")],
        dependencies: Vec::new(),
        applies: Vec::new(),
    };
    let text = compiler.file(source)?;

    // The theme, with what every `@tema` adds or changes
    let mut theme = Theme::default();
    for d in directives(&text) {
        if d.kind == Kind::Tema {
            for (name, value) in custom_properties(&d.arg) {
                theme.user.insert(name, value);
            }
        }
    }

    let mut used = BTreeSet::new();
    let mut applied = Vec::new();
    for apply in &compiler.applies {
        let mut css = Vec::new();
        for (class, line, column) in &apply.classes {
            match classi::apply(class, &theme, &mut used) {
                Some(lines) => css.extend(lines),
                None => {
                    return Err(Error {
                        message: format!("'{class}' nun è na classe 'e par-o-css."),
                        file: apply.file.clone(),
                        line: *line,
                        column: *column,
                        suggestion: suggestion(class).map(|s| format!("Vuoi dire '{s}'?")),
                    })
                }
            }
        }
        applied.push(css);
    }

    let mut out = String::new();
    let mut last = 0;
    let mut classes_at = None;
    for d in directives(&text) {
        out.push_str(&text[last..d.start]);
        match d.kind {
            Kind::Tema => out.push_str(&format!(":root {{{}}}", d.arg)),
            // Only `@chiamm "par-o-css"` is left after `file`
            Kind::Chiamm if classes_at.is_none() => classes_at = Some(out.len()),
            Kind::Chiamm | Kind::Applica => {}
        }
        last = d.end;
    }
    out.push_str(&text[last..]);
    // One declaration per line, with the indent of the `@applica`
    for (i, lines) in applied.iter().enumerate() {
        let marker = apply_marker(i);
        let Some(at) = out.find(&marker) else {
            continue;
        };
        let line_start = out[..at].rfind('\n').map_or(0, |n| n + 1);
        let indent: String = out[line_start..at]
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect();
        out.replace_range(at..at + marker.len(), &lines.join(&format!("\n{indent}")));
    }

    let block = match classes_at {
        Some(_) => {
            let (rules, mut rules_used) =
                generate(options.classes.iter().map(String::as_str), &theme);
            rules_used.extend(used.iter().cloned());
            rules_used.extend(["scrittura-sans".into(), "scrittura-mono".into()]);
            let block = format!("{}{}{}", theme.root(&rules_used), tema::PULIZIA, rules);
            block.trim_end().to_string()
        }
        None => theme.root(&used),
    };
    let at = classes_at.unwrap_or(0);
    out.insert_str(at, &block);
    Ok(Output {
        css: out,
        dependencies: compiler.dependencies,
    })
}

/// The `.cess` files a source pastes in with `@chiamm`, as paths joined to
/// the folder of `filename`: they are pieces of it, not stylesheets of
/// their own.
pub fn pieces(source: &str, filename: &str) -> Vec<String> {
    directives(&terminate(source))
        .into_iter()
        .filter(|d| d.kind == Kind::Chiamm)
        .filter_map(|d| quoted(&d.arg).map(str::to_string))
        .filter(|path| path.ends_with(".cess"))
        .map(|path| join(&filename.replace('\\', "/"), &path))
        .collect()
}

struct Compiler<'a> {
    options: &'a Options<'a>,
    /// The files being compiled, the outermost first, to catch a file that
    /// pastes itself in
    chain: Vec<String>,
    dependencies: Vec<String>,
    applies: Vec<Apply>,
}

/// An `@applica`: its classes, each with its place in its file.
struct Apply {
    file: String,
    classes: Vec<(String, usize, usize)>,
}

fn apply_marker(i: usize) -> String {
    format!("/*@applica {i}*/")
}

impl Compiler<'_> {
    fn current(&self) -> &str {
        self.chain.last().map_or("", String::as_str)
    }

    fn error(&self, at: (usize, usize), message: String, suggestion: Option<String>) -> Error {
        Error {
            message,
            file: self.current().to_string(),
            line: at.0,
            column: at.1,
            suggestion,
        }
    }

    /// One file, translated and checked, with its `@chiamm "….cess"` pasted
    /// in and its `@applica` replaced by markers.
    fn file(&mut self, source: &str) -> Result<String, Error> {
        let source = terminate(source);
        let chunks = [source.clone()];
        let translated = stile::translate(&chunks)
            .map_err(|e| self.stile_error(e))?
            .remove(0);
        let english = stile::english_words(&chunks).map_err(|e| self.stile_error(e))?;
        if let Some(word) = english.first() {
            return Err(self.english_error(&source, word));
        }
        let from = directives(&source);
        let to = directives(&translated);
        let mut out = String::new();
        let mut last = 0;
        // `stile` copies at-rules as they are, so both have the same ones
        for (s, d) in from.iter().zip(&to) {
            out.push_str(&translated[last..d.start]);
            last = d.end;
            let at = place(&source, s.start);
            match d.kind {
                Kind::Tema => out.push_str(&translated[d.start..d.end]),
                Kind::Applica => {
                    let arg_at = s.start + "@applica".len();
                    let classes = words(&source[arg_at..s.end])
                        .into_iter()
                        .map(|(w, offset)| {
                            let (line, column) = place(&source, arg_at + offset);
                            (w.to_string(), line, column)
                        })
                        .collect();
                    out.push_str(&apply_marker(self.applies.len()));
                    self.applies.push(Apply {
                        file: self.current().to_string(),
                        classes,
                    });
                }
                Kind::Chiamm => {
                    let Some(path) = quoted(&d.arg) else {
                        return Err(self.error(
                            at,
                            "@chiamm vo' nu nomme tra virgolette.".into(),
                            Some(
                                "Scrive @chiamm \"par-o-css\"; o @chiamm \"pezzi/carta.cess\";"
                                    .into(),
                            ),
                        ));
                    };
                    if path == PAROCSS {
                        out.push_str("@chiamm \"par-o-css\";");
                    } else if path.ends_with(".cess") {
                        out.push_str(&self.include(path, at)?);
                    } else {
                        // A plain stylesheet loads at run time
                        out.push_str(&format!("@import \"{path}\";"));
                    }
                }
            }
        }
        out.push_str(&translated[last..]);
        Ok(out)
    }

    /// `@chiamm "pezzi/carta.cess"`: the other file, compiled.
    fn include(&mut self, path: &str, at: (usize, usize)) -> Result<String, Error> {
        let full = join(self.current(), path);
        if self.chain.contains(&full) {
            return Err(self.error(
                at,
                format!("'{path}' se chiamma a sé stesso: nun fernesce maje."),
                None,
            ));
        }
        let Some(source) = (self.options.read)(&full) else {
            return Err(self.error(at, format!("Nun trovo '{path}'."), None));
        };
        if !self.dependencies.contains(&full) {
            self.dependencies.push(full.clone());
        }
        self.chain.push(full);
        let css = self.file(&source);
        self.chain.pop();
        css
    }

    fn stile_error(&self, e: stile::StileError) -> Error {
        self.error(
            (e.position.line + 1, e.position.column + 1),
            e.message,
            None,
        )
    }

    fn english_error(&self, source: &str, word: &stile::EnglishWord) -> Error {
        let (line, column) = (word.position.line + 1, word.position.column + 1);
        // A pseudo-class keeps its colons in the message: `:hover`, `::before`
        let before: String = source
            .lines()
            .nth(word.position.line)
            .unwrap_or("")
            .chars()
            .take(word.position.column)
            .collect();
        let colons: String = before.chars().rev().take_while(|c| *c == ':').collect();
        self.error(
            (line, column),
            format!("'{colons}{}' nun è napoletano.", word.word),
            Some(format!("Vuoi dire '{colons}{}'?", word.neapolitan)),
        )
    }
}

/// Ends with a `;` the lines that hold an `@chiamm` or an `@applica` and
/// end without one: `stile` reads a new line as the end of a declaration,
/// but not of an at-rule.
fn terminate(source: &str) -> String {
    let mut out = String::with_capacity(source.len() + 8);
    for line in source.split_inclusive('\n') {
        let (body, newline) = match line.strip_suffix('\n') {
            Some(b) => (b, "\n"),
            None => (line, ""),
        };
        let (body, cr) = match body.strip_suffix('\r') {
            Some(b) => (b, "\r"),
            None => (body, ""),
        };
        let trimmed = body.trim();
        let directive = trimmed.starts_with("@chiamm") || trimmed.starts_with("@applica");
        out.push_str(body);
        if directive && !trimmed.ends_with([';', '{', '}']) {
            out.push(';');
        }
        out.push_str(cr);
        out.push_str(newline);
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Kind {
    Chiamm,
    Applica,
    Tema,
}

/// An at-rule of par-o-css in a text: where it starts and ends (byte
/// offsets, the `;` or the `}` included) and what it holds.
#[derive(Debug)]
struct Directive {
    kind: Kind,
    start: usize,
    end: usize,
    /// After the name: the path of `@chiamm`, the classes of `@applica`,
    /// the inside of the braces of `@tema`
    arg: String,
}

/// The par-o-css at-rules of a text, skipping strings and comments.
fn directives(text: &str) -> Vec<Directive> {
    let b = text.as_bytes();
    let mut found = Vec::new();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'/' if b.get(i + 1) == Some(&b'*') => i = skip(b, i),
            b'"' | b'\'' => i = skip(b, i),
            b'@' => {
                let name_end = (i + 1..b.len())
                    .find(|&j| !(b[j].is_ascii_alphanumeric() || b[j] == b'-' || b[j] == b'_'))
                    .unwrap_or(b.len());
                let kind = match &text[i + 1..name_end] {
                    "chiamm" => Kind::Chiamm,
                    "applica" => Kind::Applica,
                    "tema" => Kind::Tema,
                    _ => {
                        i = name_end;
                        continue;
                    }
                };
                let (end, arg) = if kind == Kind::Tema {
                    match (name_end..b.len()).find(|&j| b[j] == b'{') {
                        Some(open) => {
                            let close = closing(b, open);
                            let inner_end = close.min(b.len());
                            (
                                (close + 1).min(b.len()),
                                text[open + 1..inner_end].to_string(),
                            )
                        }
                        None => (b.len(), String::new()),
                    }
                } else {
                    let stop = statement_end(b, name_end);
                    // The `;` is part of it; without one, the spaces before
                    // the `}` stay where they are
                    let end = if b.get(stop) == Some(&b';') {
                        stop + 1
                    } else {
                        name_end + text[name_end..stop].trim_end().len()
                    };
                    (end, text[name_end..stop].trim().to_string())
                };
                found.push(Directive {
                    kind,
                    start: i,
                    end,
                    arg,
                });
                i = end;
            }
            _ => i += 1,
        }
    }
    found
}

/// Index after the string or comment starting at `at`.
fn skip(b: &[u8], at: usize) -> usize {
    if b[at] == b'/' {
        return (at + 2..b.len().saturating_sub(1))
            .find(|&j| b[j] == b'*' && b[j + 1] == b'/')
            .map_or(b.len(), |j| j + 2);
    }
    let quote = b[at];
    let mut j = at + 1;
    while j < b.len() {
        match b[j] {
            b'\\' => j += 1,
            c if c == quote => return j + 1,
            b'\n' => return j,
            _ => {}
        }
        j += 1;
    }
    b.len()
}

/// Index of the `}` that closes the `{` at `open`, or the end.
fn closing(b: &[u8], open: usize) -> usize {
    let mut depth = 0;
    let mut j = open;
    while j < b.len() {
        match b[j] {
            b'/' if b.get(j + 1) == Some(&b'*') => {
                j = skip(b, j);
                continue;
            }
            b'"' | b'\'' => {
                j = skip(b, j);
                continue;
            }
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return j;
                }
            }
            _ => {}
        }
        j += 1;
    }
    b.len()
}

/// Index of the `;`, `{` or `}` that ends the statement starting at `from`.
fn statement_end(b: &[u8], from: usize) -> usize {
    let mut depth = 0;
    let mut j = from;
    while j < b.len() {
        match b[j] {
            b'/' if b.get(j + 1) == Some(&b'*') => {
                j = skip(b, j);
                continue;
            }
            b'"' | b'\'' => {
                j = skip(b, j);
                continue;
            }
            b'(' | b'[' => depth += 1,
            b')' | b']' => depth -= 1,
            b';' | b'{' | b'}' if depth <= 0 => return j,
            _ => {}
        }
        j += 1;
    }
    b.len()
}

/// The words of an `@applica`, with their byte offset in `text`, without
/// the `;` and comments.
fn words(text: &str) -> Vec<(&str, usize)> {
    let mut out = Vec::new();
    let b = text.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'/' && b.get(i + 1) == Some(&b'*') {
            i = skip(b, i);
            continue;
        }
        if b[i].is_ascii_whitespace() || b[i] == b';' {
            i += 1;
            continue;
        }
        let start = i;
        while i < b.len() && !b[i].is_ascii_whitespace() && b[i] != b';' {
            i += 1;
        }
        out.push((&text[start..i], start));
    }
    out
}

/// The text of a string in quotes, or `None`.
fn quoted(arg: &str) -> Option<&str> {
    let arg = arg.trim();
    let quote = arg.chars().next().filter(|c| *c == '"' || *c == '\'')?;
    arg.strip_prefix(quote)?.strip_suffix(quote)
}

/// The `--name: value` declarations of a `@tema`.
fn custom_properties(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let b = text.as_bytes();
    let mut start = 0;
    let mut j = 0;
    let mut depth = 0;
    let mut take = |part: &str| {
        if let Some((name, value)) = part.split_once(':') {
            let name = name.trim();
            if let Some(name) = name.strip_prefix("--") {
                out.push((name.to_string(), value.trim().to_string()));
            }
        }
    };
    while j < b.len() {
        match b[j] {
            b'/' if b.get(j + 1) == Some(&b'*') => {
                j = skip(b, j);
                continue;
            }
            b'"' | b'\'' => {
                j = skip(b, j);
                continue;
            }
            b'(' => depth += 1,
            b')' => depth -= 1,
            b';' if depth == 0 => {
                take(&text[start..j]);
                start = j + 1;
            }
            _ => {}
        }
        j += 1;
    }
    take(&text[start..]);
    out
}

/// Line and column (from 1, in chars) of a byte offset.
fn place(text: &str, at: usize) -> (usize, usize) {
    let before = &text[..at.min(text.len())];
    let line = before.matches('\n').count() + 1;
    let column = before.rsplit('\n').next().unwrap_or("").chars().count() + 1;
    (line, column)
}

/// `path` relative to the folder of `file`, or as written when it starts
/// with `/`.
fn join(file: &str, path: &str) -> String {
    if path.starts_with('/') {
        return path.to_string();
    }
    let mut parts: Vec<&str> = match file.rsplit_once('/') {
        Some((dir, _)) => dir.split('/').collect(),
        None => Vec::new(),
    };
    for part in path.split('/') {
        match part {
            "." | "" => {}
            ".." if parts.last().is_some_and(|p| *p != "..") => {
                parts.pop();
            }
            _ => parts.push(part),
        }
    }
    parts.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn css(source: &str) -> String {
        compile(source, &Options::default())
            .unwrap_or_else(|e| panic!("{e}"))
            .css
    }

    fn error(source: &str) -> Error {
        compile(source, &Options::default()).unwrap_err()
    }

    fn with_classes(source: &str, classes: &[&str]) -> String {
        let classes: BTreeSet<String> = classes.iter().map(|c| c.to_string()).collect();
        let options = Options {
            classes: &classes,
            ..Options::default()
        };
        compile(source, &options)
            .unwrap_or_else(|e| panic!("{e}"))
            .css
    }

    #[test]
    fn neapolitan_css_is_translated() {
        assert_eq!(
            css("cascett > chiacchier:passanno {\n  culore: russo\n  spazio: 8px\n}"),
            "div > p:hover {\n  color: red;\n  padding: 8px\n}"
        );
        assert_eq!(
            css("@quanno (larghezza-massima: 600px) { core { mostra: nisciuno } }"),
            "@media (max-width: 600px) { main { display: none } }"
        );
        assert_eq!(
            css(".a { mostra: flessibile-in-linea }"),
            ".a { display: inline-flex }"
        );
        // Names without a Neapolitan word and class names pass as they are
        assert_eq!(
            css(".red > pizza-card:nth-child(2) { grid-column: 1 / 3 }"),
            ".red > pizza-card:nth-child(2) { grid-column: 1 / 3 }"
        );
    }

    #[test]
    fn english_is_an_error_with_the_word_to_write() {
        let e = error(".carta {\n  color: russo\n}");
        assert_eq!(e.message, "'color' nun è napoletano.");
        assert_eq!(e.suggestion.as_deref(), Some("Vuoi dire 'culore'?"));
        assert_eq!((e.line, e.column), (2, 3));
        let e = error("citofono:hover { culore: russo }");
        assert_eq!(e.message, "':hover' nun è napoletano.");
        assert_eq!(e.suggestion.as_deref(), Some("Vuoi dire ':passanno'?"));
        let e = error("chiacchier::before { culore: russo }");
        assert_eq!(e.suggestion.as_deref(), Some("Vuoi dire '::primma'?"));
        let e = error("@media (larghezza-massima: 600px) { core { spazio: 0 } }");
        assert_eq!(e.suggestion.as_deref(), Some("Vuoi dire '@quanno'?"));
        let e = error("div { culore: russo }");
        assert_eq!(e.suggestion.as_deref(), Some("Vuoi dire 'cascett'?"));
        let e = error("cascett { culore: red }");
        assert_eq!(e.suggestion.as_deref(), Some("Vuoi dire 'russo'?"));
        // Syntax errors come from stile, with their place
        let e = error(".a {\n  culore russo\n}");
        assert!(e.message.contains("nun è na proprietà"), "{}", e.message);
        assert_eq!((e.line, e.column), (2, 3));
    }

    #[test]
    fn classes_come_with_the_theme_and_the_reset() {
        let out = with_classes(
            "@chiamm \"par-o-css\";\n.carta { spazio: 16px }\n",
            &["flessibile", "culore-russo-500", "menu", "p-4"],
        );
        assert!(
            out.starts_with(
                ":root {\n  --culore-russo-500: oklch(63.7% 0.237 25.331);\n  --scrittura-sans:"
            ),
            "{out}"
        );
        assert!(out.contains("@layer pulizia {"));
        assert!(out.contains("  .flessibile {\n    display: flex;\n  }\n"));
        assert!(out.contains("  .culore-russo-500 {\n    color: var(--culore-russo-500);\n  }\n"));
        assert!(!out.contains(".menu"));
        assert!(!out.contains(".p-4"));
        assert!(out.ends_with("}\n.carta { padding: 16px }\n"), "{out}");
        // Without `@chiamm "par-o-css"` there are no classes
        assert_eq!(
            with_classes(".carta { spazio: 16px }", &["flessibile"]),
            ".carta { padding: 16px }"
        );
    }

    #[test]
    fn the_theme_can_change() {
        let out = with_classes(
            "@chiamm \"par-o-css\"\n@tema {\n  --culore-pummarola-500: russo\n  --culore-russo-500: #c00\n}\n",
            &["sfunno-pummarola-500", "culore-russo-500"],
        );
        // The colours of @tema are the @tema's: not written twice
        assert!(!out.contains("--culore-russo-500: oklch"), "{out}");
        assert!(
            out.contains(":root {\n  --culore-pummarola-500: red;\n  --culore-russo-500: #c00\n}"),
            "{out}"
        );
        assert!(out.contains(
            ".sfunno-pummarola-500 {\n    background-color: var(--culore-pummarola-500);"
        ));
        // English in @tema is an error too
        let e = error("@tema { --culore-pummarola-500: red }");
        assert_eq!(e.suggestion.as_deref(), Some("Vuoi dire 'russo'?"));
    }

    #[test]
    fn applica_writes_the_declarations_of_classes() {
        assert_eq!(
            css(".carta {\n  @applica spazio-4 passanno:sfunno-russo-600\n  culore: niro\n}"),
            ":root {\n  --culore-russo-600: oklch(57.7% 0.245 27.325);\n  --spazio: 0.25rem;\n}\n.carta {\n  padding: calc(var(--spazio) * 4);\n  &:hover { background-color: var(--culore-russo-600); }\n  color: black\n}"
        );
        let e = error(".carta {\n  @applica flessibile p-4\n}");
        assert_eq!(e.message, "'p-4' nun è na classe 'e par-o-css.");
        assert_eq!(e.suggestion.as_deref(), Some("Vuoi dire 'spazio-4'?"));
        assert_eq!((e.line, e.column), (2, 23));
        assert_eq!(error(".carta { @applica pizza }").suggestion, None);
        assert_eq!(
            css(".carta { @applica bordo }"),
            ".carta { border-width: 1px; }"
        );
    }

    #[test]
    fn chiamm_pastes_files_and_keeps_plain_imports() {
        let read = |path: &str| match path {
            "pezzi/carta.cess" => {
                Some(".carta { culore: russo }\n@chiamm \"bottone.cess\"".to_string())
            }
            "pezzi/bottone.cess" => Some("citofono { spazio: 0 }".to_string()),
            "giro.cess" => Some("@chiamm \"giro.cess\";".to_string()),
            "sbagliato.cess" => Some(".a { color: red }".to_string()),
            _ => None,
        };
        let classes = BTreeSet::new();
        let compile_with = |source: &str| {
            compile(
                source,
                &Options {
                    filename: "stile.cess",
                    read: &read,
                    classes: &classes,
                },
            )
        };
        let out = compile_with("@chiamm \"https://fonts.example/a.css\";\n@chiamm \"pezzi/carta.cess\";\ncore { spazio: 0 }").unwrap();
        assert_eq!(
            out.css,
            "@import \"https://fonts.example/a.css\";\n.carta { color: red }\nbutton { padding: 0 }\nmain { padding: 0 }"
        );
        assert_eq!(out.dependencies, ["pezzi/carta.cess", "pezzi/bottone.cess"]);
        let e = compile_with("@chiamm \"giro.cess\"").unwrap_err();
        assert!(
            e.message.contains("se chiamma a sé stesso"),
            "{}",
            e.message
        );
        let e = compile_with("@chiamm \"nisciuno.cess\"").unwrap_err();
        assert_eq!(e.message, "Nun trovo 'nisciuno.cess'.");
        // An error in a pasted file names that file
        let e = compile_with("\n@chiamm \"sbagliato.cess\"").unwrap_err();
        assert_eq!(
            (e.file.as_str(), e.line, e.column),
            ("sbagliato.cess", 1, 6)
        );
        let e = error("@chiamm par-o-css;");
        assert!(e.message.contains("tra virgolette"));
    }

    #[test]
    fn directives_skip_strings_and_comments() {
        assert_eq!(
            css("/* @chiamm \"x.cess\" */ .a::doppo { contenuto: \"@applica pizza\" }"),
            "/* @chiamm \"x.cess\" */ .a::after { content: \"@applica pizza\" }"
        );
    }

    #[test]
    fn pieces_are_the_files_pasted_in() {
        assert_eq!(
            pieces(
                "@chiamm \"par-o-css\"\n@chiamm \"pezzi/carta.cess\"\n/* @chiamm \"no.cess\" */\n.a { culore: russo }\n@chiamm \"a.css\";",
                "sito/stile.cess"
            ),
            ["sito/pezzi/carta.cess"]
        );
    }

    #[test]
    fn paths_join_the_folder_of_the_file() {
        assert_eq!(join("stile.cess", "pezzi/a.cess"), "pezzi/a.cess");
        assert_eq!(join("sito/stile.cess", "../a.cess"), "a.cess");
        assert_eq!(join("sito/pezzi/b.cess", "./a.cess"), "sito/pezzi/a.cess");
        assert_eq!(join("sito/stile.cess", "/a.cess"), "/a.cess");
    }
}
