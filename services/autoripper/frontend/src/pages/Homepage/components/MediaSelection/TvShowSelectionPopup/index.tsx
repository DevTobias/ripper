import { useState } from 'react';

import { Dialog, DialogTrigger, DialogContent } from '$/components/common/ui/dialog';
import { TvShowSelectionForm } from '$/pages/Homepage/components/MediaSelection/TvShowSelectionPopup/components/TvShowSelectionForm';
import { TvShowSettingsCard } from '$/pages/Homepage/components/MediaSelection/TvShowSelectionPopup/components/TvShowSettingsCard';

export const TvShowSelectionPopup = () => {
  const [open, setOpen] = useState(false);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <TvShowSettingsCard />
      </DialogTrigger>
      <DialogContent className='max-w-4xl'>
        <TvShowSelectionForm onSubmit={() => setOpen(false)} />
      </DialogContent>
    </Dialog>
  );
};
