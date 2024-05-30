import { DependencyList, useEffect } from 'react';

import { useTimeoutFn } from '$/hooks/useTimeoutFn';

export type UseDebounceReturn = [() => boolean | null, () => void];

export const useDebounce = (fn: () => void, ms: number = 0, deps: DependencyList = []): UseDebounceReturn => {
  const [isReady, cancel, reset] = useTimeoutFn(fn, ms);
  useEffect(reset, [reset, ...deps]);
  return [isReady, cancel];
};
