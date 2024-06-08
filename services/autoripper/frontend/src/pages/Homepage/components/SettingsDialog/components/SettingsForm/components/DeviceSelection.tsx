import { useQuery } from '@tanstack/react-query';
import { FC, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useShallow } from 'zustand/react/shallow';

import { FormControl, FormField, FormItem, FormLabel } from '$/components/common/ui/form';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '$/components/common/ui/select';
import { cn } from '$/lib/utils';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';
import { devicesQuery } from '$/services/devices';

import type { MetadataFormControl } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm';

interface Props {
  form: MetadataFormControl;
}

const prettifyDeviceName = (device: string) => {
  return device.replace(/_/g, ' ').toLocaleLowerCase();
};

export const DeviceSelection: FC<Props> = ({ form }) => {
  const { t } = useTranslation();
  const { data, isLoading, isRefetching } = useQuery(devicesQuery);

  const rippingInProgress = useMediaStore(useShallow((state) => state.rippingInProgress));

  const loading = isLoading || isRefetching;

  useEffect(() => {
    if ((data?.length ?? 0) > 0 && !form.getValues('device')) form.setValue('device', data![0].path);
  }, [data, form]);

  return (
    <FormField
      control={form.control}
      name='device'
      render={({ field, fieldState }) => (
        <FormItem className='w-full'>
          <FormLabel className='flex items-center gap-1 text-sm'>
            <span>{t('homepage.device.label')}</span>
          </FormLabel>
          <Select disabled={loading || rippingInProgress} onValueChange={field.onChange} value={field.value}>
            <FormControl>
              <div className='flex gap-3'>
                <SelectTrigger
                  className={cn('w-full', fieldState.error && 'border-red-500')}
                  isLoading={loading}
                  ref={field.ref}
                >
                  <SelectValue placeholder={t('homepage.device.placeholder')} />
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
      )}
    />
  );
};
