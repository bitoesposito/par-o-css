//! `parocss fmt`: puts a `.cess` file in order, changing only spaces and
//! new lines. Two spaces for each open `{`, the `{` at the end of the line
//! of its selector, the `}` on a line of its own, one declaration per line
//! where they are split by `;`, no spaces at the end of a line, at most one
//! blank line in a row and none right after `{` or before `}`. Strings and
//! comments stay as they are.

use fratm_core::stile;

use crate::Error;

/// The formatted text, or the syntax error that stops it: a file that does
/// not compile is left alone until the error is fixed.
pub fn format(source: &str, filename: &str) -> Result<String, Error> {
    if let Err(e) = stile::translate(&[source.to_string()]) {
        return Err(Error {
            message: e.message,
            file: filename.to_string(),
            line: e.position.line + 1,
            column: e.position.column + 1,
            suggestion: None,
        });
    }
    let mut out = Writer::default();
    let chars: Vec<char> = source.chars().collect();
    let mut parens = 0usize;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        match c {
            '/' if chars.get(i + 1) == Some(&'*') => {
                let end = (i + 2..chars.len().saturating_sub(1))
                    .find(|&j| chars[j] == '*' && chars[j + 1] == '/')
                    .map_or(chars.len(), |j| j + 2);
                out.text(&chars[i..end]);
                i = end;
                continue;
            }
            '"' | '\'' => {
                let mut j = i + 1;
                while j < chars.len() && chars[j] != c && chars[j] != '\n' {
                    if chars[j] == '\\' {
                        j += 1;
                    }
                    j += 1;
                }
                let end = (j + 1).min(chars.len());
                out.text(&chars[i..end]);
                i = end;
                continue;
            }
            '(' | '[' => {
                parens += 1;
                out.char(c);
            }
            ')' | ']' => {
                parens = parens.saturating_sub(1);
                out.char(c);
            }
            '{' if parens == 0 => out.open(),
            '}' if parens == 0 => out.close(),
            ';' if parens == 0 => {
                out.char(';');
                out.end_line();
            }
            '\n' => out.new_line(),
            c => out.char(c),
        }
        i += 1;
    }
    Ok(out.finish())
}

/// Writes the formatted lines, one piece at a time.
#[derive(Default)]
struct Writer {
    lines: Vec<String>,
    /// The line being written, without its indent
    line: String,
    depth: usize,
    /// New lines in the source since the last code: two make a blank line
    breaks: usize,
    /// A blank line is due before the next line
    blank: bool,
    /// The last line left a value open (it ends with `,` or `:`): the next
    /// one continues it, one step further in
    value_open: bool,
}

impl Writer {
    fn char(&mut self, c: char) {
        if !(self.line.is_empty() && c.is_whitespace()) {
            self.line.push(c);
            self.breaks = 0;
        }
    }

    fn text(&mut self, text: &[char]) {
        self.line.extend(text);
        self.breaks = 0;
    }

    /// Ends the line being written, if it has anything.
    fn end_line(&mut self) {
        let text = self.line.trim_end().to_string();
        self.line.clear();
        if text.is_empty() {
            return;
        }
        let after_open = self.lines.last().is_some_and(|l| l.ends_with('{'));
        if self.blank && !self.lines.is_empty() && !after_open {
            self.lines.push(String::new());
        }
        self.blank = false;
        let continues = self.value_open && !text.ends_with('{');
        let indent = "  ".repeat(self.depth + usize::from(continues));
        // A declaration, or a line of its value, that ends with `,` or `:`
        // goes on in the next line
        self.value_open =
            self.depth > 0 && text.ends_with([',', ':']) && (continues || text.contains(':'));
        // A comment over several lines keeps the indent of its inner lines
        self.lines.push(format!("{indent}{text}"));
    }

    fn new_line(&mut self) {
        self.end_line();
        self.breaks += 1;
        if self.breaks >= 2 {
            self.blank = true;
        }
    }

    fn open(&mut self) {
        self.value_open = false;
        let text = self.line.trim_end().to_string();
        self.line = if text.is_empty() {
            "{".to_string()
        } else {
            format!("{text} {{")
        };
        self.end_line();
        self.depth += 1;
        self.breaks = 0;
    }

    fn close(&mut self) {
        self.end_line();
        self.value_open = false;
        self.depth = self.depth.saturating_sub(1);
        // No blank line right before `}`
        self.blank = false;
        self.line = "}".to_string();
        self.end_line();
        self.breaks = 0;
    }

    fn finish(mut self) -> String {
        self.end_line();
        let mut out = self.lines.join("\n");
        out.push('\n');
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fmt(source: &str) -> String {
        format(source, "stile.cess").unwrap_or_else(|e| panic!("{e}"))
    }

    #[test]
    fn indents_splits_and_keeps_words() {
        assert_eq!(
            fmt(".carta{culore: russo; spazio:8px}\n\n\n\ncitofono:passanno {\n      sfunno-culore: giallo\n\n}\n"),
            ".carta {\n  culore: russo;\n  spazio:8px\n}\n\ncitofono:passanno {\n  sfunno-culore: giallo\n}\n"
        );
        assert_eq!(
            fmt("@chiamm \"par-o-css\"\n@quanno (larghezza-massima: 600px) {\ncore {\n@applica spazio-4 md:spazio-8\n}\n}"),
            "@chiamm \"par-o-css\"\n@quanno (larghezza-massima: 600px) {\n  core {\n    @applica spazio-4 md:spazio-8\n  }\n}\n"
        );
    }

    #[test]
    fn a_value_over_several_lines_is_one_step_further_in() {
        let text = ".a {\n  ombra:\n    0 1px 2px russo,\n    0 2px 4px niro\n  culore: russo\n}\n\nchiacchier,\ncascett {\n  spazio: 0\n}\n";
        assert_eq!(fmt(text), text);
        assert_eq!(
            fmt(".a {\nombra: 0 1px 2px russo,\n0 2px 4px niro\n}"),
            ".a {\n  ombra: 0 1px 2px russo,\n    0 2px 4px niro\n}\n"
        );
    }

    #[test]
    fn strings_comments_and_parentheses_stay() {
        let text = "/* na nota { ; } */\n.a {\n  contenuto: \"{ ; }\"\n  sfunno: url(a;b.png)\n}\n";
        assert_eq!(fmt(text), text);
    }

    #[test]
    fn formatting_twice_changes_nothing() {
        let once = fmt("\n\n.a{culore:russo}\n.b {  }\n\n\n/* c */ .c { spazio: 0 ;}");
        assert_eq!(fmt(&once), once);
    }

    #[test]
    fn a_file_with_a_syntax_error_is_left_alone() {
        let e = format(".a {\n  culore: russo\n", "stile.cess").unwrap_err();
        assert!(e.message.starts_with("manca na '}'"), "{}", e.message);
        // English is not a syntax error: the file is formatted, check says the rest
        assert_eq!(fmt(".a{color:red}"), ".a {\n  color:red\n}\n");
    }
}
