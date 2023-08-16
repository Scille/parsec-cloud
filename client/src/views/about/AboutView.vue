<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="about">
    <div class="logo">
      <img
        src="@/assets/images/Logo/logo_column_gradient.svg"
        alt="Parsec logo"
      >
    </div>

    <ion-title
      class="title-h2 about-title"
    >
      {{ $t('app.name') }}
    </ion-title>

    <div class="app-info">
      <!-- version -->
      <ion-item class="body-lg">
        <ion-label class="app-info-key">
          {{ $t('AboutPage.appInfo.version') }}
        </ion-label>
        <ion-text class="app-info-value">
          {{ $t('app.name') }} v{{ getAppVersion() }}
        </ion-text>
      </ion-item>

      <!-- developper -->
      <ion-item class="body-lg">
        <ion-label class="app-info-key">
          {{ $t('AboutPage.appInfo.developer') }}
        </ion-label>
        <ion-text class="app-info-value">
          <a
            :href="$t('app.developerLink')"
            target="_blank"
            class="link"
          >
            <img
              src="@/assets/images/logo-scille.png"
              alt="Scille logo"
            >
            <span>{{ $t('app.developer') }}</span>
          </a>
        </ion-text>
      </ion-item>

      <!-- license -->
      <ion-item class="body-lg">
        <ion-label class="app-info-key">
          {{ $t('AboutPage.appInfo.license') }}
        </ion-label>
        <ion-text class="app-info-value">
          <a
            :href="$t('app.licenseLink')"
            target="_blank"
          >
            {{ $t('app.license') }}
          </a>
        </ion-text>
      </ion-item>

      <ion-item class="body-lg">
        <ion-label class="app-info-key">
          {{ $t('AboutPage.appInfo.project') }}
        </ion-label>
        <ion-text class="app-info-value">
          <a
            target="_blank"
            :href="$t('app.projectSources')"
            class="link"
          >
            <ion-icon
              :icon="logoGithub"
            />
            GitHub
          </a>
        </ion-text>
      </ion-item>
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  IonText,
  IonLabel,
  IonItem,
  IonIcon,
  IonTitle,
} from '@ionic/vue';
import {
  logoGithub,
} from 'ionicons/icons';
import { useI18n } from 'vue-i18n';
import { getAppVersion } from '@/common/mocks';

useI18n();
</script>

<style scoped lang="scss">
.logo {
  width: 6.5em;
  margin-bottom: 2em;
}

.about-title {
  margin-bottom: 1em;
  color: var(--parsec-color-light-primary-700);
  padding: 0;
}

.app-info {
  display: flex;
  flex-direction: column;
  gap: 1em;
  padding-bottom: 2rem;
}
.app-info-key {
  max-width: 12rem;
  margin: 0;
  color: var(--parsec-color-light-secondary-grey);
}

.app-info-value {
  color: var(--parsec-color-light-primary-800);

  .link {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: .5rem;
    color: var(--parsec-color-light-primary-800);
    border-radius: var(--parsec-radius-6);
    background: var(--parsec-color-light-secondary-background);
    padding: .5rem;

    img {
      width: 2.2rem;
    }

    ion-icon {
      color: var(--parsec-color-light-primary-800);
      font-size: 1.25rem;
    }
  }
}
</style>
