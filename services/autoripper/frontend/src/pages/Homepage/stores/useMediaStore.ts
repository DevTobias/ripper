import { WebSocketMessage } from 'react-use-websocket/dist/lib/types';
import { create } from 'zustand';

import { MovieDetails, TvShowDetails } from '$/services/metadata';

import type { MovieSelectionFormValues } from '$/pages/Homepage/components/MediaSelection/MovieSelectionPopup/components/MovieSelectionForm';
import type { TvShowSelectionFormValues } from '$/pages/Homepage/components/MediaSelection/TvShowSelectionPopup/components/TvShowSelectionForm';

export type ProgressPayload = {
  progressState: 'idle' | 'ripping' | 'encoding' | 'uploading';
  progress: number;
  step: number;
  label: string;
  eta: number;
};

type State = {
  mediaType: 'movie' | 'tv_show' | null;

  movieSelectionValues: MovieSelectionFormValues | null;
  selectedMovie: MovieDetails | null;

  tvShowSelectionValues: TvShowSelectionFormValues | null;
  selectedTvShow: TvShowDetails | null;

  selectedTitles: string[];

  rippingInProgress: boolean;
  rippingProgress: ProgressPayload;

  sendWebsocketMessage?: (message: WebSocketMessage, keep?: boolean | undefined) => void;
};

const defaultState: State = {
  mediaType: null,

  movieSelectionValues: null,
  selectedMovie: null,

  tvShowSelectionValues: null,
  selectedTvShow: null,

  selectedTitles: [],

  rippingInProgress: false,
  rippingProgress: { progress: 0, step: 0, eta: 0, label: '', progressState: 'idle' },
};

export const useMediaStore = create<State>()(() => defaultState);
