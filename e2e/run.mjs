// End-to-end checks of the playground in Chromium, on its production build.
// Needs packages/par-o-css built (npm run build) and packages/playground built.
// CHROMIUM_PATH picks a Chromium other than Playwright's.
import { spawn } from 'node:child_process';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import assert from 'node:assert/strict';
import { chromium } from 'playwright';

const repo = join(dirname(fileURLToPath(import.meta.url)), '..');
const servers = [];

async function serve(command, args, cwd, url) {
  const child = spawn(command, args, { cwd, stdio: 'ignore' });
  servers.push(child);
  for (let i = 0; i < 100; i++) {
    try {
      if ((await fetch(url)).status < 500) return url.replace(/\/$/, '');
    } catch {}
    await new Promise((r) => setTimeout(r, 100));
  }
  throw new Error(`${command} did not start on ${url}`);
}

/** A page that fails the run on console errors (except a missing favicon). */
async function open(context, url) {
  const page = await context.newPage();
  page.problems = [];
  page.on('console', (m) => {
    if (m.type() === 'error' && !m.location().url.endsWith('favicon.ico')) page.problems.push(m.text());
  });
  page.on('pageerror', (e) => page.problems.push(e.message));
  await page.goto(url);
  return page;
}

/** Writes at the end of an editor. */
async function typeAtEnd(page, editor, text) {
  await page.click(`${editor} .cm-content`);
  await page.keyboard.press('Control+End');
  await page.keyboard.type(text);
}

