import { Captions, GripVertical, Trash2, Volume1 } from 'lucide-react';
import { FC } from 'react';

import { Badge } from '$/components/common/ui/badge';
import { Button } from '$/components/common/ui/button';
import { Skeleton } from '$/components/common/ui/skeleton';
import { randomText } from '$/lib/utils';

interface Props {
  id: number;
}

export const LoadingTitleCard: FC<Props> = ({ id }) => {
  return (
    <>
      <div className='flex h-fit items-center justify-between rounded-md border bg-white px-4 py-3 shadow-sm'>
        <div className='flex flex-col gap-2'>
          <span className='flex gap-1'>
            <Skeleton className='w-fit text-sm font-medium'>{randomText(id + 1, 10, 15)}</Skeleton>
            <Skeleton className='w-fit text-xs'>{randomText(id + 1, 5, 10)}</Skeleton>
          </span>
          <div className='flex flex-col gap-1'>
            <span className='flex flex-wrap gap-1'>
              <Badge variant='loading'>34 Chapters</Badge>
              <Badge variant='loading'>130 min</Badge>
              <Badge variant='loading'>32 GB</Badge>
              {['German', 'Japanese'].map((lang) => (
                <Badge key={lang} variant='loading'>
                  <Volume1 className='mr-px size-3' />
                  <span>{lang}</span>
                </Badge>
              ))}
              {['German'].map((lang) => (
                <Badge key={lang} variant='loading'>
                  <Captions className='mr-1 size-3' />
                  <span>{lang}</span>
                </Badge>
              ))}
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
    </>
  );
};
