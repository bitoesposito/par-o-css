//! The `parocss` command: builds, checks and watches `.cess` files.

use std::collections::BTreeSet;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, SystemTime};

use parocss_core::{
    compile, format, generate, is_class, names, pieces, scan, suggestion, Error, Options, Theme,
};
use serde_json::json;

const AIUTO: &str = "\
parocss: CSS, ma comme si deve

  parocss build <file|cartella...>   Compila 'e .cess in .css
      --out <cartella>               Scrive llà invece che vicino ê sorgenti
      --stdout                       Stampa 'o CSS (nu file sulo, o - pe' stdin)
  parocss check <file|cartella...>   Controlla senza scrivere niente
  parocss watch <file|cartella...>   Ricompila ogni vota ca cagne quaccosa
  parocss fmt <file|cartella...>     Mette a posto rientri e righe vacante
      --check                        Dice sulo quali file nun so' a posto
  parocss classi [classe...]         'O CSS 'e chelli classi, o tutte 'e parole
  parocss guida                      'E istruzioni pe' n'agente (llms.txt)

  --pagine <cartella>   Addò cercà 'e classi (si no 'a cartella addò staje)
  --json                Na risposta JSON sola, pe' editor e agenti

@chiamm \"par-o-css\" dint'a nu .cess mette 'o tema, 'a pulizia e 'e classi
ca truove dint'ê pagine (.emmurt, .fratm, .html, .js…). Na cartella compila
sulo 'e .cess ca nisciun ato .cess chiamma.
";

/// The instructions for AI agents, served by the playground as llms.txt.
const GUIDA: &str = include_str!("../../../packages/playground/public/llms.txt");

/// The files whose words may be classes.
const PAGES: &[&str] = &[
    "emmurt", "fratm", "html", "htm", "js", "jsx", "mjs", "ts", "tsx", "vue", "svelte", "astro",
    "md",
];

#[derive(Default)]
struct Flags {
    json: bool,
    stdout: bool,
    check: bool,
    out: Option<PathBuf>,
    pages: Option<PathBuf>,
}

/// A file to compile, and the folder named on the command line it came from.
struct Input {
    path: PathBuf,
    root: PathBuf,
    from_folder: bool,
}

struct Compiled {
    input: Input,
    result: Result<(String, Vec<String>), Error>,
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_default();
    let mut flags = Flags::default();
    let mut paths = Vec::new();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--json" => flags.json = true,
            "--stdout" => flags.stdout = true,
            "--check" => flags.check = true,
            "--out" | "--pagine" => match args.next() {
                Some(dir) if arg == "--out" => flags.out = Some(dir.into()),
                Some(dir) => flags.pages = Some(dir.into()),
                None => return fail(&format!("{arg} vo' na cartella.")),
            },
            s if s.starts_with("--") => return fail(&format!("'{s}' nun 'o cunosco.")),
            _ => paths.push(arg),
        }
    }
    match command.as_str() {
        "build" | "check" | "watch" | "fmt" if paths.is_empty() => {
            fail("Dimme quale file: parocss build stile.cess")
        }
        "build" => build(&paths, &flags, true),
        "check" => build(&paths, &flags, false),
        "fmt" => fmt(&paths, &flags),
        "watch" => watch(&paths, &flags),
        "classi" => classes(&paths, flags.json),
        "guida" => {
            print!("{GUIDA}");
            ExitCode::SUCCESS
        }
        "--version" | "-V" => {
            println!("parocss {}", env!("CARGO_PKG_VERSION"));
            ExitCode::SUCCESS
        }
        "" | "aiuto" | "--help" | "-h" => {
            print!("{AIUTO}");
            ExitCode::SUCCESS
        }
        other => fail(&format!("'{other}' nun è nu comando. Prova: parocss aiuto")),
    }
}

fn fail(message: &str) -> ExitCode {
    eprintln!("✗ {message}");
    ExitCode::FAILURE
}

