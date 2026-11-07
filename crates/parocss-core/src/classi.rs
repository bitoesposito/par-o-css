//! The classes: `flessibile`, `spazio-4`, `culore-russo-500/50`,
//! `md:passanno:sfunno-blu-600`, `larghezza-[37px]`.
//!
//! A class is a list of variants and a body, separated by `:`. The body is
//! an alias (`flessibile`), a root with a value (`spazio` and `4`), or a
//! property of `stile` with one of its values (`giustifica-spartuto`) or an
//! arbitrary one in brackets. Sizes and breakpoints use Tailwind's codes
//! (`sm`, `md`, `lg`, `2xl`), and every root knows its Tailwind name, so a
//! Tailwind class can suggest the class to write instead.

use std::collections::{BTreeMap, BTreeSet};

use fratm_core::stile;

use crate::tema::THEME;

/// The custom properties of the theme (names without `--`): the starting
/// ones of [`THEME`] and the ones a `@tema` adds or changes.
#[derive(Debug, Clone, Default)]
pub struct Theme {
    /// Set by `@tema`: written by the `.cess` itself, so never emitted again
    pub user: BTreeMap<String, String>,
}

impl Theme {
    fn get(&self, name: &str) -> Option<&str> {
        self.user.get(name).map(String::as_str).or_else(|| {
            THEME
                .iter()
                .find(|(n, _)| *n == name)
                .map(|(_, value)| *value)
        })
    }

    fn has(&self, name: &str) -> bool {
        self.get(name).is_some()
    }

    /// The breakpoints, smallest first: (code, width).
    fn breakpoints(&self) -> Vec<(String, String)> {
        let mut all: BTreeMap<String, String> = THEME
            .iter()
            .map(|(n, v)| (n.to_string(), v.to_string()))
            .collect();
        all.extend(self.user.clone());
        let mut points: Vec<(String, String)> = all
            .into_iter()
            .filter_map(|(n, v)| Some((n.strip_prefix("schermo-")?.to_string(), v)))
            .collect();
        points.sort_by(|a, b| rem(&a.1).total_cmp(&rem(&b.1)));
        points
    }

    /// `:root` with the starting values of the custom properties in `used`,
    /// except the ones a `@tema` writes.
    pub fn root(&self, used: &BTreeSet<String>) -> String {
        let mut out = String::new();
        for (name, value) in THEME {
            if used.contains(*name) && !self.user.contains_key(*name) {
                out.push_str(&format!("  --{name}: {value};\n"));
            }
        }
        if out.is_empty() {
            return out;
        }
        format!(":root {{\n{out}}}\n")
    }
}

/// A length in rem, for sorting breakpoints (`px` counts as 1/16 rem).
fn rem(value: &str) -> f64 {
    let number: String = value
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.')
        .collect();
    let n = number.parse().unwrap_or(f64::MAX);
    if value.ends_with("px") {
        n / 16.0
    } else {
        n
    }
}

/// How a root reads its value.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Kind {
    /// `4` is `calc(var(--spazio) * 4)`, `px` is `1px`
    Spacing,
    /// `1/2` is `calc(1 / 2 * 100%)`
    Fraction,
    /// A colour of the theme: `russo-500`, `janco`, `trasparente`, with an
    /// opacity after `/`: `russo-500/50`
    Color,
    /// A key of the theme under this name: `lg` for `bordo-tunno` is
    /// `var(--bordo-tunno-lg)`
    Theme(&'static str),
    /// A whole number, as it is
    Integer,
    /// `50` is `50%`
    Percent,
    /// `3` is `repeat(3, minmax(0, 1fr))`
    Columns,
    /// `16/9` is `16 / 9`
    Ratio,
    /// `2` is `2px`
    Pixels,
    /// A value word of `stile`: `automatico` is `auto`
    Keyword,
}

use Kind::*;

/// A root with a value: `spazio-4`, `culore-russo-500`, `bordo-tunno-lg`.
struct Root {
    nap: &'static str,
    /// The Tailwind root, for the suggestions: `p` for `spazio`
    tw: &'static str,
    /// The properties that get the value. One with a `:` is a declaration
    /// of its own: `outline-style: solid` goes with `outline-width`.
    props: &'static [&'static str],
    kinds: &'static [Kind],
    /// Values with a meaning of their own for this root: `chino` is `100%`
    fixed: &'static [(&'static str, &'static str)],
    /// Accepts a `-` in front: `-margine-ncoppa-4`
    negative: bool,
}

const fn root(
    nap: &'static str,
    tw: &'static str,
    props: &'static [&'static str],
    kinds: &'static [Kind],
) -> Root {
    Root {
        nap,
        tw,
        props,
        kinds,
        fixed: &[],
        negative: false,
    }
}

impl Root {
    const fn fixed(mut self, fixed: &'static [(&'static str, &'static str)]) -> Self {
        self.fixed = fixed;
        self
    }

    const fn negative(mut self) -> Self {
        self.negative = true;
        self
    }
}

const SIZE: &[Kind] = &[Spacing, Fraction, Keyword];
const WIDTH: &[Kind] = &[Spacing, Fraction, Theme("contenitore"), Keyword];
const FULL_X: &[(&str, &str)] = &[("chino", "100%"), ("schermo", "100vw")];
const FULL_Y: &[(&str, &str)] = &[("chino", "100%"), ("schermo", "100vh")];

