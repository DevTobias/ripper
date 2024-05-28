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

export const randomText = (seed: number, min: number, max: number) => {
  const length = randomRange(seed, min, max);
  return repeat(length)
    .map(() => 'A')
    .join('');
};
