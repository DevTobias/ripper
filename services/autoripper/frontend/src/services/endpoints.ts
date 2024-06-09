export const BASE_URL = import.meta.env.VITE_BACKEND_BASE_URL;
export const WEBSOCKET_BASE_URL = import.meta.env.VITE_BACKEND_WEBSOCKET_BASE_URL;

export const DEVICE_ENDPOINT = `${BASE_URL}/makemkv/devices`;
export const MOVIE_DISC_PROPERTIES = `${BASE_URL}/makemkv/titles/movie`;
export const TV_SHOW_DISC_PROPERTIES = `${BASE_URL}/makemkv/titles/tv`;
export const RIP_WEB_SOCKET_ENDPOINT = `${WEBSOCKET_BASE_URL}/makemkv/rip`;

export const ENCODING_PRESETS_ENDPOINT = `${BASE_URL}/handbrake/encoding-presets`;

export const QUALITY_PROFILE_ENDPOINT = `${BASE_URL}/management/quality-profiles`;
export const ROOT_FOLDER_ENDPOINT = `${BASE_URL}/management/root-folders`;

export const SEARCH_MOVIE_ENDPOINT = `${BASE_URL}/tmdb/search/movie`;
export const SEARCH_TV_SHOW_ENDPOINT = `${BASE_URL}/tmdb/search/tv`;
export const TV_SHOW_DETAILS_ENDPOINT = `${BASE_URL}/tmdb/tv`;
export const MOVIE_DETAILS_ENDPOINT = `${BASE_URL}/tmdb/movie`;

export const endpoints = {
  DEVICE_ENDPOINT,
  SEARCH_MOVIE_ENDPOINT,
  SEARCH_TV_SHOW_ENDPOINT,
  TV_SHOW_DETAILS_ENDPOINT,
  MOVIE_DETAILS_ENDPOINT,
  ENCODING_PRESETS_ENDPOINT,
  RIP_WEB_SOCKET_ENDPOINT,
  QUALITY_PROFILE_ENDPOINT,
  ROOT_FOLDER_ENDPOINT,
};

export const endpointFactory = {
  ripWebsocket: (payload: {
    titles: string[];
    device: string;
    profile: string;
    qualityProfile: string;
    rootFolder: string;
    mediaType: 'movie' | 'tv_show';
    metadata: object;
  }) => {
    const params = new URLSearchParams(
      Object.entries({
        device: payload.device,
        encoding_profile: payload.profile,
        quality_profile: payload.qualityProfile,
        root_folder: payload.rootFolder,
        media_type: payload.mediaType,
        metadata: JSON.stringify(payload.metadata),
      })
    );
    payload.titles.forEach((title) => params.append('titles', title));
    return `${RIP_WEB_SOCKET_ENDPOINT}?${params.toString()}`;
  },
  searchMovie: (query: string, lang: string) => {
    const params = new URLSearchParams(Object.entries({ query, lang }));
    return `${SEARCH_MOVIE_ENDPOINT}?${params.toString()}`;
  },
  searchTvShow: (query: string, lang: string) => {
    const params = new URLSearchParams(Object.entries({ query, lang }));
    return `${SEARCH_TV_SHOW_ENDPOINT}?${params.toString()}`;
  },
  tvShowDetails: (id: number, lang: string) => {
    const params = new URLSearchParams(Object.entries({ lang }));
    return `${TV_SHOW_DETAILS_ENDPOINT}/${id}?${params.toString()}`;
  },
  movieDetails: (id: number, lang: string) => {
    const params = new URLSearchParams(Object.entries({ lang }));
    return `${MOVIE_DETAILS_ENDPOINT}/${id}?${params.toString()}`;
  },
  movieDiscProperties: (id: number, device: string, langs: string[]) => {
    const params = new URLSearchParams(Object.entries({ tmdb_id: id.toString(), device }));
    langs.forEach((lang) => params.append('langs', lang));
    return `${MOVIE_DISC_PROPERTIES}?${params.toString()}`;
  },
  tvShowDiscProperties: (id: number, device: string, langs: string[], season: number, episodes: number[]) => {
    const params = new URLSearchParams(Object.entries({ tmdb_id: id.toString(), device, season: season.toString() }));
    langs.forEach((lang) => params.append('langs', lang));
    episodes.forEach((episode) => params.append('episodes', episode.toString()));
    return `${TV_SHOW_DISC_PROPERTIES}?${params.toString()}`;
  },
  qualityProfiles: (media_type: 'movie' | 'tv_show') => {
    const params = new URLSearchParams(Object.entries({ media_type }));
    return `${QUALITY_PROFILE_ENDPOINT}?${params.toString()}`;
  },
  rootFolders: (media_type: 'movie' | 'tv_show') => {
    const params = new URLSearchParams(Object.entries({ media_type }));
    return `${ROOT_FOLDER_ENDPOINT}?${params.toString()}`;
  },
};
