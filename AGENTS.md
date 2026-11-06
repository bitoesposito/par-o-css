# AGENTS.md

Regole per agenti (Claude, Codex, Copilot, Cursor…) e persone che lavorano su par-o-css. Se una regola qui va contro un'istruzione del maintainer, vince il maintainer.

## Cos'è il repo

par-o-css è CSS in napoletano, con le classi pronte alla Tailwind. I file `.cess` sono CSS napoletano tradotto dallo `stile` di `fratm-core` (dipendenza git, fissata in `Cargo.toml`), ma severo come htmlemmurt: un nome inglese che ha la sua parola napoletana è un errore. `@chiamm "par-o-css"` dentro un `.cess` mette il tema, la pulizia di base e le classi (`flessibile`, `spazio-4`, `culore-russo-500`, `md:colonne-2`) che le pagine usano davvero.

| Percorso | Cosa contiene |
| --- | --- |
| `crates/parocss-core/src/lib.rs` | Compilatore dei `.cess`: controllo severo, `@chiamm`, `@tema`, `@applica`, errori |
| `crates/parocss-core/src/classi.rs` | Le classi: radici, varianti, valori, il CSS che generano e i nomi Tailwind che suggeriscono |
| `crates/parocss-core/src/tema.rs` | Il tema di partenza (colori, misure, breakpoint) e la pulizia di base |
| `crates/parocss-cli` | Il comando `parocss` (`build`, `check`, `watch`, `classi`, `guida`) |
| `crates/parocss-wasm` | Binding WebAssembly usati da npm ed estensione |
| `packages/par-o-css` | Pacchetto npm: API Node, plugin Vite, runtime browser (`wasm/` è generato) |
| `packages/vscode-extension` | Grammatica, completamento delle classi, diagnostica (`compiler/` è generato) |
| `packages/playground` | Playground web su GitHub Pages, scritto in htmlemmurt e par-o-css |
| `packages/playground/public/llms.txt` | Istruzioni per gli agenti AI: il sito le serve come `llms.txt`, `parocss guida` le stampa |
| `examples/` | `.cess` d'esempio con snapshot golden |

## Lingua

- Codice e commenti in inglese.
- Parole del linguaggio, nomi delle classi, messaggi del compilatore ed esempi in napoletano: sono il prodotto.
- README, commit e issue in italiano.

## Build e test

Gli stessi controlli della CI (`.github/workflows/ci.yml`):

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo build -p parocss-cli && ./target/debug/parocss check examples
cd packages/par-o-css && npm run build && npm test           # serve wasm-pack
cd packages/vscode-extension && npm run compiler && npm test
cd packages/playground && npm install && npm run build      # dopo il WebAssembly
```

Dopo una modifica al compilatore, `npm run build` in `packages/par-o-css` rigenera il WebAssembly: plugin Vite, runtime e playground usano quello.

## Regole per le modifiche

- La CI deve essere verde. Non saltare, cancellare o indebolire un test per farla passare, e non zittire clippy con un `allow` generico.
- Ogni correzione del compilatore arriva con un test che fallisce prima e passa dopo, accanto al codice, e controlla il CSS generato.
- I golden test (`crates/parocss-core/tests/golden.rs`) fotografano il CSS di ogni `examples/*.cess` e di ogni `tests/golden/cases/*.cess`; i casi `*.sbagliato.cess` fotografano l'errore. Un nuovo messaggio d'errore porta un caso nuovo. Dopo una modifica voluta: `PAROCSS_UPDATE_GOLDEN=1 cargo test -p parocss-core --test golden`, poi rileggi il diff.
- Le parole del CSS (proprietà, valori, pseudo-classi, `@quanno`) sono di FratmScript: una parola nuova o cambiata si propone a kkratterf/fratmscript in `crates/fratm-core/src/stile.rs`, poi si aggiorna la dipendenza. Qui vivono solo i nomi che il CSS non ha: radici delle classi senza proprietà (`spazio-x`), varianti (`notte:`), chiavi del tema (`chino`, `schermo`).
- I nomi delle classi seguono una regola sola: `proprietà-valore` con le parole di `stile` (`giustifica-spartuto`), più gli alias corti di `ALIASES` in `classi.rs` (`flessibile`, `annascuso`). Misure e breakpoint usano le sigle di Tailwind (`sm`, `md`, `lg`, `2xl`).
- Non cambiare il significato di una classe o di una parola esistente senza una issue che lo chieda: i `.cess` e le pagine già scritti devono continuare a funzionare.
- Il compilatore resta severo: `color`, `:hover`, `@media` e `div` sono errori che suggeriscono `culore`, `:passanno`, `@quanno` e `cascett`. I nomi CSS senza parola napoletana passano come sono. Una classe Tailwind nota (`p-4`, `flex`) suggerisce la sua classe napoletana.
- Un limite di FratmScript non si aggira qui dentro: si segnala a kkratterf/fratmscript con issue e PR.
- Niente dipendenze nuove senza un motivo scritto nella PR.

## Documentazione e backlog

- Niente `PLAN.md`, `ROADMAP.md` o note di progetto nel repo. Documentazione ammessa: i README, i commenti nel codice, questo file, `CLAUDE.md` e `packages/playground/public/llms.txt`.
- Il backlog sta nelle issue di GitHub.

## Commit e PR

- Una sola riga, [Conventional Commits](https://www.conventionalcommits.org/) in italiano: `tipo(scope): messaggio`, minuscolo, senza punto finale. Niente corpo, niente trailer `Co-Authored-By` né altri riferimenti a strumenti.
- Tipi: `feat`, `fix`, `perf`, `refactor`, `test`, `docs`, `ci`, `build`, `chore`.
- Scope: `core`, `cli`, `wasm`, `npm`, `vite`, `browser`, `vscode`, `playground`, `examples`, `ci`.
- Una PR, un argomento. Il titolo della PR è una riga di commit.

## Rilasci

Un tag `vX.Y.Z` fa partire `.github/workflows/release.yml`: binari di `parocss`, il pacchetto npm (messo in attesa su npmjs.com, si approva a mano) e il `.vsix`, tutti nella release di GitHub. SemVer. La versione sta in `Cargo.toml` (`[workspace.package]`), `packages/par-o-css/package.json` e `packages/vscode-extension/package.json`: si cambiano insieme. Fa eccezione l'estensione, che sale da sola quando cambia solo lei (grammatica, completamento).
