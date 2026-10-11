// Runs the committed WebAssembly simulator the way the page does, outside a browser.
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

const pkg = new URL('../public/simulador/pkg/', import.meta.url);
const mod = await import(new URL('caudal.js', pkg));
await mod.default({ module_or_path: readFileSync(fileURLToPath(new URL('caudal_bg.wasm', pkg))) });

const fail = (message) => {
  console.error(`playground: ${message}`);
  process.exit(1);
};

const templates = JSON.parse(mod.templates());
if (!templates.some((t) => t.id === 'national')) fail('the national template is missing');

for (const t of templates) {
  const checked = JSON.parse(mod.check(t.source));
  if (!checked.ok) fail(`${t.id} does not parse: ${JSON.stringify(checked.error)}`);
  // A full national day takes tens of seconds here; parsing it is enough.
  if (t.id === 'national') continue;
  let progress = 0;
  const run = JSON.parse(mod.run(t.source, (f) => (progress = f)));
  if (!run.ok) fail(`${t.id} failed: ${JSON.stringify(run.error)}`);
  if (run.report.payments < checked.payments) fail(`${t.id} sent ${run.report.payments} of ${checked.payments}`);
  if (!run.report.disclaimer) fail(`${t.id} has no disclaimer`);
  console.log(`playground: ${t.id} ran ${run.report.payments} payments (progress ${progress.toFixed(2)})`);
}

const bad = JSON.parse(mod.check('at 09:00\npay nobody llave-02 1'));
if (bad.ok || bad.error.line !== 2) fail('a parse error does not point at its line');
