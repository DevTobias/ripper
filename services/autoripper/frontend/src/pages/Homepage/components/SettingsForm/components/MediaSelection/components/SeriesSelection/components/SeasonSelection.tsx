import { FC } from 'react';

import { Button } from '$/components/common/ui/button';
import { FormControl, FormField, FormItem } from '$/components/common/ui/form';
import { repeat } from '$/lib/utils';
import { LoadingSelectionButton } from '$/pages/Homepage/components/SettingsForm/components/LoadingSelectionButton';

import type { MetadataFormControl } from '$/pages/Homepage/components/SettingsForm';

interface Props {
  form: MetadataFormControl;
  seasonNumbers?: number[];
  isLoading: boolean;
}

export const SeasonSelection: FC<Props> = ({ form, seasonNumbers, isLoading }) => {
  return (
    <FormField
      control={form.control}
      name='selectedSeason'
      render={({ field }) => (
        <FormItem>
          <FormControl>
            <div className='flex h-[80px] flex-wrap gap-1 overflow-y-auto'>
              {seasonNumbers?.map((season) => (
                <Button
                  key={season}
                  className='aspect-square size-9 min-w-0 p-0'
                  variant={field.value === season ? 'default' : 'outline'}
                  onClick={() => field.onChange(season)}
                  type='button'
                >
                  S{season}
                </Button>
              ))}
              {isLoading && repeat(5).map((id) => <LoadingSelectionButton key={id} />)}
            </div>
          </FormControl>
        </FormItem>
      )}
    />
  );
};
