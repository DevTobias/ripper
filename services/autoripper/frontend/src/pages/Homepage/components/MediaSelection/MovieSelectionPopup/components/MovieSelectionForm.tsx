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
import { MovieSelection } from '$/pages/Homepage/components/MediaSelection/MovieSelectionPopup/components/MovieSelection';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';
import { queryClient } from '$/services/fetcher';
import { SearchResultItem, getMovieDetailsQuery } from '$/services/metadata';

const movieSelectionForm = z.object({
  device: z.string().min(1, { message: 'formErrors.required' }),
  encodingProfile: z.string().min(1, { message: 'formErrors.required' }),
  qualityProfile: z.string().min(1, { message: 'formErrors.required' }),
  rootFolder: z.string().min(1, { message: 'formErrors.required' }),
  selectedMedia: z.number({ required_error: 'formErrors.required' }),
});

export type MovieSelectionFormValues = z.infer<typeof movieSelectionForm>;
export type MovieSelectionFormControl = UseFormReturn<MovieSelectionFormValues>;

interface Props {
  onSubmit?: (values: MovieSelectionFormValues) => void;
}

export const MovieSelectionForm: FC<Props> = ({ onSubmit }) => {
  const form = useForm<MovieSelectionFormValues>({ resolver: zodResolver(movieSelectionForm) });

  const onSubmitHandler = (movieSelectionValues: MovieSelectionFormValues) => {
    useMediaStore.setState({ movieSelectionValues, mediaType: 'movie' });
    onSubmit?.(movieSelectionValues);
  };

  useEffect(() => {
    return useMediaStore.subscribe(
      (state, prevState) =>
        !state.movieSelectionValues && prevState.movieSelectionValues && setTimeout(() => form.reset(), 0)
    );
  }, [form]);

  const reloadSelectionFields = () => {
    form.resetField('device');
    form.resetField('encodingProfile');
    form.resetField('qualityProfile');
    form.resetField('rootFolder');
  };

  const selectMovie = async (item: SearchResultItem | null) => {
    if (!item) return form.resetField('selectedMedia');
    form.setValue('selectedMedia', item.id);

    const selectedMovie = await queryClient.ensureQueryData(getMovieDetailsQuery({ id: item.id, lang: 'de' }));
    return useMediaStore.setState({ selectedMovie });
  };

  return (
    <Form {...form}>
      <form onSubmit={form.handleSubmit(onSubmitHandler)} className='flex flex-col gap-5'>
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
              <QualityProfileSelection onChange={onChange} hasError={!!error} value={value} ref={ref} type='movie' />
            )}
          />

          <FormField
            control={form.control}
            name='rootFolder'
            render={({ field: { onChange, value, ref }, fieldState: { error } }) => (
              <RootFolderSelection onChange={onChange} hasError={!!error} value={value} ref={ref} type='movie' />
            )}
          />
        </div>

        <FormField
          control={form.control}
          name='selectedMedia'
          render={({ field: { value }, fieldState: { error } }) => (
            <MovieSelection hasError={!!error} value={value} onChange={selectMovie} />
          )}
        />

        <div className='flex gap-2'>
          <MetadataSubmitButton />
          <ReloadButton onReload={reloadSelectionFields} type='movie' />
        </div>
      </form>
    </Form>
  );
};
