import { queryOptions } from '@tanstack/react-query';

import { endpointFactory } from '$/services/endpoints';
import { fetcher } from '$/services/fetcher';
import { QualityProfileSchema } from '$/services/management/mapper/QualityProfileSchema';
import { RootFoldersSchema } from '$/services/management/mapper/RootFoldersSchema';

export const qualityProfileQuery = (payload: { media_type: 'movie' | 'tv_show'; enabled?: boolean }) =>
  queryOptions({
    enabled: payload.enabled ?? true,
    queryKey: ['quality-profile', payload.media_type],
    queryFn: ({ signal }) =>
      fetcher(endpointFactory.qualityProfiles(payload.media_type), {
        msg: `could not fetch quality profiles`,
        parser: (data) => QualityProfileSchema.parse(data),
        signal,
      }),
  });

export const rootFoldersQuery = (payload: { media_type: 'movie' | 'tv_show'; enabled?: boolean }) =>
  queryOptions({
    enabled: payload.enabled ?? true,
    queryKey: ['root-folders', payload.media_type],
    queryFn: ({ signal }) =>
      fetcher(endpointFactory.rootFolders(payload.media_type), {
        msg: `could not fetch root folders`,
        parser: (data) => RootFoldersSchema.parse(data),
        signal,
      }),
  });
