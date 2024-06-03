import { z } from 'zod';

export const DevicesSchema = z.array(
  z.object({
    name: z.string(),
    description: z.string(),
    path: z.string(),
  })
);
