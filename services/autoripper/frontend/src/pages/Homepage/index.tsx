import { ProcessProgress } from '$/pages/Homepage/components/ProcessProgress';
import { SettingsForm } from '$/pages/Homepage/components/SettingsForm';
import { TitleSelectionList } from '$/pages/Homepage/components/TitleSelectionList';

export const Homepage = () => {
  return (
    <div className='container size-full px-32 py-10'>
      <div className='grid h-full grid-rows-[auto,minmax(0,_1fr)]'>
        <div className='flex flex-col gap-4'>
          <SettingsForm />
        </div>

        <div className='grid h-full grid-cols-2 '>
          <div className='flex size-full items-center justify-center'>
            <TitleSelectionList />
          </div>
          <div className='flex size-full items-center justify-center'>
            <ProcessProgress />
          </div>
        </div>
      </div>
    </div>
  );
};
