import { z } from 'zod';

export const QualityProfileSchema = z
  .array(z.object({ id: z.number(), name: z.string() }))
  .transform((profiles) => profiles.map((profile) => ({ id: profile.id.toString(), name: profile.name })));
