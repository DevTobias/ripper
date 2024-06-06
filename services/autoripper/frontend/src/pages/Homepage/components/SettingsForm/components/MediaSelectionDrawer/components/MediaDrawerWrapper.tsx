import { FC, ReactNode } from 'react';
import { useTranslation } from 'react-i18next';

import { Drawer, DrawerClose, DrawerContent, DrawerHeader, DrawerTrigger } from '$/components/common/ui/drawer';
import { FormControl, FormDescription, FormItem, FormLabel, useFormField } from '$/components/common/ui/form';
import { Input } from '$/components/common/ui/input';
import { cn, repeat } from '$/lib/utils';
import { MediaCard } from '$/pages/Homepage/components/SettingsForm/components/MediaCard';
import { LoadingMediaCard } from '$/pages/Homepage/components/SettingsForm/components/MediaSelectionDrawer/components/LoadingMediaCard';
import { SearchResult, SearchResultItem } from '$/services/metadata';

interface Props {
  children: ReactNode;
  type: 'movie' | 'tv_show';
  data: SearchResult | undefined;
  isLoading: boolean;
  query: string;
  debouncedQuery: string;
  setQuery: (value: string) => void;
  onMediaSelect: (selectedMedia: SearchResultItem) => void;
}

export const MediaDrawerWrapper: FC<Props> = ({
  type,
  query,
  debouncedQuery,
  data,
  isLoading,
  children,
  setQuery,
  onMediaSelect,
}) => {
  const { t } = useTranslation();

  const isActualLoading = isLoading || (debouncedQuery.length < 3 && query.length >= 3);
  const hasNoResults = (!data || data.results.length === 0) && !isActualLoading && debouncedQuery.length >= 3;
  const hasData = data && data.results.length > 0 && !isActualLoading;

  const field = useFormField();

  return (
    <Drawer>
      <DrawerTrigger asChild>{children}</DrawerTrigger>
      <DrawerContent>
        <DrawerHeader className='text-left'>
          <FormItem>
            <FormLabel className='flex items-center justify-between text-sm'>
              <span>
                {t('homepage.metadata.media.label', {
                  type: type === 'movie' ? t('homepage.metadata.media.movie') : t('homepage.metadata.media.tvShow'),
                })}
              </span>
            </FormLabel>

            <FormControl>
              <Input
                placeholder={t('homepage.metadata.media.placeholder')}
                className={cn(field.error && 'border-red-500')}
                value={query}
                onChange={(e) => setQuery(e.target.value)}
                autoFocus
                onKeyUp={(e) => {
                  if (e.key === 'Enter') {
                    e.currentTarget.blur();
                  }
                }}
              />
            </FormControl>
            <FormDescription>{t('homepage.metadata.media.description')}</FormDescription>
          </FormItem>
        </DrawerHeader>
        <div className='min-h-[545px] p-4'>
          {hasNoResults && <FormDescription>{t('homepage.metadata.media.noResults')}</FormDescription>}
          <div className='flex flex-col gap-3'>
            {isActualLoading && repeat(5).map((i) => <LoadingMediaCard key={i} id={i} />)}
            {hasData &&
              data.results
                .sort((a, b) => b.popularity - a.popularity)
                .slice(0, 5)
                .map((media) => (
                  <DrawerClose asChild key={media.id}>
                    <MediaCard item={media} mediaType={type} onClick={() => onMediaSelect(media)} />
                  </DrawerClose>
                ))}
          </div>
        </div>
      </DrawerContent>
    </Drawer>
  );
};
