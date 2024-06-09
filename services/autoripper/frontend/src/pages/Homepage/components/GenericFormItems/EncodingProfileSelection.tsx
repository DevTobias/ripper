import { useQuery } from '@tanstack/react-query';
import { forwardRef, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useShallow } from 'zustand/react/shallow';

import { FormControl, FormItem, FormLabel } from '$/components/common/ui/form';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '$/components/common/ui/select';
import { cn } from '$/lib/utils';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';
import { encodingPresetsQuery } from '$/services/presets';

interface Props {
  onChange: (value: string) => void;
  value: string;
  hasError: boolean;
  ref: React.RefObject<HTMLDivElement>;
}

export const EncodingProfileSelection = forwardRef<HTMLButtonElement, Props>(function EncodingProfileSelection(
  { value, hasError, onChange },
  ref
) {
  const { t } = useTranslation();

  const rippingInProgress = useMediaStore(useShallow((state) => state.rippingInProgress));
  const { data, isLoading, isRefetching } = useQuery(encodingPresetsQuery);
  const loading = isLoading || isRefetching;

  useEffect(() => {
    if (data?.length === 1 && value !== data[0].id) onChange(data[0].id);
  });

  useEffect(() => {
    if (!value && (data?.length ?? 0) > 0) onChange(data![0].id);
  });

  return (
    <FormItem className='w-full'>
      <FormLabel className='flex items-center gap-1 text-sm'>
        <span>{t('genericFormItems.encodingProfile.title')}</span>
      </FormLabel>
      <Select disabled={loading || rippingInProgress} onValueChange={onChange} value={value}>
        <FormControl>
          <div className='flex gap-3'>
            <SelectTrigger className={cn('w-full', hasError && 'border-red-500')} isLoading={loading} ref={ref}>
              <SelectValue placeholder={t('genericFormItems.encodingProfile.placeholder')} />
            </SelectTrigger>
          </div>
        </FormControl>
        <SelectContent>
          {data?.map((profile) => (
            <SelectItem key={profile.id} value={profile.id} className='cursor-pointer'>
              <span>{profile.label}</span>
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </FormItem>
  );
});
