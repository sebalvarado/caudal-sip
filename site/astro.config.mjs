import { defineConfig } from 'astro/config';
import mdx from '@astrojs/mdx';
import sitemap from '@astrojs/sitemap';

export default defineConfig({
  // One switch for the public origin: canonical URLs, sitemap, RSS, robots and
  // llms.txt all derive from it. Set SITE_URL in the host's build settings when
  // the custom domain goes live; previews keep their own origin meanwhile.
  site: process.env.SITE_URL ?? 'https://caudal-sip.pages.dev',
  integrations: [mdx(), sitemap()],
  i18n: {
    defaultLocale: 'es',
    locales: ['es', 'en'],
    routing: { prefixDefaultLocale: false },
  },
});
