// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { WorkspaceID } from '@/parsec';
import { getConnectionHandle, getCurrentRouteQuery } from '@/router/params';
import { Routes, getRouter } from '@/router/types';

export function isLoggedIn(): boolean {
  return getConnectionHandle() !== null;
}

export function hasHistory(): boolean {
  const router = getRouter();
  if (!router) {
    console.error('Router has not been initialized');
    return false;
  }
  const handle = getConnectionHandle();

  const previousRoute = router.options.history.state.back?.toString();

  if (!handle || !previousRoute) {
    return false;
  }
  return previousRoute.startsWith(`/${handle}`);
}

export function currentRouteIs(route: Routes): boolean {
  const router = getRouter();

  return router.currentRoute.value.name === route;
}

export function currentRouteIsOneOf(routes: Routes[]): boolean {
  const router = getRouter();
  const currentRouteName = router.currentRoute.value.name;

  if (!currentRouteName) {
    return false;
  }
  return (routes as string[]).includes(currentRouteName as string);
}

export function currentRouteIsWorkspaceRoute(workspaceId: WorkspaceID): boolean {
  const query = getCurrentRouteQuery();
  return currentRouteIs(Routes.Documents) && query.workspaceId === workspaceId;
}

export function currentRouteIsUserRoute(): boolean {
  return currentRouteIsOneOf([Routes.ActiveUsers, Routes.RevokedUsers, Routes.Invitations]);
}

export function currentRouteIsOrganizationManagementRoute(): boolean {
  return currentRouteIsUserRoute() || currentRouteIsOneOf([Routes.Storage, Routes.Organization]);
}

export function currentRouteIsFileRoute(): boolean {
  return currentRouteIsOneOf([Routes.Documents, Routes.Workspaces]);
}
