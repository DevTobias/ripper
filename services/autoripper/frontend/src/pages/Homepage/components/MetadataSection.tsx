import { useQuery } from '@tanstack/react-query';
import { useTranslation } from 'react-i18next';

import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '$/components/common/ui/select';
import { useDiscStore } from '$/pages/Homepage/stores/useDiscStore';
import { devicesQuery } from '$/services/devices';

const prettifyDeviceName = (device: string) => {
  return device.replace(/_/g, ' ').toLocaleLowerCase();
};

export const MetadataSection = () => {
  const { t } = useTranslation();

  const { data, isLoading } = useQuery(devicesQuery);

  const selectDevice = useDiscStore.useSelectDevice();
  const selectedDevice = useDiscStore.useSelectedDevice?.();

  return (
    <div className='flex flex-col gap-3'>
      <h2 className='font-medium'>{t('homepage.metadata.title')}</h2>
      <Select disabled={isLoading || !data} onValueChange={selectDevice} defaultValue={selectedDevice}>
        <SelectTrigger className='w-full' isLoading={isLoading}>
          <SelectValue placeholder={t('homepage.metadata.chooseDisc')} />
        </SelectTrigger>
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
    </div>
  );
};
