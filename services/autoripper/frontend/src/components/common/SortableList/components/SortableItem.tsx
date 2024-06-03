/* eslint-disable @typescript-eslint/no-explicit-any */

import { AnimateLayoutChanges, defaultAnimateLayoutChanges, useSortable } from '@dnd-kit/sortable';
import { CSS } from '@dnd-kit/utilities';
import { createContext, useMemo } from 'react';

import { cn } from '$/lib/utils';

import type { DraggableSyntheticListeners, UniqueIdentifier } from '@dnd-kit/core';
import type { CSSProperties, FC, ReactNode } from 'react';

interface Props {
  children: ReactNode;
  id: UniqueIdentifier;
  className?: string;
}

interface Context {
  attributes: Record<string, any>;
  listeners: DraggableSyntheticListeners;
  ref(node: HTMLElement | null): void;
}

export const SortableItemContext = createContext<Context>({
  attributes: {},
  listeners: undefined,
  ref() {},
});

export const SortableItem: FC<Props> = ({ children, className, id }) => {
  const animateLayoutChanges: AnimateLayoutChanges = (args) => defaultAnimateLayoutChanges({ ...args, wasDragging: true });

  const { attributes, isDragging, listeners, setNodeRef, setActivatorNodeRef, transform, transition } = useSortable({
    id,
    animateLayoutChanges,
  });

  const context = useMemo(
    () => ({
      attributes,
      listeners,
      ref: setActivatorNodeRef,
    }),
    [attributes, listeners, setActivatorNodeRef]
  );

  const style: CSSProperties = {
    opacity: isDragging ? 0.4 : undefined,
    transform: CSS.Translate.toString(transform),
    transition,
  };

  return (
    <SortableItemContext.Provider value={context}>
      <li
        className={cn('flex items-center justify-between rounded-md border bg-white px-4 py-1 shadow-sm', className)}
        ref={setNodeRef}
        style={style}
        data-vaul-no-drag
      >
        {children}
      </li>
    </SortableItemContext.Provider>
  );
};
