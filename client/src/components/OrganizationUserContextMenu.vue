<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-content>
    <ion-list>
      <ion-item
        button
        lines="none"
        @click="onDetailsClick()"
      >
        <ion-icon
          :icon="informationCircle"
          slot="start"
        />
        <ion-label>{{ $t('OrganizationPage.OrganizationPageUsers.OrganizationUserContextMenu.details') }}</ion-label>
      </ion-item>
      <ion-item
        button
        lines="none"
        @click="onRevokeClick()"
      >
        <ion-icon
          :icon="ban"
          color="danger"
          slot="start"
        />
        <ion-label color="danger">
          {{ $t('OrganizationPage.OrganizationPageUsers.OrganizationUserContextMenu.revoke') }}
        </ion-label>
      </ion-item>
    </ion-list>
  </ion-content>
</template>

<script setup lang = "ts" >
import { IonContent, IonItem, IonLabel, IonList, popoverController, IonIcon } from '@ionic/vue';
import { informationCircle, ban } from 'ionicons/icons';

function onDetailsClick(): Promise<boolean> {
  return popoverController.dismiss(null, 'details');
}

function onRevokeClick(): Promise<boolean> {
  return popoverController.dismiss(null, 'revoke');
}
</script>
