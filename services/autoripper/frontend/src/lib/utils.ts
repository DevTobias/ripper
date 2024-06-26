import { type ClassValue, clsx } from 'clsx';
import { twMerge } from 'tailwind-merge';

export const cn = (...inputs: ClassValue[]) => {
  return twMerge(clsx(inputs));
};

export const repeat = (n: number) => {
  return [...Array<number>(n).keys()];
};

export const randomRange = (seed: number, min: number, max: number) => {
  const rand = () => {
    const x = Math.sin(seed + 1) * 10000;
    return x - Math.floor(x);
  };

  return Math.round(rand() * (max - min) + min);
};

export const random = (min: number, max: number) => {
  return Math.floor(Math.random() * (max - min + 1) + min);
};

export const randomText = (seed: number, min: number, max: number) => {
  const length = randomRange(seed, min, max);

  return repeat(length)
    .map(() => (randomRange(seed, 0, 100) < 25 ? 'A ' : 'A'))
    .join('');
};
