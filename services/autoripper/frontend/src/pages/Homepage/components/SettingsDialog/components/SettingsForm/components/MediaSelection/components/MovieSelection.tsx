import { FC } from 'react';

import { MediaCard } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm/components/MediaCard';
import { QualityProfileSelection } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm/components/MediaSelection/components/QualityProfileSelection';
import { RootFolderSelection } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm/components/MediaSelection/components/RootFolderSelection';
import { SeriesTypeSelection } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm/components/MediaSelection/components/SeriesSelection/components/SeriesTypeSelection';
import { LoadingMediaCard } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm/components/MediaSelectionDrawer/components/LoadingMediaCard';

import type { MetadataFormControl } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm';

interface Props {
  form: MetadataFormControl;
}

export const MovieSelection: FC<Props> = ({ form }) => {
  const selectedMedia = form.watch('selectedMedia');

  return (
    <div className='flex flex-col gap-4'>
      {!selectedMedia && <LoadingMediaCard id={42} />}
      {selectedMedia && <MediaCard item={selectedMedia} mediaType='movie' disabled />}
      <div className='flex items-end gap-2'>
        <QualityProfileSelection form={form} />
        <RootFolderSelection form={form} />
        <SeriesTypeSelection form={form} className='hidden' />
      </div>
    </div>
  );
};
