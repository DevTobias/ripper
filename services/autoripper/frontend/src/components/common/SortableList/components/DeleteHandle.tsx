import { UniqueIdentifier } from '@dnd-kit/core';
import { Trash2 } from 'lucide-react';

import { Button } from '$/components/common/ui/button';

interface BaseItem {
  id: UniqueIdentifier;
}

interface Props<T extends BaseItem> {
  id: UniqueIdentifier;
  items: T[];
  onChange(items: T[]): void;
  disabled?: boolean;
}

export const DeleteHandle = <T extends BaseItem>({ id, items, disabled, onChange }: Props<T>) => {
  return (
    <Button
      variant='ghost'
      className='p-3'
      onClick={() => {
        if (items.length === 1) return;
        onChange(items.filter((prevItem) => prevItem.id !== id));
      }}
      disabled={disabled}
    >
      <Trash2 className='size-4 text-neutral-400' />
    </Button>
  );
};
