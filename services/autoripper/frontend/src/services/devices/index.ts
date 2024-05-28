import { queryOptions } from '@tanstack/react-query';

import { DevicesSchema } from '$/services/devices/mapper/DeviceSchema';
import { endpoints } from '$/services/endpoints';
import { fetcher } from '$/services/fetcher';

export const devicesQuery = queryOptions({
  queryKey: ['devices'],
  queryFn: ({ signal }) =>
    fetcher(endpoints.DEVICE_ENDPOINT, {
      msg: `could not fetch devices`,
      parser: (data) => DevicesSchema.parse(data),
      signal,
    }),
});
