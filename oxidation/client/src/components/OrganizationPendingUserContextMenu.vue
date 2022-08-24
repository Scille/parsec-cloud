<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-content>
    <ion-list>
      <ion-item
        button
        lines="none"
      >
        <ion-icon
          slot="start"
          :icon="personAdd"
        />
        <ion-label>{{ $t('OrganizationPage.OrganizationPageUsers.OrganizationPendingUserContextMenu.greetUser') }}</ion-label>
      </ion-item>
      <ion-item
        button
        lines="none"
      >
        <ion-icon
          slot="start"
          :icon="personRemove"
          color="danger"
        />
        <ion-label color="danger">
          {{ $t('OrganizationPage.OrganizationPageUsers.OrganizationPendingUserContextMenu.cancelInvitation') }}
        </ion-label>
      </ion-item>
    </ion-list>
  </ion-content>
</template>

<script setup lang = "ts" >
import { IonContent, IonItem, IonLabel, IonList, IonIcon } from '@ionic/vue';
import {
  personAdd,
  personRemove
} from 'ionicons/icons';
</script>
