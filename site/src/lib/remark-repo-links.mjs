import path from 'node:path';
import { visit } from 'unist-util-visit';

const REPO_BLOB = 'https://github.com/sebalvarado/caudal-sip/blob/main';

/**
 * Documents under `docs/` are rendered on the site straight from the repository,
 * so their relative links (`../architecture/blueprint.md`) point at files that
 * have no web route. Rewrite those to the file on GitHub, resolved against the
 * source document's own directory. Links to pages the site does publish are
 * mapped in `WEB_ROUTES` instead.
 */
const WEB_ROUTES = {
  'docs/research/catalogo-operacional.md': '/estandar/',
  'MANIFIESTO.md': '/manifiesto/',
};

export function remarkRepoLinks() {
  return (tree, file) => {
    const abs = file.history?.[0] ?? file.path;
    if (!abs) return;

    const repoRoot = abs.split(`${path.sep}docs${path.sep}`)[0];
    if (!repoRoot || repoRoot === abs) return;
    const fromDir = path.posix.dirname(path.relative(repoRoot, abs).split(path.sep).join('/'));

    visit(tree, 'link', (node) => {
      const url = node.url ?? '';
      if (/^([a-z]+:|\/|#)/i.test(url)) return;

      const [target, hash] = url.split('#');
      const resolved = path.posix.normalize(path.posix.join(fromDir, target));
      const route = WEB_ROUTES[resolved];

      node.url = route
        ? route + (hash ? `#${hash}` : '')
        : `${REPO_BLOB}/${resolved}${hash ? `#${hash}` : ''}`;
    });
  };
}
