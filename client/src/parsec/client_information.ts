// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { UserProfile } from '@/parsec/types';
import { getClientInfo } from '@/parsec/functions';

export async function getClientProfile(): Promise<UserProfile | null> {
  const result = await getClientInfo();

  if (result.ok) {
    return result.value.currentProfile;
  } else {
    return null;
  }
}

export async function isAdmin(): Promise<boolean> {
  return await getClientProfile() === UserProfile.Admin;
}

export async function isOutsider(): Promise<boolean> {
  return await getClientProfile() === UserProfile.Outsider;
}
