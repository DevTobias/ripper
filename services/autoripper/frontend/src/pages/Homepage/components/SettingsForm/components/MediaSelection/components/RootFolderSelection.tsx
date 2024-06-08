import { useQuery } from '@tanstack/react-query';
import { FC, useEffect } from 'react';
import { useTranslation } from 'react-i18next';
import { useShallow } from 'zustand/react/shallow';

import { FormControl, FormField, FormItem, FormLabel } from '$/components/common/ui/form';
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '$/components/common/ui/select';
import { cn } from '$/lib/utils';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';
import { rootFoldersQuery } from '$/services/management';

import type { MetadataFormControl } from '$/pages/Homepage/components/SettingsForm';

interface Props {
  form: MetadataFormControl;
}

export const RootFolderSelection: FC<Props> = ({ form }) => {
  const { t } = useTranslation();
  const { data, isLoading, isRefetching } = useQuery(rootFoldersQuery({ media_type: form.getValues('type') }));

  const rippingInProgress = useMediaStore(useShallow((state) => state.rippingInProgress));

  const loading = isLoading || isRefetching;

  useEffect(() => {
    if ((data?.length ?? 0) > 0 && !form.getValues('rootFolder')) form.setValue('rootFolder', data![0].path);
  }, [data, form]);

  return (
    <FormField
      control={form.control}
      name='rootFolder'
      render={({ field, fieldState }) => (
        <FormItem className='w-full'>
          <FormLabel className='flex items-center gap-1 text-sm'>
            <span>{t('homepage.rootFolder.title')}</span>
          </FormLabel>
          <Select disabled={loading || rippingInProgress} onValueChange={field.onChange} value={field.value}>
            <FormControl>
              <div className='flex gap-3'>
                <SelectTrigger
                  className={cn('w-full', fieldState.error && 'border-red-500')}
                  isLoading={loading}
                  ref={field.ref}
                >
                  <SelectValue placeholder={t('homepage.rootFolder.placeholder')} />
                </SelectTrigger>
              </div>
            </FormControl>
            <SelectContent>
              {data?.map((profile) => (
                <SelectItem key={profile.id} value={profile.path} className='cursor-pointer'>
                  <span>{profile.path}</span>
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </FormItem>
      )}
    />
  );
};