async function playgroundWorks(context, base) {
  const page = await open(context, base + '/');
  const anteprima = page.frameLocator('#anteprima iframe');
  // The status bar says how things are without words: pronto, sbaglio, attesa, carica
  const stato = (s) => page.locator(`#barra-stato[data-stato="${s}"]`).waitFor();
  await stato('pronto');

  // The page with the CSS of stile.cess and of its classes, and its console
  assert.equal(await anteprima.locator('h1').textContent(), 'Uè, munno!');
  assert.equal(await anteprima.locator('h1').evaluate((h) => getComputedStyle(h).fontWeight), '700');
  await anteprima.locator('button').click();
  await page.locator('#console li', { hasText: 'Trase, trase!' }).waitFor();
  await page.locator('#css .cm-content', { hasText: ':root' }).waitFor();

  // Two files, two tabs
  await page.click('[data-file-scheda="index.emmurt"]');
  await page.locator('#editor-paggena .cm-content', { hasText: '<cristian' }).waitFor();
  assert.ok(await page.locator('#editor-cess').isHidden());
  await page.click('[data-file-scheda="stile.cess"]');
  await page.locator('#editor-cess .cm-content', { hasText: '@chiamm' }).waitFor();

  // The help: classes with their Tailwind name, words with their CSS name
  await page.click('#aiuto-apri');
  await page.fill('#cerca', 'flessibile');
  assert.ok((await page.locator('#parole tr:visible').allTextContents()).some((r) => r.includes('flex')));
  await page.fill('#cerca', 'passanno');
  assert.ok((await page.locator('#parole tr:visible').allTextContents()).some((r) => r.includes(':hover')));
  await page.click('#aiuto-chiude');

  // A mistake on purpose: the error in Neapolitan, with its suggestion, in the tab of its file
  await page.selectOption('#esempi', 'sbagliato');
  await stato('sbaglio');
  assert.match(await page.locator('#sbagli').textContent(), /'color' nun è napoletano\.\s*Vuoi dire 'culore'\?/);
  await page.locator('[data-file-scheda="stile.cess"][data-sbaglio]').waitFor();
  await page.locator('#editor-cess .cm-lintRange-error').first().waitFor();
  // An error in the page goes to the page's tab
  await page.selectOption('#esempi', 'primmo');
  await stato('pronto');
  await page.click('[data-file-scheda="index.emmurt"]');
  await typeAtEnd(page, '#editor-paggena', '\n<div>rotto</div>');
  await page.keyboard.press('Control+Enter');
  await stato('sbaglio');
  await page.locator('[data-file-scheda="index.emmurt"][data-sbaglio]').waitFor();
  assert.equal(await page.locator('[data-file-scheda="stile.cess"][data-sbaglio]').count(), 0);
  await page.locator('#editor-paggena .cm-lintRange-error').first().waitFor();
  assert.match(await page.locator('#sbagli').textContent(), /index\.emmurt/);

  // The editor completes par-o-css classes in classe="…", with their Tailwind name
  await page.selectOption('#esempi', 'primmo');
  await stato('pronto');
  await page.click('#editor-paggena .cm-content');
  await page.keyboard.press('Control+Home');
  await page.keyboard.type('<cascett classe="flessi');
  await page.locator('.cm-tooltip-autocomplete li').first().waitFor();
  const proposte = await page.locator('.cm-tooltip-autocomplete li').allTextContents();
  assert.ok(proposte.some((p) => p.startsWith('flessibile') && p.includes('flex')), proposte.join());
  await page.keyboard.press('Escape');

  // Format: .cess with par-o-css; the page waits for htmlemmurt's format, and says so
  await page.selectOption('#esempi', 'primmo');
  await stato('pronto');
  assert.equal(await page.locator('#formatta').getAttribute('aria-disabled'), 'true');
  await page.click('#editor-paggena .cm-content');
  await page.keyboard.press('Shift+Alt+F');
  await page.locator('#avviso:not([hidden])', { hasText: 'htmlemmurt' }).waitFor();
  await page.click('[data-file-scheda="stile.cess"]');
  assert.equal(await page.locator('#formatta').getAttribute('aria-disabled'), 'false');
  await page.click('#editor-cess .cm-content');
  await page.keyboard.press('Control+End');
  await page.keyboard.type('\n.nova{culore:russo}');
  await page.click('#formatta');
  await page.locator('#editor-cess .cm-line', { hasText: /^\.nova \{$/ }).waitFor();
  await page.locator('#editor-cess .cm-line', { hasText: /^ {2}culore:russo$/ }).waitFor();
  // In the .cess, @applica completes the classes too
  await typeAtEnd(page, '#editor-cess', '\n.x {\n@applica grass');
  await page.locator('.cm-tooltip-autocomplete li', { hasText: 'grassetto' }).waitFor();
  await page.keyboard.press('Escape');

  // Learn: the first lesson is done with Soluzione; the fourth with a rule
  // in the .cess instead of the classes of its solution
  await page.click('#impara');
  await page.locator('#lezione:not([hidden])').waitFor();
  assert.equal(await page.locator('#lezione-compito[data-fatto]').count(), 0);
  await page.click('#lezione-soluzione');
  await page.locator('#lezione-compito[data-fatto]').waitFor();
  await page.selectOption('#lezione-scegli', '3');
  await page.locator('#lezione-compito:not([data-fatto])').waitFor();
  await page.click('[data-file-scheda="stile.cess"]');
  await typeAtEnd(page, '#editor-cess', '\ncappiell {\nmostra: flessibile\ngiustifica: spartuto\nallinea-cose: miezo\n}');
  await page.locator('#lezione-compito[data-fatto]').waitFor();
  await page.click('#lezione-esci');

  // The guide: every example runs in its own preview and opens in the editor
  await page.click('#guida');
  await page.frameLocator('#guida-esempi iframe').first().locator('h1').waitFor();
  assert.equal(
    await page.frameLocator('#guida-esempi iframe').first().locator('h1').evaluate((h) => getComputedStyle(h).color),
    'rgb(255, 0, 0)',
  );
  await page.click('#guida-esempi [data-azione="apri"]');
  await page.locator('#guida-paggena').waitFor({ state: 'hidden' });
  await stato('pronto');
  await page.locator('#editor-cess .cm-content', { hasText: 'luccolone' }).waitFor();

  // The shared link opens the same two files
  await page.selectOption('#esempi', 'pizzeria');
  await stato('pronto');
  await page.click('#condividi');
  await page.waitForFunction(() => location.hash.startsWith('#codice='));
  const link = page.url();
  assert.deepEqual(page.problems, []);
  await page.close();

  const shared = await open(context, link);
  await shared.locator('#barra-stato[data-stato="pronto"]').waitFor();
  await shared.frameLocator('#anteprima iframe').locator('h1', { hasText: "'O menù" }).waitFor();
  await shared.locator('#editor-cess .cm-content', { hasText: 'pezzi/carta.cess' }).waitFor();
  // Once open, the code lives in localStorage: an edit leaves the link
  await shared.click('#editor-cess .cm-content');
  await shared.keyboard.type(' ');
  assert.equal(await shared.evaluate(() => location.hash), '');
  assert.deepEqual(shared.problems, []);
  await shared.close();
}

const browser = await chromium.launch({ executablePath: process.env.CHROMIUM_PATH });
try {
  const dist = join(repo, 'packages', 'playground', 'dist');
  const site = await serve('python3', ['-m', 'http.server', '5294'], dist, 'http://localhost:5294/');
  // Italian, as the interface defaults to the browser's language
  const context = await browser.newContext({ locale: 'it-IT', permissions: ['clipboard-read', 'clipboard-write'] });
  await playgroundWorks(context, site);
  console.log('✓ playground');
} finally {
  await browser.close();
  for (const server of servers) server.kill();
}
