// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ConnectionHandle, EntryName, listStartedClients, WorkspaceHandle } from '@/parsec';
import { getConnectionHandle } from '@/router/params';
import { ClientAreaQuery, getCurrentRoute, getRouter, Query, RouteBackup, Routes } from '@/router/types';
import { Base64 } from 'megashark-lib';
import { LocationQueryRaw, RouteParamsRaw } from 'vue-router';

export interface NavigationOptions {
  params?: object;
  query?: Query | ClientAreaQuery;
  replace?: boolean;
  skipHandle?: boolean;
}

export async function navigateTo(routeName: Routes, options?: NavigationOptions): Promise<void> {
  window.electronAPI.log('debug', `Navigating to ${routeName}`);

  const router = getRouter();

  const params = options?.params ?? {};
  // Handle is provided in params by login, and skipHandle is used for logged out navigation
  if (!('handle' in params) && !options?.skipHandle) {
    const currentHandle = getConnectionHandle();
    (params as any).handle = currentHandle;
  }

  if (options?.replace) {
    await router.replace({
      name: routeName,
      params: params as RouteParamsRaw,
      query: options ? (options.query as LocationQueryRaw) : {},
    });
  } else {
    await router.push({
      name: routeName,
      params: params as RouteParamsRaw,
      query: options ? (options.query as LocationQueryRaw) : {},
    });
  }
}

export async function navigateToWorkspace(
  workspaceHandle: WorkspaceHandle,
  path = '/',
  selectFile?: EntryName,
  replace = false,
): Promise<void> {
  await navigateTo(Routes.Documents, {
    query: { documentPath: path, selectFile: selectFile, workspaceHandle: workspaceHandle },
    replace: replace,
  });
}

export async function routerGoBack(): Promise<void> {
  const router = getRouter();
  router.back();
}

const routesBackup: Array<RouteBackup> = [];

export async function backupCurrentOrganization(): Promise<void> {
  const currentHandle = getConnectionHandle();

  if (currentHandle === null) {
    window.electronAPI.log('error', 'Cannot backup the current organization, no handle found');
  } else {
    // Backup the current route
    const currentRoute = getCurrentRoute();
    const index = routesBackup.findIndex((bk) => bk.handle === currentHandle);
    if (index !== -1) {
      routesBackup.splice(index, 1);
    }
    routesBackup.push({
      handle: currentHandle,
      data: {
        route: currentRoute.value.name as Routes,
        params: currentRoute.value.params,
        query: currentRoute.value.query as Query,
      },
    });
  }
}

export async function switchOrganization(handle: ConnectionHandle | null, backup = true): Promise<void> {
  if (backup) {
    await backupCurrentOrganization();
  }

  const startedClients = await listStartedClients();
  // No handle, or an invalid handle, navigate to organization list
  if (!handle) {
    await navigateTo(Routes.Home, { skipHandle: true, replace: true });
    return;
  } else if (startedClients.find(([sHandle, _deviceId]) => sHandle === handle) === undefined) {
    window.electronAPI.log('warn', `Handle '${handle}' not found in the list of started clients`);
    await navigateTo(Routes.Home, { skipHandle: true, replace: true });
    return;
  }
  const backupIndex = routesBackup.findIndex((bk) => bk.handle === handle);

  if (backupIndex !== -1) {
    // We have a backup, just navigate
    const backup = routesBackup[backupIndex];
    try {
      await navigateTo(Routes.Loading, { skipHandle: true, replace: true, query: { loginInfo: Base64.fromObject(backup) } });
    } catch (e: any) {
      // We encounter an error, probably the base64 serialization, we remove the backup and log in to the default page
      window.electronAPI.log('error', `Error when switching organization, using default logged in page: ${e}`);
      routesBackup.splice(backupIndex, 1);
      await navigateTo(Routes.Loading, {
        replace: true,
        skipHandle: true,
        query: {
          loginInfo: Base64.fromObject({ handle: handle, data: { route: Routes.Workspaces, params: { handle: handle } } }),
        },
      });
    }
  } else {
    // Don't have any backup, but we know the handle is valid, just navigate to default logged in page
    await navigateTo(Routes.Loading, {
      replace: true,
      skipHandle: true,
      query: {
        loginInfo: Base64.fromObject({ handle: handle, data: { route: Routes.Workspaces, params: { handle: handle } } }),
      },
    });
  }
}
