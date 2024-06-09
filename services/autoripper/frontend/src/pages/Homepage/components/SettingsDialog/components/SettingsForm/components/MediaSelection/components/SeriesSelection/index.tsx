import { useQuery } from '@tanstack/react-query';
import { FC, useEffect } from 'react';

import { cn } from '$/lib/utils';
import { MediaCard } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm/components/MediaCard';
import { QualityProfileSelection } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm/components/MediaSelection/components/QualityProfileSelection';
import { RootFolderSelection } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm/components/MediaSelection/components/RootFolderSelection';
import { EpisodeSelection } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm/components/MediaSelection/components/SeriesSelection/components/EpisodeSelection';
import { SeasonSelection } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm/components/MediaSelection/components/SeriesSelection/components/SeasonSelection';
import { SeriesTypeSelection } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm/components/MediaSelection/components/SeriesSelection/components/SeriesTypeSelection';
import { LoadingMediaCard } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm/components/MediaSelectionDrawer/components/LoadingMediaCard';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';
import { getTvDetailsQuery } from '$/services/metadata';

import type { MetadataFormControl } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm';

interface Props {
  form: MetadataFormControl;
}

export const SeriesSelection: FC<Props> = ({ form }) => {
  const selectedMedia = form.watch('selectedMedia');
  const selectedSeason = form.watch('selectedSeason');

  const { data, isLoading } = useQuery(
    getTvDetailsQuery({ id: selectedMedia?.id, enabled: form.watch('type') === 'tv_show', lang: 'de' })
  );

  useEffect(() => {
    useMediaStore.setState({ selectedTvId: data?.external_ids.tvdb_id });
  }, [data]);

  return (
    <div className='flex flex-col gap-4'>
      {!selectedMedia && <LoadingMediaCard id={42} />}
      {selectedMedia && <MediaCard item={selectedMedia} mediaType='tv_show' disabled />}

      <div
        className={cn('grid grid-cols-[auto,minmax(0,1fr)] gap-4', (data?.seasons.length ?? 0) > 7 && 'grid-cols-[30%,68%]')}
      >
        <SeasonSelection
          form={form}
          isLoading={isLoading || !data}
          seasonNumbers={data?.seasons.map((season) => season.season_number)}
        />
        <EpisodeSelection form={form} isLoading={isLoading || !data} episodes={data?.seasons[selectedSeason - 1].episodes} />
      </div>

      <div className='flex items-end gap-2'>
        <QualityProfileSelection form={form} />
        <RootFolderSelection form={form} />
        <SeriesTypeSelection form={form} />
      </div>
    </div>
  );
};
