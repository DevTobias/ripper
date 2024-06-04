import { Captions, Volume1 } from 'lucide-react';
import { FC } from 'react';
import { useTranslation } from 'react-i18next';

import { Badge } from '$/components/common/ui/badge';
import { Title } from '$/services/properties';

interface Props {
  title: Title;
}

export const TitleCard: FC<Props> = ({ title }) => {
  const { t } = useTranslation();

  return (
    <div className='flex flex-col gap-2'>
      <span className='flex items-center gap-1 text-sm font-medium'>
        <span>
          {title.id} {title.name}
        </span>
        <span className='text-xs font-normal'>{title.videoSize}</span>
      </span>
      <div className='flex flex-col gap-1'>
        <span className='flex max-h-[48px] flex-wrap gap-1 overflow-y-hidden'>
          <Badge variant='secondary'>{t('homepage.titleSelection.chapters', { amount: title.chapterCount })}</Badge>
          <Badge variant='secondary'>{(title.duration / 60).toFixed(0)}min</Badge>
          <Badge variant='secondary'>{title.diskSize}</Badge>
          {[...new Set(title.audioStreams.map((audio) => audio.langName))].map((lang) => (
            <Badge key={lang} variant='outline'>
              <Volume1 className='mr-px size-3' />
              <span>{lang}</span>
            </Badge>
          ))}
          {[...new Set(title.subtitleStreams)].map((lang) => (
            <Badge key={lang} variant='outline'>
              <Captions className='mr-1 size-3' />
              <span>{lang}</span>
            </Badge>
          ))}
        </span>
      </div>
    </div>
  );
};
