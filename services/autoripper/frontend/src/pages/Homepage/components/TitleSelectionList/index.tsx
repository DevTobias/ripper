import { Loader, Save } from 'lucide-react';
import { useEffect, useState } from 'react';
import { useShallow } from 'zustand/react/shallow';

import { SortableList } from '$/components/common/SortableList';
import { DeleteHandle } from '$/components/common/SortableList/components/DeleteHandle';
import { DragHandle } from '$/components/common/SortableList/components/DragHandle';
import { SortableItem } from '$/components/common/SortableList/components/SortableItem';
import { Button } from '$/components/common/ui/button';
import { repeat } from '$/lib/utils';
import { LoadingTitleCard } from '$/pages/Homepage/components/TitleSelectionList/components/LoadingTitleCard';
import { TitleCard } from '$/pages/Homepage/components/TitleSelectionList/components/TitleCard';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';
import { queryClient } from '$/services/fetcher';
import { Title, movieDiscPropertiesQuery, tvShowDiscPropertiesQuery } from '$/services/properties';

export const TitleSelectionList = () => {
  const metadata = useMediaStore(useShallow((state) => state.metadata));
  const setSelectedTitles = useMediaStore(useShallow((state) => state.setSelectedTitles));

  const [isLoading, setIsLoading] = useState(false);
  const [items, setItems] = useState<Title[]>([]);

  const loadItems = async () => {
    if (!metadata) return;

    setIsLoading(true);
    setItems([]);

    if (metadata.type === 'tv_show') {
      const data = await queryClient
        .ensureQueryData(
          tvShowDiscPropertiesQuery({
            langs: ['deu'],
            id: metadata.selectedMedia.id,
            device: metadata.device,
            episodes: metadata.selectedEpisodes,
            season: metadata.selectedSeason,
          })
        )
        .finally(() => setIsLoading(false));

      setItems(data.titles);
    }

    if (metadata.type === 'movie') {
      const data = await queryClient
        .ensureQueryData(
          movieDiscPropertiesQuery({
            langs: ['deu'],
            id: metadata.selectedMedia.id,
            device: metadata.device,
          })
        )
        .finally(() => setIsLoading(false));

      setItems(data.titles);
    }
  };

  useEffect(() => {
    return useMediaStore.subscribe((curr, prev) => {
      if (curr.metadata !== prev.metadata) {
        setItems([]);
      }
    });
  }, []);

  return (
    <div className='flex w-full flex-col gap-2 '>
      <div className='flex w-full gap-2'>
        <Button className='flex w-full gap-2' disabled={isLoading || !metadata} onClick={loadItems}>
          <span>Disc scannen</span>
          {isLoading && <Loader className='size-4 animate-spin' />}
        </Button>
        <Button
          className='aspect-square p-0'
          disabled={items.length === 0}
          onClick={() => setSelectedTitles(items.map((item) => item.id))}
        >
          <Save className='size-4' />
        </Button>
      </div>
      {items.length === 0 && repeat(2).map((i) => <LoadingTitleCard key={i} id={i + 2} />)}
      <SortableList
        items={items}
        onChange={setItems}
        renderItem={(item) => (
          <SortableItem id={item.id.toString()} className='flex h-fit py-3'>
            <TitleCard title={item} />
            <div className='flex'>
              <DeleteHandle id={item.id.toString()} items={items} onChange={setItems} />
              <DragHandle />
            </div>
          </SortableItem>
        )}
      />
    </div>
  );
};
