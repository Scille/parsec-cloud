// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ConnectionHandle, EntryName, FsPath, ParsecWorkspacePathAddr, WorkspaceHandle, WorkspaceName } from '@/parsec';
import { DeviceID, OrganizationID } from '@/plugins/libparsec';
import { ServerType } from '@/services/parsecServers';
import { createRouter, createWebHistory } from '@ionic/vue-router';
import { Ref } from 'vue';
import { RouteLocationNormalizedLoaded, RouteRecordRaw, Router } from 'vue-router';

export enum Routes {
  Home = 'home',
  Workspaces = 'workspaces',
  Documents = 'documents',
  Users = 'users',
  Storage = 'storage',
  Organization = 'organization',
  About = 'about',
  MyProfile = 'myProfile',
  Loading = 'loading',
  ClientAreaLogin = 'clientLogin',
  ClientArea = 'clientArea',
  History = 'history',
  Viewer = 'viewer',
}

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: `/${Routes.Home}`,
  },
  {
    path: `/${Routes.Home}`,
    name: Routes.Home,
    component: () => import('@/views/home/HomePage.vue'),
  },
  {
    path: '/:unknown(.*)*',
    redirect: (): any => {
      return { path: '/', query: {} };
    },
  },
  {
    path: `/${Routes.Loading}`,
    name: Routes.Loading,
    component: () => import('@/views/layouts/LoadingLayout.vue'),
  },
  {
    path: `/${Routes.ClientAreaLogin}`,
    name: Routes.ClientAreaLogin,
    component: () => import('@/views/client-area/ClientAreaLoginPage.vue'),
  },
  {
    path: `/${Routes.ClientArea}`,
    name: Routes.ClientArea,
    component: () => import('@/views/client-area/ClientAreaLayout.vue'),
  },
  {
    // ConnectedLayout ensure that every children components are provided
    // with a fileOperationManager, informationManager and eventDistributor
    // that correspond with the current ConnectionHandle.
    path: '/connected',
    component: () => import('@/views/layouts/ConnectedLayout.vue'),
    children: [
      {
        path: '/sidebar',
        component: () => import('@/views/menu/MenuPage.vue'),
        children: [
          {
            path: '/header',
            component: () => import('@/views/header/HeaderPage.vue'),
            children: [
              {
                path: '/fileOp',
                component: () => import('@/views/layouts/FileOperationLayout.vue'),
                children: [
                  {
                    path: `/:handle(\\d+)/${Routes.Workspaces}`,
                    name: Routes.Workspaces,
                    component: () => import('@/views/workspaces/WorkspacesPage.vue'),
                  },
                  {
                    path: `/:handle(\\d+)/${Routes.Documents}`,
                    name: Routes.Documents,
                    component: () => import('@/views/files/FoldersPage.vue'),
                  },
                  {
                    path: `/:handle(\\d+)/${Routes.History}`,
                    name: Routes.History,
                    component: () => import('@/views/workspaces/WorkspaceHistoryPage.vue'),
                  },
                ],
              },
              {
                path: '/:handle(\\d+)',
                redirect: { name: Routes.Workspaces },
              },
              {
                path: '/:handle(\\d+)/:unknown(.+)',
                redirect: (to): any => {
                  return { path: `/${to.params.handle}`, query: {} };
                },
              },
              {
                path: `/:handle(\\d+)/${Routes.Users}`,
                name: Routes.Users,
                component: () => import('@/views/users/UsersPage.vue'),
              },
              {
                path: `/:handle(\\d+)/${Routes.Storage}`,
                name: Routes.Storage,
                component: () => import('@/views/organizations/StoragePage.vue'),
              },
              {
                path: `/:handle(\\d+)/${Routes.Organization}`,
                name: Routes.Organization,
                component: () => import('@/views/organizations/OrganizationInformationPage.vue'),
              },
              {
                path: `/:handle(\\d+)/${Routes.About}`,
                name: Routes.About,
                component: () => import('@/views/about/AboutPage.vue'),
              },
              {
                path: `/:handle(\\d+)/${Routes.MyProfile}`,
                name: Routes.MyProfile,
                component: () => import('@/views/users/MyProfilePage.vue'),
              },
              {
                path: `/:handle(\\d+)/${Routes.Viewer}`,
                name: Routes.Viewer,
                component: () => import('@/views/viewers/FileViewer.vue'),
              },
            ],
          },
        ],
      },
    ],
  },
];

if (import.meta.env.PARSEC_APP_TEST_MODE?.toLowerCase() === 'true') {
  routes.push({
    path: '/test',
    name: 'Test',
    component: () => import('@/views/testing/TestPage.vue'),
  });
}

if (!import.meta.env.BASE_URL.startsWith('/')) {
  console.warn(`BASE_URL is not an absolute path, this will cause issue with vue-router (BASE_URL=${import.meta.env.BASE_URL})`);
}
const router: Router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

const visitedLastHistory = new Map<Routes, RouteBackup>();

router.beforeEach((to, from, next) => {
  // Clearing history if we come from a page that has no handle.
  // Not taking any risk.
  if (!to.params.handle || !from.params.handle) {
    visitedLastHistory.clear();
  }
  if (!to.name || !to.params.handle) {
    next();
    return;
  }
  const routeName = to.name as Routes;
  if (visitedLastHistory.has(to.name as Routes)) {
    visitedLastHistory.delete(routeName);
  }
  visitedLastHistory.set(routeName, {
    handle: Number(to.params.handle),
    data: {
      route: routeName,
      params: to.params,
      query: to.query,
    },
  });
  next();
});

export function getRouter(): Router {
  return router;
}

export function getVisitedLastHistory(): Map<Routes, RouteBackup> {
  return visitedLastHistory;
}

export function getCurrentRoute(): Ref<RouteLocationNormalizedLoaded> {
  return (router as Router).currentRoute;
}

export const InvalidConnectionHandle: ConnectionHandle = 0;

export interface RouteBackup {
  handle: ConnectionHandle;
  data: {
    route: Routes;
    params: object;
    query: Query;
  };
}

export enum ProfilePages {
  Devices = 'devices',
  Recovery = 'recovery',
  Authentication = 'authentication',
  Settings = 'settings',
  About = 'about',
}

export interface Query {
  documentPath?: FsPath;
  deviceId?: DeviceID;
  claimLink?: string;
  bootstrapLink?: string;
  fileLink?: ParsecWorkspacePathAddr;
  openInvite?: true;
  workspaceName?: WorkspaceName;
  selectFile?: EntryName;
  loginInfo?: string;
  workspaceHandle?: WorkspaceHandle;
  bmsOrganizationId?: OrganizationID;
  createOrg?: ServerType;
  fileTypeInfo?: string;
  timestamp?: string;
  profilePage?: ProfilePages;
}

export interface ClientAreaQuery {
  page?: string;
  organization?: string;
}
