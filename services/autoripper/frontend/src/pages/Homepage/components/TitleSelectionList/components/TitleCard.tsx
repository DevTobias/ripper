import { Captions, Volume1 } from 'lucide-react';
import { FC } from 'react';

import { Title } from '$/services/properties';

interface Props {
  title: Title;
}

export const TitleCard: FC<Props> = ({ title }) => {
  return (
    <div className='flex flex-col gap-1'>
      <span className='text-sm font-medium'>
        {title.id} {title.name}
      </span>
      <div className='flex flex-col gap-1'>
        <span className='text-xs'>
          {title.videoSize} • {title.chapterCount} Kapitel • {(title.duration / 60).toFixed(0)}min • {title.diskSize}
        </span>
        <span className='flex items-center gap-1 text-xs'>
          <span className='flex size-4 items-center justify-center'>
            <Volume1 className='size-3' />
          </span>
          {[...new Set(title.audioStreams.map((audio) => audio.langName))].join(', ')}
        </span>
        <span className='flex items-center gap-1 text-xs'>
          <span className='flex size-4 items-center'>
            <Captions className='size-3' />
          </span>
          {[...new Set(title.subtitleStreams)].join(', ')}
        </span>
      </div>
    </div>
  );
};
