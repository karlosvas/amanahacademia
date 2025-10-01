import { defineCollection, z } from "astro:content";

const articlesCollection = defineCollection({
  type: "data",
  schema: z.object({
    title: z.string(),
    pdfUrl: z.string(),
    image: z.string(),
    publishedAt: z.coerce.date().optional(),
  }),
});

export const collections = {
  articles: articlesCollection,
};
