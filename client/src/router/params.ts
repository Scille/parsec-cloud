// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ConnectionHandle, FsPath, WorkspaceHandle, WorkspaceID } from '@/parsec';
import { Query, Routes, getRouter } from '@/router/types';

export function getConnectionHandle(): ConnectionHandle | null {
  const router = getRouter();
  const currentRoute = router.currentRoute.value;

  if (currentRoute && currentRoute.params && 'handle' in currentRoute.params) {
    return parseInt(currentRoute.params.handle as string) as ConnectionHandle;
  }
  return null;
}

export function getWorkspaceHandle(): WorkspaceHandle | undefined {
  const router = getRouter();
  const currentRoute = router.currentRoute.value;

  if (currentRoute && currentRoute.name === Routes.Documents && currentRoute.params && 'workspaceHandle' in currentRoute.params) {
    return parseInt(currentRoute.params.workspaceHandle as string) as WorkspaceHandle;
  }
  return undefined;
}

export function getWorkspaceId(): WorkspaceID {
  const query = getCurrentRouteQuery();

  if (getCurrentRouteName() === Routes.Documents && query.workspaceId) {
    return query.workspaceId;
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

export function getDocumentPath(): FsPath {
  const query = getCurrentRouteQuery();
  if (query.documentPath) {
    return query.documentPath;
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

export function getCurrentRouteQuery(): Query {
  const router = getRouter();

  const currentRoute = router.currentRoute.value;
  if (currentRoute) {
    return currentRoute.query as Query;
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
