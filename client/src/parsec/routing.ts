// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { Handle, WorkspaceHandle } from '@/parsec';
import AppManager from '@/services/appManager';

export function getParsecHandle(): Handle | null {
  return 42;
  // return AppManager.get().getCurrentHandle();
  // const currentRoute = router.currentRoute.value;

  // if (currentRoute && currentRoute.params && 'handle' in currentRoute.params) {
  //   return parseInt(currentRoute.params.handle as string);
  // }
  // return null;
}

export function getWorkspaceHandle(): WorkspaceHandle | null {
  return 42;
  // const currentRoute = router.currentRoute.value;

  // if (currentRoute && currentRoute.name === 'folder' && currentRoute.params && 'workspaceHandle' in currentRoute.params) {
  //   return parseInt(currentRoute.params.workspaceHandle as string);
  // }
  // return null;
}

export function isLoggedIn(): boolean {
  return getParsecHandle() !== null;
}

export function hasHistory(): boolean {
  return true;
  // const handle = getParsecHandle();

  // const previousRoute = router.options.history.state.back?.toString();

  // if (!handle || !previousRoute) {
  //   return false;
  // }
  // return previousRoute.startsWith(`/${handle}`);
}

export function isHomeRoute(): boolean {
  return false;
  //   const currentRoute = router.currentRoute.value;
//   return currentRoute.name === 'home';
}

export function isDocumentRoute(): boolean {
  return true;
  // const currentRoute = router.currentRoute.value;

  // return currentRoute.name === 'workspaces' || currentRoute.name === 'folder';
}

export function isOrganizationManagementRoute(): boolean {
  return false;
  // const currentRoute = router.currentRoute.value;
  // return currentRoute.name
  //   ? ['activeUsers', 'revokedUsers', 'invitations', 'storage', 'organization'].includes(currentRoute.name.toString())
  //   : false;
}

export function isRoute(name: string): boolean {
  return false;
  // const currentRoute = router.currentRoute.value;
  // return currentRoute.name === name;
}

export function isUserRoute(): boolean {
  return false;
  // const currentRoute = router.currentRoute.value;
  // return currentRoute.name ? ['activeUsers', 'revokedUsers', 'invitations'].includes(currentRoute.name.toString()) : false;
}

export function isSpecificWorkspaceRoute(workspaceId: string): boolean {
  return false;
  // const currentRoute = router.currentRoute.value;
  // return currentRoute.query && currentRoute.query.workspaceId === workspaceId;
}
