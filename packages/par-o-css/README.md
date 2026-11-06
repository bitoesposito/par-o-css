# par-o-css

Il compilatore di [par-o-css](https://github.com/bitoesposito/par-o-css) (CSS in napoletano, con le classi pronte alla Tailwind) in WebAssembly, con un plugin Vite e un runtime per il browser.

```bash
npm i -D par-o-css
```

## Da Node

```js
import { compile, findClasses, formatError } from 'par-o-css';

const { css, dependencies, errors } = compile('@chiamm "par-o-css"\ncitofono:passanno { culore: russo }', {
  filename: 'stile.cess',
  classes: findClasses('.'),
});
```

`compile` non lancia mai eccezioni: gli errori stanno in `errors`, con `message`, `file`, `line`, `column` e `suggestion`, in napoletano. Le opzioni sono `filename`, `classes` (le parole delle pagine che possono essere classi: `findClasses(cartella)` le cerca nei file `.emmurt`, `.fratm`, `.html`, `.js`… di una cartella), `root` (la cartella per i percorsi che cominciano con `/`) e `read` (per leggere i file in un altro modo).

C'è anche `scan(testo)` (le parole che possono essere classi), `suggestion('p-4')` (`'spazio-4'`), `isClass(nome)`, `classCss(classi)` (il CSS di alcune classi col tema di partenza) e `names()` (tutte le classi, radici e varianti).

## Con Vite

```js
// vite.config.js
import parocss from 'par-o-css/vite';

export default { plugins: [parocss()] };
```

Un `.cess` si importa da un modulo (`import './stile.cess'`, in FratmScript `chiamm "./stile.cess"`) o si collega da una pagina (`<link rel="stylesheet" href="stile.cess">`, in htmlemmurt `<attacca rapporto="stile" addo="stile.cess">`). Il plugin lo compila con le classi che trova nei file del progetto, e lo ricompila quando una pagina usa una classe nuova. Va insieme a `htmlemmurt/vite` e `fratmscript/vite`.

## Nel browser, senza build

```html
<script type="module" src="https://cdn.jsdelivr.net/npm/par-o-css@0.1/browser.js"></script>
<link rel="stylesheet" type="text/cess" href="stile.cess">
```

Ogni `<link type="text/cess">` e `<style type="text/cess">` diventa uno `<style>` con il CSS compilato e le classi della pagina. Senza nessun `.cess`, la pagina ha comunque tema, pulizia e classi, come se ci fosse `@chiamm "par-o-css"`. Quando il codice aggiunge una classe che la pagina non aveva, il CSS si ricompila. Gli errori compaiono nella pagina e nella console. Serve un server (anche `python3 -m http.server`), perché il browser deve scaricare i file.

`par-o-css/web` è il compilatore per il browser senza il runtime: `await init()`, poi le stesse funzioni di Node.
