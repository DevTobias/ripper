import { useQuery } from '@tanstack/react-query';
import { forwardRef, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useShallow } from 'zustand/react/shallow';

import { FormControl, FormItem, FormLabel } from '$/components/common/ui/form';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '$/components/common/ui/select';
import { cn } from '$/lib/utils';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';
import { devicesQuery } from '$/services/devices';

const prettifyDeviceName = (device: string) => {
  return device.replace(/_/g, ' ').toLocaleLowerCase();
};

interface Props {
  onChange: (value: string) => void;
  value: string;
  hasError: boolean;
  ref: React.RefObject<HTMLDivElement>;
}

export const DeviceSelection = forwardRef<HTMLButtonElement, Props>(function DeviceSelection(
  { value, hasError, onChange },
  ref
) {
  const { t } = useTranslation();

  const rippingInProgress = useMediaStore(useShallow((state) => state.rippingInProgress));
  const { data, isLoading, isRefetching } = useQuery(devicesQuery);
  const loading = isLoading || isRefetching;

  useEffect(() => {
    if (data?.length === 1 && value !== data[0].path) onChange(data[0].path);
  });

  return (
    <FormItem className='w-full'>
      <FormLabel className='flex items-center gap-1 text-sm'>
        <span>{t('genericFormItems.device.label')}</span>
      </FormLabel>
      <Select disabled={loading || rippingInProgress} onValueChange={onChange} value={value}>
        <FormControl>
          <div className='flex gap-3'>
            <SelectTrigger className={cn('w-full', hasError && 'border-red-500')} isLoading={loading} ref={ref}>
              <SelectValue placeholder={t('genericFormItems.device.placeholder')} />
            </SelectTrigger>
          </div>
        </FormControl>
        <SelectContent>
          {data?.map((device) => (
            <SelectItem key={device.path} value={device.path} className='cursor-pointer'>
              <div className='flex items-center gap-2'>
                <span className='capitalize'>{prettifyDeviceName(device.name)}</span>
                <span className='text-xs text-neutral-500'>({device.path})</span>
              </div>
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </FormItem>
  );
});
