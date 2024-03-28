// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getCurrentRoute } from '@/router/types';
import { WatchStopHandle, watch } from 'vue';

export function watchRoute(callback: () => Promise<void>): WatchStopHandle {
  const currentRoute = getCurrentRoute();
  return watch(currentRoute, callback);
}
