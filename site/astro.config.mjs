import { defineConfig } from 'astro/config';
import mdx from '@astrojs/mdx';
import sitemap from '@astrojs/sitemap';
import { unified } from '@astrojs/markdown-remark';
import { remarkRepoLinks } from './src/lib/remark-repo-links.mjs';
import { rehypeWrapTables } from './src/lib/rehype-wrap-tables.mjs';
import { rehypeStripDocTitle } from './src/lib/rehype-strip-doc-title.mjs';

export default defineConfig({
  // One switch for the public origin: canonical URLs, sitemap, RSS, robots and
  // llms.txt all derive from it. Set SITE_URL in the host's build settings when
  // the custom domain goes live; previews keep their own origin meanwhile.
  site: process.env.SITE_URL ?? 'https://caudal-sip.pages.dev',
  integrations: [mdx(), sitemap()],
  // Documents rendered from `docs/` carry repo-relative links; rewrite them so
  // they resolve on the web (see src/lib/remark-repo-links.mjs).
  markdown: {
    processor: unified({
      remarkPlugins: [remarkRepoLinks],
      rehypePlugins: [rehypeWrapTables, rehypeStripDocTitle],
    }),
  },
  i18n: {
    defaultLocale: 'es',
    locales: ['es', 'en'],
    routing: { prefixDefaultLocale: false },
  },
});
