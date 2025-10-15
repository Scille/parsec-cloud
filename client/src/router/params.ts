// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ConnectionHandle, FsPath, WorkspaceHandle } from '@/parsec';
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

  if (currentRoute && currentRoute.query && 'workspaceHandle' in currentRoute.query) {
    return parseInt(currentRoute.query.workspaceHandle as string) as WorkspaceHandle;
  }
  return undefined;
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
  if (query.documentPath && query.documentPath.startsWith('/')) {
    return query.documentPath;
  }
  return '/';
}

export function getCurrentRouteParams(): object {
  const router = getRouter();
  const currentRoute = router.currentRoute.value;
  if (currentRoute) {
    return currentRoute.params;
  }
  return {};
}

export function getCurrentRouteQuery<T = Query>(): T {
  const router = getRouter();

  const currentRoute = router.currentRoute.value;
  if (currentRoute) {
    return {
      ...(currentRoute.query as T),
      readOnly: currentRoute.query.readOnly ? currentRoute.query.readOnly === 'true' : undefined,
    };
  }
  return {} as object as T;
}

export function getCurrentRouteName(): Routes | null {
  const router = getRouter();
  const currentRoute = router.currentRoute.value;
  if (currentRoute) {
    return currentRoute.name as Routes;
  }
  return null;
}
