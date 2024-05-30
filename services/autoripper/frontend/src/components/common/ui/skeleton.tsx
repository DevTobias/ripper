import { HTMLAttributes, ReactNode } from 'react';

import { cn } from '$/lib/utils';

function Skeleton({ className, children, ...props }: HTMLAttributes<HTMLDivElement> & { children?: ReactNode }) {
  return (
    <div className={cn('animate-pulse rounded-md bg-muted text-transparent', className)} {...props}>
      {children}
    </div>
  );
}

export { Skeleton };
