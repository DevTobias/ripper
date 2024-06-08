import { SquarePlus } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { useShallow } from 'zustand/react/shallow';

import { MediaCard } from '$/pages/Homepage/components/SettingsForm/components/MediaCard';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';

export const MediaSettingsCard = () => {
  const { t } = useTranslation();
  const metadata = useMediaStore(useShallow((state) => state.metadata));

  if (!metadata) {
    return (
      <div className='flex h-[90px] items-center overflow-hidden rounded-[4px] border text-left transition-colors hover:bg-slate-50'>
        <div className='aspect-[2/3] h-full border-r'>
          <div className='flex size-full items-center justify-center bg-slate-200 text-slate-600 dark:bg-slate-700 dark:text-slate-300'>
            <SquarePlus />
          </div>
        </div>
        <div className='flex w-full flex-col gap-1 p-3'>
          <div className='flex w-full gap-2'>
            <span className='max-w-[70%] truncate text-sm font-medium'>{t('homepage.metadata.selectMetadataTitle')}</span>
          </div>
          <div className='line-clamp-2 text-sm text-slate-500 dark:text-slate-400'>
            {t('homepage.metadata.selectMetadataDescription')}
          </div>
        </div>
      </div>
    );
  }

  return (
    <MediaCard
      item={metadata.selectedMedia}
      mediaType={metadata.type}
      selectedSeason={metadata.selectedSeason}
      selectedEpisodes={metadata.selectedEpisodes}
      disabled
    />
  );
};