/// The roots, in the order of their rules in the CSS: a later one wins, so
/// `spazio-ncoppa-2` comes after `spazio-4`.
const ROOTS: &[Root] = &[
    root("ncoppa", "top", &["top"], SIZE)
        .fixed(&[("chino", "100%")])
        .negative(),
    root("sotto", "bottom", &["bottom"], SIZE)
        .fixed(&[("chino", "100%")])
        .negative(),
    root("manca", "left", &["left"], SIZE)
        .fixed(&[("chino", "100%")])
        .negative(),
    root("destra", "right", &["right"], SIZE)
        .fixed(&[("chino", "100%")])
        .negative(),
    root("livello", "z", &["z-index"], &[Integer, Keyword]).negative(),
    root("flessibile", "flex", &["flex"], &[Integer, Keyword]),
    root("cresce", "grow", &["flex-grow"], &[Integer]),
    root(
        "colonne",
        "grid-cols",
        &["grid-template-columns"],
        &[Columns, Keyword],
    ),
    root(
        "righe",
        "grid-rows",
        &["grid-template-rows"],
        &[Columns, Keyword],
    ),
    root("spazio-mmiez", "gap", &["gap"], &[Spacing]),
    root("spazio-mmiez-x", "gap-x", &["column-gap"], &[Spacing]),
    root("spazio-mmiez-y", "gap-y", &["row-gap"], &[Spacing]),
    root("spazio", "p", &["padding"], &[Spacing]),
    root("spazio-x", "px", &["padding-inline"], &[Spacing]),
    root("spazio-y", "py", &["padding-block"], &[Spacing]),
    root("spazio-ncoppa", "pt", &["padding-top"], &[Spacing]),
    root("spazio-sotto", "pb", &["padding-bottom"], &[Spacing]),
    root("spazio-manca", "pl", &["padding-left"], &[Spacing]),
    root("spazio-destra", "pr", &["padding-right"], &[Spacing]),
    root("margine", "m", &["margin"], &[Spacing, Keyword]).negative(),
    root("margine-x", "mx", &["margin-inline"], &[Spacing, Keyword]).negative(),
    root("margine-y", "my", &["margin-block"], &[Spacing, Keyword]).negative(),
    root("margine-ncoppa", "mt", &["margin-top"], &[Spacing, Keyword]).negative(),
    root(
        "margine-sotto",
        "mb",
        &["margin-bottom"],
        &[Spacing, Keyword],
    )
    .negative(),
    root("margine-manca", "ml", &["margin-left"], &[Spacing, Keyword]).negative(),
    root(
        "margine-destra",
        "mr",
        &["margin-right"],
        &[Spacing, Keyword],
    )
    .negative(),
    root("larghezza", "w", &["width"], WIDTH).fixed(FULL_X),
    root("larghezza-minima", "min-w", &["min-width"], WIDTH).fixed(FULL_X),
    root("larghezza-massima", "max-w", &["max-width"], WIDTH).fixed(FULL_X),
    root("altezza", "h", &["height"], SIZE).fixed(FULL_Y),
    root("altezza-minima", "min-h", &["min-height"], SIZE).fixed(FULL_Y),
    root("altezza-massima", "max-h", &["max-height"], SIZE).fixed(FULL_Y),
    root("scrittura", "font", &["font-family"], &[Theme("scrittura")]),
    root(
        "scrittura-grannezza",
        "text",
        &["font-size"],
        &[Theme("scrittura-grannezza")],
    ),
    root(
        "scrittura-peso",
        "font",
        &["font-weight"],
        &[Integer, Keyword],
    ),
    root(
        "altezza-riga",
        "leading",
        &["line-height"],
        &[Spacing, Keyword],
    ),
    root(
        "spazio-lettere",
        "tracking",
        &["letter-spacing"],
        &[Keyword],
    ),
    root("culore", "text", &["color"], &[Color]),
    root("sfunno", "bg", &["background-color"], &[Color]),
    root("sfunno-culore", "bg", &["background-color"], &[Color]),
    root("bordo", "border", &["border-color"], &[Color]),
    root("bordo", "border", &["border-width"], &[Pixels]),
    root("bordo-culore", "border", &["border-color"], &[Color]),
    root("bordo-larghezza", "border", &["border-width"], &[Pixels]),
    root("bordo-x", "border-x", &["border-inline-color"], &[Color]),
    root("bordo-x", "border-x", &["border-inline-width"], &[Pixels]),
    root("bordo-y", "border-y", &["border-block-color"], &[Color]),
    root("bordo-y", "border-y", &["border-block-width"], &[Pixels]),
    root("bordo-ncoppa", "border-t", &["border-top-color"], &[Color]),
    root("bordo-ncoppa", "border-t", &["border-top-width"], &[Pixels]),
    root(
        "bordo-sotto",
        "border-b",
        &["border-bottom-color"],
        &[Color],
    ),
    root(
        "bordo-sotto",
        "border-b",
        &["border-bottom-width"],
        &[Pixels],
    ),
    root("bordo-manca", "border-l", &["border-left-color"], &[Color]),
    root("bordo-manca", "border-l", &["border-left-width"], &[Pixels]),
    root(
        "bordo-destra",
        "border-r",
        &["border-right-color"],
        &[Color],
    ),
    root(
        "bordo-destra",
        "border-r",
        &["border-right-width"],
        &[Pixels],
    ),
    root(
        "bordo-tunno",
        "rounded",
        &["border-radius"],
        &[Theme("bordo-tunno")],
    )
    .fixed(&[("chino", "calc(infinity * 1px)"), ("nisciuno", "0")]),
    root("ombra", "shadow", &["box-shadow"], &[Theme("ombra")]).fixed(&[("nisciuno", "0 0 #0000")]),
    root("opacità", "opacity", &["opacity"], &[Percent]),
    root("opacita", "opacity", &["opacity"], &[Percent]),
    root("contorno", "outline", &["outline-color"], &[Color]),
    root(
        "contorno",
        "outline",
        &["outline-style: solid", "outline-width"],
        &[Pixels],
    ),
    root(
        "proporzione",
        "aspect",
        &["aspect-ratio"],
        &[Ratio, Keyword],
    ),
    // Last, so that its `content` wins over the empty one of `primma:`
    root("contenuto", "content", &["content"], &[Keyword]),
];

/// Classes that are a word of their own, like Tailwind's `flex` and
/// `hidden`: (class, Tailwind class, declarations).
const ALIASES: &[(&str, &str, &str)] = &[
    ("blocco", "block", "display: block"),
    ("in-linea", "inline", "display: inline"),
    ("blocco-in-linea", "inline-block", "display: inline-block"),
    ("flessibile", "flex", "display: flex"),
    ("flessibile-in-linea", "inline-flex", "display: inline-flex"),
    ("griglia", "grid", "display: grid"),
    ("griglia-in-linea", "inline-grid", "display: inline-grid"),
    ("annascuso", "hidden", "display: none"),
    (
        "annascuso-all-uocchie",
        "sr-only",
        "position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip-path: inset(50%); white-space: nowrap; border-width: 0",
    ),
    ("statico", "static", "position: static"),
    ("fisso", "fixed", "position: fixed"),
    ("assoluto", "absolute", "position: absolute"),
    ("relativo", "relative", "position: relative"),
    ("appiccecato", "sticky", "position: sticky"),
    ("a-capo", "flex-wrap", "flex-wrap: wrap"),
    ("nun-va-a-capo", "flex-nowrap", "flex-wrap: nowrap"),
    ("cresce", "grow", "flex-grow: 1"),
    ("grassetto", "font-bold", "font-weight: bold"),
    ("corsivo", "italic", "font-style: italic"),
    ("sottolineato", "underline", "text-decoration-line: underline"),
    ("sbarrato", "line-through", "text-decoration-line: line-through"),
    ("maiuscolo", "uppercase", "text-transform: uppercase"),
    ("minuscolo", "lowercase", "text-transform: lowercase"),
    ("bordo", "border", "border-width: 1px"),
    ("bordo-tunno", "rounded", "border-radius: var(--bordo-tunno-sm)"),
    ("ombra", "shadow", "box-shadow: var(--ombra-sm)"),
    ("contorno", "outline", "outline-style: solid; outline-width: 1px"),
    (
        "transizione",
        "transition",
        "transition-property: color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, translate, scale, rotate, filter, backdrop-filter; transition-timing-function: var(--transizione-andatura); transition-duration: var(--transizione-durata)",
    ),
];

