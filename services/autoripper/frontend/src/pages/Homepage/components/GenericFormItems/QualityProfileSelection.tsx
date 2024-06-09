import { useQuery } from '@tanstack/react-query';
import { forwardRef, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useShallow } from 'zustand/react/shallow';

import { FormControl, FormItem, FormLabel } from '$/components/common/ui/form';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '$/components/common/ui/select';
import { cn } from '$/lib/utils';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';
import { qualityProfileQuery } from '$/services/management';

interface Props {
  onChange: (value: string) => void;
  value: string;
  hasError: boolean;
  ref: React.RefObject<HTMLDivElement>;
  type: 'movie' | 'tv_show';
}

export const QualityProfileSelection = forwardRef<HTMLButtonElement, Props>(function QualityProfileSelection(
  { value, hasError, onChange, type },
  ref
) {
  const { t } = useTranslation();

  const rippingInProgress = useMediaStore(useShallow((state) => state.rippingInProgress));
  const { data, isLoading, isRefetching } = useQuery(qualityProfileQuery({ media_type: type }));
  const loading = isLoading || isRefetching;

  useEffect(() => {
    if (data?.length === 1 && value !== data[0].id) onChange(data[0].id);
  });

  return (
    <FormItem className='w-full'>
      <FormLabel className='flex items-center gap-1 text-sm'>
        <span>{t('genericFormItems.qualityProfile.title')}</span>
      </FormLabel>
      <Select disabled={loading || rippingInProgress} onValueChange={onChange} value={value}>
        <FormControl>
          <div className='flex gap-3'>
            <SelectTrigger className={cn('w-full', hasError && 'border-red-500')} isLoading={loading} ref={ref}>
              <SelectValue placeholder={t('genericFormItems.qualityProfile.placeholder')} />
            </SelectTrigger>
          </div>
        </FormControl>
        <SelectContent>
          {data?.map((profile) => (
            <SelectItem key={profile.id} value={profile.id} className='cursor-pointer'>
              <span>{profile.name}</span>
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </FormItem>
  );
});
