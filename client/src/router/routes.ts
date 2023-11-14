// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { RouteRecordRaw } from 'vue-router';

export const LOGIN_ROUTES: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/home',
  },
  {
    path: '/home',
    name: 'home',
    component: () => import('@/views/home/HomePage.vue'),
  },
];

export const LOGGED_IN_ROUTES: Array<RouteRecordRaw> = [
  {
    path: '/home',
    name: 'home',
    component: () => import('@/views/home/HomePage.vue'),
  },
  {
    path: '/menu',
    component: () => import('@/views/sidebar-menu/SidebarMenuPage.vue'),
    children: [
      {
        path: '/documents',
        component: () => import('@/views/header/HeaderPage.vue'),
        children: [
          {
            path: '/:handle(\\d+)',
            redirect: { name: 'workspaces' },
          },
          {
            path: '/:handle(\\d+)/workspaces',
            name: 'workspaces',
            component: () => import('@/views/workspaces/WorkspacesPage.vue'),
          },
          {
            path: '/:handle(\\d+)/workspaces/:workspaceHandle(\\d+)',
            name: 'folder',
            component: () => import('@/views/files/FoldersPage.vue'),
          },
          {
            path: '/:handle(\\d+)/settings',
            name: 'settings',
            component: () => import('@/views/settings/SettingsPage.vue'),
          },
          {
            path: '/:handle(\\d+)/devices',
            name: 'devices',
            component: () => import('@/views/devices/DevicesPage.vue'),
          },
          {
            path: '/:handle(\\d+)/activeUsers',
            name: 'activeUsers',
            component: () => import('@/views/users/ActiveUsersPage.vue'),
          },
          {
            path: '/:handle(\\d+)/revokedUsers',
            name: 'revokedUsers',
            component: () => import('@/views/users/RevokedUsersPage.vue'),
          },
          {
            path: '/:handle(\\d+)/invitations',
            name: 'invitations',
            component: () => import('@/views/users/InvitationsPage.vue'),
          },
          {
            path: '/:handle(\\d+)/storage',
            name: 'storage',
            component: () => import('@/views/organizations/StoragePage.vue'),
          },
          {
            path: '/:handle(\\d+)/organization',
            name: 'organization',
            component: () => import('@/views/organizations/OrganizationInformationPage.vue'),
          },
          {
            path: '/:handle(\\d+)/about',
            name: 'about',
            component: () => import('@/views/about/AboutPage.vue'),
          },
        ],
      },
    ],
  },
];

if (import.meta.env.VITE_APP_TEST_MODE?.toLowerCase() === 'true') {
  LOGIN_ROUTES.push({
    path: '/test',
    name: 'Test',
    component: () => import('@/views/testing/TestPage.vue'),
  });
}