/// Tailwind classes with a value that has a name of its own in Tailwind,
/// for the suggestions: (Tailwind class, class).
const TAILWIND: &[(&str, &str)] = &[
    ("flex-row", "direzione-riga"),
    ("flex-col", "direzione-colonna"),
    ("items-start", "allinea-cose-inizio"),
    ("items-center", "allinea-cose-miezo"),
    ("items-end", "allinea-cose-fine"),
    ("items-stretch", "allinea-cose-stiracchiato"),
    ("self-start", "allinea-me-inizio"),
    ("self-center", "allinea-me-miezo"),
    ("self-end", "allinea-me-fine"),
    ("self-stretch", "allinea-me-stiracchiato"),
    ("justify-start", "giustifica-inizio"),
    ("justify-center", "giustifica-miezo"),
    ("justify-end", "giustifica-fine"),
    ("justify-between", "giustifica-spartuto"),
    ("justify-around", "giustifica-attuorno"),
    ("text-left", "allinea-testo-manca"),
    ("text-center", "allinea-testo-miezo"),
    ("text-right", "allinea-testo-destra"),
    ("font-normal", "scrittura-peso-normale"),
    ("font-thin", "scrittura-peso-100"),
    ("font-extralight", "scrittura-peso-200"),
    ("font-light", "scrittura-peso-300"),
    ("font-medium", "scrittura-peso-500"),
    ("font-semibold", "scrittura-peso-600"),
    ("font-extrabold", "scrittura-peso-800"),
    ("font-black", "scrittura-peso-900"),
    ("no-underline", "decorazione-testo-nisciuno"),
    ("overflow-hidden", "trabocca-nascosto"),
    ("overflow-auto", "trabocca-automatico"),
    ("overflow-scroll", "trabocca-scorre"),
    ("overflow-visible", "trabocca-visibile"),
    ("cursor-pointer", "cursore-manella"),
    ("cursor-wait", "cursore-aspetta"),
    ("pointer-events-none", "puntatore-nisciuno"),
    ("select-none", "selezione-nisciuno"),
    ("object-cover", "adatta-oggetto-copre"),
    ("object-contain", "adatta-oggetto-contiene"),
    ("bg-cover", "sfunno-grannezza-copre"),
    ("bg-contain", "sfunno-grannezza-contiene"),
    ("list-none", "lista-stile-nisciuno"),
    ("invisible", "visibilità-nascosto"),
    ("visible", "visibilità-visibile"),
    ("box-border", "scatola-bordo-incluso"),
    ("float-left", "galleggia-manca"),
    ("float-right", "galleggia-destra"),
    ("aspect-square", "proporzione-1/1"),
    ("aspect-video", "proporzione-16/9"),
];

/// Tailwind value names in a functional class: (Tailwind, par-o-css).
const TAILWIND_VALUES: &[(&str, &str)] = &[
    ("full", "chino"),
    ("screen", "schermo"),
    ("auto", "automatico"),
    ("none", "nisciuno"),
    ("red", "russo"),
    ("orange", "arancione"),
    ("yellow", "giallo"),
    ("green", "verde"),
    ("sky", "azzurro"),
    ("blue", "blu"),
    ("purple", "viola"),
    ("pink", "rosa"),
    ("gray", "grigio"),
    ("white", "janco"),
    ("black", "niro"),
    ("transparent", "trasparente"),
    ("current", "culore-corrente"),
];

/// The states, with the pseudo-class words of `stile`: (variant, Tailwind
/// variant, selector).
const STATES: &[(&str, &str, &str)] = &[
    ("passanno", "hover", ":hover"),
    ("premuto", "active", ":active"),
    ("fuoco", "focus", ":focus"),
    ("fuoco-visibile", "focus-visible", ":focus-visible"),
    ("stutato", "disabled", ":disabled"),
    ("spuntato", "checked", ":checked"),
    ("primmo-figlio", "first", ":first-child"),
    ("urdemo-figlio", "last", ":last-child"),
    ("vacante", "empty", ":empty"),
    ("esempio", "placeholder", "::placeholder"),
    ("primma", "before", "::before"),
    ("doppo", "after", "::after"),
];

/// Variants that are a media query: (variant, Tailwind variant, query).
const MEDIA: &[(&str, &str, &str)] = &[
    ("notte", "dark", "(prefers-color-scheme: dark)"),
    ("stampa", "print", "print"),
];

#[derive(Debug, Clone, PartialEq)]
enum Variant {
    /// Added to the selector: `:hover`, `::before`
    Selector(&'static str),
    /// A media query around the rule, with its place among the others
    Media(String, usize),
}

/// A class read: the CSS it stands for and where its rule goes.
#[derive(Debug)]
struct Utility {
    variants: Vec<Variant>,
    declarations: Vec<(String, String)>,
    /// Place of the root among all roots: later rules win
    order: usize,
}

/// The parts of a class separated by `:`, outside brackets.
fn parts(class: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut start = 0;
    for (i, c) in class.char_indices() {
        match c {
            '[' => depth += 1,
            ']' => depth -= 1,
            ':' if depth == 0 => {
                parts.push(&class[start..i]);
                start = i + 1;
            }
            _ => {}
        }
    }
    parts.push(&class[start..]);
    parts
}

fn variant(name: &str, theme: &Theme) -> Option<Variant> {
    if let Some((_, _, selector)) = STATES.iter().find(|(nap, _, _)| *nap == name) {
        return Some(Variant::Selector(selector));
    }
    if let Some(i) = MEDIA.iter().position(|(nap, _, _)| *nap == name) {
        return Some(Variant::Media(MEDIA[i].2.to_string(), 1000 + i));
    }
    let points = theme.breakpoints();
    let i = points.iter().position(|(code, _)| code == name)?;
    Some(Variant::Media(format!("(width >= {})", points[i].1), i + 1))
}

/// Reads a class, or `None` when it is not one of par-o-css.
fn utility(class: &str, theme: &Theme) -> Option<Utility> {
    let parts = parts(class);
    let (body, names) = parts.split_last()?;
    let variants = names
        .iter()
        .map(|n| variant(n, theme))
        .collect::<Option<Vec<_>>>()?;
    let (body, important) = match body.strip_suffix('!') {
        Some(b) => (b, true),
        None => (*body, false),
    };
    let (order, mut declarations) = declarations(body, theme)?;
    if important {
        for (_, value) in &mut declarations {
            value.push_str(" !important");
        }
    }
    let pseudo_element = variants
        .iter()
        .any(|v| matches!(v, Variant::Selector("::before" | "::after")));
    if pseudo_element && !declarations.iter().any(|(p, _)| p == "content") {
        declarations.insert(0, ("content".into(), "\"\"".into()));
    }
    Some(Utility {
        variants,
        declarations,
        order,
    })
}

/// The declarations of the body of a class, with the place of its rule.
fn declarations(body: &str, theme: &Theme) -> Option<(usize, Vec<(String, String)>)> {
    if body.is_empty() {
        return None;
    }
    if let Some(i) = ALIASES.iter().position(|(nap, _, _)| *nap == body) {
        return Some((i, parse_declarations(ALIASES[i].2)));
    }
    let (negative, rest) = match body.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, body),
    };
    // The longest root first: `spazio-x-4` is `spazio-x`, not `spazio`
    let mut roots: Vec<(usize, &Root)> = ROOTS.iter().enumerate().collect();
    roots.sort_by_key(|(_, r)| std::cmp::Reverse(r.nap.len()));
    for (i, root) in roots {
        let Some(value) = rest
            .strip_prefix(root.nap)
            .and_then(|v| v.strip_prefix('-'))
        else {
            continue;
        };
        if negative && !root.negative {
            continue;
        }
        if let Some(value) = root_value(root, value, theme, negative) {
            let mut out = Vec::new();
            for prop in root.props {
                match prop.split_once(':') {
                    Some((p, v)) => out.push((p.trim().to_string(), v.trim().to_string())),
                    None => out.push((prop.to_string(), value.clone())),
                }
            }
            // A text size brings its line height, like Tailwind's
            if root.nap == "scrittura-grannezza" {
                let name = value.trim_start_matches("var(--").trim_end_matches(')');
                let height = format!("{name}--altezza-riga");
                if theme.has(&height) {
                    out.push(("line-height".into(), format!("var(--{height})")));
                }
            }
            return Some((ALIASES.len() + i, out));
        }
    }
    if negative {
        return None;
    }
    // A property of stile with a value in brackets, or with one of its
    // value words when no root has its name: `culore-russo` is not a class,
    // the colours of `culore` are the theme's
    let mut properties: Vec<(usize, &(&str, &str))> =
        stile::PROPERTIES.iter().enumerate().collect();
    properties.sort_by_key(|(_, (nap, _))| std::cmp::Reverse(nap.len()));
    for (i, (nap, css)) in properties {
        let Some(value) = body.strip_prefix(nap).and_then(|v| v.strip_prefix('-')) else {
            continue;
        };
        let has_root = ROOTS.iter().any(|r| r.nap == *nap);
        let found = arbitrary(value).or_else(|| (!has_root).then(|| keyword(value)).flatten());
        if let Some(value) = found {
            return Some((
                ALIASES.len() + ROOTS.len() + i,
                vec![(css.to_string(), value)],
            ));
        }
    }
    // `[proprietà:valore]`
    let inner = body.strip_prefix('[')?.strip_suffix(']')?;
    let (property, value) = inner.split_once(':')?;
    let property = property.trim();
    if property.is_empty()
        || !property.chars().all(|c| c.is_alphanumeric() || c == '-')
        || english(&format!("{property}:x"))
    {
        return None;
    }
    let property = stile::PROPERTIES
        .iter()
        .find(|(nap, _)| *nap == property)
        .map_or(property, |(_, css)| css);
    let value = arbitrary(&format!("[{value}]"))?;
    Some((usize::MAX, vec![(property.to_string(), value)]))
}

