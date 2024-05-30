import { FC } from 'react';
import { useTranslation } from 'react-i18next';

import { Button } from '$/components/common/ui/button';
import { FormControl, FormField, FormItem, FormLabel } from '$/components/common/ui/form';
import { repeat } from '$/lib/utils';
import { LoadingSelectionButton } from '$/pages/Homepage/components/MetadataForm/components/LoadingSelectionButton';

import type { MetadataFormControl } from '$/pages/Homepage/components/MetadataForm';

interface Props {
  form: MetadataFormControl;
  seasonNumbers?: number[];
  isLoading: boolean;
}

export const SeasonSelection: FC<Props> = ({ form, seasonNumbers, isLoading }) => {
  const { t } = useTranslation();

  return (
    <FormField
      control={form.control}
      name='selectedSeason'
      render={({ field }) => (
        <FormItem>
          <FormLabel>{t('homepage.metadata.media.seasonSelection')}</FormLabel>
          <FormControl>
            <div className='flex flex-wrap gap-1'>
              {seasonNumbers?.map((season) => (
                <Button
                  key={season}
                  className='aspect-square'
                  variant={field.value === season ? 'default' : 'outline'}
                  onClick={() => field.onChange(season)}
                  type='button'
                >
                  {season}
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
