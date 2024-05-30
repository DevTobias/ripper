import { Search } from 'lucide-react';
import { FC } from 'react';
import { useTranslation } from 'react-i18next';

import { Button } from '$/components/common/ui/button';
import { FormField } from '$/components/common/ui/form';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '$/components/common/ui/tabs';
import { MovieSelection } from '$/pages/Homepage/components/MetadataForm/components/MediaSelection/components/MovieSelection';
import { MediaSelectionDrawer } from '$/pages/Homepage/components/MetadataForm/components/MediaSelectionDrawer';

import type { MetadataFormControl } from '$/pages/Homepage/components/MetadataForm';

interface Props {
  form: MetadataFormControl;
}

export const MediaSelection: FC<Props> = ({ form }) => {
  const { t } = useTranslation();

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
          }}
        >
          <div className='flex gap-3'>
            <TabsList className='w-full'>
              <TabsTrigger value='movie' className='w-full'>
                {t('homepage.metadata.media.movie')}
              </TabsTrigger>
              <TabsTrigger value='tv_show' className='w-full'>
                {t('homepage.metadata.media.tvShow')}
              </TabsTrigger>
            </TabsList>

            <FormField
              control={form.control}
              name='selectedMedia'
              render={({ field: mediaField }) => (
                <MediaSelectionDrawer type={field.value} onMediaSelect={mediaField.onChange}>
                  <Button
                    className='aspect-square h-full p-3'
                    type='button'
                    variant={form.getFieldState('selectedMedia').error ? 'destructive' : 'default'}
                  >
                    <Search className='size-4' />
                  </Button>
                </MediaSelectionDrawer>
              )}
            />
          </div>

          <TabsContent value='movie'>
            <MovieSelection form={form} />
          </TabsContent>
          <TabsContent value='tv_show'>
            <MovieSelection form={form} />
          </TabsContent>
        </Tabs>
      )}
    />
  );
};
