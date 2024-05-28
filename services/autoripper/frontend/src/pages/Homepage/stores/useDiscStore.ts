import { createSelectorHooks, ZustandHookSelectors } from 'auto-zustand-selectors-hook';
import { create } from 'zustand';

type State = {
  selectedDevice?: string;
};

type Actions = {
  selectDevice: (device: string) => void;
};

const initial: State = {
  selectedDevice: undefined,
};

const store = create<State & Actions>((set) => ({
  ...initial,
  selectDevice: (device) => set({ selectedDevice: device }),
}));

export const useDiscStore = createSelectorHooks(store) as typeof store & ZustandHookSelectors<State & Actions>;
