import { useEffect } from 'react';
import useWebSocket from 'react-use-websocket';
import { z } from 'zod';
import { useShallow } from 'zustand/react/shallow';

import { RadialProgressBar } from '$/components/common/RadialProgressBar';
import { Badge } from '$/components/common/ui/badge';
import { cn } from '$/lib/utils';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';
import { endpointFactory } from '$/services/endpoints';

const WebsocketMessageSchema = z.object({
  type: z.string(),
  payload: z
    .object({
      stepTitle: z.string(),
      stepDetails: z.string(),
      progress: z.number(),
      step: z.number(),
      eta: z.number(),
    })
    .optional(),
});

type WebsocketMessage = z.infer<typeof WebsocketMessageSchema>;

export const ProcessProgress = () => {
  const rippingInProgress = useMediaStore(useShallow((state) => state.rippingInProgress));
  const rippingProgress = useMediaStore(useShallow((state) => state.rippingProgress));
  const metadata = useMediaStore(useShallow((state) => state.metadata));
  const selectedTitles = useMediaStore(useShallow((state) => state.selectedTitles));

  const handleWebsocketMessage = (message: WebsocketMessage) => {
    if (message.type === 'ripping_progress' || message.type === 'encoding_progress') {
      useMediaStore.setState({
        rippingProgress: {
          ...message.payload!,
          progressState: message.type === 'ripping_progress' ? 'ripping' : 'encoding',
        },
      });
    }

    if (message.type === 'ripping_done') {
      useMediaStore.setState({ rippingProgress: { ...useMediaStore.getState().rippingProgress, progressState: 'idle' } });
    }

    if (message.type === 'encoding_done') {
      useMediaStore.setState({
        selectedTitles: [],
        metadata: null,
        rippingInProgress: false,
        rippingProgress: { progress: 0, step: 0, eta: 0, stepDetails: '', stepTitle: '', progressState: 'idle' },
      });
    }
  };

  const { sendMessage } = useWebSocket(
    metadata ? endpointFactory.ripMovieWebsocket(selectedTitles, metadata.device, metadata.profile) : '',
    { onMessage: (event) => handleWebsocketMessage(WebsocketMessageSchema.parse(JSON.parse(event.data as string))) },
    rippingInProgress && !!metadata && selectedTitles.length > 0
  );

  useEffect(() => {
    useMediaStore.getState().sendWebsocketMessage = sendMessage;
  }, [sendMessage]);

  const getCurrentProgress = (type: 'ripping' | 'encoding' | 'any') => {
    if (type === 'ripping') {
      if (rippingProgress.progressState === 'ripping') return rippingProgress.progress * 100;
      if (rippingProgress.progressState === 'encoding') return 100;
    }

    if (type === 'encoding') {
      if (rippingProgress.progressState === 'ripping') return 0;
      if (rippingProgress.progressState === 'encoding') return rippingProgress.progress * 100;
    }

    return rippingProgress.progress * 100;
  };

  return (
    <div className='relative flex flex-col items-center'>
      <RadialProgressBar
        progress={getCurrentProgress('ripping')}
        size={200}
        strokeWidth={20}
        ringClassName='text-neutral-900'
        bgClassName={cn('text-neutral-900 opacity-30', rippingInProgress && 'text-neutral-700 animate-pulse opacity-40')}
      >
        <div className='flex flex-col items-center gap-2 text-center text-xs text-slate-500 dark:text-slate-400'>
          {rippingProgress.stepDetails && <div className='font-medium'>{rippingProgress.stepDetails}</div>}

          {getCurrentProgress('any') > 0 && (
            <div className='flex flex-wrap justify-center gap-1'>
              <Badge variant='secondary' className='w-fit px-[7px]'>
                {getCurrentProgress('any').toFixed(2)}%
              </Badge>
              {metadata?.type === 'tv_show' && (
                <Badge variant='secondary' className='w-fit px-[7px]'>
                  {rippingProgress.step + 1}/{selectedTitles.length}
                </Badge>
              )}
              {Math.round(rippingProgress.eta / 60) > 0 && (
                <Badge variant='secondary' className='w-fit px-[7px]'>
                  {Math.round(rippingProgress.eta / 60)} min
                </Badge>
              )}
            </div>
          )}
        </div>
      </RadialProgressBar>
      <RadialProgressBar
        progress={getCurrentProgress('encoding')}
        size={250}
        strokeWidth={20}
        ringClassName='text-neutral-900'
        bgClassName={cn('text-neutral-900 opacity-30', rippingInProgress && 'text-neutral-700 animate-pulse opacity-40')}
        className='absolute left-0 top-0 z-10 translate-x-[-25px] translate-y-[-25px]'
      >
        <div></div>
      </RadialProgressBar>
    </div>
  );
};
