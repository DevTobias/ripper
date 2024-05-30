import { ImageOff } from 'lucide-react';
import { FC } from 'react';

import { cn } from '$/lib/utils';
import { SearchResultItem } from '$/services/metadata';

interface Props {
  item: SearchResultItem;
  isDisabled?: boolean;
  onClick?: () => void;
}

export const MediaCard: FC<Props> = ({ item, isDisabled = false, onClick }) => {
  return (
    <button
      className={cn(
        'flex items-center overflow-hidden rounded-md border text-left shadow-sm',
        !isDisabled && 'transition-colors hover:bg-slate-100'
      )}
      type='button'
      onClick={!isDisabled ? onClick : undefined}
      disabled={isDisabled}
    >
      <div className='aspect-[2/3] h-[90px] border-r'>
        {item.posterPath === null && (
          <div className='flex size-full items-center justify-center bg-slate-200 dark:bg-slate-800'>
            <ImageOff />
          </div>
        )}
        {item.posterPath && <img src={`https://image.tmdb.org/t/p/w200/${item.posterPath}`} alt='' className='size-full' />}
      </div>
      <div className='flex w-full flex-col gap-1 p-3'>
        <div className='flex gap-2'>
          <span className='max-w-[70%] truncate text-sm font-medium'>{item.title}</span>
          <span className='text-sm text-slate-500 dark:text-slate-400'>{item.releaseDate.toLocaleDateString()}</span>
        </div>
        <div className='line-clamp-2 max-w-[95%] text-sm text-slate-500 dark:text-slate-400'>{item.description}</div>
      </div>
    </button>
  );
};
