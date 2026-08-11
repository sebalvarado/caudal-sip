# site/

The public website and blog for Proyecto Caudal — [Astro](https://astro.build), static output, no runtime.

```bash
npm install
npm run dev      # http://localhost:4321
npm run build    # static output in dist/ — must pass before merging
npm run preview  # serve the built output
```

## What lives where

| Path | Contents |
|---|---|
| `src/config.ts` | Project identity: name, tagline, description, GitHub URL. The one file to edit for branding. |
| `src/content/blog/` | Blog posts (Markdown). Frontmatter schema in `src/content.config.ts`. |
| `src/pages/estandar/` | Renders `docs/research/catalogo-operacional.md` — see below. |
| `src/pages/manifiesto.md` | Mirror of the root `MANIFIESTO.md` — **change both together.** |
| `src/layouts/` | `Base.astro` (shell, SEO, JSON-LD) and `Prose.astro` (long-form pages). |
| `src/lib/remark-repo-links.mjs` | Rewrites repo-relative links in rendered docs to GitHub or to site routes. |
| `src/styles/global.css` | All styling. No framework, no build-time CSS tooling. |
| `src/pages/robots.txt.ts`, `llms.txt.ts` | Generated at build so they always match the live origin. |

## Documents rendered from `docs/`

Reference documents are **not copied into the site**. The `docs` content collection in `src/content.config.ts` loads them from `../docs/`, so the repository keeps a single canonical copy and there is nothing to keep in sync. Page metadata — title, description, JSON-LD — lives in the route, which keeps the source markdown clean for readers on GitHub.

Two consequences worth knowing. Editing a file under `docs/` changes the site, which is why the CI workflow watches `docs/**` as well as `site/**`. And links inside those documents are repo-relative, so `remark-repo-links.mjs` rewrites them at build: to a site route when the page exists here, otherwise to the file on GitHub. Add new mappings to `WEB_ROUTES` in that file when you publish another document.

The `manifiesto.md` mirror predates this arrangement and is the one exception; it stays a mirror because it is edited as public copy rather than as documentation.

Public-facing copy is Spanish and must follow [`docs/spanish-style.md`](../docs/spanish-style.md).

## The site origin

`site` in `astro.config.mjs` drives canonical URLs, the sitemap, RSS, `robots.txt` and `llms.txt`. It defaults to the Cloudflare Pages subdomain and is overridden by the `SITE_URL` environment variable, so pointing the project at a custom domain means setting that variable in the host's build settings — no code change.

## Deployment

Cloudflare Pages builds and deploys `main` automatically, and gives every pull request a preview URL.

| Setting | Value |
|---|---|
| Framework preset | Astro |
| Root directory | `site` |
| Build command | `npm run build` |
| Output directory | `dist` |
| Node version | from `.node-version` (22) |

GitHub Actions builds the site on every pull request as a second check ([`.github/workflows/site.yml`](../.github/workflows/site.yml)); it does not deploy.

### Adding the custom domain later

1. Cloudflare Pages → the project → *Custom domains* → add the apex and `www`.
2. Add `SITE_URL=https://<domain>` to the production environment variables and redeploy, so canonical URLs and the sitemap point at the real domain.
3. Redirect `www` to the apex, and submit the sitemap in Google Search Console.
