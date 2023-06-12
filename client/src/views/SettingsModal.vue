<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <!-- header -->
    <ion-header class="modal-header">
      <ion-toolbar class="modal-header__toolbar">
        <ion-title class="title-h2">
          {{ $t('SettingsPage.pageTitle') }}
        </ion-title>
      </ion-toolbar>
      <ion-buttons
        slot="end"
        class="closeBtn-container"
      >
        <ion-button
          slot="icon-only"
          @click="closeModal()"
          class="closeBtn"
        >
          <ion-icon
            :icon="close"
            size="large"
            class="closeBtn__icon"
          />
        </ion-button>
      </ion-buttons>
    </ion-header>
    <!-- content -->
    <ion-content>
      <settings-view />
    </ion-content>
  </ion-page>
</template>

<script setup lang = "ts" >
import {
  IonPage,
  IonHeader,
  IonToolbar,
  IonTitle,
  IonButtons,
  IonButton,
  IonIcon,
  modalController,
  IonContent
} from '@ionic/vue';

import {
  close
} from 'ionicons/icons';
import SettingsView from '@/components/SettingsView.vue';

function closeModal(): Promise<boolean> {
  return modalController.dismiss(null, 'cancel');
}
</script>

<style lang="scss" scoped>
.modal {
  padding: 2.5rem;
}

.modal-header {
  position: relative;

  &__toolbar {
    --min-height: 0;
  }
}

.title-h2 {
  color: var(--parsec-color-light-primary-700);
  padding-inline: 0;
  margin-bottom: 2rem;
}

closeBtn-container, .closeBtn {
  margin: 0;
  --padding-start: 0;
  --padding-end: 0;
}

.closeBtn-container {
  position: absolute;
  top: 0;
  right: 0;
}

.closeBtn {
  border-radius: 4px;
  width: fit-content;
  height: fit-content;

  &:hover {
    --background-hover: var(--parsec-color-light-primary-50);
    --border-radius: 4px;
  }

  &:active {
    background: var(--parsec-color-light-primary-100);
    --border-radius: 4px;
  }

  &__icon {
    padding: 4px;
    color: var(--parsec-color-light-primary-500);
  }
}
</style>
