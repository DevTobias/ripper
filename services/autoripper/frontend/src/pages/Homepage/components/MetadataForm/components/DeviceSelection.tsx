import { useQuery } from '@tanstack/react-query';
import { RefreshCcw } from 'lucide-react';
import { FC } from 'react';
import { useTranslation } from 'react-i18next';

import { Button } from '$/components/common/ui/button';
import { FormControl, FormDescription, FormField, FormItem, FormLabel, FormMessage } from '$/components/common/ui/form';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '$/components/common/ui/select';
import { cn } from '$/lib/utils';
import { devicesQuery } from '$/services/devices';

import type { MetadataFormControl } from '$/pages/Homepage/components/MetadataForm';

interface Props {
  form: MetadataFormControl;
}

const prettifyDeviceName = (device: string) => {
  return device.replace(/_/g, ' ').toLocaleLowerCase();
};

export const DeviceSelection: FC<Props> = ({ form }) => {
  const { t } = useTranslation();
  const { data, isLoading, isRefetching, refetch } = useQuery(devicesQuery);

  const reloadDevices = () => {
    form.resetField('device');
    void refetch();
  };

  const loading = isLoading || isRefetching;

  return (
    <FormField
      control={form.control}
      name='device'
      render={({ field }) => (
        <FormItem>
          <FormLabel className='flex items-center justify-between text-sm'>
            <span>{t('homepage.metadata.device.label')}</span>
            <FormMessage isTranslated />
          </FormLabel>
          <Select disabled={loading} onValueChange={field.onChange} value={field.value}>
            <FormControl>
              <div className='flex gap-3'>
                <SelectTrigger className='w-full' isLoading={loading}>
                  <SelectValue placeholder={t('homepage.metadata.device.placeholder')} />
                </SelectTrigger>
                <Button className='aspect-square h-full p-3' type='button' onClick={reloadDevices} disabled={loading}>
                  <RefreshCcw className={cn('size-4', loading && 'animate-spin')} />
                </Button>
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
            <FormDescription>{t('homepage.metadata.device.description')}</FormDescription>
          </Select>
        </FormItem>
      )}
    />
  );
};
