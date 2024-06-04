import { Ban, Info, Loader, Save } from 'lucide-react';
import { useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { useShallow } from 'zustand/react/shallow';

import { SortableList } from '$/components/common/SortableList';
import { DeleteHandle } from '$/components/common/SortableList/components/DeleteHandle';
import { DragHandle } from '$/components/common/SortableList/components/DragHandle';
import { SortableItem } from '$/components/common/SortableList/components/SortableItem';
import { Button } from '$/components/common/ui/button';
import { Popover, PopoverContent, PopoverTrigger } from '$/components/common/ui/popover';
import { repeat } from '$/lib/utils';
import { LoadingTitleCard } from '$/pages/Homepage/components/TitleSelectionList/components/LoadingTitleCard';
import { TitleCard } from '$/pages/Homepage/components/TitleSelectionList/components/TitleCard';
import { useMediaStore } from '$/pages/Homepage/stores/useMediaStore';
import { queryClient } from '$/services/fetcher';
import { Title, movieDiscPropertiesQuery, tvShowDiscPropertiesQuery } from '$/services/properties';

export const TitleSelectionList = () => {
  const { t } = useTranslation();

  const rippingInProgress = useMediaStore(useShallow((state) => state.rippingInProgress));
  const metadata = useMediaStore(useShallow((state) => state.metadata));

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

  const startRipper = () => {
    useMediaStore.setState({
      rippingInProgress: true,
      selectedTitles: items.map((item) => item.id),
      rippingProgress: { progress: 0, step: 0, stepDetails: 'Disc einlesen', stepTitle: 'Lese Disc Daten' },
    });
  };

  const stopRipper = () => {
    useMediaStore.getState().sendWebsocketMessage?.('cancel');
    useMediaStore.setState({
      rippingInProgress: false,
      selectedTitles: [],
      rippingProgress: { progress: 0, step: 0, stepDetails: '', stepTitle: '' },
    });
  };

  const savingEnabled =
    (metadata?.type === 'movie' && items.length === 1) ||
    (metadata?.type === 'tv_show' && items.length === metadata.selectedEpisodes.length);

  return (
    <div className='flex w-full flex-col gap-2 '>
      <div className='flex w-full gap-2'>
        <Button className='flex w-full gap-2' disabled={isLoading || !metadata || rippingInProgress} onClick={loadItems}>
          <span>{t('homepage.titleSelection.scanDisc')}</span>
          {isLoading && <Loader className='size-4 animate-spin' />}
        </Button>

        {!rippingInProgress && (
          <Button className='aspect-square p-0' disabled={!savingEnabled} onClick={startRipper}>
            <Save className='size-4' />
          </Button>
        )}

        {rippingInProgress && (
          <Button className='aspect-square p-0' onClick={stopRipper}>
            <Ban className='size-4' />
          </Button>
        )}

        <Popover>
          <PopoverTrigger asChild>
            <Button className='aspect-square p-0' variant='outline'>
              <Info className='size-4 text-slate-500' />
            </Button>
          </PopoverTrigger>
          <PopoverContent
            className='w-96 text-sm'
            dangerouslySetInnerHTML={{ __html: t('homepage.titleSelection.description') }}
          />
        </Popover>
      </div>
      {items.length === 0 && repeat(2).map((i) => <LoadingTitleCard key={i} id={i + 2} />)}
      <div className='max-h-[355px] overflow-y-auto'>
        <SortableList
          items={items}
          onChange={setItems}
          renderItem={(item) => (
            <SortableItem id={item.id.toString()} className='flex h-fit py-3'>
              <TitleCard title={item} />
              <div className='flex'>
                <DeleteHandle id={item.id.toString()} items={items} onChange={setItems} disabled={rippingInProgress} />
                <DragHandle disabled={rippingInProgress} />
              </div>
            </SortableItem>
          )}
        />
      </div>
    </div>
  );
};