/// `"display: flex; color: red"` as pairs.
fn parse_declarations(css: &str) -> Vec<(String, String)> {
    css.split(';')
        .filter_map(|d| d.split_once(':'))
        .map(|(p, v)| (p.trim().to_string(), v.trim().to_string()))
        .collect()
}

/// The CSS value for `value` on `root`.
fn root_value(root: &Root, value: &str, theme: &Theme, negative: bool) -> Option<String> {
    let sign = if negative { "-" } else { "" };
    if let Some(inner) = arbitrary(value) {
        // A root with two meanings (`bordo` is a colour or a width) takes
        // the colour one only for a value that looks like a colour
        if root.kinds.contains(&Color) && !looks_like_color(&inner) {
            return None;
        }
        return Some(if negative {
            format!("calc({inner} * -1)")
        } else {
            inner
        });
    }
    if let Some((_, css)) = root.fixed.iter().find(|(nap, _)| *nap == value) {
        return Some(format!("{sign}{css}"));
    }
    for kind in root.kinds {
        let found = match kind {
            Spacing if value == "px" => Some(format!("{sign}1px")),
            Spacing => number(value).map(|n| format!("calc(var(--spazio) * {sign}{n})")),
            Fraction => fraction(value).map(|(a, b)| format!("calc({sign}{a} / {b} * 100%)")),
            Color if !negative => color(value, theme),
            Theme(space) => {
                let name = format!("{space}-{value}");
                theme.has(&name).then(|| format!("var(--{name})"))
            }
            Integer => integer(value).map(|n| format!("{sign}{n}")),
            Percent if !negative => integer(value).map(|n| format!("{n}%")),
            Columns if !negative => integer(value).map(|n| format!("repeat({n}, minmax(0, 1fr))")),
            Ratio if !negative => fraction(value).map(|(a, b)| format!("{a} / {b}")),
            Pixels if !negative => integer(value).map(|n| format!("{n}px")),
            Keyword if !negative => keyword(value),
            _ => None,
        };
        if found.is_some() {
            return found;
        }
    }
    None
}

fn integer(value: &str) -> Option<&str> {
    (!value.is_empty() && value.len() < 6 && value.chars().all(|c| c.is_ascii_digit()))
        .then_some(value)
}

/// A number like `4` or `0.5` (no `.5`, no `4.`).
fn number(value: &str) -> Option<&str> {
    let (whole, fraction) = value.split_once('.').unwrap_or((value, "0"));
    (integer(whole).is_some() && integer(fraction).is_some()).then_some(value)
}

fn fraction(value: &str) -> Option<(&str, &str)> {
    let (a, b) = value.split_once('/')?;
    Some((integer(a)?, integer(b)?))
}

/// A value word of `stile`, as CSS.
fn keyword(value: &str) -> Option<String> {
    stile::VALUES
        .iter()
        .find(|(nap, _)| *nap == value)
        .map(|(_, css)| css.to_string())
}

/// The value inside `[…]`: `_` is a space (`\_` an underscore) and the
/// Neapolitan words are translated as in a `stile` value. `None` when it
/// has an English word that has a Neapolitan one: `[red]` is `[russo]`.
fn arbitrary(value: &str) -> Option<String> {
    let inner = value.strip_prefix('[')?.strip_suffix(']')?;
    if inner.is_empty() {
        return None;
    }
    let mut text = String::new();
    let mut chars = inner.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' if chars.peek() == Some(&'_') => {
                text.push('_');
                chars.next();
            }
            '_' => text.push(' '),
            c => text.push(c),
        }
    }
    if english(&format!("x:{text}")) {
        return None;
    }
    Some(stile::style_value(&text))
}

/// Whether a declaration has an English name that has a Neapolitan word.
fn english(declaration: &str) -> bool {
    stile::english_words(&[declaration.to_string()]).map_or(true, |w| !w.is_empty())
}

/// The class with the English words inside its brackets written in
/// Neapolitan (`[color:red]` is `[culore:russo]`), or `None` when there are
/// none to change.
fn neapolitan_brackets(class: &str) -> Option<String> {
    let mut out = String::new();
    let mut changed = false;
    let mut rest = class;
    while let Some(open) = rest.find('[') {
        let Some(close) = rest[open..].find(']').map(|c| open + c) else {
            break;
        };
        out.push_str(&rest[..=open]);
        let inner = &rest[open + 1..close];
        let declaration = inner.split_once(':').is_some_and(|(p, _)| {
            !p.is_empty() && p.chars().all(|c| c.is_alphanumeric() || c == '-')
        });
        // `_` and the space are one char each, so the places stay the same
        let (text, skip) = match declaration {
            true => (inner.replace('_', " "), 0),
            false => (format!("x:{}", inner.replace('_', " ")), 2),
        };
        let mut chars: Vec<char> = inner.chars().collect();
        if !inner.contains('\\') {
            let mut words = stile::english_words(&[text]).unwrap_or_default();
            words.sort_by_key(|w| std::cmp::Reverse(w.position.column));
            for w in words {
                let at = w.position.column - skip;
                let len = w.word.chars().count();
                chars.splice(at..at + len, w.neapolitan.chars());
                changed = true;
            }
        }
        out.extend(chars);
        out.push(']');
        rest = &rest[close + 1..];
    }
    out.push_str(rest);
    changed.then_some(out)
}

