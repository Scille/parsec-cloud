<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="about">
    <!-- new version available -->
    <div
      v-show="!upToDate"
      id="notuptodate"
      class="update-container body"
    >
      <ion-text class="update-container__text title-h5">
        {{ $t('app.name') }} v.{{ getAppVersion() }}
        <span class="body-sm">
          {{ $t('AboutPage.update.notUpToDate') }}
        </span>
      </ion-text>

      <ion-button
        @click="update"
        class="update-container__btn"
      >
        {{ $t('AboutPage.update.update') }}
      </ion-button>
    </div>

    <ion-list class="info-list">
      <!-- version -->
      <ion-item class="body info-list__item">
        <ion-label class="app-info-key">
          {{ $t('AboutPage.appInfo.version') }}
        </ion-label>
        <div class="text-btn">
          <ion-text class="app-info-value"> v{{ getAppVersion() }} </ion-text>
          <ion-button
            class="changelog-btn button-outline"
            target="_blank"
            :href="$t('app.history')"
          >
            {{ $t('AboutPage.update.showChangelog') }}
          </ion-button>
        </div>
      </ion-item>

      <!-- developper -->
      <ion-item class="body info-list__item">
        <ion-label class="app-info-key">
          {{ $t('AboutPage.appInfo.developer') }}
        </ion-label>
        <ion-text class="app-info-value">
          <a
            :href="$t('app.developerLink')"
            target="_blank"
            class="link"
          >
            <span>{{ $t('app.developer') }}</span>
          </a>
        </ion-text>
      </ion-item>

      <!-- license -->
      <ion-item class="body info-list__item">
        <ion-label class="app-info-key">
          {{ $t('AboutPage.appInfo.license') }}
        </ion-label>
        <ion-text class="app-info-value">
          <a
            :href="$t('app.licenseLink')"
            target="_blank"
            class="link"
          >
            {{ $t('app.license') }}
          </a>
        </ion-text>
      </ion-item>

      <!-- github -->
      <ion-item class="body info-list__item">
        <ion-label class="app-info-key">
          {{ $t('AboutPage.appInfo.project') }}
        </ion-label>
        <ion-text class="app-info-value">
          <a
            target="_blank"
            :href="$t('app.projectSources')"
            class="link"
          >
            <ion-icon :icon="logoGithub" />
            GitHub
          </a>
        </ion-text>
      </ion-item>
    </ion-list>
  </div>
</template>

<script lang="ts">
/*
 * Keeping it in case we decide to use it

import ChangesModal from '@/views/about/ChangesModal.vue';
import { modalController } from '@ionic/vue';

async function showChangelog(): Promise<void> {
  const top = await modalController.getTop();
  if (top) {
    top.classList.add('overlapped-modal');
  }

  const modal = await modalController.create({
    component: ChangesModal,
    cssClass: 'changes-modal',
    canDismiss: true,
    backdropDismiss: false,
    // showBackdrop: true,
  });
  await modal.present();
  const result = await modal.onWillDismiss();
  await modal.dismiss();

  if (top) {
    if (result.role === 'cancel') {
      top.classList.remove('overlapped-modal');
    }
  }
}
*/
</script>

<script setup lang="ts">
import { getAppVersion } from '@/common/mocks';
import { IonButton, IonIcon, IonItem, IonLabel, IonList, IonText } from '@ionic/vue';
import { logoGithub } from 'ionicons/icons';
import { ref } from 'vue';

const upToDate = ref(false);

async function update(): Promise<void> {
  console.log('update');
}
</script>

<style scoped lang="scss">
.about {
  display: flex;
  flex-direction: column;
  gap: 1.5em;
}

.update-container {
  background: var(--parsec-color-light-gradient);
  color: var(--parsec-color-light-secondary-grey);
  display: flex;
  align-items: center;
  padding: 1.5rem;
  border-radius: var(--parsec-radius-6);
  justify-content: space-between;

  &__text {
    display: flex;
    color: var(--parsec-color-light-secondary-inversed-contrast);
    flex-direction: column;
    gap: 0.5rem;
  }

  &__btn {
    --background: var(--parsec-color-light-primary-30-opacity15) !important;
    outline: none;
    border-radius: var(--parsec-radius-6);

    &:hover {
      --background-hover: none !important;
      outline: 1px solid var(--parsec-color-light-primary-200);
    }

    &:active {
      --background: var(--parsec-color-light-primary-200) !important;
    }
  }
}

.info-list {
  display: flex;
  flex-direction: column;
  gap: 1em;
  padding: 0;
  padding-bottom: 2rem;

  &__item {
    --padding-start: 0;
  }

  .text-btn {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 1rem;
  }

  .changelog-btn {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    cursor: pointer;

    &::part(native) {
      background: none;
    }
  }
}
.app-info-key {
  max-width: 10rem;
  margin: 0;
  color: var(--parsec-color-light-secondary-grey);
}

.app-info-value {
  color: var(--parsec-color-light-primary-800);
  padding: 0.2rem 0.3rem;
  border-radius: var(--parsec-radius-6);
  transition: background-color 0.1s ease-in-out;

  &:hover:has(.link) {
    background-color: var(--parsec-color-light-primary-30);
  }

  .link {
    color: var(--parsec-color-light-primary-800);
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 0.5rem;

    ion-icon {
      color: var(--parsec-color-light-primary-800);
      font-size: 1.25rem;
    }
  }
}
</style>
