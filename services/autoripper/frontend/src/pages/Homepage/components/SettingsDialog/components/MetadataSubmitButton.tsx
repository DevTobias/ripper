import { Save } from 'lucide-react';
import { useFormContext } from 'react-hook-form';
import { useTranslation } from 'react-i18next';
import { useShallow } from 'zustand/react/shallow';

import { Button } from '$/components/common/ui/button';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';

export const MetadataSubmitButton = () => {
  const { t } = useTranslation();
  const rippingInProgress = useMediaStore(useShallow((state) => state.rippingInProgress));
  const form = useFormContext();

  return (
    <Button type='submit' className='flex w-full gap-3' disabled={rippingInProgress || !form.formState.isValid}>
      <span>{t('homepage.metadata.saveMetadata')}</span>
      <Save className='size-4' />
    </Button>
  );
};
