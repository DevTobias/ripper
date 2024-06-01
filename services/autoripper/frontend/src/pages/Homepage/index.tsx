import { ProcessProgress } from '$/pages/Homepage/components/ProcessProgress';
import { SettingsForm } from '$/pages/Homepage/components/SettingsForm';

export const Homepage = () => {
  return (
    <div className='container size-full px-32 py-10'>
      <div className='grid h-full grid-rows-[auto,minmax(0,_1fr)]'>
        <div className='flex flex-col gap-4'>
          <SettingsForm />
        </div>
        <div className='flex size-full items-center justify-center'>
          <ProcessProgress />
        </div>
      </div>
    </div>
  );
};
