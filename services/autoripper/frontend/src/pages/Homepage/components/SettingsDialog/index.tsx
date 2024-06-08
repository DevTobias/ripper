import { useState } from 'react';

import { Dialog, DialogTrigger, DialogContent } from '$/components/common/ui/dialog';
import { MediaSettingsCard } from '$/pages/Homepage/components/SettingsDialog/components/MediaSettingsCard';
import { MetadataSubmitButton } from '$/pages/Homepage/components/SettingsDialog/components/MetadataSubmitButton';
import { SettingsForm } from '$/pages/Homepage/components/SettingsDialog/components/SettingsForm';

export const SettingsDialog = () => {
  const [open, setOpen] = useState(false);

  return (
    <Dialog modal={false} open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <MediaSettingsCard />
      </DialogTrigger>
      <DialogContent className='size-full max-w-full px-32 py-10'>
        <SettingsForm onSubmit={() => setOpen(false)}>
          <MetadataSubmitButton />
        </SettingsForm>
      </DialogContent>
    </Dialog>
  );
};
