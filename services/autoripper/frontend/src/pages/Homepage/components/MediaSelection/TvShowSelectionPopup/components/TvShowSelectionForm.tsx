import { zodResolver } from '@hookform/resolvers/zod';
import { FC, useEffect } from 'react';
import { UseFormReturn, useForm } from 'react-hook-form';
import { z } from 'zod';

import { Form, FormField } from '$/components/common/ui/form';
import { DeviceSelection } from '$/pages/Homepage/components/GenericFormItems/DeviceSelection';
import { EncodingProfileSelection } from '$/pages/Homepage/components/GenericFormItems/EncodingProfileSelection';
import { QualityProfileSelection } from '$/pages/Homepage/components/GenericFormItems/QualityProfileSelection';
import { RootFolderSelection } from '$/pages/Homepage/components/GenericFormItems/RootFolderSelection';
import { MetadataSubmitButton } from '$/pages/Homepage/components/MediaSelection/components/MetadataSubmitButton';
import { ReloadButton } from '$/pages/Homepage/components/MediaSelection/components/ReloadButton';
import { EpisodeSelection } from '$/pages/Homepage/components/MediaSelection/TvShowSelectionPopup/components/EpisodeSelection';
import { SeriesTypeSelection } from '$/pages/Homepage/components/MediaSelection/TvShowSelectionPopup/components/SeriesTypeSelection';
import { TvShowSelection } from '$/pages/Homepage/components/MediaSelection/TvShowSelectionPopup/components/TvShowSelection';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';
import { queryClient } from '$/services/fetcher';
import { SearchResultItem, getTvDetailsQuery } from '$/services/metadata';

const tvShowSelectionForm = z.object({
  device: z.string().min(1, { message: 'formErrors.required' }),
  encodingProfile: z.string().min(1, { message: 'formErrors.required' }),
  qualityProfile: z.string().min(1, { message: 'formErrors.required' }),
  rootFolder: z.string().min(1, { message: 'formErrors.required' }),
  selectedMedia: z.number({ required_error: 'formErrors.required' }),
  selectedSeason: z.number(),
  selectedEpisodes: z.array(z.number()),
  seriesType: z.enum(['standard', 'anime']),
});

export type TvShowSelectionFormValues = z.infer<typeof tvShowSelectionForm>;
export type TvShowSelectionFormControl = UseFormReturn<TvShowSelectionFormValues>;

interface Props {
  onSubmit?: (values: TvShowSelectionFormValues) => void;
}

export const TvShowSelectionForm: FC<Props> = ({ onSubmit }) => {
  const form = useForm<TvShowSelectionFormValues>({
    resolver: zodResolver(tvShowSelectionForm),
    defaultValues: { selectedSeason: 1, selectedEpisodes: [] },
  });

  const onSubmitHandler = (tvShowSelectionValues: TvShowSelectionFormValues) => {
    useMediaStore.setState({ tvShowSelectionValues, mediaType: 'tv_show' });
    onSubmit?.(tvShowSelectionValues);
  };

  useEffect(() => {
    return useMediaStore.subscribe(
      (state, prevState) =>
        !state.tvShowSelectionValues && prevState.tvShowSelectionValues && setTimeout(() => form.reset(), 0)
    );
  }, [form]);

  const reloadSelectionFields = () => {
    form.resetField('device');
    form.resetField('encodingProfile');
    form.resetField('qualityProfile');
    form.resetField('rootFolder');
    form.resetField('seriesType');
  };

  const selectMovie = async (item: SearchResultItem | null) => {
    if (!item) return form.resetField('selectedMedia');
    form.setValue('selectedMedia', item.id);

    const selectedTvShow = await queryClient.ensureQueryData(getTvDetailsQuery({ id: item.id, lang: 'de' }));
    return useMediaStore.setState({ selectedTvShow });
  };

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmitHandler)} className='flex flex-col gap-3'>
        <div className='flex items-end gap-2'>
          <FormField
            control={form.control}
            name='device'
            render={({ field, fieldState }) => (
              <DeviceSelection hasError={!!fieldState.error} onChange={field.onChange} value={field.value} ref={field.ref} />
            )}
          />

          <FormField
            control={form.control}
            name='encodingProfile'
            render={({ field: { onChange, value, ref }, fieldState: { error } }) => (
              <EncodingProfileSelection onChange={onChange} hasError={!!error} value={value} ref={ref} />
            )}
          />
        </div>

        <div className='flex items-end gap-2'>
          <FormField
            control={form.control}
            name='qualityProfile'
            render={({ field: { onChange, value, ref }, fieldState: { error } }) => (
              <QualityProfileSelection onChange={onChange} hasError={!!error} value={value} ref={ref} type='tv_show' />
            )}
          />

          <FormField
            control={form.control}
            name='rootFolder'
            render={({ field: { onChange, value, ref }, fieldState: { error } }) => (
              <RootFolderSelection onChange={onChange} hasError={!!error} value={value} ref={ref} type='tv_show' />
            )}
          />

          <FormField
            control={form.control}
            name='seriesType'
            render={({ field: { onChange, value, ref }, fieldState: { error } }) => (
              <SeriesTypeSelection onChange={onChange} hasError={!!error} value={value} ref={ref} />
            )}
          />
        </div>

        <FormField
          control={form.control}
          name='selectedMedia'
          render={({ field: { value }, fieldState: { error } }) => (
            <TvShowSelection hasError={!!error} value={value} onChange={selectMovie} />
          )}
        />

        <EpisodeSelection form={form} />

        <div className='flex gap-2'>
          <MetadataSubmitButton />
          <ReloadButton onReload={reloadSelectionFields} type='tv_show' />
        </div>
      </form>
    </Form>
  );
};
