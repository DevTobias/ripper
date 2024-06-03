import { GripVertical } from 'lucide-react';
import { useContext } from 'react';

import { SortableItemContext } from '$/components/common/SortableList/components/SortableItem';
import { Button } from '$/components/common/ui/button';

export const DragHandle = () => {
  const { attributes, listeners, ref } = useContext(SortableItemContext);

  return (
    <Button
      {...attributes}
      {...listeners}
      ref={ref}
      type='button'
      className='touch-none p-3'
      data-vaul-no-drag
      variant='ghost'
    >
      <GripVertical className='size-4 text-neutral-400' />
    </Button>
  );
};
