import { forwardRef, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useShallow } from 'zustand/react/shallow';

import { FormControl, FormItem, FormLabel } from '$/components/common/ui/form';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '$/components/common/ui/select';
import { cn } from '$/lib/utils';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';

interface Props {
  onChange: (value: string) => void;
  value: string;
  hasError: boolean;
  ref: React.RefObject<HTMLDivElement>;
}

const seriesTypes = [
  { id: 'standard', name: 'Standard' },
  { id: 'anime', name: 'Anime' },
];

export const SeriesTypeSelection = forwardRef<HTMLButtonElement, Props>(function SeriesTypeSelection(
  { value, hasError, onChange },
  ref
) {
  const { t } = useTranslation();

  const rippingInProgress = useMediaStore(useShallow((state) => state.rippingInProgress));

  useEffect(() => {
    if (!value) onChange(seriesTypes[0].id);
  });

  return (
    <FormItem className='w-full'>
      <FormLabel className='flex items-center gap-1 text-sm'>
        <span>{t('tvShowSelection.seriesType.title')}</span>
      </FormLabel>
      <Select disabled={rippingInProgress} onValueChange={onChange} value={value}>
        <FormControl>
          <div className='flex gap-3'>
            <SelectTrigger className={cn('w-full', hasError && 'border-red-500')} ref={ref}>
              <SelectValue placeholder={t('tvShowSelection.seriesType.placeholder')} />
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
  );
});
