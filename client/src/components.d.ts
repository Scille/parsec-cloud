// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// eslint-disable-next-line @typescript-eslint/no-unused-vars
import { ComponentCustomProperties } from 'vue';
import { Translatable } from '@/services/translation';

declare module '@vue/runtime-core' {
  interface ComponentCustomProperties {
    $msTranslate: (translatable: Translatable) => string;
  }
}
