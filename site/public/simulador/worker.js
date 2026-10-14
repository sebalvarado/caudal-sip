import init, { check, run, templates } from './pkg/caudal.js';

const ready = init();

self.onmessage = async ({ data: { id, kind, source } }) => {
  await ready;
  try {
    let result;
    if (kind === 'templates') result = JSON.parse(templates());
    if (kind === 'check') result = JSON.parse(check(source));
    if (kind === 'run') {
      result = JSON.parse(run(source, (fraction) => self.postMessage({ id, kind: 'progress', fraction })));
    }
    self.postMessage({ id, kind, result });
  } catch (error) {
    self.postMessage({ id, kind, crash: String(error) });
  }
};
