import { z } from 'zod';

export const PresetSchema = z.array(
  z.object({
    label: z.string(),
    name: z.string(),
  })
);
