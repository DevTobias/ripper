import { FC } from 'react';

import { Button } from '$/components/common/ui/button';
import { FormControl, FormItem } from '$/components/common/ui/form';
import { cn, repeat } from '$/lib/utils';
import { LoadingSelectionButton } from '$/pages/Homepage/components/MediaSelection/TvShowSelectionPopup/components/EpisodeSelection/components/LoadingSelectionButton';
import { Episode } from '$/services/metadata';

interface Props {
  value: number[];
  episodes?: Episode[];
  isLoading: boolean;
  hasError?: boolean;
  onChange: (value: number[]) => void;
}

export const EpisodeSelectionButtons: FC<Props> = ({ episodes, isLoading, value, hasError, onChange }) => {
  return (
    <FormItem>
      <FormControl>
        <div className='flex h-[80px] flex-wrap gap-1 overflow-y-auto'>
          {(!episodes || isLoading) && repeat(8).map((id) => <LoadingSelectionButton key={id} />)}
          {episodes?.map((episode) => {
            const isActive = value.includes(episode.episodeNumber);

            return (
              <Button
                key={episode.episodeNumber}
                className={cn('h-9 w-12 min-w-0 p-0 flex flex-col', hasError && 'border-red-500')}
                variant={isActive ? 'default' : 'outline'}
                type='button'
                onClick={() => {
                  if (isActive) {
                    onChange(value.filter((val) => val !== episode.episodeNumber));
                  } else {
                    onChange([...value, episode.episodeNumber]);
                  }
                }}
              >
                <span className='leading-none'>E{episode.episodeNumber}</span>
                {episode.runtime && <span className='text-[10px] leading-none opacity-60'>{episode.runtime} min</span>}
              </Button>
            );
          })}
        </div>
      </FormControl>
    </FormItem>
  );
};
