import { Dialog, DialogTrigger, DialogContent, DialogClose } from '$/components/common/ui/dialog';
import { MediaSettingsCard } from '$/pages/Homepage/components/SettingsDialog/components/MediaSettingsCard';
import { SettingsForm } from '$/pages/Homepage/components/SettingsForm';
import { MetadataSubmitButton } from '$/pages/Homepage/components/SettingsForm/components/MediaSelectionDrawer/components/MetadataSubmitButton';

export const SettingsDialog = () => {
  return (
    <Dialog>
      <DialogTrigger>
        <MediaSettingsCard />
      </DialogTrigger>
      <DialogContent className='size-full max-w-full px-32 py-10'>
        <SettingsForm>
          <DialogClose asChild>
            <MetadataSubmitButton />
          </DialogClose>
        </SettingsForm>
      </DialogContent>
    </Dialog>
  );
};
