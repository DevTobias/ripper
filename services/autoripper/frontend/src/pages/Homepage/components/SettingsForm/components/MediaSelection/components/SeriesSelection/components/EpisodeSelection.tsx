import { FC } from 'react';

import { Button } from '$/components/common/ui/button';
import { FormControl, FormField, FormItem } from '$/components/common/ui/form';
import { cn, repeat } from '$/lib/utils';
import { LoadingSelectionButton } from '$/pages/Homepage/components/SettingsForm/components/LoadingSelectionButton';

import type { MetadataFormControl } from '$/pages/Homepage/components/SettingsForm';

interface Props {
  form: MetadataFormControl;
  episodeNumbers?: number[];
  isLoading: boolean;
}

export const EpisodeSelection: FC<Props> = ({ form, episodeNumbers, isLoading }) => {
  return (
    <FormField
      control={form.control}
      name='selectedEpisodes'
      render={({ field, fieldState }) => (
        <FormItem>
          <FormControl>
            <div className='flex h-[80px] flex-wrap gap-1 overflow-y-auto'>
              {episodeNumbers?.map((episode) => {
                const isActive = field.value.includes(episode);

                return (
                  <Button
                    key={episode}
                    className={cn('aspect-square size-9 min-w-0 p-0', fieldState.error && 'border-red-500')}
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
                    E{episode}
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
