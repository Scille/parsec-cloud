// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { AvailableDevice, FsPath, WorkspaceID } from '@/parsec';
import { createRouter, createWebHistory } from '@ionic/vue-router';
import { Ref } from 'vue';
import { RouteLocationNormalizedLoaded, RouteRecordRaw, Router } from 'vue-router';

export enum Routes {
  Home = 'home',
  Workspaces = 'workspaces',
  Documents = 'documents',
  Settings = 'settings',
  Devices = 'devices',
  ActiveUsers = 'activeUsers',
  RevokedUsers = 'revokedUsers',
  Invitations = 'invitations',
  Storage = 'storage',
  Organization = 'organization',
  About = 'about',
  ContactDetails = 'contactDetails',
  RecoveryExport = 'recoveryExport',
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
    path: '/menu',
    component: () => import('@/views/sidebar-menu/SidebarMenuPage.vue'),
    children: [
      {
        path: '/connected',
        component: () => import('@/views/header/HeaderPage.vue'),
        children: [
          {
            path: '/import',
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
            path: `/:handle(\\d+)/${Routes.Settings}`,
            name: Routes.Settings,
            component: () => import('@/views/settings/SettingsPage.vue'),
          },
          {
            path: `/:handle(\\d+)/${Routes.Devices}`,
            name: Routes.Devices,
            component: () => import('@/views/devices/DevicesPage.vue'),
          },
          {
            path: `/:handle(\\d+)/${Routes.ActiveUsers}`,
            name: Routes.ActiveUsers,
            component: () => import('@/views/users/ActiveUsersPage.vue'),
          },
          {
            path: `/:handle(\\d+)/${Routes.RevokedUsers}`,
            name: Routes.RevokedUsers,
            component: () => import('@/views/users/RevokedUsersPage.vue'),
          },
          {
            path: `/:handle(\\d+)/${Routes.Invitations}`,
            name: Routes.Invitations,
            component: () => import('@/views/users/InvitationsPage.vue'),
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
            path: `/:handle(\\d+)/${Routes.ContactDetails}`,
            name: Routes.ContactDetails,
            component: () => import('@/views/users/MyContactDetailsPage.vue'),
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

export interface Query {
  documentPath?: FsPath;
  device?: AvailableDevice;
  workspaceId?: WorkspaceID;
  claimLink?: string;
  openInvite?: true;
}