/// A colour of the theme, `trasparente` or `culore-corrente`, with an
/// optional opacity after `/` (`russo-500/50`), or one in brackets.
fn color(value: &str, theme: &Theme) -> Option<String> {
    let (name, alpha) = match value.rsplit_once('/') {
        Some((name, alpha)) => (name, Some(integer(alpha)?)),
        None => (value, None),
    };
    let base = match name {
        "trasparente" => "transparent".to_string(),
        "culore-corrente" => "currentcolor".to_string(),
        _ => match arbitrary(name) {
            Some(inner) => inner,
            None => {
                let key = format!("culore-{name}");
                theme.has(&key).then(|| format!("var(--{key})"))?
            }
        },
    };
    Some(match alpha {
        Some(a) => format!("color-mix(in oklab, {base} {a}%, transparent)"),
        None => base,
    })
}

/// Whether an arbitrary value is a colour rather than a length.
// ponytail: a guess from the first word; `bordo-[color:…]` hints if this gets it wrong
fn looks_like_color(value: &str) -> bool {
    let v = value.trim().to_ascii_lowercase();
    v.starts_with('#')
        || [
            "rgb(", "rgba(", "hsl(", "hsla(", "oklch(", "oklab(", "lab(", "lch(", "color(",
            "color-mix(", "var(--culore", "transparent", "currentcolor",
        ]
        .iter()
        .any(|p| v.starts_with(p))
        // The colour words of stile come first in its table, up to `nisciuno`
        || stile::VALUES
            .iter()
            .take_while(|(nap, _)| *nap != "nisciuno")
            .any(|(_, css)| *css == v)
}

/// The CSS rules for the classes that are par-o-css's, in the `classi`
/// layer, and the theme properties they use. Other classes are skipped.
pub fn generate<'a>(
    classes: impl IntoIterator<Item = &'a str>,
    theme: &Theme,
) -> (String, BTreeSet<String>) {
    let mut found: Vec<(String, Utility)> = classes
        .into_iter()
        .filter_map(|c| Some((c.to_string(), utility(c, theme)?)))
        .collect();
    found.sort_by(|(a, x), (b, y)| {
        let rank = |u: &Utility| {
            u.variants
                .iter()
                .map(|v| match v {
                    Variant::Media(_, r) => *r,
                    Variant::Selector(_) => 0,
                })
                .max()
                .unwrap_or(0)
        };
        (rank(x), x.variants.len(), x.order, a).cmp(&(rank(y), y.variants.len(), y.order, b))
    });
    found.dedup_by(|a, b| a.0 == b.0);
    let mut used = BTreeSet::new();
    let mut out = String::new();
    // The media queries open around the last rule: rules that share them
    // share the block
    let mut open: Vec<&str> = Vec::new();
    for (class, u) in &found {
        let mut selector = format!(".{}", escape(class));
        let mut media = Vec::new();
        for v in &u.variants {
            match v {
                Variant::Selector(s) => selector.push_str(s),
                Variant::Media(q, _) => media.push(q.as_str()),
            }
        }
        let shared = open.iter().zip(&media).take_while(|(a, b)| a == b).count();
        while open.len() > shared {
            open.pop();
            out.push_str(&format!("{}}}\n", "  ".repeat(open.len() + 1)));
        }
        for q in &media[shared..] {
            out.push_str(&format!("{}@media {q} {{\n", "  ".repeat(open.len() + 1)));
            open.push(q);
        }
        let indent = "  ".repeat(media.len() + 1);
        out.push_str(&format!("{indent}{selector} {{\n"));
        for (p, v) in &u.declarations {
            vars(v, &mut used);
            out.push_str(&format!("{indent}  {p}: {v};\n"));
        }
        out.push_str(&format!("{indent}}}\n"));
    }
    while !open.is_empty() {
        open.pop();
        out.push_str(&format!("{}}}\n", "  ".repeat(open.len() + 1)));
    }
    if out.is_empty() {
        return (out, used);
    }
    (format!("@layer classi {{\n{out}}}\n"), used)
}

/// The declarations of a class for `@applica`, one per line, to put inside
/// a rule: variants become a nested rule on a line of its own
/// (`&:hover { … }`, `@media … { … }`). `None` when it is not a class of
/// par-o-css.
pub fn apply(class: &str, theme: &Theme, used: &mut BTreeSet<String>) -> Option<Vec<String>> {
    let u = utility(class, theme)?;
    let declarations: Vec<String> = u
        .declarations
        .iter()
        .map(|(p, v)| {
            vars(v, used);
            format!("{p}: {v};")
        })
        .collect();
    let selector: String = u
        .variants
        .iter()
        .filter_map(|v| match v {
            Variant::Selector(s) => Some(*s),
            Variant::Media(..) => None,
        })
        .collect();
    let mut open: Vec<String> = u
        .variants
        .iter()
        .filter_map(|v| match v {
            Variant::Media(q, _) => Some(format!("@media {q}")),
            Variant::Selector(_) => None,
        })
        .collect();
    if !selector.is_empty() {
        open.push(format!("&{selector}"));
    }
    if open.is_empty() {
        return Some(declarations);
    }
    let heads: Vec<String> = open.iter().map(|o| format!("{o} {{ ")).collect();
    Some(vec![format!(
        "{}{}{}",
        heads.concat(),
        declarations.join(" "),
        " }".repeat(open.len())
    )])
}

/// Notes the custom properties a value uses.
pub(crate) fn vars(value: &str, used: &mut BTreeSet<String>) {
    let mut rest = value;
    while let Some(at) = rest.find("var(--") {
        rest = &rest[at + 6..];
        let end = rest
            .find(|c: char| !(c.is_alphanumeric() || c == '-' || c == '_'))
            .unwrap_or(rest.len());
        used.insert(rest[..end].to_string());
        rest = &rest[end..];
    }
}

/// A class name as a CSS selector: `md:spazio-0.5` is `md\:spazio-0\.5`.
fn escape(class: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = class.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        let digit_first = c.is_ascii_digit() && (i == 0 || (i == 1 && chars[0] == '-'));
        if digit_first || c.is_control() {
            out.push_str(&format!("\\{:x} ", c as u32));
        } else if c.is_ascii_alphanumeric() || c == '-' || c == '_' || !c.is_ascii() {
            out.push(c);
        } else {
            out.push('\\');
            out.push(c);
        }
    }
    if out == "-" {
        return "\\-".into();
    }
    out
}

/// Whether `class` is a class of par-o-css.
pub fn is_class(class: &str, theme: &Theme) -> bool {
    utility(class, theme).is_some()
}

