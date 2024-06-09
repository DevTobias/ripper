import { FC } from 'react';

import { Button } from '$/components/common/ui/button';
import { FormControl, FormItem } from '$/components/common/ui/form';
import { repeat } from '$/lib/utils';
import { LoadingSelectionButton } from '$/pages/Homepage/components/MediaSelection/TvShowSelectionPopup/components/EpisodeSelection/components/LoadingSelectionButton';

interface Props {
  value: number;
  seasonNumbers?: number[];
  isLoading: boolean;
  onChange: (value: number) => void;
}

export const SeasonSelectionButtons: FC<Props> = ({ value, seasonNumbers, isLoading, onChange }) => {
  return (
    <FormItem>
      <FormControl>
        <div className='flex h-[80px] flex-wrap gap-1 overflow-y-auto'>
          {(!seasonNumbers || isLoading) && repeat(5).map((id) => <LoadingSelectionButton key={id} />)}
          {seasonNumbers?.map((season) => (
            <Button
              key={season}
              className='aspect-square size-9 min-w-0 p-0'
              variant={value === season ? 'default' : 'outline'}
              onClick={() => onChange(season)}
              type='button'
            >
              S{season}
            </Button>
          ))}
        </div>
      </FormControl>
    </FormItem>
  );
};
