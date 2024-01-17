// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { startWorkspace, WorkspaceID } from '@/parsec';
import { getConnectionHandle } from '@/router/params';
import { getRouter, Routes } from '@/router/types';
import { LocationQueryRaw, RouteParamsRaw } from 'vue-router';

export interface NavigationOptions {
  params?: object;
  query?: object;
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

export async function navigateToWorkspace(workspaceId: WorkspaceID, path = '/'): Promise<void> {
  startWorkspace(workspaceId).then(async (result) => {
    if (result.ok) {
      await navigateTo(Routes.Documents, { params: { workspaceHandle: result.value }, query: { path: path, workspaceId: workspaceId } });
    } else {
      console.error(`Failed to navigate to workspace: ${result.error.tag}`);
    }
  });
}

export async function routerGoBack(): Promise<void> {
  const router = getRouter();
  router.go(-1);
}
