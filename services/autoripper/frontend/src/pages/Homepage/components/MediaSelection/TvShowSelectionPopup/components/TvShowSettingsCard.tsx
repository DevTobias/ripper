import { forwardRef } from 'react';
import { useShallow } from 'zustand/react/shallow';

import { ButtonProps } from '$/components/common/ui/button';
import { MediaCard } from '$/pages/Homepage/components/MediaSelection/components/MediaCard';
import { MissingMetadataCard } from '$/pages/Homepage/components/MediaSelection/components/MissingMetadataCard';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';

export const TvShowSettingsCard = forwardRef<HTMLButtonElement, ButtonProps>(function TvShowSettingsCard(props, ref) {
  const selectedTvShow = useMediaStore(useShallow((state) => state.selectedTvShow));
  const tvShowSelectionValues = useMediaStore(useShallow((state) => state.tvShowSelectionValues));

  if (!selectedTvShow || !tvShowSelectionValues) {
    return <MissingMetadataCard ref={ref} {...props} />;
  }

  const episodes = selectedTvShow.seasons[tvShowSelectionValues.selectedSeason - 1]?.episodes.filter((episode) =>
    tvShowSelectionValues.selectedEpisodes.includes(episode.episodeNumber)
  );

  return (
    <MediaCard
      item={{
        id: selectedTvShow.id,
        title: selectedTvShow.title,
        description: selectedTvShow.description,
        popularity: selectedTvShow.popularity,
        posterPath: selectedTvShow.posterPath,
        releaseDate: selectedTvShow.releaseDate,
        voteAverage: selectedTvShow.voteAverage,
      }}
      selectedSeason={tvShowSelectionValues.selectedSeason}
      selectedEpisodes={episodes}
      mediaType='tv_show'
      showEditButton
      ref={ref}
      {...props}
    />
  );
});
