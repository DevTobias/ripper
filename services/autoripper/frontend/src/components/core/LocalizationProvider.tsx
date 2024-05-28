import { FC, ReactElement } from 'react';

import { i18n } from '$/lib/translations';
import { de } from '$/lib/translations/langs/de';

interface Props {
  children: ReactElement;
}

export const LocalizationProvider: FC<Props> = ({ children }) => {
  if (!i18n.isInitialized) {
    void i18n.init({ lng: 'de', debug: true, resources: { de } });
  }

  return children;
};
