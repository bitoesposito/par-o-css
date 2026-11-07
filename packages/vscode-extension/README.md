# par-o-css per VS Code

Supporto ai file `.cess` di [par-o-css](https://github.com/bitoesposito/par-o-css), il CSS in napoletano, e alle sue classi pronte alla Tailwind.

```css
@chiamm "par-o-css"

citofono:passanno {
  culore: janco
  sfunno-culore: russo
}

.carta {
  @applica spazio-6 bordo-tunno-xl ombra-md md:spazio-8
}
```

## Cosa fa

Gli errori del compilatore compaiono mentre scrivi, in napoletano e col suggerimento: scrivi `color` e ti dice "Vuoi dire 'culore'?". Un pezzo chiamato con `@chiamm` da un altro `.cess` si controlla dentro il foglio che lo chiama, col suo `@tema`.

Il completamento propone le classi dentro `classe="…"`, `class="…"` e `className`, nelle pagine htmlemmurt, HTML, FratmScript, JSX, Vue, Svelte e Astro, e dopo `@applica` nei `.cess`. Accanto a ogni classe c'è il suo nome in Tailwind, e il CSS che genera. Nei `.cess` completa anche le proprietà e i valori napoletani.

Passando il mouse su una classe vedi il suo CSS; su una classe Tailwind, quella da scrivere al posto suo; su una parola di un `.cess`, il suo CSS inglese.

I colori seguono il CSS, con le parole napoletane colorate come quelle inglesi, anche dentro `<cess>` nelle pagine htmlemmurt. Ctrl+clic (Cmd+clic su macOS) sul percorso di `@chiamm "…"` apre il file.

"Formatta documento" (Maiusc+Alt+F) rimette a posto un `.cess` come `parocss fmt`: due spazi per livello, una dichiarazione per riga, al massimo una riga vuota di fila. Su un file con un errore di sintassi non fa niente.

L'impostazione `parocss.diagnostics` spegne gli errori mentre scrivi.
