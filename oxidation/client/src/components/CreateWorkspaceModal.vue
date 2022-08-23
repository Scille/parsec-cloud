<template>
  <ion-header>
    <ion-toolbar>
      <ion-title>{{ $t('WorkspacesPage.CreateWorkspaceModal.pageTitle') }}</ion-title>
      <ion-buttons slot="end">
        <ion-button
          @click="closeModal()"
        >
          <ion-icon
            slot="icon-only"
            :icon="close"
            size="large"
          />
        </ion-button>
      </ion-buttons>
    </ion-toolbar>
  </ion-header>
  <ion-content class="ion-padding">
    <ion-item>
      <ion-label position="stacked">
        {{ $t('WorkspacesPage.CreateWorkspaceModal.name') }}
      </ion-label>
      <ion-input
        :autofocus="true"
        type="text"
        v-model="workspaceName"
        :placeholder="$t('WorkspacesPage.CreateWorkspaceModal.namePlaceholder')"
      />
    </ion-item>
  </ion-content>
  <ion-footer>
    <ion-toolbar>
      <ion-buttons slot="primary">
        <ion-button
          @click="closeModal()"
        >
          {{ $t('WorkspacesPage.CreateWorkspaceModal.cancel') }}
        </ion-button>
        <ion-button
          @click="confirm()"
        >
          {{ $t('WorkspacesPage.CreateWorkspaceModal.create') }}
        </ion-button>
      </ion-buttons>
    </ion-toolbar>
  </ion-footer>
</template>

<script setup lang="ts">
import {
  IonContent,
  IonHeader,
  IonTitle,
  IonToolbar,
  IonButtons,
  IonButton,
  IonItem,
  IonLabel,
  IonInput,
  modalController,
  IonFooter,
  IonIcon
} from '@ionic/vue';
import { ref } from 'vue';
import {
  close
} from 'ionicons/icons';

const workspaceName = ref('');

function closeModal(): Promise<boolean> {
  return modalController.dismiss(null, 'cancel');
}

function confirm(): Promise<boolean> {
  return modalController.dismiss(workspaceName.value, 'confirm');
}
</script>

<style scoped>
</style>
