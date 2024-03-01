// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ConnectionHandle, EntryName, startWorkspace, WorkspaceID } from '@/parsec';
import { getConnectionHandle } from '@/router/params';
import { getCurrentRoute, getRouter, Query, Routes } from '@/router/types';
import { organizationKey } from '@/router/watchers';
import { LocationQueryRaw, RouteParamsRaw } from 'vue-router';

export interface NavigationOptions {
  params?: object;
  query?: Query;
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

export async function navigateToWorkspace(workspaceId: WorkspaceID, path = '/', selectFile?: EntryName): Promise<void> {
  startWorkspace(workspaceId).then(async (result) => {
    if (result.ok) {
      await navigateTo(Routes.Documents, {
        params: { workspaceHandle: result.value },
        query: { documentPath: path, workspaceId: workspaceId, selectFile: selectFile },
      });
    } else {
      console.error(`Failed to navigate to workspace: ${result.error.tag}`);
    }
  });
}

export async function routerGoBack(): Promise<void> {
  const router = getRouter();
  router.go(-1);
}

interface RouteBackup {
  handle: ConnectionHandle;
  data: {
    route: Routes;
    params: object;
    query: Query;
  };
}

const routesBackup: Array<RouteBackup> = [];

export async function switchOrganization(handle: ConnectionHandle | null, backup = true): Promise<void> {
  if (backup) {
    const currentHandle = getConnectionHandle();

    if (currentHandle === null) {
      console.error('No current handle');
    } else if (handle === currentHandle) {
      console.error('Cannot switch to same organization');
    } else {
      // Backup the current route
      const currentRoute = getCurrentRoute();
      const index = routesBackup.findIndex((bk) => bk.handle === currentHandle);
      if (index !== -1) {
        routesBackup.splice(index, 1);
      }
      console.log('Saving', currentRoute.value.name, currentRoute.value.params, currentRoute.value.query);
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

  // No handle, navigate to organization list
  if (!handle) {
    await navigateTo(Routes.Home, { skipHandle: true, replace: true });
    organizationKey.value += 1;
  } else {
    const backup = routesBackup.find((bk) => bk.handle === handle);
    if (!backup) {
      console.error('No backup, organization was not connected');
      return;
    }
    console.log('Restoring', backup.data.route, backup.data.params, backup.data.query);
    await navigateTo(backup.data.route, { params: backup.data.params, query: backup.data.query, skipHandle: true, replace: true });
    organizationKey.value += 1;
  }
}
