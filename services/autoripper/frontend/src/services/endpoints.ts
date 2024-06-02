export const BASE_URL = 'http://localhost:3000/api';

export const DEVICE_ENDPOINT = `${BASE_URL}/devices`;
export const ENCODING_PRESETS_ENDPOINT = `${BASE_URL}/encoding-presets`;
export const SEARCH_MOVIE_ENDPOINT = `${BASE_URL}/tmdb/search/movie`;
export const SEARCH_SERIES_ENDPOINT = `${BASE_URL}/tmdb/search/tv`;
export const SERIES_DETAILS_ENDPOINT = `${BASE_URL}/tmdb/tv`;
export const MOVIE_DETAILS_ENDPOINT = `${BASE_URL}/tmdb/movie`;
export const RIP_MOVIE_WEB_SOCKET_ENDPOINT = `${BASE_URL}/rip/movie/ws`;

export const endpoints = {
  DEVICE_ENDPOINT,
  SEARCH_MOVIE_ENDPOINT,
  SEARCH_SERIES_ENDPOINT,
  SERIES_DETAILS_ENDPOINT,
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
};
