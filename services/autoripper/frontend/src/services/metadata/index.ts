import { queryOptions } from '@tanstack/react-query';
import { z } from 'zod';

import { endpoints } from '$/services/endpoints';
import { fetcher } from '$/services/fetcher';
import { SearchMovieSchema } from '$/services/metadata/mapper/SearchMovieSchema';
import { SearchSeriesSchema } from '$/services/metadata/mapper/SearchSeriesSchema';

export type SearchResult = z.infer<typeof SearchMovieSchema>;
export type SearchResultItem = z.infer<typeof SearchMovieSchema>['results'][0];

export const searchMovieQuery = (payload: { query: string; lang: string; enabled?: boolean }) =>
  queryOptions({
    enabled: payload.enabled ?? true,
    queryKey: ['devices', payload.query],
    queryFn: ({ signal }) =>
      fetcher(endpoints.SEARCH_MOVIE_ENDPOINT, {
        msg: `could not search for movie with query: ${payload.query}`,
        parser: (data) => SearchMovieSchema.parse(data),
        body: { query: payload.query, lang: payload.lang },
        method: 'POST',
        signal,
      }),
  });

export const searchTvShowQuery = (payload: { query: string; lang: string; enabled?: boolean }) =>
  queryOptions({
    enabled: payload.enabled ?? true,
    queryKey: ['devices', payload.query],
    queryFn: ({ signal }) =>
      fetcher(endpoints.SEARCH_SERIES_ENDPOINT, {
        msg: `could not search for tv show with query: ${payload.query}`,
        parser: (data) => SearchSeriesSchema.parse(data),
        body: { query: payload.query, lang: payload.lang },
        method: 'POST',
        signal,
      }),
  });
