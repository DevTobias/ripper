import { FC } from 'react';

import { Button } from '$/components/common/ui/button';
import { FormControl, FormField, FormItem } from '$/components/common/ui/form';
import { cn, repeat } from '$/lib/utils';
import { LoadingSelectionButton } from '$/pages/Homepage/components/SettingsForm/components/LoadingSelectionButton';
import { Episode } from '$/services/metadata';

import type { MetadataFormControl } from '$/pages/Homepage/components/SettingsForm';

interface Props {
  form: MetadataFormControl;
  episodes?: Episode[];
  isLoading: boolean;
}

export const EpisodeSelection: FC<Props> = ({ form, episodes, isLoading }) => {
  return (
    <FormField
      control={form.control}
      name='selectedEpisodes'
      render={({ field, fieldState }) => (
        <FormItem>
          <FormControl>
            <div className='flex h-[80px] flex-wrap gap-1 overflow-y-auto'>
              {episodes?.map((episode) => {
                const isActive = field.value.includes(episode.episode_number);

                return (
                  <Button
                    key={episode.episode_number}
                    className={cn('h-9 w-12 min-w-0 p-0 flex flex-col', fieldState.error && 'border-red-500')}
                    variant={isActive ? 'default' : 'outline'}
                    onClick={() => {
                      if (isActive) {
                        field.onChange(field.value.filter((value) => value !== episode.episode_number));
                      } else {
                        field.onChange([...field.value, episode.episode_number]);
                      }
                    }}
                    type='button'
                  >
                    <span className='leading-none'>E{episode.episode_number}</span>
                    {episode.runtime && <span className='text-[10px] leading-none opacity-60'>{episode.runtime} min</span>}
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
