import { Captions, GripVertical, Trash2, Volume1 } from 'lucide-react';
import { FC } from 'react';

import { Button } from '$/components/common/ui/button';
import { Skeleton } from '$/components/common/ui/skeleton';
import { randomText } from '$/lib/utils';

interface Props {
  id: number;
}

export const LoadingTitleCard: FC<Props> = ({ id }) => {
  return (
    <div className='flex h-fit items-center justify-between rounded-md border bg-white px-4 py-3 shadow-sm'>
      <div className='flex flex-col gap-1'>
        <Skeleton className='w-fit text-sm font-medium'>{randomText(id + 1, 10, 15)}</Skeleton>
        <div className='flex flex-col gap-1'>
          <Skeleton className='w-fit text-xs'>{randomText(id + 2, 20, 30)}</Skeleton>
          <span className='flex items-center gap-1 text-xs'>
            <span className='flex size-4 items-center justify-center'>
              <Volume1 className='size-3' />
            </span>
            <Skeleton>{randomText(id + 3, 15, 20)}</Skeleton>
          </span>
          <span className='flex items-center gap-1 text-xs'>
            <span className='flex size-4 items-center'>
              <Captions className='size-3' />
            </span>
            <Skeleton>{randomText(id + 4, 15, 20)}</Skeleton>
          </span>
        </div>
      </div>
      <div className='flex'>
        <Button variant='ghost' className='p-3' disabled>
          <Trash2 className='size-4 text-neutral-400' />
        </Button>
        <Button variant='ghost' className='touch-none p-3' disabled>
          <GripVertical className='size-4 text-neutral-400' />
        </Button>
      </div>
    </div>
  );
};
