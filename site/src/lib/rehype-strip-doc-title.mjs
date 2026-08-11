/**
 * Documents under `docs/` open with their own `# Title`, which the site route
 * renders itself so the heading, the intro note and the table of contents come
 * in a sensible order. Drop the source heading to avoid two `h1`s on the page.
 * Only applies to content loaded from `docs/` — blog posts are untouched.
 */
export function rehypeStripDocTitle() {
  return (tree, file) => {
    const abs = file.history?.[0] ?? file.path ?? '';
    if (!abs.includes('/docs/')) return;

    const first = tree.children.findIndex(
      (n) => n.type === 'element' && n.tagName === 'h1',
    );
    if (first !== -1) tree.children.splice(first, 1);
  };
}
