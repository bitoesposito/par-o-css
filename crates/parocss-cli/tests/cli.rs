//! The `parocss` command on a small project in a temporary folder.

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn project(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("parocss-cli-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("stile/pezzi")).unwrap();
    fs::write(
        dir.join("index.emmurt"),
        "<cascett classe=\"flessibile md:spazio-4 carta\">Uè</cascett>",
    )
    .unwrap();
    fs::write(
        dir.join("stile/stile.cess"),
        "@chiamm \"par-o-css\"\n@tema { --culore-mozzarella: #fffdf7 }\n@chiamm \"pezzi/carta.cess\"\n",
    )
    .unwrap();
    fs::write(
        dir.join("stile/pezzi/carta.cess"),
        ".carta { @applica sfunno-mozzarella }\n",
    )
    .unwrap();
    dir
}

fn parocss(dir: &PathBuf, args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_parocss"))
        .args(args)
        .current_dir(dir)
        .output()
        .unwrap();
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn build_writes_the_stylesheets_with_the_classes_of_the_pages() {
    let dir = project("build");
    let (ok, _, err) = parocss(&dir, &["build", "stile", "--out", "dist"]);
    assert!(ok, "{err}");
    let css = fs::read_to_string(dir.join("dist/stile.css")).unwrap();
    assert!(css.contains(".flessibile {"), "{css}");
    assert!(css.contains(".md\\:spazio-4 {"), "{css}");
    assert!(
        css.contains("background-color: var(--culore-mozzarella)"),
        "{css}"
    );
    // A piece is pasted in, not written on its own
    assert!(!dir.join("dist/pezzi/carta.css").exists());
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn check_json_gives_the_error_and_the_word_to_write() {
    let dir = project("check");
    fs::write(
        dir.join("stile/sbagliato.cess"),
        ".a {\n  color: russo\n}\n",
    )
    .unwrap();
    let (ok, out, _) = parocss(&dir, &["check", "stile", "--json"]);
    assert!(!ok);
    let json: serde_json::Value = serde_json::from_str(&out).unwrap();
    assert_eq!(json["ok"], false);
    let e = &json["errors"][0];
    assert_eq!(e["message"], "'color' nun è napoletano.");
    assert_eq!(e["suggestion"], "Vuoi dire 'culore'?");
    assert_eq!(
        (e["line"].as_u64(), e["column"].as_u64()),
        (Some(2), Some(3))
    );
    fs::remove_dir_all(dir).unwrap();
}

#[test]
fn classi_prints_the_css_and_suggests_for_tailwind() {
    let dir = project("classi");
    let (ok, out, err) = parocss(&dir, &["classi", "annascuso", "p-4"]);
    assert!(!ok);
    assert!(out.contains(".annascuso {\n    display: none;"), "{out}");
    assert!(err.contains("Vuoi dire 'spazio-4'?"), "{err}");
    fs::remove_dir_all(dir).unwrap();
}
