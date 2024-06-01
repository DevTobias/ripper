import { RadialProgressBar } from '$/components/common/RadialProgressBar';
import { MetadataForm } from '$/pages/Homepage/components/MetadataForm';
import { RipperForm } from '$/pages/Homepage/components/RipperForm';

export const Homepage = () => {
  return (
    <div className='container size-full px-32 py-10'>
      <div className='grid h-full grid-rows-[auto,minmax(0,_1fr)]'>
        <div className='flex flex-col gap-4'>
          <MetadataForm />
          <RipperForm />
        </div>
        <div className='flex size-full items-center justify-center'>
          <RadialProgressBar
            progress={20}
            size={200}
            strokeWidth={20}
            ringClassName='text-neutral-900'
            bgClassName='text-neutral-200'
          >
            <div></div>
          </RadialProgressBar>
        </div>
      </div>
    </div>
  );
};

// <Disc3 className='size-16 animate-spin text-neutral-900' />
