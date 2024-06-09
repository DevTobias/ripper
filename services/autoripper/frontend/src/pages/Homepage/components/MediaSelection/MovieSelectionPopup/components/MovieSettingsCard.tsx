import { forwardRef } from 'react';
import { useShallow } from 'zustand/react/shallow';

import { ButtonProps } from '$/components/common/ui/button';
import { MediaCard } from '$/pages/Homepage/components/MediaSelection/components/MediaCard';
import { MissingMetadataCard } from '$/pages/Homepage/components/MediaSelection/components/MissingMetadataCard';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';

export const MovieSettingsCard = forwardRef<HTMLButtonElement, ButtonProps>(function MediaSettingsCard(props, ref) {
  const selectedMovie = useMediaStore(useShallow((state) => state.selectedMovie));

  if (!selectedMovie) {
    return <MissingMetadataCard ref={ref} {...props} />;
  }

  return (
    <MediaCard item={selectedMovie} runtime={selectedMovie.runtime} mediaType='movie' showEditButton ref={ref} {...props} />
  );
});
