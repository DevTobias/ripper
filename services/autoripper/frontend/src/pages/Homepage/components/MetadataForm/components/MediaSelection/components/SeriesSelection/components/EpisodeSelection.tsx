import { FC } from 'react';
import { useTranslation } from 'react-i18next';

import { Button } from '$/components/common/ui/button';
import { FormControl, FormField, FormItem, FormLabel, FormMessage } from '$/components/common/ui/form';
import { repeat } from '$/lib/utils';
import { LoadingSelectionButton } from '$/pages/Homepage/components/MetadataForm/components/LoadingSelectionButton';

import type { MetadataFormControl } from '$/pages/Homepage/components/MetadataForm';

interface Props {
  form: MetadataFormControl;
  episodeNumbers?: number[];
  isLoading: boolean;
}

export const EpisodeSelection: FC<Props> = ({ form, episodeNumbers, isLoading }) => {
  const { t } = useTranslation();

  return (
    <FormField
      control={form.control}
      name='selectedEpisodes'
      render={({ field }) => (
        <FormItem>
          <FormLabel className='flex items-center justify-between text-sm'>
            <span>{t('homepage.metadata.media.episodeSelection')}</span>
            <FormMessage isTranslated />
          </FormLabel>
          <FormControl>
            <div className='flex flex-wrap gap-1'>
              {episodeNumbers?.map((episode) => {
                const isActive = field.value.includes(episode);

                return (
                  <Button
                    key={episode}
                    className='aspect-square'
                    variant={isActive ? 'default' : 'outline'}
                    onClick={() => {
                      if (isActive) {
                        field.onChange(field.value.filter((value) => value !== episode));
                      } else {
                        field.onChange([...field.value, episode]);
                      }
                    }}
                    type='button'
                  >
                    {episode}
                  </Button>
                );
              })}
              {isLoading && repeat(8).map((id) => <LoadingSelectionButton key={id} />)}
            </div>
          </FormControl>
        </FormItem>
      )}
    />
  );
};
