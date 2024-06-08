import { FC } from 'react';

import { MediaCard } from '$/pages/Homepage/components/SettingsForm/components/MediaCard';
import { MetadataSubmitButton } from '$/pages/Homepage/components/SettingsForm/components/MediaSelection/components/MetadataSubmitButton';
import { QualityProfileSelection } from '$/pages/Homepage/components/SettingsForm/components/MediaSelection/components/QualityProfileSelection';
import { RootFolderSelection } from '$/pages/Homepage/components/SettingsForm/components/MediaSelection/components/RootFolderSelection';
import { LoadingMediaCard } from '$/pages/Homepage/components/SettingsForm/components/MediaSelectionDrawer/components/LoadingMediaCard';

import type { MetadataFormControl } from '$/pages/Homepage/components/SettingsForm';

interface Props {
  form: MetadataFormControl;
}

export const MovieSelection: FC<Props> = ({ form }) => {
  const selectedMedia = form.watch('selectedMedia');

  return (
    <div className='flex flex-col gap-3'>
      <div className='flex items-end gap-2'>
        <QualityProfileSelection form={form} />
        <RootFolderSelection form={form} />
        <MetadataSubmitButton />
      </div>
      {!selectedMedia && <LoadingMediaCard id={42} />}
      {selectedMedia && <MediaCard item={selectedMedia} mediaType='movie' disabled />}
    </div>
  );
};
