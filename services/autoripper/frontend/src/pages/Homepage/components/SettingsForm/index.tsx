import { zodResolver } from '@hookform/resolvers/zod';
import { Save } from 'lucide-react';
import { useEffect } from 'react';
import { useForm } from 'react-hook-form';
import { z } from 'zod';
import { useShallow } from 'zustand/react/shallow';

import { Button } from '$/components/common/ui/button';
import { Form } from '$/components/common/ui/form';
import { DeviceSelection } from '$/pages/Homepage/components/SettingsForm/components/DeviceSelection';
import { EncodingPresetSelection } from '$/pages/Homepage/components/SettingsForm/components/EncodingPresetSelection';
import { MediaSelection } from '$/pages/Homepage/components/SettingsForm/components/MediaSelection';
import { ReloadButton } from '$/pages/Homepage/components/SettingsForm/components/ReloadButton';
import { MetadataFormValues, metadataFormSchema, useMediaStore } from '$/pages/Homepage/stores/useMediaStore';

import type { UseFormReturn } from 'react-hook-form';

export type MetadataFormControl = UseFormReturn<MetadataFormValues>;

export const SettingsForm = () => {
  const rippingInProgress = useMediaStore(useShallow((state) => state.rippingInProgress));
  const setMetadata = useMediaStore(useShallow((state) => state.setMetadata));

  const form = useForm<z.infer<typeof metadataFormSchema>>({
    resolver: zodResolver(metadataFormSchema),
    defaultValues: {
      device: '',
      profile: '',
      type: 'movie',
      selectedSeason: 1,
      selectedEpisodes: [],
    },
  });

  const onSubmit = (values: MetadataFormValues) => {
    setMetadata(values);
  };

  useEffect(() => {
    return useMediaStore.subscribe((state, prevState) => {
      if (!state.metadata && prevState.metadata) setTimeout(() => form.reset(), 0);
    });
  }, [form]);

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-8'>
        <div className='flex items-end gap-2'>
          <DeviceSelection form={form} />
          <EncodingPresetSelection form={form} />
          <ReloadButton form={form} />
        </div>

        <MediaSelection form={form}>
          <Button type='submit' className='aspect-square h-full p-3' disabled={rippingInProgress}>
            <Save className='size-4' />
          </Button>
        </MediaSelection>
      </form>
    </Form>
  );
};
