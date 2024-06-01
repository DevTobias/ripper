import { useQuery } from '@tanstack/react-query';
import { FC, ReactNode, useEffect, useState } from 'react';

import { useDebouncedState } from '$/hooks/useDebouncedState';
import { MediaDrawerWrapper } from '$/pages/Homepage/components/SettingsForm/components/MediaSelectionDrawer/components/MediaDrawerWrapper';
import { SearchResultItem, searchMovieQuery, searchTvShowQuery } from '$/services/metadata';

interface Props {
  children: ReactNode;
  type: 'movie' | 'tv_show';
  onMediaSelect: (selectedMedia: SearchResultItem) => void;
}

export const MediaSelectionDrawer: FC<Props> = ({ type, onMediaSelect, children }) => {
  const [query, setQuery] = useState('');
  const [debouncedQuery, overrideDebouncedQuery] = useDebouncedState(query, '', 500);

  const movieQuery = useQuery(
    searchMovieQuery({ query: debouncedQuery, lang: 'de-DE', enabled: type === 'movie' && debouncedQuery.length >= 3 })
  );

  const tvShowQuery = useQuery(
    searchTvShowQuery({ query: debouncedQuery, lang: 'de-DE', enabled: type === 'tv_show' && debouncedQuery.length >= 3 })
  );

  useEffect(() => {
    setQuery('');
    overrideDebouncedQuery('');
  }, [overrideDebouncedQuery, type]);

  return (
    <MediaDrawerWrapper
      data={type === 'movie' ? movieQuery.data : tvShowQuery.data}
      isLoading={type === 'movie' ? movieQuery.isLoading : tvShowQuery.isLoading}
      debouncedQuery={debouncedQuery}
      query={query}
      type={type}
      setQuery={setQuery}
      onMediaSelect={onMediaSelect}
    >
      {children}
    </MediaDrawerWrapper>
  );
};