/// The par-o-css class for a Tailwind class (`p-4` is `spazio-4`,
/// `hover:bg-red-500` is `passanno:sfunno-russo-500`), or `None` when it
/// is not a Tailwind class we know, or already a par-o-css class.
pub fn suggestion(class: &str) -> Option<String> {
    let theme = Theme::default();
    if is_class(class, &theme) {
        return None;
    }
    // English inside the brackets: `[color:red]` is `[culore:russo]`
    let fixed = neapolitan_brackets(class);
    let class = fixed.as_deref().unwrap_or(class);
    if fixed.is_some() && is_class(class, &theme) {
        return Some(class.to_string());
    }
    let parts = parts(class);
    let (body, names) = parts.split_last()?;
    let mut out: Vec<String> = Vec::new();
    for name in names {
        let nap = STATES
            .iter()
            .chain(MEDIA)
            .find(|(nap, tw, _)| tw == name || nap == name)
            .map(|(nap, _, _)| nap.to_string())
            .unwrap_or_else(|| name.to_string());
        out.push(nap);
    }
    let (body, important) = match body.strip_suffix('!') {
        Some(b) => (b, "!"),
        None => (*body, ""),
    };
    out.push(format!("{}{important}", tailwind_body(body, &theme)?));
    let class = out.join(":");
    is_class(&class, &theme).then_some(class)
}

/// The par-o-css body for the body of a Tailwind class.
fn tailwind_body(body: &str, theme: &Theme) -> Option<String> {
    if let Some((_, nap)) = TAILWIND.iter().find(|(tw, _)| *tw == body) {
        return Some(nap.to_string());
    }
    if let Some((nap, _, _)) = ALIASES.iter().find(|(_, tw, _)| *tw == body) {
        return Some(nap.to_string());
    }
    if declarations(body, theme).is_some() {
        return Some(body.to_string());
    }
    let (sign, rest) = match body.strip_prefix('-') {
        Some(rest) => ("-", rest),
        None => ("", body),
    };
    let mut roots: Vec<&Root> = ROOTS.iter().collect();
    roots.sort_by_key(|r| std::cmp::Reverse(r.tw.len()));
    for root in roots {
        let Some(value) = rest.strip_prefix(root.tw).and_then(|v| v.strip_prefix('-')) else {
            continue;
        };
        // `red-500/50` is `russo-500/50`
        let (name, alpha) = match value.rsplit_once('/') {
            Some((n, a)) if root.kinds.contains(&Color) => (n, format!("/{a}")),
            _ => (value, String::new()),
        };
        let (first, shade) = name.split_once('-').unwrap_or((name, ""));
        let first = TAILWIND_VALUES
            .iter()
            .find(|(tw, _)| *tw == first)
            .map_or(first, |(_, nap)| nap);
        let name = if shade.is_empty() {
            first.to_string()
        } else {
            format!("{first}-{shade}")
        };
        let value = format!("{name}{alpha}");
        if root_value(root, &value, theme, !sign.is_empty()).is_some() {
            return Some(format!("{sign}{}-{value}", root.nap));
        }
    }
    None
}

/// The words that may be classes in a text (a page, a component, a
/// script): runs of the characters a class can have. Like Tailwind's, the
/// scan does not know the language of the file, so it finds classes in
/// strings built by code too; words that are not classes cost nothing.
pub fn scan(text: &str, into: &mut BTreeSet<String>) {
    let mut word = String::new();
    let mut depth = 0;
    for c in text.chars() {
        let keep = if depth > 0 {
            !c.is_whitespace() && !matches!(c, '"' | '`' | '<' | '>')
        } else {
            c.is_alphanumeric() || matches!(c, '-' | '_' | ':' | '/' | '.' | '!' | '[')
        };
        if keep {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                _ => {}
            }
            word.push(c);
        } else {
            add(&word, into);
            word.clear();
            depth = 0;
        }
    }
    add(&word, into);
}

fn add(word: &str, into: &mut BTreeSet<String>) {
    let word = word.trim_start_matches('.').trim_end_matches(['.', ':']);
    if !word.is_empty()
        && word.len() <= 120
        && word.chars().any(char::is_alphabetic)
        && !word.contains("//")
    {
        into.insert(word.to_string());
    }
}

/// Classes to offer while typing, with the starting theme: the aliases, the
/// curated `proprietà-valore` classes and each root with the usual values
/// (spacing steps, fractions, theme keys, colours with their shades).
pub fn examples() -> Vec<String> {
    let theme = Theme::default();
    let mut values: Vec<String> = [
        "0", "0.5", "1", "1.5", "2", "2.5", "3", "4", "5", "6", "8", "10", "12", "16", "20", "24",
        "32", "40", "48", "64", "96", "px", "1/2", "1/3", "2/3", "1/4", "3/4", "16/9", "1/1", "25",
        "50", "75", "100", "200", "300", "400", "500", "600", "700", "800", "900",
    ]
    .iter()
    .map(|v| v.to_string())
    .collect();
    for (name, _) in THEME {
        if let Some((_, key)) = name.split_once('-') {
            values.push(key.to_string());
            if let Some(color) = key.strip_prefix("russo-") {
                values.push(color.to_string());
            }
        }
    }
    values.extend(stile::VALUES.iter().map(|(nap, _)| nap.to_string()));
    let mut out: Vec<String> = ALIASES.iter().map(|(nap, _, _)| nap.to_string()).collect();
    out.extend(TAILWIND.iter().map(|(_, nap)| nap.to_string()));
    for root in ROOTS {
        for value in &values {
            if root.fixed.iter().any(|(f, _)| f == value)
                || root_value(root, value, &theme, false).is_some()
            {
                out.push(format!("{}-{value}", root.nap));
            }
        }
        for (fixed, _) in root.fixed {
            out.push(format!("{}-{fixed}", root.nap));
        }
    }
    let mut seen = BTreeSet::new();
    out.retain(|c| seen.insert(c.clone()));
    out
}

