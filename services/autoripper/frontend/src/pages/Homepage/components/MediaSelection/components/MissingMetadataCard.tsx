import { SquarePlus } from 'lucide-react';
import { forwardRef } from 'react';
import { useTranslation } from 'react-i18next';

import { ButtonProps } from '$/components/common/ui/button';

export const MissingMetadataCard = forwardRef<HTMLButtonElement, ButtonProps>(function MissingMetadataCard(props, ref) {
  const { t } = useTranslation();

  return (
    <button
      {...props}
      ref={ref}
      className='flex h-[90px] w-full items-center overflow-hidden rounded-[4px] border text-left transition-colors hover:bg-slate-50'
    >
      <div className='aspect-[2/3] h-full border-r'>
        <div className='flex size-full items-center justify-center bg-slate-200 text-slate-600 dark:bg-slate-700 dark:text-slate-300'>
          <SquarePlus />
        </div>
      </div>
      <div className='flex w-full flex-col gap-1 p-3'>
        <div className='flex w-full gap-2'>
          <span className='max-w-[70%] truncate text-sm font-medium'>{t('genericSelection.selectMetadataTitle')}</span>
        </div>
        <div className='line-clamp-2 text-sm text-slate-500 dark:text-slate-400'>
          {t('genericSelection.selectMetadataDescription')}
        </div>
      </div>
    </button>
  );
});
