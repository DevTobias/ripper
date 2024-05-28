import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { useTranslation } from 'react-i18next';
import { z } from 'zod';

import { Button } from '$/components/common/ui/button';
import { Form } from '$/components/common/ui/form';
import { DeviceSelection } from '$/pages/Homepage/components/MetadataForm/components/DeviceSelection';

import type { UseFormReturn } from 'react-hook-form';

const formSchema = z.object({
  device: z.string().min(1, { message: 'formErrors.required' }),
});

export type MetadataFormControl = UseFormReturn<z.infer<typeof formSchema>>;

export const MetadataForm = () => {
  const { t } = useTranslation();

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: { device: '' },
  });

  const onSubmit = (values: z.infer<typeof formSchema>) => {
    console.log('submitting', values);
  };

  return (
    <div className='flex flex-col gap-3'>
      <h2 className='font-medium'>{t('homepage.metadata.title')}</h2>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)} className='space-y-8'>
          <DeviceSelection form={form} />

          <Button type='submit' className='w-full'>
            {t('homepage.metadata.saveMetadata')}
          </Button>
        </form>
      </Form>
    </div>
  );
};
