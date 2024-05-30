import { z } from 'zod';

export const SearchSeriesSchema = z
  .object({
    page: z.number(),
    total_results: z.number(),
    total_pages: z.number(),
    results: z.array(
      z.object({
        id: z.number(),
        name: z.string(),
        original_language: z.string(),
        overview: z.string(),
        popularity: z.number(),
        first_air_date: z.string(),
        poster_path: z.string().nullable(),
        vote_average: z.number(),
        vote_count: z.number(),
      })
    ),
  })
  .transform((data) => ({
    ...data,
    results: data.results.map((result) => ({
      id: result.id,
      title: result.name,
      description: result.overview,
      popularity: result.popularity,
      originalLanguage: result.original_language,
      posterPath: result.poster_path,
      voteAverage: result.vote_average,
      releaseDate: new Date(result.first_air_date),
    })),
  }));
