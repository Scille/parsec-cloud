// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { WorkspaceHandle } from '@/parsec';
import { getConnectionHandle, getWorkspaceHandle } from '@/router/params';
import { RouteBackup, Routes, getRouter, getVisitedLastHistory } from '@/router/types';

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

export function currentRouteIsWorkspaceRoute(workspaceHandle: WorkspaceHandle): boolean {
  return currentRouteIs(Routes.Documents) && getWorkspaceHandle() === workspaceHandle;
}

export function currentRouteIsUserRoute(): boolean {
  return currentRouteIsOneOf([Routes.Users]);
}

export function currentRouteIsOrganizationManagementRoute(): boolean {
  return currentRouteIsUserRoute() || currentRouteIsOneOf([Routes.Storage, Routes.Organization]);
}

export function currentRouteIsFileRoute(): boolean {
  return currentRouteIsOneOf([Routes.Documents, Routes.Workspaces]);
}

export function getLastVisited(route: Routes): RouteBackup | undefined {
  const history = getVisitedLastHistory();

  return history.get(route);
}

export function hasVisited(route: Routes): boolean {
  return getLastVisited(route) !== undefined;
}

export function currentRouteIsLoggedRoute(): boolean {
  return currentRouteIsOneOf([
    Routes.Workspaces,
    Routes.Documents,
    Routes.Users,
    Routes.Storage,
    Routes.Organization,
    Routes.About,
    Routes.MyProfile,
    Routes.History,
    Routes.Viewer,
  ]);
}
