import { FC } from 'react';

import { MediaCard } from '$/pages/Homepage/components/SettingsForm/components/MediaCard';
import { LoadingMediaCard } from '$/pages/Homepage/components/SettingsForm/components/MediaSelectionDrawer/components/LoadingMediaCard';

import type { MetadataFormControl } from '$/pages/Homepage/components/SettingsForm';

interface Props {
  form: MetadataFormControl;
}

export const MovieSelection: FC<Props> = ({ form }) => {
  const selectedMedia = form.watch('selectedMedia');

  return (
    <div>
      {!selectedMedia && <LoadingMediaCard id={42} />}
      {selectedMedia && <MediaCard item={selectedMedia} mediaType='movie' disabled />}
    </div>
  );
};
