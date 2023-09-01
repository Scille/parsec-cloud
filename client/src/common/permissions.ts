// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getParsecHandle } from '@/router/conditions';

export function isAdmin(): boolean {
  const handle = getParsecHandle();

  return handle !== null;
}

export function isOutsider(): boolean {
  // const handle = getParsecHandle();
  return false;
}
