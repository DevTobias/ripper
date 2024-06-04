import { WebSocketMessage } from 'react-use-websocket/dist/lib/types';
import { z } from 'zod';
import { create } from 'zustand';

export const metadataFormSchema = z
  .object({
    device: z.string().min(1, { message: 'formErrors.required' }),
    preset: z.string().min(1, { message: 'formErrors.required' }),
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

export type MetadataFormValues = z.infer<typeof metadataFormSchema>;

export type ProgressPayload = {
  progress: number;
  step: number;
  stepTitle: string;
  stepDetails: string;
};

type State = {
  metadata: MetadataFormValues | null;
  selectedTitles: string[];
  rippingInProgress: boolean;
  rippingProgress: ProgressPayload;
  sendWebsocketMessage?: (message: WebSocketMessage, keep?: boolean | undefined) => void;
};

type Actions = {
  setMetadata: (mediaInfo: MetadataFormValues) => void;
};

const defaultState: State = {
  metadata: null,
  selectedTitles: [],
  rippingInProgress: false,
  rippingProgress: { progress: 0, step: 0, stepDetails: '', stepTitle: '' },
};

export const useMediaStore = create<State & Actions>()((set) => ({
  ...defaultState,
  setMetadata: (metadata) => set({ metadata }),
}));