fn build(paths: &[String], flags: &Flags, write: bool) -> ExitCode {
    let found = page_classes(flags);
    if paths == ["-"] {
        return build_stdin(flags, write, &found.0);
    }
    let inputs = match inputs(paths, flags.out.as_deref()) {
        Ok(inputs) => inputs,
        Err(e) => return fail(&e),
    };
    if flags.stdout && inputs.len() != 1 {
        return fail("--stdout vo' nu file sulo.");
    }
    let compiled: Vec<Compiled> = without_pieces(inputs)
        .into_iter()
        .map(|i| compile_input(i, &found.0))
        .collect();
    let mut errors = Vec::new();
    let mut written = Vec::new();
    for c in &compiled {
        match &c.result {
            Err(e) => errors.push(e.clone()),
            Ok((css, _)) if flags.stdout => print!("{css}"),
            Ok((css, _)) if write => match write_css(&c.input, css, flags.out.as_deref()) {
                Ok(out) => written.push(out.display().to_string()),
                Err(e) => return fail(&e),
            },
            Ok(_) => {}
        }
    }
    report(&errors, flags.json, json!({ "written": written }), None);
    if !flags.json && !flags.stdout {
        for w in &written {
            eprintln!("✓ {w}");
        }
        if !write && errors.is_empty() {
            eprintln!("✓ Tutto a posto: {} file.", compiled.len());
        }
    }
    if errors.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn build_stdin(flags: &Flags, write: bool, classes: &BTreeSet<String>) -> ExitCode {
    let mut source = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut source) {
        return fail(&format!("Nun riesco a leggere stdin: {e}"));
    }
    let read = |path: &str| read_file(path, Path::new("."));
    let options = Options {
        filename: "stdin.cess",
        read: &read,
        classes,
    };
    match compile(&source, &options) {
        Ok(out) if flags.json => {
            report(&[], true, json!({ "css": write.then_some(&out.css) }), None);
            ExitCode::SUCCESS
        }
        Ok(out) => {
            if write {
                print!("{}", out.css);
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            report(
                std::slice::from_ref(&e),
                flags.json,
                json!({}),
                Some(&source),
            );
            ExitCode::FAILURE
        }
    }
}

fn watch(paths: &[String], flags: &Flags) -> ExitCode {
    loop {
        let inputs = match inputs(paths, flags.out.as_deref()) {
            Ok(inputs) => inputs,
            Err(e) => return fail(&e),
        };
        let (classes, mut watched) = page_classes(flags);
        watched.extend(inputs.iter().map(|i| i.path.clone()));
        let compiled: Vec<Compiled> = without_pieces(inputs)
            .into_iter()
            .map(|i| compile_input(i, &classes))
            .collect();
        for c in &compiled {
            match &c.result {
                Ok((css, deps)) => {
                    watched.extend(deps.iter().map(|d| resolve(d, &c.input.root)));
                    match write_css(&c.input, css, flags.out.as_deref()) {
                        Ok(out) => eprintln!("✓ {} → {}", c.input.path.display(), out.display()),
                        Err(e) => eprintln!("✗ {e}"),
                    }
                }
                Err(e) => print_error(e, None),
            }
        }
        eprintln!("… aspetto ca cagne quaccosa (Ctrl+C pe' fernì)");
        let seen: Vec<(PathBuf, Option<SystemTime>)> = watched
            .into_iter()
            .map(|p| (p.clone(), modified(&p)))
            .collect();
        // ponytail: polls every 300 ms; a file watcher crate if this gets slow
        loop {
            std::thread::sleep(Duration::from_millis(300));
            let new_files = page_files(flags)
                .iter()
                .chain(
                    paths
                        .iter()
                        .flat_map(|p| collect(Path::new(p), flags.out.as_deref(), &["cess"]))
                        .collect::<Vec<_>>()
                        .iter(),
                )
                .any(|f| !seen.iter().any(|(s, _)| s == f));
            if new_files || seen.iter().any(|(p, t)| modified(p) != *t) {
                break;
            }
        }
    }
}

/// Formats files in place, or with --check only lists the ones that would
/// change. `-` formats stdin to stdout.
fn fmt(paths: &[String], flags: &Flags) -> ExitCode {
    if paths == ["-"] {
        let mut source = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut source) {
            return fail(&format!("Nun riesco a leggere stdin: {e}"));
        }
        return match format(&source, "stdin.cess") {
            Ok(text) => {
                print!("{text}");
                ExitCode::SUCCESS
            }
            Err(e) => {
                report(
                    std::slice::from_ref(&e),
                    flags.json,
                    json!({}),
                    Some(&source),
                );
                ExitCode::FAILURE
            }
        };
    }
    let inputs = match inputs(paths, None) {
        Ok(inputs) => inputs,
        Err(e) => return fail(&e),
    };
    let mut errors = Vec::new();
    let mut changed = Vec::new();
    for input in &inputs {
        let file = input.path.to_string_lossy().replace('\\', "/");
        let source = match fs::read_to_string(&input.path) {
            Ok(source) => source,
            Err(e) => return fail(&format!("Nun riesco a leggere '{file}': {e}")),
        };
        match format(&source, &file) {
            Err(e) => errors.push(e),
            Ok(text) if text != source => {
                if !flags.check {
                    if let Err(e) = fs::write(&input.path, &text) {
                        return fail(&format!("Nun riesco a scrivere '{file}': {e}"));
                    }
                }
                changed.push(file);
            }
            Ok(_) => {}
        }
    }
    report(&errors, flags.json, json!({ "changed": changed }), None);
    if !flags.json {
        let (mark, what) = match flags.check {
            true => ("✗", "nun è a posto"),
            false => ("✓", "aggiustato"),
        };
        for file in &changed {
            eprintln!("{mark} {file}: {what}");
        }
        if errors.is_empty() && changed.is_empty() {
            eprintln!("✓ Tutto a posto: {} file.", inputs.len());
        }
    }
    if errors.is_empty() && !(flags.check && !changed.is_empty()) {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn modified(path: &Path) -> Option<SystemTime> {
    fs::metadata(path).and_then(|m| m.modified()).ok()
}

/// `parocss classi`: the CSS of the classes given, or every word.
fn classes(given: &[String], as_json: bool) -> ExitCode {
    let theme = Theme::default();
    if given.is_empty() {
        let all = names();
        if as_json {
            let list: Vec<_> = all
                .iter()
                .map(|(name, kind, tw)| json!({ "nomme": name, "che": kind, "tailwind": tw }))
                .collect();
            println!("{}", json!(list));
            return ExitCode::SUCCESS;
        }
        for (title, kind) in [
            ("Classi sane", "classe"),
            ("Radici: radice-valore, comme spazio-4", "radice"),
            (
                "Varianti: variante:classe, comme passanno:culore-russo-600",
                "variante",
            ),
        ] {
            println!("{title}");
            for (name, _, tw) in all.iter().filter(|(_, k, _)| *k == kind) {
                println!("  {name}  ({tw})");
            }
            println!();
        }
        println!("E ogne proprietà 'e stile cu nu valore: giustifica-spartuto, mostra-blocco…");
        println!("o cu nu valore tra parentesi quadre: larghezza-[37px], [grid-column:span_2]");
        return ExitCode::SUCCESS;
    }
    let (css, _) = generate(given.iter().map(String::as_str), &theme);
    let unknown: Vec<_> = given
        .iter()
        .filter(|c| !is_class(c, &theme))
        .map(|c| match suggestion(c) {
            Some(s) => format!("'{c}' nun è na classe 'e par-o-css. Vuoi dire '{s}'?"),
            None => format!("'{c}' nun è na classe 'e par-o-css."),
        })
        .collect();
    if as_json {
        println!("{}", json!({ "css": css, "errors": unknown }));
    } else {
        print!("{css}");
        for u in &unknown {
            eprintln!("✗ {u}");
        }
    }
    if unknown.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// The words that may be classes in the pages, and the files they are in.
fn page_classes(flags: &Flags) -> (BTreeSet<String>, Vec<PathBuf>) {
    let files = page_files(flags);
    let mut found = BTreeSet::new();
    for f in &files {
        if let Ok(text) = fs::read_to_string(f) {
            scan(&text, &mut found);
        }
    }
    (found, files)
}

fn page_files(flags: &Flags) -> Vec<PathBuf> {
    let root = flags.pages.clone().unwrap_or_else(|| PathBuf::from("."));
    collect(&root, flags.out.as_deref(), PAGES)
}

/// Files named on the command line, folders expanded to their `.cess` files.
fn inputs(paths: &[String], out: Option<&Path>) -> Result<Vec<Input>, String> {
    let mut inputs = Vec::new();
    for p in paths {
        let path = Path::new(p);
        if path.is_dir() {
            inputs.extend(collect(path, out, &["cess"]).into_iter().map(|f| Input {
                path: f,
                root: path.to_path_buf(),
                from_folder: true,
            }));
        } else if path.is_file() {
            inputs.push(Input {
                path: path.to_path_buf(),
                root: path.parent().unwrap_or(Path::new("")).to_path_buf(),
                from_folder: false,
            });
        } else {
            return Err(format!("Nun trovo '{p}'."));
        }
    }
    Ok(inputs)
}

/// The files with one of `extensions` under a folder, skipping hidden
/// folders, `node_modules`, `target`, `dist` and the output folder.
fn collect(dir: &Path, out: Option<&Path>, extensions: &[&str]) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return files;
    };
    let mut entries: Vec<PathBuf> = entries.filter_map(|e| Some(e.ok()?.path())).collect();
    entries.sort();
    for path in entries {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or_default();
        if path.is_dir() {
            let skip = name.starts_with('.')
                || matches!(name, "node_modules" | "target" | "dist")
                || out.is_some_and(|o| same_path(o, &path));
            if !skip {
                files.extend(collect(&path, out, extensions));
            }
        } else if path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| extensions.contains(&e))
        {
            files.push(path);
        }
    }
    files
}

