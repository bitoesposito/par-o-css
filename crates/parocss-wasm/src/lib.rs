//! WebAssembly bindings: the compiler for Node, Vite, the browser runtime
//! and the VS Code extension.

use std::collections::BTreeSet;

use js_sys::Function;
use parocss_core::{Error, Options, Theme};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct Response {
    css: Option<String>,
    dependencies: Vec<String>,
    errors: Vec<Error>,
}

/// Compiles a `.cess` file. `read(path)` returns the text of a file named
/// by `@chiamm`, or null when it does not exist; `classes` are the words of
/// the pages that may be classes.
#[wasm_bindgen]
pub fn compile(
    source: &str,
    filename: &str,
    read: Option<Function>,
    classes: Vec<String>,
) -> JsValue {
    // Anything but a string, or a throw, means the file is not there
    let read = |path: &str| -> Option<String> {
        read.as_ref()?
            .call1(&JsValue::NULL, &JsValue::from_str(path))
            .ok()?
            .as_string()
    };
    let classes: BTreeSet<String> = classes.into_iter().collect();
    let options = Options {
        filename,
        read: &read,
        classes: &classes,
    };
    let response = match parocss_core::compile(source, &options) {
        Ok(out) => Response {
            css: Some(out.css),
            dependencies: out.dependencies,
            errors: vec![],
        },
        Err(e) => Response {
            css: None,
            dependencies: vec![],
            errors: vec![e],
        },
    };
    let serializer = serde_wasm_bindgen::Serializer::json_compatible();
    response.serialize(&serializer).unwrap_or(JsValue::NULL)
}

/// The words of a text that may be classes.
#[wasm_bindgen]
pub fn scan(text: &str) -> Vec<String> {
    let mut found = BTreeSet::new();
    parocss_core::scan(text, &mut found);
    found.into_iter().collect()
}

/// The par-o-css class for a Tailwind class, if it is one.
#[wasm_bindgen]
pub fn suggestion(class: &str) -> Option<String> {
    parocss_core::suggestion(class)
}

/// Whether a word is a class of par-o-css, with the starting theme.
#[wasm_bindgen(js_name = isClass)]
pub fn is_class(class: &str) -> bool {
    parocss_core::is_class(class, &Theme::default())
}

/// The CSS of some classes, with the starting theme: the rules and the
/// `:root` with the theme properties they use.
#[wasm_bindgen(js_name = classCss)]
pub fn class_css(classes: Vec<String>) -> String {
    let theme = Theme::default();
    let (rules, used) = parocss_core::generate(classes.iter().map(String::as_str), &theme);
    format!("{}{rules}", theme.root(&used))
}

/// Every alias, root and variant, as JSON: `[{ nomme, che, tailwind }]`.
#[wasm_bindgen]
pub fn names() -> String {
    let list: Vec<_> = parocss_core::names()
        .into_iter()
        .map(|(name, kind, tw)| serde_json::json!({ "nomme": name, "che": kind, "tailwind": tw }))
        .collect();
    serde_json::Value::Array(list).to_string()
}

/// Classes to offer while typing.
#[wasm_bindgen]
pub fn examples() -> Vec<String> {
    parocss_core::examples()
}

/// The `.cess` files a source pastes in with `@chiamm`.
#[wasm_bindgen]
pub fn pieces(source: &str, filename: &str) -> Vec<String> {
    parocss_core::pieces(source, filename)
}

/// The words of CSS in Neapolitan, as JSON: `{ properties, values, pseudo,
/// tags }`, each a list of `[Neapolitan, CSS]`.
#[wasm_bindgen]
pub fn vocabulary() -> String {
    use fratm_core::{html, stile};
    serde_json::json!({
        "properties": stile::PROPERTIES,
        "values": stile::VALUES,
        "pseudo": stile::PSEUDO,
        "tags": html::TAGS.iter().map(|(nap, tag, _)| (nap, tag)).collect::<Vec<_>>(),
    })
    .to_string()
}

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
