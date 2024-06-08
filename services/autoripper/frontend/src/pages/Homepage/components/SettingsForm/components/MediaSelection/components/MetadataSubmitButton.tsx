import { Save } from 'lucide-react';
import { useShallow } from 'zustand/react/shallow';

import { Button } from '$/components/common/ui/button';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';

export const MetadataSubmitButton = () => {
  const rippingInProgress = useMediaStore(useShallow((state) => state.rippingInProgress));

  return (
    <Button type='submit' className='aspect-square h-full p-3' disabled={rippingInProgress}>
      <Save className='size-4' />
    </Button>
  );
};
