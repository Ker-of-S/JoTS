import { z, defineCollection } from 'astro:content';
import { glob } from 'astro/loaders'

const publicationsCollection = defineCollection({
    loader: glob({ pattern: "**/*.md",
base: "src/content/publications"}),
    schema: z.object({
        title: z.string(),
        category: z.string(),
        authors: z.string(),
        abstract: z.string(),
        pdfUrl: z.string().optional(),
        status: z.enum(['published', 'archived', 'recycled']).default('published'),
    }),
});

export const collections = {
    'publications':
publicationsCollection,
};