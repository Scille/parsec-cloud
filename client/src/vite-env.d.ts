// Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS

// eslint-disable-next-line spaced-comment
/// <reference types="vite/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue';
  // eslint-disable-next-line @typescript-eslint/ban-types
  const component: DefineComponent<{}, {}, any>;
  export default component;
}

declare module '@libparsec_trampoline' {
  export function LoadWebLibParsecPlugin(): Promise<any>;
}

declare const __APP_VERSION__: string;
