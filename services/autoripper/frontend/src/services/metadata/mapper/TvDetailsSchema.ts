import { z } from 'zod';

export const EpisodeSchema = z
  .object({
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
  })
  .transform((data) => ({
    id: data.id,
    title: data.name,
    description: data.overview,
    voteAverage: data.vote_average,
    releaseDate: data.air_date ? new Date(data.air_date) : null,
    runtime: data.runtime ?? 0,
    episodeNumber: data.episode_number,
  }));

export const TvSeasonSchema = z
  .object({
    id: z.number(),
    season_number: z.number(),
    name: z.string(),
    overview: z.string(),
    air_date: z.string(),
    poster_path: z.string().nullable(),
    vote_average: z.number(),
    episodes: z.array(EpisodeSchema),
  })
  .transform((data) => ({
    id: data.id,
    seasonNumber: data.season_number,
    title: data.name,
    description: data.overview,
    voteAverage: data.vote_average,
    releaseDate: data.air_date ? new Date(data.air_date) : null,
    episodes: data.episodes,
  }));

export const TvDetailsSchema = z
  .object({
    id: z.number(),
    name: z.string(),
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
    external_ids: z.object({
      imdb_id: z.string(),
      tvdb_id: z.number(),
    }),
    seasons: z.array(TvSeasonSchema),
  })
  .transform((data) => ({
    id: data.id,
    title: data.name,
    popularity: data.popularity,
    posterPath: data.poster_path,
    description: data.overview,
    voteAverage: data.vote_average,
    releaseDate: new Date(data.first_air_date),
    external_ids: {
      imdbId: data.external_ids.imdb_id,
      tvdbId: data.external_ids.tvdb_id,
    },
    seasons: data.seasons,
  }));
