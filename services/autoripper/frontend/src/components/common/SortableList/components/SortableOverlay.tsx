import { DragOverlay, defaultDropAnimationSideEffects } from '@dnd-kit/core';

import type { DropAnimation } from '@dnd-kit/core';
import type { FC, ReactNode } from 'react';

const dropAnimationConfig: DropAnimation = {
  sideEffects: defaultDropAnimationSideEffects({
    styles: { active: { opacity: '0.4' } },
  }),
};

interface Props {
  children: ReactNode;
}

export const SortableOverlay: FC<Props> = ({ children }) => {
  return (
    <DragOverlay dropAnimation={dropAnimationConfig} adjustScale>
      {children}
    </DragOverlay>
  );
};
