import { z } from 'zod';

export const DevicesSchema = z.array(
  z.object({
    name: z.string(),
    type: z.string(),
    path: z.string(),
  })
);
