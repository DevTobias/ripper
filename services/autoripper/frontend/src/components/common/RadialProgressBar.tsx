import { ReactNode } from 'react';

import { cn } from '$/lib/utils';

interface Props {
  progress: number;
  size?: number;
  strokeWidth?: number;
  ringClassName?: string;
  bgClassName?: string;
  children?: ReactNode;
  className?: string;
}

export const RadialProgressBar = ({
  progress,
  children,
  size = 100,
  strokeWidth = 10,
  ringClassName = 'text-indigo-500 dark:text-indigo-400',
  bgClassName = 'text-gray-200 dark:text-gray-600',
  className,
}: Props) => {
  const radius = size / 2 - 10;
  const circumference = 2 * Math.PI * radius;
  const offset = circumference - (progress / 100) * circumference;

  return (
    <div className={cn('relative w-fit', className)}>
      <svg height={size} width={size} className='block -rotate-90'>
        <circle
          className={bgClassName}
          stroke='currentColor'
          fill='transparent'
          strokeWidth={strokeWidth}
          r={radius}
          cx={size / 2}
          cy={size / 2}
        />
        <circle
          className={cn('transition-[stroke-dashoffset]', ringClassName)}
          stroke='currentColor'
          strokeLinecap='round'
          fill='transparent'
          strokeWidth={strokeWidth}
          r={radius}
          cx={size / 2}
          cy={size / 2}
          strokeDasharray={circumference}
          strokeDashoffset={offset}
        />
      </svg>
      <div className='absolute left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2'>
        {children ?? <span>{progress}%</span>}
      </div>
    </div>
  );
};
