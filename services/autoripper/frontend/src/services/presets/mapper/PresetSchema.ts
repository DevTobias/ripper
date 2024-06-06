import { z } from 'zod';

export const PresetSchema = z
  .array(
    z.object({
      id: z.string(),
      label: z.string(),
      file_name: z.string(),
      preset_name: z.string(),
    })
  )
  .transform((presets) =>
    presets.map((preset) => ({
      id: preset.id,
      label: preset.label,
      fileName: preset.file_name,
      presetName: preset.preset_name,
    }))
  );
