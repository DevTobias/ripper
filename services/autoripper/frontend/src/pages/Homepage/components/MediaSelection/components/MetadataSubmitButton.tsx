import { Save } from 'lucide-react';
import { useTranslation } from 'react-i18next';
import { useShallow } from 'zustand/react/shallow';

import { Button } from '$/components/common/ui/button';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';

export const MetadataSubmitButton = () => {
  const { t } = useTranslation();
  const rippingInProgress = useMediaStore(useShallow((state) => state.rippingInProgress));

  return (
    <Button type='submit' className='flex w-full gap-3' disabled={rippingInProgress}>
      <span>{t('genericSelection.saveMetadata')}</span>
      <Save className='size-4' />
    </Button>
  );
};
