import { useState } from 'react';

export const useArray = <T>(defaultValue: T[]) => {
  const [array, setArray] = useState<T[]>(defaultValue);

  const push = (element: T) => {
    setArray((a) => [...a, element]);
  };

  const filter = (callback: (value: T, index: number, array: T[]) => boolean) => {
    setArray((a) => a.filter(callback));
  };

  const update = (index: number, newElement: T) => {
    setArray((a) => [...a.slice(0, index), newElement, ...a.slice(index + 1)]);
  };

  const remove = (index: number) => {
    setArray((a) => [...a.slice(0, index), ...a.slice(index + 1)]);
  };

  const clear = () => {
    setArray([]);
  };

  const move = (fromIndex: number, toIndex: number) => {
    setArray((a) => {
      const newArray = [...a];
      const temp = newArray[fromIndex];
      newArray[fromIndex] = newArray[toIndex];
      newArray[toIndex] = temp;
      return newArray;
    });
  };

  return { array, set: setArray, push, filter, update, remove, clear, move };
};
