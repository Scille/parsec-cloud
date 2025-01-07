// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ConnectionHandle, EntryName, WorkspaceHandle } from '@/parsec';
import { getConnectionHandle } from '@/router/params';
import { ClientAreaQuery, Query, RouteBackup, Routes, getCurrentRoute, getRouter } from '@/router/types';
import { Base64 } from 'megashark-lib';
import { LocationQueryRaw, RouteParamsRaw } from 'vue-router';

export interface NavigationOptions {
  params?: object;
  query?: Query | ClientAreaQuery;
  replace?: boolean;
  skipHandle?: boolean;
}

export async function navigateTo(routeName: Routes, options?: NavigationOptions): Promise<void> {
  const router = getRouter();

  const params = options && options.params ? options.params : {};
  // Handle is provided in params by login, and skipHandle is used for logged out navigation
  if (!('handle' in params) && (!options || !options.skipHandle)) {
    const currentHandle = getConnectionHandle();
    (params as any).handle = currentHandle;
  }

  if (options && options.replace) {
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
  router.go(-1);
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

  // No handle, navigate to organization list
  if (!handle) {
    await navigateTo(Routes.Home, { skipHandle: true, replace: true });
  } else {
    const backupIndex = routesBackup.findIndex((bk) => bk.handle === handle);
    // Don't have any backup, just navigate to home
    if (backupIndex === -1) {
      window.electronAPI.log('error', 'Trying to switch to an organization for which we have no backup information');
      return;
    }
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
          loginInfo: Base64.fromObject({ handle: backup.handle, data: { route: Routes.Workspaces, params: { handle: backup.handle } } }),
        },
      });
    }
  }
}