/// Every alias, root, variant and breakpoint, for `parocss classi` and the
/// editors: (name, what it is, Tailwind name).
pub fn names() -> Vec<(&'static str, &'static str, &'static str)> {
    let mut out = Vec::new();
    for (nap, tw, _) in ALIASES {
        out.push((*nap, "classe", *tw));
    }
    for r in ROOTS {
        if !out.iter().any(|(n, k, _)| *n == r.nap && *k == "radice") {
            out.push((r.nap, "radice", r.tw));
        }
    }
    for (nap, tw, _) in STATES.iter().chain(MEDIA) {
        out.push((*nap, "variante", *tw));
    }
    for (name, _) in THEME {
        if let Some(code) = name.strip_prefix("schermo-") {
            out.push((code, "variante", code));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn css(classes: &[&str]) -> String {
        generate(classes.iter().copied(), &Theme::default()).0
    }

    fn rule(class: &str) -> Vec<(String, String)> {
        utility(class, &Theme::default())
            .unwrap_or_else(|| panic!("{class} is not a class"))
            .declarations
    }

    fn one(class: &str) -> String {
        rule(class)
            .iter()
            .map(|(p, v)| format!("{p}: {v}"))
            .collect::<Vec<_>>()
            .join("; ")
    }

    #[test]
    fn aliases_roots_and_derived_names() {
        assert_eq!(one("flessibile"), "display: flex");
        assert_eq!(one("annascuso"), "display: none");
        assert_eq!(one("spazio-4"), "padding: calc(var(--spazio) * 4)");
        assert_eq!(
            one("spazio-x-0.5"),
            "padding-inline: calc(var(--spazio) * 0.5)"
        );
        assert_eq!(one("spazio-px"), "padding: 1px");
        assert_eq!(
            one("-margine-ncoppa-2"),
            "margin-top: calc(var(--spazio) * -2)"
        );
        assert_eq!(one("margine-x-automatico"), "margin-inline: auto");
        assert_eq!(one("larghezza-1/2"), "width: calc(1 / 2 * 100%)");
        assert_eq!(one("larghezza-chino"), "width: 100%");
        assert_eq!(one("altezza-schermo"), "height: 100vh");
        assert_eq!(
            one("larghezza-massima-md"),
            "max-width: var(--contenitore-md)"
        );
        assert_eq!(
            one("colonne-3"),
            "grid-template-columns: repeat(3, minmax(0, 1fr))"
        );
        assert_eq!(one("giustifica-spartuto"), "justify-content: space-between");
        assert_eq!(one("allinea-cose-miezo"), "align-items: center");
        assert_eq!(one("mostra-blocco"), "display: block");
        assert_eq!(one("opacità-50"), "opacity: 50%");
        assert_eq!(one("livello-10"), "z-index: 10");
        assert_eq!(one("bordo"), "border-width: 1px");
        assert_eq!(one("bordo-2"), "border-width: 2px");
        assert_eq!(one("bordo-ncoppa-2"), "border-top-width: 2px");
        assert_eq!(
            one("bordo-tunno-lg"),
            "border-radius: var(--bordo-tunno-lg)"
        );
        assert_eq!(
            one("bordo-tunno-chino"),
            "border-radius: calc(infinity * 1px)"
        );
        assert_eq!(one("ombra-md"), "box-shadow: var(--ombra-md)");
        assert_eq!(
            one("contorno-2"),
            "outline-style: solid; outline-width: 2px"
        );
        assert_eq!(one("proporzione-16/9"), "aspect-ratio: 16 / 9");
        assert_eq!(
            one("scrittura-grannezza-lg"),
            "font-size: var(--scrittura-grannezza-lg); line-height: var(--scrittura-grannezza-lg--altezza-riga)"
        );
        assert_eq!(one("scrittura-mono"), "font-family: var(--scrittura-mono)");
        assert_eq!(one("scrittura-peso-600"), "font-weight: 600");
        assert_eq!(one("scrittura-peso-grassetto"), "font-weight: bold");
    }

    #[test]
    fn colours_and_their_opacity() {
        assert_eq!(one("culore-russo-500"), "color: var(--culore-russo-500)");
        assert_eq!(one("sfunno-janco"), "background-color: var(--culore-janco)");
        assert_eq!(one("bordo-blu-200"), "border-color: var(--culore-blu-200)");
        assert_eq!(one("sfunno-trasparente"), "background-color: transparent");
        assert_eq!(
            one("sfunno-niro/50"),
            "background-color: color-mix(in oklab, var(--culore-niro) 50%, transparent)"
        );
        // Only the shades of the theme
        assert!(utility("culore-russo", &Theme::default()).is_none());
        assert!(utility("culore-russo-550", &Theme::default()).is_none());
        assert!(utility("culore-red-500", &Theme::default()).is_none());
    }

    #[test]
    fn arbitrary_values_and_properties() {
        assert_eq!(one("larghezza-[37px]"), "width: 37px");
        assert_eq!(one("colonne-[1fr_2fr]"), "grid-template-columns: 1fr 2fr");
        assert_eq!(one("sfunno-[#bada55]"), "background-color: #bada55");
        assert_eq!(one("bordo-[3px]"), "border-width: 3px");
        assert_eq!(one("bordo-[russo]"), "border-color: red");
        assert_eq!(one("trasforma-[rotate(45deg)]"), "transform: rotate(45deg)");
        assert_eq!(one("sfunno-[url(pizza.png)]"), "background: url(pizza.png)");
        assert_eq!(one("[grid-column:span_2]"), "grid-column: span 2");
        assert_eq!(one("[culore:russo]"), "color: red");
        assert_eq!(one("contenuto-['★']"), "content: '★'");
        assert_eq!(one("-ncoppa-[3px]"), "top: calc(3px * -1)");
    }

    #[test]
    fn english_in_brackets_is_not_a_class() {
        let theme = Theme::default();
        for class in [
            "[color:red]",
            "sfunno-[white]",
            "larghezza-[auto]",
            "[white-space:nowrap]",
            "culore-[red]",
        ] {
            assert!(!is_class(class, &theme), "{class}");
        }
        for class in [
            "[culore:russo]",
            "sfunno-[janco]",
            "larghezza-[automatico]",
            "[white-space:nun-va-a-capo]",
            "sfunno-[url(red.png)]",
            "contenuto-['red']",
            "[grid-column:span_2]",
        ] {
            assert!(is_class(class, &theme), "{class}");
        }
        for (english, nap) in [
            ("[color:red]", "[culore:russo]"),
            ("[white-space:nowrap]", "[white-space:nun-va-a-capo]"),
            ("sfunno-[white]", "sfunno-[janco]"),
            ("md:larghezza-[auto]", "md:larghezza-[automatico]"),
            ("bg-[white]", "sfunno-[janco]"),
            ("w-[auto]", "larghezza-[automatico]"),
        ] {
            assert_eq!(suggestion(english).as_deref(), Some(nap), "{english}");
        }
    }

    #[test]
    fn variants_and_important() {
        let u = utility("md:passanno:sfunno-blu-600", &Theme::default()).unwrap();
        assert_eq!(
            u.variants,
            [
                Variant::Media("(width >= 48rem)".into(), 2),
                Variant::Selector(":hover")
            ]
        );
        assert_eq!(
            one("spazio-4!"),
            "padding: calc(var(--spazio) * 4) !important"
        );
        // `primma:` and `doppo:` make the pseudo-element exist
        assert_eq!(
            one("primma:culore-russo-500"),
            "content: \"\"; color: var(--culore-russo-500)"
        );
        assert_eq!(one("doppo:contenuto-['!']"), "content: '!'");
        // English variants and unknown words are not classes
        for class in [
            "hover:spazio-4",
            "pizza:spazio-4",
            "spazio",
            "spazio-",
            "spazio-pizza",
            "-spazio-4",
            "pizza",
        ] {
            assert!(utility(class, &Theme::default()).is_none(), "{class}");
        }
    }

    #[test]
    fn rules_are_escaped_ordered_and_grouped() {
        let out = css(&[
            "md:spazio-8",
            "spazio-ncoppa-2",
            "spazio-4",
            "pizza",
            "spazio-x-0.5",
            "notte:culore-janco",
            "passanno:sottolineato",
        ]);
        assert_eq!(
            out,
            "@layer classi {
  .spazio-4 {
    padding: calc(var(--spazio) * 4);
  }
  .spazio-x-0\\.5 {
    padding-inline: calc(var(--spazio) * 0.5);
  }
  .spazio-ncoppa-2 {
    padding-top: calc(var(--spazio) * 2);
  }
  .passanno\\:sottolineato:hover {
    text-decoration-line: underline;
  }
  @media (width >= 48rem) {
    .md\\:spazio-8 {
      padding: calc(var(--spazio) * 8);
    }
  }
  @media (prefers-color-scheme: dark) {
    .notte\\:culore-janco {
      color: var(--culore-janco);
    }
  }
}
"
        );
        assert_eq!(escape("2xl:flessibile"), "\\32 xl\\:flessibile");
        assert_eq!(escape("larghezza-[37px]"), "larghezza-\\[37px\\]");
        assert_eq!(escape("opacità-50"), "opacità-50");
    }

    #[test]
    fn the_theme_used_and_the_theme_changed() {
        let (_, used) = generate(
            ["culore-russo-500", "spazio-2", "md:flessibile"],
            &Theme::default(),
        );
        assert_eq!(
            used.into_iter().collect::<Vec<_>>(),
            ["culore-russo-500", "spazio"]
        );
        let mut theme = Theme::default();
        theme
            .user
            .insert("culore-pummarola-500".into(), "#e33".into());
        theme.user.insert("schermo-tablet".into(), "50rem".into());
        assert_eq!(
            utility("tablet:sfunno-pummarola-500", &theme)
                .unwrap()
                .declarations,
            [(
                "background-color".to_string(),
                "var(--culore-pummarola-500)".to_string()
            )]
        );
        // Breakpoints sort by width: tablet goes between md and lg
        let points: Vec<String> = theme.breakpoints().into_iter().map(|(c, _)| c).collect();
        assert_eq!(points, ["sm", "md", "tablet", "lg", "xl", "2xl"]);
        let mut used = BTreeSet::new();
        used.insert("culore-pummarola-500".into());
        used.insert("spazio".into());
        assert_eq!(theme.root(&used), ":root {\n  --spazio: 0.25rem;\n}\n");
    }

    #[test]
    fn apply_nests_the_variants() {
        let theme = Theme::default();
        let mut used = BTreeSet::new();
        assert_eq!(
            apply("annascuso-all-uocchie", &theme, &mut used).unwrap()[..2],
            ["position: absolute;", "width: 1px;"]
        );
        assert_eq!(
            apply("md:passanno:culore-russo-500", &theme, &mut used).unwrap(),
            ["@media (width >= 48rem) { &:hover { color: var(--culore-russo-500); } }"]
        );
        assert!(apply("pizza", &theme, &mut used).is_none());
    }

    #[test]
    fn tailwind_classes_suggest_their_class() {
        for (tw, nap) in [
            ("p-4", "spazio-4"),
            ("px-2", "spazio-x-2"),
            ("-mt-4", "-margine-ncoppa-4"),
            ("mx-auto", "margine-x-automatico"),
            ("w-full", "larghezza-chino"),
            ("h-screen", "altezza-schermo"),
            ("max-w-md", "larghezza-massima-md"),
            ("flex", "flessibile"),
            ("hidden", "annascuso"),
            ("items-center", "allinea-cose-miezo"),
            ("justify-between", "giustifica-spartuto"),
            ("grid-cols-3", "colonne-3"),
            ("gap-4", "spazio-mmiez-4"),
            ("text-red-500", "culore-russo-500"),
            ("text-lg", "scrittura-grannezza-lg"),
            ("text-center", "allinea-testo-miezo"),
            ("bg-white", "sfunno-janco"),
            ("bg-black/50", "sfunno-niro/50"),
            ("border", "bordo"),
            ("border-gray-200", "bordo-grigio-200"),
            ("rounded-lg", "bordo-tunno-lg"),
            ("rounded-full", "bordo-tunno-chino"),
            ("shadow-md", "ombra-md"),
            ("font-bold", "grassetto"),
            ("font-semibold", "scrittura-peso-600"),
            ("font-mono", "scrittura-mono"),
            ("opacity-50", "opacità-50"),
            ("z-10", "livello-10"),
            ("hover:bg-blue-600", "passanno:sfunno-blu-600"),
            ("md:flex", "md:flessibile"),
            ("dark:text-white", "notte:culore-janco"),
            ("w-[37px]", "larghezza-[37px]"),
            ("sr-only", "annascuso-all-uocchie"),
        ] {
            assert_eq!(suggestion(tw).as_deref(), Some(nap), "{tw}");
        }
        // par-o-css classes, user classes and unknown names suggest nothing
        for class in [
            "spazio-4",
            "md:flessibile",
            "menu",
            "carta",
            "pizza-4",
            "container",
        ] {
            assert_eq!(suggestion(class), None, "{class}");
        }
    }

    #[test]
    fn scanning_finds_candidates_in_any_text() {
        let mut found = BTreeSet::new();
        scan(
            "<cascett classe=\"flessibile md:spazio-4\">Uè.</cascett>\n<citofono classe={`larghezza-[37px] ${x ? 'culore-russo-500/50' : \"contenuto-['★']\"}`}>\nquerySelector('.annascuso') spazio-x-0.5.",
            &mut found,
        );
        for class in [
            "flessibile",
            "md:spazio-4",
            "larghezza-[37px]",
            "culore-russo-500/50",
            "contenuto-['★']",
            "annascuso",
            "spazio-x-0.5",
        ] {
            assert!(found.contains(class), "{class} not in {found:?}");
        }
        assert!(!found.contains("Uè."));
    }

    #[test]
    fn variants_use_the_pseudo_class_words_of_stile() {
        for (nap, _, selector) in STATES {
            let css = stile::translate(&[format!("a:{nap} {{}}")])
                .unwrap()
                .remove(0);
            assert_eq!(
                css.trim_end_matches(" {}")
                    .trim_start_matches('a')
                    .trim_start_matches(':'),
                selector.trim_start_matches(':'),
                "{nap}"
            );
        }
    }

    #[test]
    fn the_readme_lists_every_class_root_and_variant() {
        let readme = include_str!("../../../README.md");
        for (name, kind, _) in names() {
            let written = match kind {
                "variante" => format!("`{name}:`"),
                _ => format!("`{name}`"),
            };
            assert!(
                readme.contains(&written),
                "{written} missing from README.md"
            );
        }
    }

    #[test]
    fn examples_are_classes() {
        let all = examples();
        for class in [
            "flessibile",
            "spazio-4",
            "culore-russo-500",
            "bordo-tunno-lg",
            "larghezza-1/2",
            "giustifica-spartuto",
            "sfunno-janco",
            "scrittura-grannezza-2xl",
        ] {
            assert!(all.iter().any(|c| c == class), "{class} missing");
        }
        let theme = Theme::default();
        for class in &all {
            assert!(
                is_class(class, &theme),
                "{class} is offered but is not a class"
            );
        }
    }

    #[test]
    fn names_are_unique_per_kind() {
        let names = names();
        for (i, (n, k, _)) in names.iter().enumerate() {
            assert!(
                !names[i + 1..].iter().any(|(o, ok, _)| o == n && ok == k),
                "{n} twice"
            );
        }
    }
}
