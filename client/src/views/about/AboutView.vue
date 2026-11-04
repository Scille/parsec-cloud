<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="about">
    <!-- new version available -->
    <div
      v-show="!upToDate"
      id="notuptodate"
      class="update-container"
    >
      <ion-text class="update-container__text">
        {{ $msTranslate('app.name') }} v.{{ APP_VERSION }}
        <span>
          {{ $msTranslate('AboutPage.update.notUpToDate') }}
        </span>
      </ion-text>

      <ion-button
        @click="update"
        class="update-container__btn"
      >
        {{ $msTranslate('AboutPage.update.update') }}
      </ion-button>
    </div>

    <ion-list class="info-list">
      <!-- version -->
      <ion-item class="info-list__item">
        <ion-label class="app-info-key">
          {{ $msTranslate('AboutPage.appInfo.version') }}
        </ion-label>
        <div class="text-btn">
          <ion-text class="app-info-value"> v{{ APP_VERSION }} </ion-text>
          <ion-button
            fill="outline"
            class="changelog-btn color-secondary"
            @click="Env.Links.openChangelogLink()"
          >
            {{ $msTranslate('AboutPage.update.showChangelog') }}
          </ion-button>
        </div>
      </ion-item>

      <!-- developer -->
      <ion-item class="info-list__item">
        <ion-label class="app-info-key">
          {{ $msTranslate('AboutPage.appInfo.developer') }}
        </ion-label>
        <ion-text class="app-info-value">
          <span
            @click="Env.Links.openDeveloperLink()"
            class="link"
          >
            {{ $msTranslate('app.developer') }}
          </span>
        </ion-text>
      </ion-item>

      <!-- license -->
      <ion-item class="info-list__item">
        <ion-label class="app-info-key">
          {{ $msTranslate('AboutPage.appInfo.license') }}
        </ion-label>
        <ion-text class="app-info-value">
          <span
            @click="Env.Links.openLicenseLink()"
            class="link"
          >
            {{ $msTranslate('app.license') }}
          </span>
        </ion-text>
      </ion-item>

      <!-- github -->
      <ion-item class="info-list__item">
        <ion-label class="app-info-key">
          {{ $msTranslate('AboutPage.appInfo.project') }}
        </ion-label>
        <ion-text class="app-info-value">
          <span
            @click="Env.Links.openSourcesLink()"
            target="_blank"
            :href="$msTranslate('app.projectSources')"
            class="link"
          >
            <ion-icon :icon="logoGithub" />
            GitHub
          </span>
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
import { APP_VERSION, Env } from '@/services/environment';
import { IonButton, IonIcon, IonItem, IonLabel, IonList, IonText } from '@ionic/vue';
import { logoGithub } from 'ionicons/icons';
import { ref } from 'vue';

const upToDate = ref(true);

async function update(): Promise<void> {
  console.log('update');
}
</script>

<style scoped lang="scss">
.about {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');
  border-radius: ms.radius('lg');
  background: ms.color('surface-base-page');
  padding: ms.spacing('padding-2xl');
  box-shadow:
    0 1px 1px 0 rgba(0, 0, 0, 0.05),
    0 1px 4px 0 rgba(0, 0, 0, 0.03),
    0 0 1px 0 rgba(0, 0, 0, 0.2);
}

.update-container {
  @include ms.font('body-md-regular');
  background: linear-gradient(ms.color('surface-gradient-from'), ms.color('surface-gradient-to'));
  color: ms.color('text-neutral-default');
  display: flex;
  align-items: center;
  padding: ms.spacing('padding-4xl');
  border-radius: ms.radius('md');
  justify-content: space-between;

  &__text {
    @include ms.font('heading-h6');
    display: flex;
    color: ms.color('text-on-color-heading');
    flex-direction: column;
    gap: ms.spacing('gap-lg');

    span {
      @include ms.font('body-sm-regular');
    }
  }

  &__btn {
    --background: #{ms.color('opacity-inverse-500')} !important;
    outline: none;
    border-radius: ms.radius('md');

    &:hover {
      --background-hover: none !important;
      outline: ms.border('thin') solid ms.color('border-brand-default-subtle-hover');
    }

    &:active {
      --background: #{ms.color('surface-brand-default-subtle-pressed')} !important;
    }
  }
}

.info-list {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');
  padding: ms.spacing('padding-none');

  &__item {
    --background: transparent;
    @include ms.font('body-md-regular');
    --padding-start: #{ms.spacing('padding-none')};
  }

  .text-btn {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: ms.spacing('gap-3xl');
  }

  .changelog-btn {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
    cursor: pointer;

    &::part(native) {
      background: none;
    }
  }
}

.app-info-key {
  max-width: 7rem;
  margin: 0;
  color: ms.color('text-base-body');
}

.app-info-value {
  @include ms.font('body-md-bold');
  color: ms.color('text-base-body');
  padding: ms.spacing('padding-sm') ms.spacing('padding-lg');
  border-radius: ms.radius('md');
  transition: background-color 0.1s ease-in-out;

  &:has(.link) {
    background-color: ms.color('surface-neutral-default-subtle');
  }

  &:hover:has(.link) {
    background-color: ms.color('surface-neutral-default-subtle-hover');
  }

  .link {
    color: ms.color('text-base-body');
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: ms.spacing('gap-lg');
    cursor: pointer;

    ion-icon {
      color: ms.color('icon-neutral-default');
      font-size: 1.5rem;
    }
  }
}
</style>
