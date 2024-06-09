import { MediaSelection } from '$/pages/Homepage/components/MediaSelection';
import { ProcessProgress } from '$/pages/Homepage/components/ProcessProgress';
import { TitleSelectionList } from '$/pages/Homepage/components/TitleSelectionList';

export const Homepage = () => {
  return (
    <div className='container size-full px-32 py-10'>
      <div className='grid h-full grid-rows-[auto,minmax(0,_1fr)]'>
        <MediaSelection />

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
