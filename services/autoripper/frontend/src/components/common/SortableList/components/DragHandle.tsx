import { GripVertical } from 'lucide-react';
import { FC, useContext } from 'react';

import { SortableItemContext } from '$/components/common/SortableList/components/SortableItem';
import { Button } from '$/components/common/ui/button';

interface Props {
  disabled?: boolean;
}

export const DragHandle: FC<Props> = ({ disabled }) => {
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
      disabled={disabled}
    >
      <GripVertical className='size-4 text-neutral-400' />
    </Button>
  );
};
