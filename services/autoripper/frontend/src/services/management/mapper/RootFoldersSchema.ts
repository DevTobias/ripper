import { z } from 'zod';

export const RootFoldersSchema = z
  .array(z.object({ id: z.number(), path: z.string() }))
  .transform((folders) => folders.map((folder) => ({ id: folder.id.toString(), path: folder.path })));
