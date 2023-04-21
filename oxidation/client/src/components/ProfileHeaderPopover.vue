<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-list>
    <ion-item class="profile-email">
      <ion-text class="body-sm">thomas.linere@mycompany.fr</ion-text>
    </ion-item>
    <!-- item -->
    <ion-item class="item">
      <ion-icon
        :icon="phonePortrait"
        slot="start"
      />
      <ion-label class="body">Mes appareils</ion-label>
    </ion-item>
    <!-- end of item -->
  </ion-list>
</template>

<script setup lang="ts">
import {
  IonList,
  IonItem,
  IonIcon,
  IonLabel,
  IonText
} from '@ionic/vue';
import {
  phonePortrait
} from 'ionicons/icons';
</script>

<style lang="scss" scoped>
ion-item {
  --background: none;
}

.profile-email {
  color: var(--parsec-color-light-secondary-grey);
}

.item:not(:first-child) {
  margin-inline-end: 2px;
  color: var(--parsec-color-light-secondary-text);
  border-radius: 0.25rem;

  &:hover {
    --background: var(--parsec-color-light-primary-30);
    color: var(--parsec-color-light-primary-600);

    ion-icon {
      color: var(--parsec-color-light-primary-600);
    }
  }

  ion-icon {
    color: var(--parsec-color-light-secondary-text);
    margin-inline-end: 0.75rem;
    margin-top: 1rem;
    margin-bottom: 1rem;
  }
}
</style>
