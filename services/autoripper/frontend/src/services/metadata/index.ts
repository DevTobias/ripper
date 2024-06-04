import { queryOptions } from '@tanstack/react-query';
import { z } from 'zod';

import { endpointFactory } from '$/services/endpoints';
import { fetcher } from '$/services/fetcher';
import { SearchMovieSchema } from '$/services/metadata/mapper/SearchMovieSchema';
import { SearchSeriesSchema } from '$/services/metadata/mapper/SearchSeriesSchema';
import { TvDetailsSchema } from '$/services/metadata/mapper/TvDetailsSchema';

export type SearchResult = z.infer<typeof SearchMovieSchema>;
export type SearchResultItem = z.infer<typeof SearchMovieSchema>['results'][0];
export type Episode = z.infer<typeof TvDetailsSchema>['seasons'][0]['episodes'][0];

export const searchMovieQuery = (payload: { query: string; lang: string; enabled?: boolean }) =>
  queryOptions({
    enabled: payload.enabled ?? true,
    queryKey: ['search-movie', payload.query],
    queryFn: ({ signal }) =>
      fetcher(endpointFactory.searchMovie(payload.query, payload.lang), {
        msg: `could not search for movie with query: ${payload.query}`,
        parser: (data) => SearchMovieSchema.parse(data),
        signal,
      }),
  });

export const searchTvShowQuery = (payload: { query: string; lang: string; enabled?: boolean }) =>
  queryOptions({
    enabled: payload.enabled ?? true,
    queryKey: ['search-tv', payload.query],
    queryFn: ({ signal }) =>
      fetcher(endpointFactory.searchTvShow(payload.query, payload.lang), {
        msg: `could not search for tv show with query: ${payload.query}`,
        parser: (data) => SearchSeriesSchema.parse(data),
        signal,
      }),
  });

export const getTvDetailsQuery = (payload: { id?: number; lang: string; enabled?: boolean }) => {
  return queryOptions({
    enabled: !!payload.id && (payload.enabled ?? true),
    queryKey: ['tv-details', payload.id],
    queryFn: ({ signal }) =>
      fetcher(endpointFactory.tvShowDetails(payload.id!, payload.lang), {
        msg: `could not get tv details for id: ${payload.id}`,
        parser: (data) => TvDetailsSchema.parse(data),
        signal,
      }),
  });
};
