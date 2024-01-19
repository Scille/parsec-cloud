// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getCurrentRoute } from '@/router/types';
import { Ref, WatchStopHandle, ref, watch } from 'vue';

export const organizationKey: Ref<number> = ref(0);

export function watchRoute(callback: () => Promise<void>): WatchStopHandle {
  const currentRoute = getCurrentRoute();
  return watch(currentRoute, callback);
}

export function watchOrganizationSwitch(callback: () => Promise<void>): WatchStopHandle {
  return watch(organizationKey, callback);
}
