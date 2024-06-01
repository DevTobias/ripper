import { queryOptions } from '@tanstack/react-query';

import { endpoints } from '$/services/endpoints';
import { fetcher } from '$/services/fetcher';
import { PresetSchema } from '$/services/presets/mapper/PresetSchema';

export const encodingPresetsQuery = queryOptions({
  queryKey: ['encoding-presets'],
  queryFn: ({ signal }) =>
    fetcher(endpoints.ENCODING_PRESETS_ENDPOINT, {
      msg: `could not fetch encoding presets`,
      parser: (data) => PresetSchema.parse(data),
      signal,
    }),
});
