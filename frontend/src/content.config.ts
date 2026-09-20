import { defineCollection, z } from "astro:content";
import { glob } from "astro/loaders";

const articlesCollection = defineCollection({
  loader: glob({ pattern: "**/*.json", base: "./src/content/articles" }),
  schema: z.object({
    title: z.string(),
    pdfUrl: z.string(),
    imageUrl: z.string(),
    publishedAt: z.coerce.date().optional(),
  }),
});

export const collections = {
  articles: articlesCollection,
};
