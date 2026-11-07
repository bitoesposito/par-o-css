//! Snapshots of what the compiler writes for every `examples/*.cess` and
//! every `tests/golden/cases/*.cess`: the CSS in a `.css` file, or, for
//! `*.sbagliato.cess` cases, the error in a `.err` file. The classes of
//! `x.cess` are the words of `x.emmurt` next to it, when there is one.
//!
//! After an intended change: `PAROCSS_UPDATE_GOLDEN=1 cargo test -p parocss-core --test golden`.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use parocss_core::{compile, format, scan, Options};

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// `(source, snapshot)` pairs, paths relative to the repository.
fn cases() -> Vec<(String, String)> {
    let mut cases = Vec::new();
    for (dir, snapshots) in [
        ("examples", "crates/parocss-core/tests/golden/examples"),
        (
            "crates/parocss-core/tests/golden/cases",
            "crates/parocss-core/tests/golden/cases",
        ),
    ] {
        let mut files: Vec<_> = fs::read_dir(repo().join(dir))
            .unwrap()
            .filter_map(|e| e.ok()?.file_name().into_string().ok())
            .filter(|n| n.ends_with(".cess"))
            .collect();
        files.sort();
        for name in files {
            let stem = name.trim_end_matches(".cess");
            let ext = if stem.ends_with(".sbagliato") {
                "err"
            } else {
                "css"
            };
            cases.push((format!("{dir}/{name}"), format!("{snapshots}/{stem}.{ext}")));
        }
    }
    cases
}

#[test]
fn golden() {
    let update = std::env::var_os("PAROCSS_UPDATE_GOLDEN").is_some();
    // Paths starting with `/` are from the repository root
    let read = |path: &str| fs::read_to_string(repo().join(path.trim_start_matches('/'))).ok();
    let mut failures = Vec::new();
    for (source, snapshot) in cases() {
        let mut classes = BTreeSet::new();
        let page = repo().join(source.replace(".cess", ".emmurt"));
        if let Ok(text) = fs::read_to_string(page) {
            scan(&text, &mut classes);
        }
        let options = Options {
            filename: &source,
            read: &read,
            classes: &classes,
        };
        let text = fs::read_to_string(repo().join(&source)).unwrap();
        let actual = match (compile(&text, &options), snapshot.ends_with(".err")) {
            (Ok(out), false) => out.css,
            (Err(e), true) => format!("{e}\n"),
            (Ok(_), true) => {
                failures.push(format!("{source}: compiled, but it should fail"));
                continue;
            }
            (Err(e), false) => {
                failures.push(format!("{source}: {e}"));
                continue;
            }
        };
        let path = repo().join(&snapshot);
        if update {
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, &actual).unwrap();
        } else if fs::read_to_string(&path).ok().as_deref() != Some(actual.as_str()) {
            failures.push(format!("{snapshot} is out of date:\n{actual}"));
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n\n"));
}

/// The playground's guide and lessons: every `.cess` compiles with the
/// classes of the `.emmurt` page it goes with (the `*.sbagliato.cess` ones
/// must fail) and is formatted, and every lesson's solution makes different
/// CSS from its start. A lesson's solution file that is missing is the same
/// as its start. `@chiamm` reads `examples/`, as in the playground.
#[test]
fn playground_snippets_compile() {
    let examples = repo().join("examples");
    let read = |path: &str| fs::read_to_string(examples.join(path)).ok();
    let css = |cess: &str, page: &str| {
        let mut classes = BTreeSet::new();
        scan(page, &mut classes);
        let options = Options {
            filename: "stile.cess",
            read: &read,
            classes: &classes,
        };
        compile(cess, &options).map(|out| out.css)
    };
    for dir in ["guida", "scola"] {
        let dir = repo().join("packages/playground").join(dir);
        let text = |name: &str| fs::read_to_string(dir.join(name)).ok();
        let mut names: Vec<String> = fs::read_dir(&dir)
            .unwrap()
            .filter_map(|e| e.ok()?.file_name().into_string().ok())
            .filter(|n| n.ends_with(".cess"))
            .collect();
        names.sort();
        assert!(names.len() >= 9, "{}", dir.display());
        for name in &names {
            let source = text(name).unwrap();
            let stem = name.trim_end_matches(".cess");
            let page = text(&format!("{stem}.emmurt"))
                .or_else(|| text(&format!("{}.emmurt", stem.trim_end_matches(".soluzione"))))
                .unwrap_or_else(|| panic!("{name} has no .emmurt page"));
            let result = css(&source, &page);
            assert_eq!(
                result.is_err(),
                name.ends_with(".sbagliato.cess"),
                "{name}: {:?}",
                result.err()
            );
            assert_eq!(
                format(&source, name).unwrap(),
                source,
                "{name} is not formatted"
            );
        }
        if !dir.ends_with("scola") {
            continue;
        }
        for name in names.iter().filter(|n| !n.contains(".soluzione.")) {
            let id = name.trim_end_matches(".cess");
            let (cess, page) = (text(name).unwrap(), text(&format!("{id}.emmurt")).unwrap());
            let solution_cess = text(&format!("{id}.soluzione.cess"));
            let solution_page = text(&format!("{id}.soluzione.emmurt"));
            assert!(
                solution_cess.is_some() || solution_page.is_some(),
                "{id} has no solution"
            );
            let solution = css(
                solution_cess.as_deref().unwrap_or(&cess),
                solution_page.as_deref().unwrap_or(&page),
            );
            assert_ne!(
                css(&cess, &page).unwrap(),
                solution.unwrap(),
                "{id}: the solution makes the same CSS as the start"
            );
        }
    }
}

/// Formatting a formatted file changes nothing, for every example and case
/// that has no syntax error.
#[test]
fn formatting_is_stable() {
    for (source, _) in cases() {
        let text = fs::read_to_string(repo().join(&source)).unwrap();
        let Ok(once) = format(&text, &source) else {
            continue;
        };
        assert_eq!(format(&once, &source).unwrap(), once, "{source}");
    }
}
