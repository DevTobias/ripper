export const BASE_URL = 'http://192.168.178.47:3000/api';

export const DEVICE_ENDPOINT = `${BASE_URL}/makemkv/devices`;
export const MOVIE_DISC_PROPERTIES = `${BASE_URL}/makemkv/titles/movie`;
export const TV_SHOW_DISC_PROPERTIES = `${BASE_URL}/makemkv/titles/tv`;

export const RIP_MOVIE_WEB_SOCKET_ENDPOINT = `${BASE_URL}/rip/movie/ws`;

export const ENCODING_PRESETS_ENDPOINT = `${BASE_URL}/handbrake/encoding-presets`;

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
  RIP_MOVIE_WEB_SOCKET_ENDPOINT,
};

export const endpointFactory = {
  ripMovieWebsocket: (id: number, langs: string[], device: string) => {
    const params = new URLSearchParams(Object.entries({ tmdb_id: id.toString(), device }));
    langs.forEach((lang) => params.append('langs', lang));
    return `${RIP_MOVIE_WEB_SOCKET_ENDPOINT}?${params.toString()}`;
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
};
