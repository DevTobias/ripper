import { useTranslation } from 'react-i18next';

import { Tabs, TabsContent, TabsList, TabsTrigger } from '$/components/common/ui/tabs';
import { MovieSelectionPopup } from '$/pages/Homepage/components/MediaSelection/MovieSelectionPopup';
import { TvShowSelectionPopup } from '$/pages/Homepage/components/MediaSelection/TvShowSelectionPopup';

export const MediaSelection = () => {
  const { t } = useTranslation();

  return (
    <Tabs defaultValue='movie'>
      <TabsList className='w-full'>
        <TabsTrigger value='movie' className='w-full'>
          {t('genericSelection.movie')}
        </TabsTrigger>
        <TabsTrigger value='tv_show' className='w-full'>
          {t('genericSelection.tvShow')}
        </TabsTrigger>
      </TabsList>
      <TabsContent value='movie'>
        <MovieSelectionPopup />
      </TabsContent>
      <TabsContent value='tv_show'>
        <TvShowSelectionPopup />
      </TabsContent>
    </Tabs>
  );
};
