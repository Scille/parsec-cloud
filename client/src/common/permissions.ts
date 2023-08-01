// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { useRoute } from 'vue-router';

export function isAdmin(): boolean {
  const currentRoute = useRoute();
  const deviceId = currentRoute.params.deviceId;
  if (deviceId) {
    return true;
  }
  return false;
}

export function isOutsider(): boolean {
  const currentRoute = useRoute();
  const deviceId = currentRoute.params.deviceId;
  if (deviceId) {
    return false;
  }
  return false;
}
