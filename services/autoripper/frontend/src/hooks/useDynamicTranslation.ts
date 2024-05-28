import { useTranslation } from 'react-i18next';

export const useDynamicTranslation = () => {
  const { t } = useTranslation();

  // eslint-disable-next-line @typescript-eslint/no-unsafe-return, @typescript-eslint/no-unsafe-argument, @typescript-eslint/no-explicit-any
  return (key: string): string => t(key as any);
};
