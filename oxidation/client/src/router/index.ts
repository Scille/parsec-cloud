// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { createRouter, createWebHistory } from '@ionic/vue-router';
import { RouteRecordRaw } from 'vue-router';
import TestPage from '../views/TestPage.vue';

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/home'
  },
  {
    path: '/home',
    name: 'home',
    component: () => import('@/views/HomePage.vue')
  },
  {
    path: '/menu',
    component: () => import('@/views/SidebarMenuPage.vue'),
    children: [
      {
        path: '',
        redirect: '/'
      },
      {
        path: '/documents',
        component: () => import('@/views/HeaderPage.vue'),
        children: [
          {
            path: '',
            redirect: '/'
          },
          {
            path: 'workspaces',
            name: 'workspaces',
            component: () => import('@/views/WorkspacesPage.vue')
          }
        ]
      },
      {
        path: '/settings',
        name: 'settings',
        component: () => import('@/views/SettingsPage.vue')
      },
      {
        path: '/organization',
        name: 'organization',
        redirect: { name: 'OrganizationPageUsers' },
        component: () => import('@/views/OrganizationPage.vue'),
        children: [
          {
            path: 'users',
            name: 'OrganizationPageUsers',
            component: () => import('@/views/OrganizationPageUsers.vue')
          },
          {
            path: 'workspaces',
            name: 'OrganizationPageWorkspaces',
            component: () => import('@/views/OrganizationPageWorkspaces.vue')
          },
          {
            path: 'storage',
            name: 'OrganizationPageStorage',
            component: () => import('@/views/OrganizationPageStorage.vue')
          }
        ]
      }
    ]
  }
];

if (import.meta.env.VUE_APP_TEST_MODE === 'True') {
  routes.push(
    {
      path: '/test',
      name: 'Test',
      component: TestPage
    }
  );
}

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes
});

export default router;
