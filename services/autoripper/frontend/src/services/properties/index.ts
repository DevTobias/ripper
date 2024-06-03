import { queryOptions } from '@tanstack/react-query';
import { z } from 'zod';

import { endpointFactory } from '$/services/endpoints';
import { fetcher } from '$/services/fetcher';
import { DiscPropertiesSchema } from '$/services/properties/mapper/DiscPropertiesSchema';

export type DiscProperties = z.infer<typeof DiscPropertiesSchema>;
export type Title = DiscProperties['titles'][0];

export const movieDiscPropertiesQuery = (payload: { langs: string[]; id: number; device: string }) =>
  queryOptions({
    queryKey: ['movie-disc-properties', payload.id, payload.device, payload.langs],
    queryFn: ({ signal }) => {
      const { id, device, langs } = payload;
      return fetcher(endpointFactory.movieDiscProperties(id, device, langs), {
        msg: `could not fetch movie disc properties`,
        parser: (data) => DiscPropertiesSchema.parse(data),
        signal,
      });
    },
  });

export const tvShowDiscPropertiesQuery = (payload: {
  langs: string[];
  id: number;
  device: string;
  season: number;
  episodes: number[];
  enabled?: boolean;
}) =>
  queryOptions({
    queryKey: ['tv-show-disc-properties', payload.id, payload.device, payload.season, payload.episodes, payload.langs],
    queryFn: ({ signal }) => {
      const { id, device, langs, season, episodes } = payload;

      return fetcher(endpointFactory.tvShowDiscProperties(id, device, langs, season, episodes), {
        msg: `could not fetch tv show disc properties`,
        parser: (data) => DiscPropertiesSchema.parse(data),
        signal,
      });
    },
  });
