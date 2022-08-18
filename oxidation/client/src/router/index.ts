// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

import { createRouter, createWebHistory } from '@ionic/vue-router';
import { RouteRecordRaw } from 'vue-router';
import TestPage from '../views/TestPage.vue';

let routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/home'
  },
  {
    path: '/menu',
    component: () => import('@/views/MenuPage.vue'),
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
            path: '/home',
            name: 'home',
            component: () => import('@/views/HomePage.vue')
          }
        ]
      },
      {
        path: '/settings',
        name: 'settings',
        component: () => import('@/views/SettingsPage.vue')
      }
    ]
  }
];

if (process.env.VUE_APP_TEST_MODE === 'True') {
  routes = [
    {
      path: '/',
      name: 'Test',
      component: TestPage
    }
  ];
}

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
});

export default router;
