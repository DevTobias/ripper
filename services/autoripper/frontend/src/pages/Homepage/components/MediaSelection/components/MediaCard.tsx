import { ImageOff, SquarePen, Star } from 'lucide-react';
import { forwardRef } from 'react';

import { ButtonProps } from '$/components/common/ui/button';
import { cn } from '$/lib/utils';
import { Episode, SearchResultItem } from '$/services/metadata';

interface Props {
  item: SearchResultItem;
  showEditButton?: boolean;
  selectedSeason?: number;
  selectedEpisodes?: Episode[];
  mediaType: 'movie' | 'tv_show';
  isActive?: boolean;
  runtime?: number;
}

export const MediaCard = forwardRef<HTMLButtonElement, ButtonProps & Props>(function MediaCard(
  { item, selectedSeason, selectedEpisodes, disabled, showEditButton, mediaType, runtime, isActive, onClick, ...rest },
  ref
) {
  const showSeasonAndEpisodes = mediaType === 'tv_show' && selectedSeason && selectedEpisodes;

  return (
    <button
      className={cn(
        ' flex items-center overflow-hidden rounded-[4px] border text-left h-[90px]',
        !disabled && 'transition-colors hover:bg-slate-100',
        isActive && 'bg-slate-100',
        showSeasonAndEpisodes && 'h-[125px]'
      )}
      type='button'
      onClick={!disabled ? onClick : undefined}
      disabled={disabled}
      ref={ref}
      {...rest}
    >
      <div className='aspect-[2/3] h-full border-r'>
        {item.posterPath === null && (
          <div className='flex size-full items-center justify-center bg-slate-200 dark:bg-slate-800'>
            <ImageOff />
          </div>
        )}
        {item.posterPath && (
          <img
            src={`https://image.tmdb.org/t/p/w200/${item.posterPath}`}
            alt={`${item.title} poster`}
            className='size-full'
          />
        )}
      </div>
      <div className='flex w-full flex-col gap-1 p-3'>
        <div className='flex justify-between'>
          <div className='flex w-full gap-2'>
            <span className='max-w-[70%] truncate text-sm font-medium'>{item.title}</span>
            <span className='text-sm text-slate-500 dark:text-slate-400'>{item.releaseDate.toLocaleDateString()}</span>
            {runtime && (
              <span className='text-sm text-slate-500 dark:text-slate-400'>{`${Math.floor(runtime / 60)}h ${runtime % 60}m`}</span>
            )}
          </div>

          <div className='flex items-center gap-1 text-slate-500 dark:text-slate-400'>
            <span className='text-sm '>{item.voteAverage.toFixed(1)}</span>
            <span className='flex gap-4'>
              <Star className='size-4 fill-yellow-500 stroke-none' />
              {showEditButton && <SquarePen className='size-4' />}
            </span>
          </div>
        </div>
        <div className='line-clamp-2 text-sm text-slate-500 dark:text-slate-400'>{item.description}</div>
        {showSeasonAndEpisodes && (
          <div className='mt-2 flex gap-1 text-sm '>
            <span className='inline-flex aspect-square h-9 items-center justify-center rounded-[4px] bg-slate-900 p-2 font-medium text-slate-50 dark:bg-slate-50 dark:text-slate-900'>{`S${selectedSeason}`}</span>

            {selectedEpisodes.map((episode) => (
              <div className='flex h-9 flex-col items-center justify-center rounded-[4px] border p-1' key={episode.id}>
                <span className='font-medium leading-none'>E{episode.episodeNumber}</span>
                {episode.runtime && <span className='text-[10px] leading-none opacity-60'>{episode.runtime} min</span>}
              </div>
            ))}
          </div>
        )}
      </div>
    </button>
  );
});
