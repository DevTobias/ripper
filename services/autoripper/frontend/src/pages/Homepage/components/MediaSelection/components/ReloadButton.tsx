import { useQuery } from '@tanstack/react-query';
import { RefreshCcw } from 'lucide-react';
import { FC } from 'react';
import { useShallow } from 'zustand/react/shallow';

import { Button } from '$/components/common/ui/button';
import { cn } from '$/lib/utils';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';
import { devicesQuery } from '$/services/devices';
import { qualityProfileQuery, rootFoldersQuery } from '$/services/management';
import { encodingPresetsQuery } from '$/services/presets';

interface Props {
  onReload: () => void;
  type: 'movie' | 'tv_show';
}

export const ReloadButton: FC<Props> = ({ type, onReload }) => {
  const rippingInProgress = useMediaStore(useShallow((state) => state.rippingInProgress));

  const devices = useQuery(devicesQuery);
  const encodingProfiles = useQuery(encodingPresetsQuery);
  const qualityProfiles = useQuery(qualityProfileQuery({ media_type: type }));
  const rootFolders = useQuery(rootFoldersQuery({ media_type: type }));

  const reloadDevices = () => {
    onReload();
    void encodingProfiles.refetch();
    void devices.refetch();
    void qualityProfiles.refetch();
    void rootFolders.refetch();
  };

  const loading =
    encodingProfiles.isLoading || devices.isRefetching || qualityProfiles.isLoading || rootFolders.isRefetching;

  return (
    <Button
      className='aspect-square h-full p-3'
      type='button'
      onClick={reloadDevices}
      disabled={loading || rippingInProgress}
    >
      <RefreshCcw className={cn('size-4', loading && 'animate-spin')} />
    </Button>
  );
};
