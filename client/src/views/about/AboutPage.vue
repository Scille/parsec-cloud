<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

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
          <ion-title
            class="title-h2 about-title"
          >
            {{ $t('AboutPage.update.title') }}
          </ion-title>
          <div
            id="uptodate"
            v-show="upToDate"
            class="update-text body-lg"
          >
            {{ $t('AboutPage.update.upToDate') }}
          </div>
          <div
            v-show="!upToDate"
            id="notuptodate"
            class="update-text body-lg"
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
            class="update-btn"
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
  IonTitle,
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
  width: 70%;
}

.update-container {
  width: 70%;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.about-title {
  margin-top: 2rem;
  color: var(--parsec-color-light-primary-700);
  padding: 0;
}

.update-text {
  color: var(--parsec-color-light-secondary-grey);
  display: flex;
  align-items: center;
  gap: 1.5rem;
}

.update-btn {
  width: fit-content;
  margin: 0;

  ion-icon {
    margin-right: .5rem;
  }
}
</style>
