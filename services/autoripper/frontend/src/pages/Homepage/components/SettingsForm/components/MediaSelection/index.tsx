import { Search } from 'lucide-react';
import { FC } from 'react';
import { useTranslation } from 'react-i18next';
import { useShallow } from 'zustand/react/shallow';

import { Button } from '$/components/common/ui/button';
import { FormField } from '$/components/common/ui/form';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '$/components/common/ui/tabs';
import { MovieSelection } from '$/pages/Homepage/components/SettingsForm/components/MediaSelection/components/MovieSelection';
import { SeriesSelection } from '$/pages/Homepage/components/SettingsForm/components/MediaSelection/components/SeriesSelection';
import { MediaSelectionDrawer } from '$/pages/Homepage/components/SettingsForm/components/MediaSelectionDrawer';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';

import type { MetadataFormControl } from '$/pages/Homepage/components/SettingsForm';

interface Props {
  form: MetadataFormControl;
}

export const MediaSelection: FC<Props> = ({ form }) => {
  const { t } = useTranslation();

  const rippingInProgress = useMediaStore(useShallow((state) => state.rippingInProgress));

  return (
    <FormField
      control={form.control}
      name='type'
      render={({ field }) => (
        <Tabs
          defaultValue={field.value}
          value={field.value}
          onValueChange={(v) => {
            field.onChange(v);
            form.resetField('selectedMedia');
            form.resetField('selectedSeason');
            form.resetField('selectedEpisodes');
          }}
        >
          <div className='flex gap-3'>
            <TabsList className='w-full'>
              <TabsTrigger value='movie' className='w-full' disabled={rippingInProgress}>
                {t('homepage.metadata.media.movie')}
              </TabsTrigger>
              <TabsTrigger value='tv_show' className='w-full' disabled={rippingInProgress}>
                {t('homepage.metadata.media.tvShow')}
              </TabsTrigger>
            </TabsList>

            <FormField
              control={form.control}
              name='selectedMedia'
              render={({ field: mediaField }) => (
                <div className='flex gap-2'>
                  <MediaSelectionDrawer
                    type={field.value}
                    onMediaSelect={(v) => {
                      form.resetField('selectedSeason');
                      form.resetField('selectedEpisodes');
                      mediaField.onChange(v);
                    }}
                  >
                    <Button
                      className='aspect-square h-full p-3'
                      type='button'
                      variant={form.getFieldState('selectedMedia').error ? 'destructive' : 'default'}
                      disabled={rippingInProgress}
                    >
                      <Search className='size-4' />
                    </Button>
                  </MediaSelectionDrawer>
                </div>
              )}
            />
          </div>

          <TabsContent value='movie' tabIndex={-1} className='mt-5'>
            <MovieSelection form={form} />
          </TabsContent>
          <TabsContent value='tv_show' tabIndex={-1} className='mt-5'>
            <SeriesSelection form={form} />
          </TabsContent>
        </Tabs>
      )}
    />
  );
};
