<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content>
      <div class="container">
        <div
          class="about-container"
        >
          <about-view />
        </div>
        <div class="update-container">
          <h1>{{ $t('AboutPage.update.title') }}</h1>
          <div
            id="uptodate"
            v-show="upToDate"
          >
            {{ $t('AboutPage.update.upToDate') }}
          </div>
          <div
            v-show="!upToDate"
            id="notuptodate"
          >
            {{ $t('AboutPage.update.notUpToDate') }}
            <ion-button
              @click="update"
            >
              {{ $t('AboutPage.update.update') }}
            </ion-button>
          </div>

          <ion-button
            @click="showChangelog"
            fill="outline"
          >
            <ion-icon
              :icon="open"
              slot="start"
            />
            {{ $t('AboutPage.update.showChangelog') }}
          </ion-button>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonButton,
  IonContent,
  IonIcon,
  IonPage,
  modalController,
} from '@ionic/vue';
import {
  open,
} from 'ionicons/icons';
import { ref } from 'vue';
import { useI18n } from 'vue-i18n';
import AboutView from '@/views/about/AboutView.vue';
import ChangesModal from '@/views/about/ChangesModal.vue';

useI18n();
const upToDate = ref(false);

async function update(): Promise<void> {
  console.log('update');
}

async function showChangelog(): Promise<void> {
  const modal = await modalController.create({
    component: ChangesModal,
    cssClass: 'changes-modal',
  });
  await modal.present();
  await modal.onWillDismiss();
}
</script>

<style scoped lang="scss">
.container {
  margin: 3rem 2rem 2rem 2rem;
}
.about-container {
  border-bottom: 1px solid var(--parsec-color-light-primary-100);
  width: 80%;
}

.update-container {
  width: 80%;
}
</style>
