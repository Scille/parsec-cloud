// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { useRoute, useRouter } from 'vue-router';

export function isLoggedIn(): boolean {
  const currentRoute = useRoute();
  const deviceId = currentRoute.params.deviceId;

  if (!deviceId) {
    return false;
  }
  return true;
}

export function hasHistory(): boolean {
  const currentRoute = useRoute();
  const router = useRouter();

  const deviceId = currentRoute.params.deviceId;
  const previousRoute = router.options.history.state.back?.toString();

  if (!deviceId || !previousRoute) {
    return false;
  }
  return previousRoute.startsWith(`/${deviceId}`);
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
