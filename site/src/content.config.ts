import { defineCollection, z } from 'astro:content';
import { glob } from 'astro/loaders';

const blog = defineCollection({
  loader: glob({ pattern: '**/*.{md,mdx}', base: './src/content/blog' }),
  schema: z.object({
    title: z.string(),
    description: z.string(),
    pubDate: z.coerce.date(),
    author: z.string().default('Proyecto Caudal'),
    lang: z.enum(['es', 'en']).default('es'),
    draft: z.boolean().default(false),
  }),
});

// Reference documents are rendered straight from `docs/` — the repository keeps
// the only copy. Page metadata lives in the route, so the source markdown stays
// clean for readers on GitHub.
const docs = defineCollection({
  loader: glob({ pattern: 'catalogo-operacional.md', base: '../docs/research' }),
});

export const collections = { blog, docs };
