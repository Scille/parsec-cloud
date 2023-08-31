// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { getParsecHandle } from '@/router/conditions';
import { getCurrentUserProfile, Profile } from '@/common/mocks';

export function isAdmin(): boolean {
  const handle = getParsecHandle();
  return handle !== null && getCurrentUserProfile(handle) === Profile.Admin;
}

export function isOutsider(): boolean {
  const handle = getParsecHandle();
  return handle !== null && getCurrentUserProfile(handle) === Profile.Outsider;
}
