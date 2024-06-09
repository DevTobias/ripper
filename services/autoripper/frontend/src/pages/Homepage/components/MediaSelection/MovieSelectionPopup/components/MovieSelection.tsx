import { useQuery } from '@tanstack/react-query';
import { FC, useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';

import { FormDescription, FormLabel } from '$/components/common/ui/form';
import { Input } from '$/components/common/ui/input';
import { useDebouncedState } from '$/hooks/useDebouncedState';
import { cn, repeat } from '$/lib/utils';
import { LoadingMediaCard } from '$/pages/Homepage/components/MediaSelection/components/LoadingMediaCard';
import { MediaCard } from '$/pages/Homepage/components/MediaSelection/components/MediaCard';
import { SearchResultItem, searchMovieQuery } from '$/services/metadata';

interface Props {
  value: number;
  hasError?: boolean;
  onChange: (item: SearchResultItem | null) => void;
}

export const MovieSelection: FC<Props> = ({ value, hasError, onChange }) => {
  const { t } = useTranslation();

  const [query, setQuery] = useState('');
  const [debouncedQuery, overrideDebouncedQuery] = useDebouncedState(query, '', 500);

  const { isLoading, data } = useQuery(
    searchMovieQuery({ query: debouncedQuery, lang: 'de', enabled: debouncedQuery.length >= 3 })
  );

  useEffect(() => {
    setQuery('');
    overrideDebouncedQuery('');
  }, [overrideDebouncedQuery]);

  useEffect(() => {
    onChange(null);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [query]);

  const isActualLoading = isLoading || !data;
  const hasNoResults = (!data || data.results.length === 0) && !isActualLoading && debouncedQuery.length >= 3;
  const hasData = data && data.results.length > 0 && !isActualLoading;

  return (
    <div className='flex flex-col gap-4'>
      <div className='flex flex-col gap-1'>
        <FormLabel className='flex items-center justify-between text-sm'>
          <span>{t('movieSelection.title')}</span>
        </FormLabel>

        <Input
          placeholder={t('movieSelection.placeholder')}
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          className={cn(hasError && 'border-red-500')}
        />
        <FormDescription>{t('movieSelection.description')}</FormDescription>
      </div>
      <div className='h-[300px] overflow-y-auto pr-3'>
        {hasNoResults && <FormDescription>{t('movieSelection.noResults')}</FormDescription>}
        <div className='flex flex-col gap-3'>
          {isActualLoading && repeat(5).map((i) => <LoadingMediaCard key={i} id={i} />)}
          {hasData &&
            data.results
              .sort((a, b) => b.popularity - a.popularity)
              .slice(0, 5)
              .map((media) => (
                <MediaCard
                  key={media.id}
                  item={media}
                  isActive={value === media.id}
                  mediaType='movie'
                  onClick={() => onChange(media)}
                />
              ))}
        </div>
      </div>
    </div>
  );
};
