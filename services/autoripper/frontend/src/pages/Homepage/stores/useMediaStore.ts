import { WebSocketMessage } from 'react-use-websocket/dist/lib/types';
import { z } from 'zod';
import { create } from 'zustand';

export const metadataFormSchema = z
  .object({
    device: z.string().min(1, { message: 'formErrors.required' }),
    profile: z.string().min(1, { message: 'formErrors.required' }),
    type: z.enum(['movie', 'tv_show']),
    selectedSeason: z.number(),
    selectedEpisodes: z.array(z.number()),
    qualityProfile: z.string().min(1, { message: 'formErrors.required' }),
    rootFolder: z.string().min(1, { message: 'formErrors.required' }),
    seriesType: z.enum(['standard', 'anime']),
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
  progressState: 'idle' | 'ripping' | 'encoding' | 'uploading';
  progress: number;
  step: number;
  label: string;
  eta: number;
};

type State = {
  metadata: MetadataFormValues | null;
  selectedTvId: number | null;
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
  selectedTvId: null,
  selectedTitles: [],
  rippingInProgress: false,
  rippingProgress: { progress: 0, step: 0, eta: 0, label: '', progressState: 'idle' },
};

export const useMediaStore = create<State & Actions>()((set) => ({
  ...defaultState,
  setMetadata: (metadata) => set({ metadata }),
}));
