// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { useRoute, useRouter } from 'vue-router';
import { Handle } from '@/plugins/libparsec/definitions';

export function getParsecHandle(): Handle | null {
  const currentRoute = useRoute();

  return parseInt(currentRoute.params.handle as string);
}

export function isLoggedIn(): boolean {
  return getParsecHandle() !== null;
}

export function hasHistory(): boolean {
  const router = useRouter();
  const handle = getParsecHandle();

  const previousRoute = router.options.history.state.back?.toString();

  if (!handle || !previousRoute) {
    return false;
  }
  return previousRoute.startsWith(`/${handle}`);
}

export function isDocumentRoute(): boolean {
  const currentRoute = useRoute();

  return currentRoute.name === 'workspaces' || currentRoute.name === 'folder';
}

export function isOrganizationManagementRoute(): boolean {
  const currentRoute = useRoute();
  return currentRoute.name ? [
    'activeUsers',
    'revokedUsers',
    'invitations',
    'storage',
    'organization',
  ].includes(currentRoute.name.toString()) : false;
}

export function isUserRoute(): boolean {
  const currentRoute = useRoute();
  return currentRoute.name ? ['activeUsers', 'revokedUsers', 'invitations'].includes(currentRoute.name.toString()) : false;
}

export function isSpecificWorkspaceRoute(workspaceId: string): boolean {
  const currentRoute = useRoute();
  return currentRoute.params.workspaceId === workspaceId;
}
