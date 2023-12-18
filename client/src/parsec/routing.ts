// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Handle, WorkspaceHandle } from '@/parsec';
import router from '@/router';

export function getParsecHandle(): Handle | null {
  const currentRoute = router.currentRoute.value;

  if (currentRoute && currentRoute.params && 'handle' in currentRoute.params) {
    return parseInt(currentRoute.params.handle as string);
  }
  return null;
}

export function getWorkspaceHandle(): WorkspaceHandle | null {
  const currentRoute = router.currentRoute.value;

  if (currentRoute && currentRoute.name === 'folder' && currentRoute.params && 'workspaceHandle' in currentRoute.params) {
    return parseInt(currentRoute.params.workspaceHandle as string);
  }
  return null;
}

export function isLoggedIn(): boolean {
  return getParsecHandle() !== null;
}

export function hasHistory(): boolean {
  const handle = getParsecHandle();

  const previousRoute = router.options.history.state.back?.toString();

  if (!handle || !previousRoute) {
    return false;
  }
  return previousRoute.startsWith(`/${handle}`);
}

export function isHomeRoute(): boolean {
  const currentRoute = router.currentRoute.value;
  return currentRoute.name === 'home';
}

export function isDocumentRoute(): boolean {
  const currentRoute = router.currentRoute.value;

  return currentRoute.name === 'workspaces' || currentRoute.name === 'folder';
}

export function isOrganizationManagementRoute(): boolean {
  const currentRoute = router.currentRoute.value;
  return currentRoute.name
    ? ['activeUsers', 'revokedUsers', 'invitations', 'storage', 'organization'].includes(currentRoute.name.toString())
    : false;
}

export function isRoute(name: string): boolean {
  const currentRoute = router.currentRoute.value;
  return currentRoute.name === name;
}

export function isUserRoute(): boolean {
  const currentRoute = router.currentRoute.value;
  return currentRoute.name ? ['activeUsers', 'revokedUsers', 'invitations'].includes(currentRoute.name.toString()) : false;
}

export function isSpecificWorkspaceRoute(workspaceId: string): boolean {
  const currentRoute = router.currentRoute.value;
  return currentRoute.query && currentRoute.query.workspaceId === workspaceId;
}
