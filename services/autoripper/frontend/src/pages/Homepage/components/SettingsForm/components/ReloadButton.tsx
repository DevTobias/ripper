import { useQuery } from '@tanstack/react-query';
import { RefreshCcw } from 'lucide-react';
import { FC } from 'react';

import { Button } from '$/components/common/ui/button';
import { cn } from '$/lib/utils';
import { devicesQuery } from '$/services/devices';
import { encodingPresetsQuery } from '$/services/presets';

import type { MetadataFormControl } from '$/pages/Homepage/components/SettingsForm';

interface Props {
  form: MetadataFormControl;
}

export const ReloadButton: FC<Props> = ({ form }) => {
  const devices = useQuery(devicesQuery);
  const presets = useQuery(encodingPresetsQuery);

  const reloadDevices = () => {
    form.resetField('device');
    form.resetField('preset');
    void devices.refetch();
    void presets.refetch();
  };

  const loading = devices.isLoading || devices.isRefetching || presets.isLoading || presets.isRefetching;

  return (
    <Button className='aspect-square h-full p-3' type='button' onClick={reloadDevices} disabled={loading}>
      <RefreshCcw className={cn('size-4', loading && 'animate-spin')} />
    </Button>
  );
};
