import { useState } from 'react';

import { Dialog, DialogTrigger, DialogContent } from '$/components/common/ui/dialog';
import { MovieSelectionForm } from '$/pages/Homepage/components/MediaSelection/MovieSelectionPopup/components/MovieSelectionForm';
import { MovieSettingsCard } from '$/pages/Homepage/components/MediaSelection/MovieSelectionPopup/components/MovieSettingsCard';

export const MovieSelectionPopup = () => {
  const [open, setOpen] = useState(false);

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <MovieSettingsCard />
      </DialogTrigger>
      <DialogContent className='max-w-4xl'>
        <MovieSelectionForm onSubmit={() => setOpen(false)} />
      </DialogContent>
    </Dialog>
  );
};
