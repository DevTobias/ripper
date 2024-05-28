import { use } from 'i18next';
import { initReactI18next } from 'react-i18next';

import { de } from '$/lib/translations/langs/de';

export const i18n = use(initReactI18next);

declare module 'i18next' {
  interface CustomTypeOptions {
    returnNull: false;
    resources: typeof de;
  }
}
