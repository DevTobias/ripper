import { FC } from 'react';
import { useTranslation } from 'react-i18next';
import { useShallow } from 'zustand/react/shallow';

import { FormControl, FormField, FormItem, FormLabel } from '$/components/common/ui/form';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '$/components/common/ui/select';
import { cn } from '$/lib/utils';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';

import type { MetadataFormControl } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm';

interface Props {
  form: MetadataFormControl;
  className?: string;
}

const seriesTypes = [
  { id: 'standard', name: 'Standard' },
  { id: 'anime', name: 'Anime' },
];

export const SeriesTypeSelection: FC<Props> = ({ form, className }) => {
  const { t } = useTranslation();

  const rippingInProgress = useMediaStore(useShallow((state) => state.rippingInProgress));

  return (
    <FormField
      control={form.control}
      name='seriesType'
      render={({ field, fieldState }) => (
        <FormItem className={cn('w-full', className)}>
          <FormLabel className='flex items-center gap-1 text-sm'>
            <span>{t('homepage.qualityProfile.title')}</span>
          </FormLabel>
          <Select disabled={rippingInProgress} onValueChange={field.onChange} value={field.value}>
            <FormControl>
              <div className='flex gap-3'>
                <SelectTrigger className={cn('w-full', fieldState.error && 'border-red-500')} ref={field.ref}>
                  <SelectValue placeholder={t('homepage.qualityProfile.placeholder')} />
                </SelectTrigger>
              </div>
            </FormControl>
            <SelectContent>
              {seriesTypes.map(({ id, name }) => (
                <SelectItem key={id} value={id} className='cursor-pointer'>
                  <span>{name}</span>
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </FormItem>
      )}
    />
  );
};
