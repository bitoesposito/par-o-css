//! The starting theme and the base reset. The values come from Tailwind CSS v4
//! (MIT, see LICENSE), under Neapolitan names: `--culore-russo-500` is
//! `--color-red-500`, `--spazio` is `--spacing`, `--schermo-md` is
//! `--breakpoint-md`. A `@tema { … }` in a `.cess` changes or adds to them.

/// (custom property without `--`, value). The colours are the ones `stile`
/// has a word for and Tailwind a palette: `viola` is `purple`, `azzurro` is `sky`.
pub const THEME: &[(&str, &str)] = &[
    ("culore-russo-50", "oklch(97.1% 0.013 17.38)"),
    ("culore-russo-100", "oklch(93.6% 0.032 17.717)"),
    ("culore-russo-200", "oklch(88.5% 0.062 18.334)"),
    ("culore-russo-300", "oklch(80.8% 0.114 19.571)"),
    ("culore-russo-400", "oklch(70.4% 0.191 22.216)"),
    ("culore-russo-500", "oklch(63.7% 0.237 25.331)"),
    ("culore-russo-600", "oklch(57.7% 0.245 27.325)"),
    ("culore-russo-700", "oklch(50.5% 0.213 27.518)"),
    ("culore-russo-800", "oklch(44.4% 0.177 26.899)"),
    ("culore-russo-900", "oklch(39.6% 0.141 25.723)"),
    ("culore-russo-950", "oklch(25.8% 0.092 26.042)"),
    ("culore-arancione-50", "oklch(98% 0.016 73.684)"),
    ("culore-arancione-100", "oklch(95.4% 0.038 75.164)"),
    ("culore-arancione-200", "oklch(90.1% 0.076 70.697)"),
    ("culore-arancione-300", "oklch(83.7% 0.128 66.29)"),
    ("culore-arancione-400", "oklch(75% 0.183 55.934)"),
    ("culore-arancione-500", "oklch(70.5% 0.213 47.604)"),
    ("culore-arancione-600", "oklch(64.6% 0.222 41.116)"),
    ("culore-arancione-700", "oklch(55.3% 0.195 38.402)"),
    ("culore-arancione-800", "oklch(47% 0.157 37.304)"),
    ("culore-arancione-900", "oklch(40.8% 0.123 38.172)"),
    ("culore-arancione-950", "oklch(26.6% 0.079 36.259)"),
    ("culore-giallo-50", "oklch(98.7% 0.026 102.212)"),
    ("culore-giallo-100", "oklch(97.3% 0.071 103.193)"),
    ("culore-giallo-200", "oklch(94.5% 0.129 101.54)"),
    ("culore-giallo-300", "oklch(90.5% 0.182 98.111)"),
    ("culore-giallo-400", "oklch(85.2% 0.199 91.936)"),
    ("culore-giallo-500", "oklch(79.5% 0.184 86.047)"),
    ("culore-giallo-600", "oklch(68.1% 0.162 75.834)"),
    ("culore-giallo-700", "oklch(55.4% 0.135 66.442)"),
    ("culore-giallo-800", "oklch(47.6% 0.114 61.907)"),
    ("culore-giallo-900", "oklch(42.1% 0.095 57.708)"),
    ("culore-giallo-950", "oklch(28.6% 0.066 53.813)"),
    ("culore-verde-50", "oklch(98.2% 0.018 155.826)"),
    ("culore-verde-100", "oklch(96.2% 0.044 156.743)"),
    ("culore-verde-200", "oklch(92.5% 0.084 155.995)"),
    ("culore-verde-300", "oklch(87.1% 0.15 154.449)"),
    ("culore-verde-400", "oklch(79.2% 0.209 151.711)"),
    ("culore-verde-500", "oklch(72.3% 0.219 149.579)"),
    ("culore-verde-600", "oklch(62.7% 0.194 149.214)"),
    ("culore-verde-700", "oklch(52.7% 0.154 150.069)"),
    ("culore-verde-800", "oklch(44.8% 0.119 151.328)"),
    ("culore-verde-900", "oklch(39.3% 0.095 152.535)"),
    ("culore-verde-950", "oklch(26.6% 0.065 152.934)"),
    ("culore-azzurro-50", "oklch(97.7% 0.013 236.62)"),
    ("culore-azzurro-100", "oklch(95.1% 0.026 236.824)"),
    ("culore-azzurro-200", "oklch(90.1% 0.058 230.902)"),
    ("culore-azzurro-300", "oklch(82.8% 0.111 230.318)"),
    ("culore-azzurro-400", "oklch(74.6% 0.16 232.661)"),
    ("culore-azzurro-500", "oklch(68.5% 0.169 237.323)"),
    ("culore-azzurro-600", "oklch(58.8% 0.158 241.966)"),
    ("culore-azzurro-700", "oklch(50% 0.134 242.749)"),
    ("culore-azzurro-800", "oklch(44.3% 0.11 240.79)"),
    ("culore-azzurro-900", "oklch(39.1% 0.09 240.876)"),
    ("culore-azzurro-950", "oklch(29.3% 0.066 243.157)"),
    ("culore-blu-50", "oklch(97% 0.014 254.604)"),
    ("culore-blu-100", "oklch(93.2% 0.032 255.585)"),
    ("culore-blu-200", "oklch(88.2% 0.059 254.128)"),
    ("culore-blu-300", "oklch(80.9% 0.105 251.813)"),
    ("culore-blu-400", "oklch(70.7% 0.165 254.624)"),
    ("culore-blu-500", "oklch(62.3% 0.214 259.815)"),
    ("culore-blu-600", "oklch(54.6% 0.245 262.881)"),
    ("culore-blu-700", "oklch(48.8% 0.243 264.376)"),
    ("culore-blu-800", "oklch(42.4% 0.199 265.638)"),
    ("culore-blu-900", "oklch(37.9% 0.146 265.522)"),
    ("culore-blu-950", "oklch(28.2% 0.091 267.935)"),
    ("culore-viola-50", "oklch(97.7% 0.014 308.299)"),
    ("culore-viola-100", "oklch(94.6% 0.033 307.174)"),
    ("culore-viola-200", "oklch(90.2% 0.063 306.703)"),
    ("culore-viola-300", "oklch(82.7% 0.119 306.383)"),
    ("culore-viola-400", "oklch(71.4% 0.203 305.504)"),
    ("culore-viola-500", "oklch(62.7% 0.265 303.9)"),
    ("culore-viola-600", "oklch(55.8% 0.288 302.321)"),
    ("culore-viola-700", "oklch(49.6% 0.265 301.924)"),
    ("culore-viola-800", "oklch(43.8% 0.218 303.724)"),
    ("culore-viola-900", "oklch(38.1% 0.176 304.987)"),
    ("culore-viola-950", "oklch(29.1% 0.149 302.717)"),
    ("culore-rosa-50", "oklch(97.1% 0.014 343.198)"),
    ("culore-rosa-100", "oklch(94.8% 0.028 342.258)"),
    ("culore-rosa-200", "oklch(89.9% 0.061 343.231)"),
    ("culore-rosa-300", "oklch(82.3% 0.12 346.018)"),
    ("culore-rosa-400", "oklch(71.8% 0.202 349.761)"),
    ("culore-rosa-500", "oklch(65.6% 0.241 354.308)"),
    ("culore-rosa-600", "oklch(59.2% 0.249 0.584)"),
    ("culore-rosa-700", "oklch(52.5% 0.223 3.958)"),
    ("culore-rosa-800", "oklch(45.9% 0.187 3.815)"),
    ("culore-rosa-900", "oklch(40.8% 0.153 2.432)"),
    ("culore-rosa-950", "oklch(28.4% 0.109 3.907)"),
    ("culore-grigio-50", "oklch(98.5% 0.002 247.839)"),
    ("culore-grigio-100", "oklch(96.7% 0.003 264.542)"),
    ("culore-grigio-200", "oklch(92.8% 0.006 264.531)"),
    ("culore-grigio-300", "oklch(87.2% 0.01 258.338)"),
    ("culore-grigio-400", "oklch(70.7% 0.022 261.325)"),
    ("culore-grigio-500", "oklch(55.1% 0.027 264.364)"),
    ("culore-grigio-600", "oklch(44.6% 0.03 256.802)"),
    ("culore-grigio-700", "oklch(37.3% 0.034 259.733)"),
    ("culore-grigio-800", "oklch(27.8% 0.033 256.848)"),
    ("culore-grigio-900", "oklch(21% 0.034 264.665)"),
    ("culore-grigio-950", "oklch(13% 0.028 261.692)"),
    ("culore-janco", "#fff"),
    ("culore-niro", "#000"),
    ("spazio", "0.25rem"),
    ("schermo-sm", "40rem"),
    ("schermo-md", "48rem"),
    ("schermo-lg", "64rem"),
    ("schermo-xl", "80rem"),
    ("schermo-2xl", "96rem"),
    ("contenitore-3xs", "16rem"),
    ("contenitore-2xs", "18rem"),
    ("contenitore-xs", "20rem"),
    ("contenitore-sm", "24rem"),
    ("contenitore-md", "28rem"),
    ("contenitore-lg", "32rem"),
    ("contenitore-xl", "36rem"),
    ("contenitore-2xl", "42rem"),
    ("contenitore-3xl", "48rem"),
    ("contenitore-4xl", "56rem"),
    ("contenitore-5xl", "64rem"),
    ("contenitore-6xl", "72rem"),
    ("contenitore-7xl", "80rem"),
    ("scrittura-sans", "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', 'Noto Sans', Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji', 'Segoe UI Symbol', 'Noto Color Emoji'"),
    ("scrittura-serif", "ui-serif, Georgia, Cambria, 'Times New Roman', Times, serif"),
    ("scrittura-mono", "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace"),
    ("scrittura-grannezza-xs", "0.75rem"),
    ("scrittura-grannezza-xs--altezza-riga", "calc(1 / 0.75)"),
    ("scrittura-grannezza-sm", "0.875rem"),
    ("scrittura-grannezza-sm--altezza-riga", "calc(1.25 / 0.875)"),
    ("scrittura-grannezza-base", "1rem"),
    ("scrittura-grannezza-base--altezza-riga", "calc(1.5 / 1)"),
    ("scrittura-grannezza-lg", "1.125rem"),
    ("scrittura-grannezza-lg--altezza-riga", "calc(1.75 / 1.125)"),
    ("scrittura-grannezza-xl", "1.25rem"),
    ("scrittura-grannezza-xl--altezza-riga", "calc(1.75 / 1.25)"),
    ("scrittura-grannezza-2xl", "1.5rem"),
    ("scrittura-grannezza-2xl--altezza-riga", "calc(2 / 1.5)"),
    ("scrittura-grannezza-3xl", "1.875rem"),
    ("scrittura-grannezza-3xl--altezza-riga", "calc(2.25 / 1.875)"),
    ("scrittura-grannezza-4xl", "2.25rem"),
    ("scrittura-grannezza-4xl--altezza-riga", "calc(2.5 / 2.25)"),
    ("scrittura-grannezza-5xl", "3rem"),
    ("scrittura-grannezza-5xl--altezza-riga", "1"),
    ("scrittura-grannezza-6xl", "3.75rem"),
    ("scrittura-grannezza-6xl--altezza-riga", "1"),
    ("scrittura-grannezza-7xl", "4.5rem"),
    ("scrittura-grannezza-7xl--altezza-riga", "1"),
    ("scrittura-grannezza-8xl", "6rem"),
    ("scrittura-grannezza-8xl--altezza-riga", "1"),
    ("scrittura-grannezza-9xl", "8rem"),
    ("scrittura-grannezza-9xl--altezza-riga", "1"),
    ("bordo-tunno-xs", "0.125rem"),
    ("bordo-tunno-sm", "0.25rem"),
    ("bordo-tunno-md", "0.375rem"),
    ("bordo-tunno-lg", "0.5rem"),
    ("bordo-tunno-xl", "0.75rem"),
    ("bordo-tunno-2xl", "1rem"),
    ("bordo-tunno-3xl", "1.5rem"),
    ("bordo-tunno-4xl", "2rem"),
    ("ombra-2xs", "0 1px rgb(0 0 0 / 0.05)"),
    ("ombra-xs", "0 1px 2px 0 rgb(0 0 0 / 0.05)"),
    ("ombra-sm", "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)"),
    ("ombra-md", "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)"),
    ("ombra-lg", "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)"),
    ("ombra-xl", "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)"),
    ("ombra-2xl", "0 25px 50px -12px rgb(0 0 0 / 0.25)"),
    ("transizione-durata", "150ms"),
    ("transizione-andatura", "cubic-bezier(0.4, 0, 0.2, 1)"),
];

