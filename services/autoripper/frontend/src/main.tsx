import { QueryClientProvider } from '@tanstack/react-query';
import { StrictMode } from 'react';
import ReactDOM from 'react-dom/client';

import { App } from '$/app';
import { LocalizationProvider } from '$/components/core/LocalizationProvider';
import { queryClient } from '$/services/fetcher';

import '$/styles/globals.css';

ReactDOM.createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <LocalizationProvider>
      <QueryClientProvider client={queryClient}>
        <App />
      </QueryClientProvider>
    </LocalizationProvider>
  </StrictMode>
);
