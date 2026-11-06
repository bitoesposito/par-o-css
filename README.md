<p align="center"><img src="icone/parocss-scudo.svg" alt="" width="96"></p>

# par-o-css

CSS, ma comme si deve 🤌

par-o-css è CSS in napoletano, con le classi pronte alla Tailwind. È il fratello di [htmlemmurt](https://github.com/bitoesposito/htmlemmurt) e di [FratmScript](https://github.com/kkratterf/fratmscript), e ne usa le stesse parole: `cascett` è `div`, `culore` è `color`, `:passanno` è `:hover`.

I file `.cess` sono CSS con le parole dello `stile` di FratmScript, e diventano CSS normale:

```css
citofono:passanno {
  culore: janco
  sfunno-culore: russo
  bordo-tunno: 8px
}

@quanno (larghezza-massima: 600px) {
  core { spazio: 1rem }
}
```

diventa

```css
button:hover {
  color: white;
  background-color: red;
  border-radius: 8px
}

@media (max-width: 600px) {
  main { padding: 1rem }
}
```

E le pagine si vestono con le classi, come con Tailwind:

```html
<cascett classe="flessibile giustifica-spartuto allinea-cose-miezo spazio-4 md:spazio-8 sfunno-janco ombra-md bordo-tunno-lg">
  <luccolo classe="scrittura-grannezza-xl grassetto culore-russo-600">Margherita</luccolo>
  <citofono classe="spazio-x-4 spazio-y-2 sfunno-blu-500 passanno:sfunno-blu-600 culore-janco">Ordina</citofono>
</cascett>
```

Basta un `.cess` con `@chiamm "par-o-css"`: il compilatore ci mette il tema, una pulizia di base e una regola per ogni classe che le pagine usano davvero, e nient'altro.

## Le regole

Dove esiste la parola napoletana, il compilatore accetta solo quella. `color` è un errore, e l'errore ti dice cosa scrivere al posto suo:

```
✗ stile.cess:2:3: 'color' nun è napoletano.
     2 |   color: russo
       |   ^
  Vuoi dire 'culore'?
```

Vale per le proprietà (`color` → `culore`), i valori (`red` → `russo`), i tag nei selettori (`div` → `cascett`), le pseudo-classi (`:hover` → `:passanno`), `@media` (`@quanno`) e `!important` (`!importante`). I nomi CSS che non hanno ancora una parola napoletana passano come sono: `grid-column`, `:nth-child`, `rotate(45deg)`, i nomi delle tue classi.

Come in FratmScript, una riga nuova chiude la dichiarazione: il `;` si può lasciare.

Le parole del CSS sono quelle di `stile` in FratmScript: stanno in [`stile.rs`](https://github.com/kkratterf/fratmscript/blob/main/crates/fratm-core/src/stile.rs) e `parocss classi` le ricorda insieme alle classi.

## Le tre regole di par-o-css

`@chiamm "par-o-css"` mette il tema, la pulizia di base e le classi usate nelle pagine. Va una volta sola, in cima al foglio principale. Con un altro file, `@chiamm "pezzi/carta.cess"` lo incolla al suo posto; un percorso che non finisce in `.cess` resta un `@import`.

`@tema { … }` cambia il tema o lo allarga. Un colore nuovo diventa subito una classe:

```css
@tema {
  --culore-pummarola-500: oklch(58% 0.21 28)
  --schermo-tablet: 50rem
}
```

e adesso ci sono `culore-pummarola-500`, `sfunno-pummarola-500`, `bordo-pummarola-500` e la variante `tablet:`.

`@applica` scrive dentro una regola le dichiarazioni di alcune classi. Le varianti diventano regole annidate:

```css
.ordina {
  @applica spazio-x-4 spazio-y-2 bordo-tunno-lg sfunno-russo-500 culore-janco
  @applica passanno:sfunno-russo-600
}
```

## Le classi

Una classe è `proprietà-valore` con le parole di `stile`: `giustifica-spartuto` è `justify-content: space-between`, `trabocca-nascosto` è `overflow: hidden`, `cursore-manella` è `cursor: pointer`. Le più usate hanno un nome corto, come in Tailwind, e le misure si scrivono con le sigle di Tailwind (`xs`, `sm`, `md`, `lg`, `xl`, `2xl`…).

Il compilatore conosce le classi di Tailwind: `p-4` in un `@applica` è un errore che suggerisce `spazio-4`.

<details>
<summary>Classi sane</summary>

| par-o-css | Tailwind | CSS |
|---|---|---|
| `blocco` | `block` | `display: block` |
| `in-linea` | `inline` | `display: inline` |
| `blocco-in-linea` | `inline-block` | `display: inline-block` |
| `flessibile` | `flex` | `display: flex` |
| `flessibile-in-linea` | `inline-flex` | `display: inline-flex` |
| `griglia` | `grid` | `display: grid` |
| `griglia-in-linea` | `inline-grid` | `display: inline-grid` |
| `annascuso` | `hidden` | `display: none` |
| `annascuso-all-uocchie` | `sr-only` | nascosto alla vista, non ai lettori di schermo |
| `statico` | `static` | `position: static` |
| `fisso` | `fixed` | `position: fixed` |
| `assoluto` | `absolute` | `position: absolute` |
| `relativo` | `relative` | `position: relative` |
| `appiccecato` | `sticky` | `position: sticky` |
| `a-capo` | `flex-wrap` | `flex-wrap: wrap` |
| `nun-va-a-capo` | `flex-nowrap` | `flex-wrap: nowrap` |
| `cresce` | `grow` | `flex-grow: 1` |
| `grassetto` | `font-bold` | `font-weight: bold` |
| `corsivo` | `italic` | `font-style: italic` |
| `sottolineato` | `underline` | `text-decoration-line: underline` |
| `sbarrato` | `line-through` | `text-decoration-line: line-through` |
| `maiuscolo` | `uppercase` | `text-transform: uppercase` |
| `minuscolo` | `lowercase` | `text-transform: lowercase` |
| `bordo` | `border` | `border-width: 1px` |
| `bordo-tunno` | `rounded` | `border-radius: var(--bordo-tunno-sm)` |
| `ombra` | `shadow` | `box-shadow: var(--ombra-sm)` |
| `contorno` | `outline` | `outline: 1px solid` |
| `transizione` | `transition` | colori, opacità, ombre e trasformazioni in 150ms |

</details>

<details>
<summary>Radici: radice-valore</summary>

I numeri di spaziatura sono multipli di `--spazio` (0.25rem): `spazio-4` è 1rem, `spazio-0.5` è 0.125rem, `spazio-px` è 1px. Le radici con `-` davanti vanno anche al negativo: `-margine-ncoppa-2`.

| par-o-css | Tailwind | CSS | Valori |
|---|---|---|---|
| `ncoppa`, `sotto`, `manca`, `destra` | `top`, `bottom`, `left`, `right` | `top`… | spaziatura, `1/2`, `chino`, `automatico` |
| `livello` | `z` | `z-index` | numero |
| `flessibile` | `flex` | `flex` | numero, `nisciuno`, `automatico` |
| `cresce` | `grow` | `flex-grow` | numero |
| `colonne`, `righe` | `grid-cols`, `grid-rows` | `grid-template-columns`, `-rows` | quante colonne: `colonne-3` |
| `spazio-mmiez`, `spazio-mmiez-x`, `spazio-mmiez-y` | `gap`, `gap-x`, `gap-y` | `gap`, `column-gap`, `row-gap` | spaziatura |
| `spazio`, `spazio-x`, `spazio-y` | `p`, `px`, `py` | `padding`, `-inline`, `-block` | spaziatura |
| `spazio-ncoppa`, `spazio-sotto`, `spazio-manca`, `spazio-destra` | `pt`, `pb`, `pl`, `pr` | `padding-top`… | spaziatura |
| `margine`, `margine-x`, `margine-y` | `m`, `mx`, `my` | `margin`, `-inline`, `-block` | spaziatura, `automatico` |
| `margine-ncoppa`, `margine-sotto`, `margine-manca`, `margine-destra` | `mt`, `mb`, `ml`, `mr` | `margin-top`… | spaziatura, `automatico` |
| `larghezza`, `larghezza-minima`, `larghezza-massima` | `w`, `min-w`, `max-w` | `width`… | spaziatura, `1/2`, `chino`, `schermo`, `3xs`…`7xl` |
| `altezza`, `altezza-minima`, `altezza-massima` | `h`, `min-h`, `max-h` | `height`… | spaziatura, `1/2`, `chino`, `schermo` |
| `scrittura` | `font` | `font-family` | `sans`, `serif`, `mono` |
| `scrittura-grannezza` | `text` | `font-size` e `line-height` | `xs`, `sm`, `base`, `lg`, `xl`, `2xl`…`9xl` |
| `scrittura-peso` | `font` | `font-weight` | `100`…`900`, `grassetto`, `normale` |
| `altezza-riga` | `leading` | `line-height` | spaziatura, `normale` |
| `spazio-lettere` | `tracking` | `letter-spacing` | `normale`, `[0.05em]` |
| `culore` | `text` | `color` | colore |
| `sfunno`, `sfunno-culore` | `bg` | `background-color` | colore |
| `bordo`, `bordo-culore`, `bordo-larghezza` | `border` | `border-color` o `border-width` | colore o numero (px) |
| `bordo-x`, `bordo-y`, `bordo-ncoppa`, `bordo-sotto`, `bordo-manca`, `bordo-destra` | `border-x`, `border-y`, `border-t`, `border-b`, `border-l`, `border-r` | colore o larghezza di un lato | colore o numero (px) |
| `bordo-tunno` | `rounded` | `border-radius` | `xs`…`4xl`, `chino`, `nisciuno` |
| `ombra` | `shadow` | `box-shadow` | `2xs`…`2xl`, `nisciuno` |
| `opacità`, `opacita` | `opacity` | `opacity` | percentuale: `opacità-50` |
| `contorno` | `outline` | `outline-color` o `outline-width` | colore o numero (px) |
| `proporzione` | `aspect` | `aspect-ratio` | `16/9`, `1/1`, `automatico` |
| `contenuto` | `content` | `content` | `nisciuno`, `['★']` |

</details>

<details>
<summary>Colori</summary>

I colori sono quelli di `stile` che hanno una tavolozza in Tailwind, con le stesse sfumature da `50` a `950`: `russo` (red), `arancione` (orange), `giallo` (yellow), `verde` (green), `azzurro` (sky), `blu` (blue), `viola` (purple), `rosa` (pink), `grigio` (gray). Poi `janco`, `niro`, `trasparente` e `culore-corrente`, senza sfumatura.

La sfumatura ci vuole: `culore-russo-500`, non `culore-russo`. L'opacità va dopo la `/`: `sfunno-niro/50`.

</details>

<details>
<summary>Varianti</summary>

Una variante va davanti alla classe, con i due punti: `passanno:sfunno-blu-600`, `md:colonne-2`, e si possono mettere una dopo l'altra: `md:passanno:sottolineato`. Gli stati usano le pseudo-classi di `stile`; i breakpoint le sigle di Tailwind.

| par-o-css | Tailwind | CSS |
|---|---|---|
| `passanno:` | `hover:` | `:hover` |
| `premuto:` | `active:` | `:active` |
| `fuoco:` | `focus:` | `:focus` |
| `fuoco-visibile:` | `focus-visible:` | `:focus-visible` |
| `stutato:` | `disabled:` | `:disabled` |
| `spuntato:` | `checked:` | `:checked` |
| `primmo-figlio:` | `first:` | `:first-child` |
| `urdemo-figlio:` | `last:` | `:last-child` |
| `vacante:` | `empty:` | `:empty` |
| `esempio:` | `placeholder:` | `::placeholder` |
| `primma:` | `before:` | `::before` |
| `doppo:` | `after:` | `::after` |
| `notte:` | `dark:` | `@media (prefers-color-scheme: dark)` |
| `stampa:` | `print:` | `@media print` |
| `sm:`, `md:`, `lg:`, `xl:`, `2xl:` | gli stessi | `@media (width >= 40rem)`… `96rem` |

</details>

Un valore che non sta nel tema va tra parentesi quadre, con `_` al posto degli spazi: `larghezza-[37px]`, `colonne-[1fr_2fr]`, `sfunno-[url(pizza.png)]`. Tra parentesi quadre ci può stare anche una proprietà intera: `[grid-column:span_2]`. Un `!` in fondo aggiunge `!important`: `spazio-0!`.

## La CLI

```bash
cargo install --git https://github.com/bitoesposito/par-o-css parocss-cli
```

```bash
parocss build stile.cess            # scrive stile.css accanto
parocss build stile/ --out dist/    # tutti i .cess di una cartella
parocss check stile/                # solo controllo
parocss watch stile/ --out dist/    # ricompila quando cambia un .cess o una pagina
parocss classi                      # tutte le classi, radici e varianti
parocss classi md:flessibile p-4    # il CSS di quelle classi (e cosa scrivere per p-4)
parocss guida                       # le istruzioni per gli agenti (llms.txt)
```

Le classi si cercano nei file della cartella dove lanci il comando (`.emmurt`, `.fratm`, `.html`, `.js`, `.jsx`, `.ts`, `.tsx`, `.vue`, `.svelte`, `.astro`, `.md`), saltando `node_modules`, `dist`, `target` e le cartelle nascoste; `--pagine <cartella>` ne sceglie un'altra. Come Tailwind, la ricerca non conosce il linguaggio del file: trova anche le classi scritte dentro le stringhe del codice. In una cartella, un `.cess` chiamato da un altro con `@chiamm` è un pezzo e non diventa un `.css` per conto suo.

`--json` stampa un solo oggetto JSON con gli errori (file, riga, colonna, suggerimento), utile per editor e agenti. `-` legge da stdin.

## Sviluppo

Le regole per chi contribuisce, persone o agenti, stanno in [`AGENTS.md`](AGENTS.md).

```bash
cargo test --workspace                                   # compilatore, CLI, golden test
cargo build -p parocss-cli && ./target/debug/parocss check examples
```

## Licenza

MIT. Il tema di partenza e la pulizia di base vengono da Tailwind CSS (MIT): vedi [`LICENSE`](LICENSE).
