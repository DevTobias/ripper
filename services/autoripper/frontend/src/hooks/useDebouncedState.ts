import { useState } from 'react';

import { useDebounce } from '$/hooks/useDebounce';

export const useDebouncedState = <T>(actual: T, defaultValue: T, delay?: number) => {
  const [value, setValue] = useState<T>(defaultValue);
  useDebounce(() => setValue(actual), delay, [actual]);
  return [value, setValue] as const;
};
