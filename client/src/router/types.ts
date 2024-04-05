// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { ConnectionHandle, EntryName, FsPath, ParsecOrganizationFileLinkAddr, WorkspaceName } from '@/parsec';
import { DeviceID } from '@/plugins/libparsec';
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
  RecoveryExport = 'recoveryExport',
  Loading = 'loading',
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
    path: `/${Routes.Loading}`,
    name: Routes.Loading,
    component: () => import('@/views/layouts/LoadingLayout.vue'),
  },
  {
    // ConnectedLayout ensure that every children components are provided
    // with an importManager, informationManager and eventDistributor
    // that correspond with the current ConnectionHandle.
    path: '/connected',
    component: () => import('@/views/layouts/ConnectedLayout.vue'),
    children: [
      {
        path: '/sidebar',
        component: () => import('@/views/sidebar-menu/SidebarMenuPage.vue'),
        children: [
          {
            path: '/header',
            component: () => import('@/views/header/HeaderPage.vue'),
            children: [
              {
                path: '/imports',
                component: () => import('@/views/layouts/ImportLayout.vue'),
                children: [
                  {
                    path: `/:handle(\\d+)/${Routes.Workspaces}`,
                    name: Routes.Workspaces,
                    component: () => import('@/views/workspaces/WorkspacesPage.vue'),
                  },
                  {
                    path: `/:handle(\\d+)/${Routes.Documents}/:workspaceHandle(\\d+)`,
                    name: Routes.Documents,
                    component: () => import('@/views/files/FoldersPage.vue'),
                  },
                ],
              },
              {
                path: '/:handle(\\d+)',
                redirect: { name: Routes.Workspaces },
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
                path: `/:handle(\\d+)/${Routes.RecoveryExport}`,
                name: Routes.RecoveryExport,
                component: () => import('@/views/devices/ExportRecoveryDevicePage.vue'),
              },
            ],
          },
        ],
      },
    ],
  },
];

if (import.meta.env.VITE_APP_TEST_MODE?.toLowerCase() === 'true') {
  routes.push({
    path: '/test',
    name: 'Test',
    component: () => import('@/views/testing/TestPage.vue'),
  });
}

const router: Router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export function getRouter(): Router {
  return router;
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

export interface Query {
  documentPath?: FsPath;
  deviceId?: DeviceID;
  claimLink?: string;
  fileLink?: ParsecOrganizationFileLinkAddr;
  openInvite?: true;
  workspaceName?: WorkspaceName;
  selectFile?: EntryName;
  loginInfo?: string;
}