fn same_path(a: &Path, b: &Path) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

fn compile_input(input: Input, classes: &BTreeSet<String>) -> Compiled {
    let filename = input.path.to_string_lossy().replace('\\', "/");
    let result = match fs::read_to_string(&input.path) {
        Ok(source) => {
            let root = input.root.clone();
            let read = move |path: &str| read_file(path, &root);
            let options = Options {
                filename: &filename,
                read: &read,
                classes,
            };
            compile(&source, &options).map(|out| (out.css, out.dependencies))
        }
        Err(e) => Err(Error {
            message: format!("Nun riesco a leggere stu file: {e}"),
            file: filename.clone(),
            line: 1,
            column: 1,
            suggestion: None,
        }),
    };
    Compiled { input, result }
}

/// The inputs without the files that another input of a folder pastes in
/// with `@chiamm`: those are pieces, compiled inside the file that calls
/// them, with its theme. A file named on the command line stays.
fn without_pieces(inputs: Vec<Input>) -> Vec<Input> {
    let pasted: Vec<PathBuf> = inputs
        .iter()
        .filter_map(|i| {
            let source = fs::read_to_string(&i.path).ok()?;
            let name = i.path.to_string_lossy().replace('\\', "/");
            Some(pieces(&source, &name))
        })
        .flatten()
        .filter_map(|p| PathBuf::from(p).canonicalize().ok())
        .collect();
    inputs
        .into_iter()
        .filter(|i| !i.from_folder || !i.path.canonicalize().is_ok_and(|p| pasted.contains(&p)))
        .collect()
}

