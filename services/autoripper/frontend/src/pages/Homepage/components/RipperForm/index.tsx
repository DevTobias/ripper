import { zodResolver } from '@hookform/resolvers/zod';
import { Save } from 'lucide-react';
import { UseFormReturn, useForm } from 'react-hook-form';
import { z } from 'zod';
import { useShallow } from 'zustand/react/shallow';

import { Button } from '$/components/common/ui/button';
import { Form } from '$/components/common/ui/form';
import { DeviceSelection } from '$/pages/Homepage/components/RipperForm/components/DeviceSelection';
import { EncodingPresetSelection } from '$/pages/Homepage/components/RipperForm/components/EncodingPresetSelection';
import { ReloadButton } from '$/pages/Homepage/components/RipperForm/components/ReloadButton';
import { RipperFormValues, ripperFormSchema, useMediaStore } from '$/pages/Homepage/stores/useMediaStore';

export type RipperFormControl = UseFormReturn<RipperFormValues>;

export const RipperForm = () => {
  const setRipperInfo = useMediaStore(useShallow((state) => state.setRipperInfo));

  const form = useForm<z.infer<typeof ripperFormSchema>>({
    resolver: zodResolver(ripperFormSchema),
    defaultValues: { device: '', preset: '' },
    shouldFocusError: true,
  });

  const onSubmit = (values: RipperFormValues) => {
    setRipperInfo(values);
  };

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className='flex items-end gap-2 space-y-8'>
        <DeviceSelection form={form} />
        <EncodingPresetSelection form={form} />
        <div className='flex gap-2'>
          <ReloadButton form={form} />
          <Button type='submit' className='aspect-square h-full p-3'>
            <Save className='size-4' />
          </Button>
        </div>
      </form>
    </Form>
  );
};
