<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div>
    <div class="logo">
      <img
        src="@/assets/images/Logo/logo_column_gradient.svg"
        alt="Parsec logo"
      >
    </div>

    <h1>{{ $t('app.name') }}</h1>

    <div class="app-info">
      <ion-item>
        <ion-label class="app-info-key">
          {{ $t('AboutPage.appInfo.version') }}
        </ion-label>
        <ion-text class="app-info-value">
          {{ $t('app.name') }} v{{ getAppVersion() }}
        </ion-text>
      </ion-item>

      <ion-item>
        <ion-label class="app-info-key">
          {{ $t('AboutPage.appInfo.developer') }}
        </ion-label>
        <ion-text class="app-info-value">
          <a
            :href="$t('app.developerLink')"
            target="_blank"
          >
            {{ $t('app.developer') }}
          </a>
        </ion-text>
      </ion-item>

      <ion-item>
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

      <ion-item>
        <ion-label class="app-info-key">
          {{ $t('AboutPage.appInfo.project') }}
        </ion-label>
        <ion-text class="app-info-value">
          <a
            target="_blank"
            :href="$t('app.projectSources')"
          >
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
} from '@ionic/vue';
import { useI18n } from 'vue-i18n';
import { getAppVersion } from '@/common/mocks';

useI18n();
</script>

<style scoped lang="scss">
.logo {
  width: 12em;
  height: 12em;
}
</style>
