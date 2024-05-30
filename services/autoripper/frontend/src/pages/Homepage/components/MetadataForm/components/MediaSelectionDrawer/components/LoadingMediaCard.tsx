import { Skeleton } from '$/components/common/ui/skeleton';
import { randomText } from '$/lib/utils';

export const LoadingMediaCard = ({ id }: { id: number }) => {
  return (
    <div className='flex items-center overflow-hidden rounded-md border shadow-sm'>
      <Skeleton className='aspect-[2/3] h-[90px]' />
      <div className='flex w-full flex-col gap-1 p-3'>
        <div className='flex gap-2'>
          <Skeleton className='max-w-[70%] truncate text-sm font-medium'>{randomText(id, 15, 30)}</Skeleton>
          <Skeleton className='text-sm'>11.11.1111</Skeleton>
        </div>
        <Skeleton className='line-clamp-2 max-w-[90%] text-sm'>{randomText(id, 50, 65)}</Skeleton>
      </div>
    </div>
  );
};
