import { useQuery } from '@tanstack/react-query';
import { FC } from 'react';

import { FormField } from '$/components/common/ui/form';
import { cn } from '$/lib/utils';
import { EpisodeSelectionButtons } from '$/pages/Homepage/components/MediaSelection/TvShowSelectionPopup/components/EpisodeSelection/components/EpisodeSelectionButtons';
import { SeasonSelectionButtons } from '$/pages/Homepage/components/MediaSelection/TvShowSelectionPopup/components/EpisodeSelection/components/SeasonSelectionButtons';
import { getTvDetailsQuery } from '$/services/metadata';

import type { TvShowSelectionFormControl } from '$/pages/Homepage/components/MediaSelection/TvShowSelectionPopup/components/TvShowSelectionForm';

interface Props {
  form: TvShowSelectionFormControl;
}

export const EpisodeSelection: FC<Props> = ({ form }) => {
  const selectedMedia = form.watch('selectedMedia');
  const selectedSeason = form.watch('selectedSeason');

  const { data, isLoading } = useQuery(getTvDetailsQuery({ id: selectedMedia, lang: 'de' }));

  return (
    <div
      className={cn('grid grid-cols-[auto,minmax(0,1fr)] gap-4', (data?.seasons.length ?? 0) > 7 && 'grid-cols-[30%,68%]')}
    >
      <FormField
        control={form.control}
        name='selectedSeason'
        render={({ field: { value, onChange } }) => (
          <SeasonSelectionButtons
            value={value}
            onChange={onChange}
            isLoading={isLoading || !data}
            seasonNumbers={data?.seasons.map((season) => season.seasonNumber)}
          />
        )}
      />

      <FormField
        control={form.control}
        name='selectedEpisodes'
        render={({ field: { value, onChange }, fieldState: { error } }) => (
          <EpisodeSelectionButtons
            value={value}
            onChange={onChange}
            isLoading={isLoading || !data}
            episodes={data?.seasons[selectedSeason - 1].episodes}
            hasError={!!error}
          />
        )}
      />
    </div>
  );
};
