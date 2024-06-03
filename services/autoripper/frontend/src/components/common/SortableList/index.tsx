import { DndContext, KeyboardSensor, MeasuringStrategy, PointerSensor, useSensor, useSensors } from '@dnd-kit/core';
import { SortableContext, arrayMove, sortableKeyboardCoordinates } from '@dnd-kit/sortable';
import { Fragment, useMemo, useState } from 'react';

import { SortableOverlay } from '$/components/common/SortableList/components/SortableOverlay';

import type { Active, UniqueIdentifier } from '@dnd-kit/core';
import type { ReactNode } from 'react';

interface BaseItem {
  id: UniqueIdentifier;
}

interface Props<T extends BaseItem> {
  items: T[];
  onChange(items: T[]): void;
  renderItem(item: T): ReactNode;
}

export const SortableList = <T extends BaseItem>({ items, onChange, renderItem }: Props<T>) => {
  const [active, setActive] = useState<Active | null>(null);
  const activeItem = useMemo(() => items.find((item) => item.id === active?.id), [active, items]);

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    })
  );

  return (
    <DndContext
      measuring={{ droppable: { strategy: MeasuringStrategy.Always } }}
      sensors={sensors}
      onDragStart={({ active: newActive }) => {
        setActive(newActive);
      }}
      onDragEnd={({ active: newActive, over }) => {
        if (over && newActive.id !== over?.id) {
          const activeIndex = items.findIndex(({ id }) => id === newActive.id);
          const overIndex = items.findIndex(({ id }) => id === over.id);

          onChange(arrayMove(items, activeIndex, overIndex));
        }
        setActive(null);
      }}
      onDragCancel={() => {
        setActive(null);
      }}
    >
      <SortableContext items={items}>
        <ul role='application' className='flex w-full list-none flex-col gap-2'>
          {items.map((item) => (
            <Fragment key={item.id}>{renderItem(item)}</Fragment>
          ))}
        </ul>
      </SortableContext>
      <SortableOverlay>{activeItem ? renderItem(activeItem) : null}</SortableOverlay>
    </DndContext>
  );
};
