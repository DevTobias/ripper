import { FC } from 'react';

import { MediaCard } from '$/pages/Homepage/components/MetadataForm/components/MediaCard';

import type { MetadataFormControl } from '$/pages/Homepage/components/MetadataForm';

interface Props {
  form: MetadataFormControl;
}

export const MovieSelection: FC<Props> = ({ form }) => {
  const selectedMedia = form.watch('selectedMedia');

  return <div>{selectedMedia && <MediaCard item={selectedMedia} isDisabled />}</div>;
};
