// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { createRouter, createWebHistory } from '@ionic/vue-router';
import { RouteRecordRaw } from 'vue-router';

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/home',
  },
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
            path: '/:deviceId([a-z0-9]+@[a-z0-9]+)',
            redirect: { name: 'workspaces' },
          },
          {
            path: '/:deviceId([a-z0-9]+@[a-z0-9]+)/workspaces',
            name: 'workspaces',
            component: () => import('@/views/workspaces/WorkspacesPage.vue'),
          },
          {
            path: '/:deviceId([a-z0-9]+@[a-z0-9]+)/workspaces/:workspaceId([a-z0-9]+)',
            name: 'folder',
            component: () => import('@/views/files/FoldersPage.vue'),
          },
          {
            path: '/:deviceId([a-z0-9]+@[a-z0-9]+)/settings',
            name: 'settings',
            component: () => import('@/views/settings/SettingsPage.vue'),
          },
          {
            path: '/:deviceId([a-z0-9]+@[a-z0-9]+)/devices',
            name: 'devices',
            component: () => import('@/views/devices/DevicesPage.vue'),
          },
          {
            path: '/:deviceId([a-z0-9]+@[a-z0-9]+)/activeUsers',
            name: 'activeUsers',
            component: () => import('@/views/users/ActiveUsersPage.vue'),
          },
          {
            path: '/:deviceId([a-z0-9]+@[a-z0-9]+)/revokedUsers',
            name: 'revokedUsers',
            component: () => import('@/views/users/RevokedUsersPage.vue'),
          },
          {
            path: '/:deviceId([a-z0-9]+@[a-z0-9]+)/invitations',
            name: 'invitations',
            component: () => import('@/views/users/InvitationsPage.vue'),
          },
          {
            path: '/:deviceId([a-z0-9]+@[a-z0-9]+)/storage',
            name: 'storage',
            component: () => import('@/views/organizations/StoragePage.vue'),
          },
          {
            path: '/:deviceId([a-z0-9]+@[a-z0-9]+)/organization',
            name: 'organization',
            component: () => import('@/views/organizations/OrganizationInformationPage.vue'),
          },
          {
            path: '/:deviceId([a-z0-9]+@[a-z0-9]+)/about',
            name: 'about',
            component: () => import('@/views/about/AboutPage.vue'),
          },
        ],
      },
    ],
  },
];

if (import.meta.env.VITE_APP_TEST_MODE?.toLowerCase() === 'true') {
  routes.push(
    {
      path: '/test',
      name: 'Test',
      component: () => import('@/views/testing/TestPage.vue'),
    },
  );
}

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
});

export default router;
