import { OctagonX, SquarePlay } from 'lucide-react';
import { useState } from 'react';
import useWebSocket from 'react-use-websocket';
import { z } from 'zod';
import { useShallow } from 'zustand/react/shallow';

import { RadialProgressBar } from '$/components/common/RadialProgressBar';
import { Button } from '$/components/common/ui/button';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';
import { endpointFactory } from '$/services/endpoints';

const ProgressSchema = z.object({
  stepTitle: z.string(),
  stepDetails: z.string(),
  progress: z.number(),
  step: z.number(),
});

export const ProcessProgress = () => {
  const [inProgress, setInProgress] = useState(false);

  const [rippingProgress, setRippingProgress] = useState<z.infer<typeof ProgressSchema>>({
    progress: 0,
    step: 0,
    stepDetails: '',
    stepTitle: '',
  });

  const metadata = useMediaStore(useShallow((state) => state.metadata));

  const { sendMessage } = useWebSocket(
    metadata ? endpointFactory.ripMovieWebsocket(metadata.selectedMedia.id, ['deu'], metadata.device) : '',
    { onMessage: (event) => setRippingProgress(ProgressSchema.parse(JSON.parse(event.data as string))) },
    inProgress && !!metadata
  );

  const start = () => {
    setInProgress(true);
  };

  const cancel = () => {
    sendMessage('cancel');
    setInProgress(false);
    setRippingProgress({ progress: 0, step: 0, stepDetails: '', stepTitle: '' });
  };

  return (
    <div className='relative flex flex-col items-center'>
      <RadialProgressBar
        progress={rippingProgress.progress * 100}
        size={200}
        strokeWidth={20}
        ringClassName='text-neutral-900'
        bgClassName='text-neutral-900 opacity-30'
        className='relative z-20'
      >
        {!inProgress && (
          <Button className='aspect-square min-w-0 p-0' disabled={!metadata} onClick={start}>
            <SquarePlay className='size-6' />
          </Button>
        )}

        {inProgress && (
          <Button className='aspect-square min-w-0 p-0' disabled={!metadata} onClick={cancel}>
            <OctagonX className='size-6 animate-[spin_2s_linear_infinite]' />
          </Button>
        )}
      </RadialProgressBar>
      <RadialProgressBar
        progress={0}
        size={250}
        strokeWidth={20}
        ringClassName='text-neutral-900'
        bgClassName='text-neutral-900 opacity-30'
        className='absolute left-0 top-0 z-10 translate-x-[-25px] translate-y-[-25px]'
      >
        <div></div>
      </RadialProgressBar>
      <div className='absolute -bottom-24 mt-8 text-center'>
        <div className=' text-sm font-medium text-slate-500 dark:text-slate-400'>{rippingProgress.stepTitle}</div>
        <div className='text-sm text-slate-500 dark:text-slate-400'>
          <span>{rippingProgress.stepDetails}</span>
          {rippingProgress.stepDetails && <span> ({(rippingProgress.progress * 100).toFixed(1)}%)</span>}
        </div>
      </div>
    </div>
  );
};
