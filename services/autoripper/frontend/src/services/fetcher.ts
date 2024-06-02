/* eslint-disable no-console */

import { QueryClient } from '@tanstack/react-query';

export const DEFAULT_STALE_TIME = 1000 * 60 * 10;

export const queryClient = new QueryClient({
  defaultOptions: { queries: { retry: 0, staleTime: DEFAULT_STALE_TIME, refetchOnWindowFocus: false } },
});

type FetcherPayload<T> = {
  msg: string;
  parser: (data: unknown) => T;
  signal?: AbortSignal;
  body?: RequestInit['body'] | object;
  method?: RequestInit['method'];
  includeCredentials?: boolean;
};

export const logError = (error: unknown) => {
  console.error(error);
};

export const fetcher = async <T>(url: string, payload: FetcherPayload<T>): Promise<T> => {
  const { msg, parser, signal, body, method = 'GET', includeCredentials = false } = payload;

  console.log('fetching', url, payload);

  const response = await fetch(url, {
    signal,
    body: typeof body === 'object' ? JSON.stringify(body) : body,
    method,
    headers: method === 'POST' ? { 'Content-Type': 'application/json' } : {},
    credentials: includeCredentials ? 'include' : 'omit',
  });

  // await new Promise((resolve) => setTimeout(resolve, 1000));

  if (response.status !== 200) {
    logError([msg, response.status, await response.text()]);
    throw new Error(msg);
  }

  try {
    return parser(await response.json());
  } catch (e) {
    logError([msg, e, response.body]);
    throw new Error(msg);
  }
};
