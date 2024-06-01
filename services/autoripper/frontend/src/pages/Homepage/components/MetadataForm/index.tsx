import { zodResolver } from '@hookform/resolvers/zod';
import { useState } from 'react';
import { useForm } from 'react-hook-form';
import { useTranslation } from 'react-i18next';
import { z } from 'zod';
import { useShallow } from 'zustand/react/shallow';

import { Button } from '$/components/common/ui/button';
import { Dialog, DialogHeader, DialogTrigger, DialogContent, DialogTitle } from '$/components/common/ui/dialog';
import { Form } from '$/components/common/ui/form';
import { MediaSelection } from '$/pages/Homepage/components/MetadataForm/components/MediaSelection';
import { MetadataButton } from '$/pages/Homepage/components/MetadataForm/components/MetadataButton';
import { MetadataFormValues, metadataFormSchema, useMediaStore } from '$/pages/Homepage/stores/useMediaStore';

import type { UseFormReturn } from 'react-hook-form';

export type MetadataFormControl = UseFormReturn<MetadataFormValues>;

export const MetadataForm = () => {
  const { t } = useTranslation();

  const setMetadata = useMediaStore(useShallow((state) => state.setMetadata));
  const [open, setOpen] = useState(false);

  const form = useForm<z.infer<typeof metadataFormSchema>>({
    resolver: zodResolver(metadataFormSchema),
    defaultValues: { type: 'movie', selectedSeason: 1, selectedEpisodes: [] },
  });

  const onSubmit = (values: MetadataFormValues) => {
    setMetadata(values);
    setOpen(false);
  };

  return (
    <Dialog open={open} onOpenChange={setOpen} modal={false}>
      <DialogTrigger asChild>
        <MetadataButton />
      </DialogTrigger>
      <DialogContent className='flex size-full max-w-full flex-col justify-center gap-16 px-32'>
        <DialogHeader>
          <DialogTitle>{t('homepage.metadata.title')}</DialogTitle>
        </DialogHeader>

        <Form {...form}>
          <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-8'>
            <MediaSelection form={form} />
            <Button type='submit' className='w-full'>
              {t('homepage.metadata.saveMetadata')}
            </Button>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
};