/// A path from the compiler: as written, or, when it starts with `/`,
/// under the folder named on the command line.
fn resolve(path: &str, root: &Path) -> PathBuf {
    let direct = PathBuf::from(path);
    match path.strip_prefix('/') {
        Some(rest) if !direct.exists() => root.join(rest),
        _ => direct,
    }
}

fn read_file(path: &str, root: &Path) -> Option<String> {
    fs::read_to_string(resolve(path, root)).ok()
}

/// Writes the CSS of a file next to it, or under `out`, and says where.
fn write_css(input: &Input, css: &str, out: Option<&Path>) -> Result<PathBuf, String> {
    let path = match out {
        Some(dir) => dir.join(input.path.strip_prefix(&input.root).unwrap_or(&input.path)),
        None => input.path.clone(),
    }
    .with_extension("css");
    path.parent()
        .map_or(Ok(()), fs::create_dir_all)
        .and_then(|_| fs::write(&path, css))
        .map_err(|e| format!("Nun riesco a scrivere '{}': {e}", path.display()))?;
    Ok(path)
}

fn report(errors: &[Error], as_json: bool, extra: serde_json::Value, stdin: Option<&str>) {
    if as_json {
        let mut out = json!({ "ok": errors.is_empty(), "errors": errors });
        if let (Some(out), Some(extra)) = (out.as_object_mut(), extra.as_object()) {
            out.extend(extra.clone());
        }
        println!("{out}");
    } else {
        for e in errors {
            print_error(e, stdin);
        }
    }
}

/// The error, the line it is on with a caret under the column, and the hint.
fn print_error(e: &Error, stdin: Option<&str>) {
    eprintln!("✗ {}:{}:{}: {}", e.file, e.line, e.column, e.message);
    let source = match stdin {
        Some(s) if e.file == "stdin.cess" => Some(s.to_string()),
        _ => fs::read_to_string(&e.file).ok(),
    };
    if let Some(line) = source
        .as_deref()
        .and_then(|s| s.lines().nth(e.line.saturating_sub(1)))
    {
        let pad: String = line
            .chars()
            .take(e.column.saturating_sub(1))
            .map(|c| if c == '\t' { '\t' } else { ' ' })
            .collect();
        eprintln!("  {:>4} | {line}", e.line);
        eprintln!("       | {pad}^");
    }
    if let Some(s) = &e.suggestion {
        eprintln!("  {s}");
    }
}
