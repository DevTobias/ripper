import { forwardRef } from 'react';
import { useTranslation } from 'react-i18next';
import { useShallow } from 'zustand/react/shallow';

import { Button, ButtonProps } from '$/components/common/ui/button';
import { MediaCard } from '$/pages/Homepage/components/SettingsForm/components/MediaCard';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';

export const MetadataButton = forwardRef<HTMLButtonElement, ButtonProps>(function MetadataButton(props, ref) {
  const { t } = useTranslation();

  const metadata = useMediaStore(useShallow((state) => state.metadata));

  if (!metadata) {
    return (
      <Button className='w-full' ref={ref} {...props}>
        {t('homepage.metadata.selectMetadata')}
      </Button>
    );
  }

  const { selectedMedia } = metadata;

  return (
    <MediaCard
      showEditButton
      item={selectedMedia}
      selectedEpisodes={metadata.selectedEpisodes}
      selectedSeason={metadata.selectedSeason}
      ref={ref}
      mediaType={metadata.type}
      {...props}
    />
  );
});
