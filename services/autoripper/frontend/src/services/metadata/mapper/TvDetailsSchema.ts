import { z } from 'zod';

export const EpisodeSchema = z.object({
  id: z.number(),
  name: z.string(),
  overview: z.string(),
  air_date: z.string().nullable(),
  episode_number: z.number(),
  episode_type: z.string(),
  runtime: z.number().nullable().default(0),
  season_number: z.number(),
  vote_average: z.number(),
  vote_count: z.number(),
  still_path: z.string().nullable(),
});

export const TvSeasonSchema = z.object({
  id: z.number(),
  season_number: z.number(),
  name: z.string(),
  overview: z.string(),
  air_date: z.string(),
  poster_path: z.string().nullable(),
  vote_average: z.number(),
  episodes: z.array(EpisodeSchema),
});

export const TvDetailsSchema = z.object({
  id: z.number(),
  original_name: z.string(),
  overview: z.string(),
  homepage: z.string(),
  popularity: z.number(),
  status: z.string(),
  first_air_date: z.string(),
  last_air_date: z.string(),
  backdrop_path: z.string().nullable(),
  poster_path: z.string().nullable(),
  vote_average: z.number(),
  vote_count: z.number(),
  number_of_episodes: z.number(),
  number_of_seasons: z.number(),
  last_episode_to_air: EpisodeSchema,
  seasons: z.array(TvSeasonSchema),
});
