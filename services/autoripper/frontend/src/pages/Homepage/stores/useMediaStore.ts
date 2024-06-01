import { z } from 'zod';
import { create } from 'zustand';

export const metadataFormSchema = z
  .object({
    type: z.enum(['movie', 'tv_show']),
    selectedSeason: z.number(),
    selectedEpisodes: z.array(z.number()),
    selectedMedia: z.object(
      {
        id: z.number(),
        title: z.string(),
        description: z.string(),
        popularity: z.number(),
        originalLanguage: z.string(),
        posterPath: z.string().nullable(),
        voteAverage: z.number(),
        releaseDate: z.date(),
      },
      { required_error: 'formErrors.required' }
    ),
  })
  .superRefine((data, ctx) => {
    if (data.type === 'tv_show' && data.selectedEpisodes.length === 0) {
      ctx.addIssue({ code: z.ZodIssueCode.custom, message: 'formErrors.minLength', path: ['selectedEpisodes'] });
    }
  });

export const ripperFormSchema = z.object({
  device: z.string().min(1, { message: 'formErrors.required' }),
  preset: z.string().min(1, { message: 'formErrors.required' }),
});

export type MetadataFormValues = z.infer<typeof metadataFormSchema>;
export type RipperFormValues = z.infer<typeof ripperFormSchema>;

type State = {
  metadata?: MetadataFormValues;
  ripperInfo?: RipperFormValues;
};

type Actions = {
  setMetadata: (mediaInfo: MetadataFormValues) => void;
  setRipperInfo: (ripperInfo: RipperFormValues) => void;
};

export const useMediaStore = create<State & Actions>()((set) => ({
  setMetadata: (metadata) => set({ metadata }),
  setRipperInfo: (ripperInfo) => set({ ripperInfo }),
}));
