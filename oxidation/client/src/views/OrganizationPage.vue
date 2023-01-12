<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-header>
      <ion-toolbar color="primary">
        <ion-buttons slot="start">
          <!-- <ion-menu-button /> -->
          <ion-back-button default-href="/" />
        </ion-buttons>
        <ion-title>{{ $t('OrganizationPage.pageTitle') }}</ion-title>
      </ion-toolbar>
      <ion-segment
        :scrollable="true"
        value="users"
      >
        <ion-segment-button
          value="users"
          @click="$router.replace({ name: 'OrganizationPageUsers' })"
        >
          <ion-label>{{ $t('OrganizationPage.users') }}</ion-label>
        </ion-segment-button>
        <ion-segment-button
          value="workspaces"
          @click="$router.replace({ name: 'OrganizationPageWorkspaces' })"
        >
          <ion-label>{{ $t('OrganizationPage.workspaces') }}</ion-label>
        </ion-segment-button>
        <ion-segment-button
          value="storage"
          @click="$router.replace({ name: 'OrganizationPageStorage' })"
        >
          <ion-label>{{ $t('OrganizationPage.storage') }}</ion-label>
        </ion-segment-button>
      </ion-segment>
    </ion-header>
    <ion-content>
      <ion-router-outlet />
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonSegment, IonSegmentButton, IonToolbar,
  IonPage, IonButtons, IonContent, IonRouterOutlet, IonHeader, IonTitle, IonLabel, IonBackButton
} from '@ionic/vue';

</script>
