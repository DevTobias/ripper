import { useQuery } from '@tanstack/react-query';
import { FC } from 'react';

import { MediaCard } from '$/pages/Homepage/components/MetadataForm/components/MediaCard';
import { EpisodeSelection } from '$/pages/Homepage/components/MetadataForm/components/MediaSelection/components/SeriesSelection/components/EpisodeSelection';
import { SeasonSelection } from '$/pages/Homepage/components/MetadataForm/components/MediaSelection/components/SeriesSelection/components/SeasonSelection';
import { getTvDetailsQuery } from '$/services/metadata';

import type { MetadataFormControl } from '$/pages/Homepage/components/MetadataForm';

interface Props {
  form: MetadataFormControl;
}

export const SeriesSelection: FC<Props> = ({ form }) => {
  const selectedMedia = form.watch('selectedMedia');
  const selectedSeason = form.watch('selectedSeason');

  const { data, isLoading } = useQuery(
    getTvDetailsQuery({ id: selectedMedia?.id, enabled: form.watch('type') === 'tv_show' })
  );

  return (
    <div className='flex flex-col gap-4'>
      {selectedMedia && (
        <>
          <MediaCard item={selectedMedia} mediaType='tv_show' disabled />

          <SeasonSelection
            form={form}
            isLoading={isLoading}
            seasonNumbers={data?.seasons.map((season) => season.season_number)}
          />
          <EpisodeSelection
            form={form}
            isLoading={isLoading}
            episodeNumbers={data?.seasons[selectedSeason - 1].episodes.map((episode) => episode.episode_number)}
          />
        </>
      )}
    </div>
  );
};