/// The base reset, in the `pulizia` layer so that every other rule wins over
/// it: no margins, `border-box`, borders ready for `bordo`, media as blocks,
/// form controls that inherit the font. Adapted from Tailwind's preflight.
pub const PULIZIA: &str = r#"@layer pulizia {
  *, ::before, ::after, ::backdrop, ::file-selector-button {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
    border: 0 solid;
  }
  html, :host {
    line-height: 1.5;
    -webkit-text-size-adjust: 100%;
    tab-size: 4;
    font-family: var(--scrittura-sans);
    -webkit-tap-highlight-color: transparent;
  }
  hr {
    height: 0;
    color: inherit;
    border-top-width: 1px;
  }
  h1, h2, h3, h4, h5, h6 {
    font-size: inherit;
    font-weight: inherit;
  }
  a {
    color: inherit;
    text-decoration: inherit;
  }
  b, strong {
    font-weight: bolder;
  }
  code, kbd, samp, pre {
    font-family: var(--scrittura-mono);
    font-size: 1em;
  }
  small {
    font-size: 80%;
  }
  table {
    text-indent: 0;
    border-color: inherit;
    border-collapse: collapse;
  }
  ol, ul, menu {
    list-style: none;
  }
  img, svg, video, canvas, audio, iframe, embed, object {
    display: block;
    vertical-align: middle;
  }
  img, video {
    max-width: 100%;
    height: auto;
  }
  button, input, select, optgroup, textarea, ::file-selector-button {
    font: inherit;
    letter-spacing: inherit;
    color: inherit;
    border-radius: 0;
    background-color: transparent;
    opacity: 1;
  }
  ::placeholder {
    opacity: 1;
    color: color-mix(in oklab, currentcolor 50%, transparent);
  }
  textarea {
    resize: vertical;
  }
  button, input:where([type="button"], [type="reset"], [type="submit"]), ::file-selector-button {
    appearance: button;
  }
  [hidden]:where(:not([hidden="until-found"])) {
    display: none !important;
  }
}
"#;
