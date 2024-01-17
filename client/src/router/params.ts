// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ConnectionHandle, WorkspaceHandle, WorkspaceID } from '@/parsec';
import { Routes, getRouter } from '@/router/types';

export function getConnectionHandle(): ConnectionHandle | null {
  const router = getRouter();
  const currentRoute = router.currentRoute.value;

  if (currentRoute && currentRoute.params && 'handle' in currentRoute.params) {
    return parseInt(currentRoute.params.handle as string) as ConnectionHandle;
  }
  return null;
}

export function getWorkspaceHandle(): WorkspaceHandle {
  const router = getRouter();
  const currentRoute = router.currentRoute.value;

  if (currentRoute && currentRoute.name === Routes.Documents && currentRoute.params && 'workspaceHandle' in currentRoute.params) {
    return parseInt(currentRoute.params.workspaceHandle as string) as WorkspaceHandle;
  }
  return -1;
}

export function getWorkspaceId(): WorkspaceID {
  const router = getRouter();
  const currentRoute = router.currentRoute.value;

  if (currentRoute && currentRoute.name === Routes.Documents && currentRoute.query && 'workspaceId' in currentRoute.query) {
    return currentRoute.query.workspaceId as WorkspaceID;
  }
  return '';
}

export function getRoutePath(): string {
  const router = getRouter();

  const currentRoute = router.currentRoute.value;
  if (currentRoute) {
    return currentRoute.path;
  }
  return '';
}

export function getDocumentPath(): string {
  const router = getRouter();
  const currentRoute = router.currentRoute.value;
  if (currentRoute && currentRoute.query && 'path' in currentRoute.query) {
    return currentRoute.query.path as string;
  }
  return '';
}

export function getCurrentRouteParams(): object {
  const router = getRouter();
  const currentRoute = router.currentRoute.value;
  if (currentRoute) {
    return currentRoute.params;
  }
  return {};
}

export function getCurrentRouteQuery(): object {
  const router = getRouter();

  const currentRoute = router.currentRoute.value;
  if (currentRoute) {
    return currentRoute.query;
  }
  return {};
}

export function getCurrentRouteName(): Routes | null {
  const router = getRouter();
  const currentRoute = router.currentRoute.value;
  if (currentRoute) {
    return currentRoute.name as Routes;
  }
  return null;
}
