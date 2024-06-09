import { z } from 'zod';

export const MovieDetailsSchema = z
  .object({
    id: z.number(),
    title: z.string(),
    overview: z.string(),
    homepage: z.string(),
    popularity: z.number(),
    status: z.string(),
    release_date: z.string(),
    runtime: z.number().nullable().default(0),
    backdrop_path: z.string().nullable(),
    poster_path: z.string().nullable(),
    vote_average: z.number(),
    vote_count: z.number(),
  })
  .transform((data) => ({
    id: data.id,
    title: data.title,
    description: data.overview,
    popularity: data.popularity,
    posterPath: data.poster_path,
    voteAverage: data.vote_average,
    releaseDate: new Date(data.release_date),
    runtime: data.runtime ?? 0,
  }));
