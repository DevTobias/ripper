import { SquarePlay } from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';

import { RadialProgressBar } from '$/components/common/RadialProgressBar';
import { Button } from '$/components/common/ui/button';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';

export const ProcessProgress = () => {
  const metadata = useMediaStore(useShallow((state) => state.metadata));

  const processReady = !!metadata;

  return (
    <RadialProgressBar
      progress={0}
      size={200}
      strokeWidth={20}
      ringClassName='text-neutral-900'
      bgClassName='text-neutral-900 opacity-30'
    >
      <Button className='aspect-square min-w-0 p-0' disabled={!processReady}>
        <SquarePlay className='size-6' />
      </Button>
    </RadialProgressBar>
  );
};
