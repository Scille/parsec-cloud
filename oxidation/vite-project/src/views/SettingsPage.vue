<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-header>
      <ion-toolbar color="primary">
        <ion-buttons slot="start">
          <ion-back-button default-href="/" />
        </ion-buttons>
        <ion-title>{{ $t('SettingsPage.pageTitle') }}</ion-title>
      </ion-toolbar>
    </ion-header>
    <ion-content class="ion-padding settings-content">
      <ion-list class="ion-padding">
        <h5>{{ $t('SettingsPage.telemetry') }}</h5>
        <ion-item class="ion-margin-bottom">
          <settings-option
            :title="$t('SettingsPage.enableTelemetry')"
            :description="$t('SettingsPage.enableTelemetryDescription')"
          />
          <ion-toggle
            v-model="config.enableTelemetry"
            value="telemetry"
          />
        </ion-item>
        <h5 v-if="isPlatform('electron')">
          {{ $t('SettingsPage.behavior') }}
        </h5>
        <ion-item v-if="isPlatform('electron')">
          <settings-option
            :title="$t('SettingsPage.minimizeToSystemTray')"
            :description="$t('SettingsPage.minimizeToSystemTrayDescription')"
          />
          <ion-toggle
            v-model="config.minimizeToTray"
            value="runbackground"
          />
        </ion-item>
        <h5>{{ $t('SettingsPage.localization') }}</h5>
        <ion-item>
          <ion-label>
            {{ $t('SettingsPage.language') }}
          </ion-label>
          <ion-select
            interface="popover"
            :selected-text="$t(`SettingsPage.lang.${$i18n.locale.replace('-', '')}`)"
            @ion-change="changeLang($event.detail.value)"
          >
            <ion-select-option value="en-US">
              {{ $t('SettingsPage.lang.enUS') }}
            </ion-select-option>
            <ion-select-option value="fr-FR">
              {{ $t('SettingsPage.lang.frFR') }}
            </ion-select-option>
          </ion-select>
        </ion-item>
        <h5>{{ $t('SettingsPage.preferences') }}</h5>
        <ion-item>
          <ion-label>
            {{ $t('SettingsPage.theme.label') }}
          </ion-label>
          <ion-select
            interface="popover"
            :value="config.theme"
            @ion-change="changeTheme($event.detail.value)"
          >
            <ion-select-option value="dark">
              {{ $t('SettingsPage.theme.dark') }}
            </ion-select-option>
            <ion-select-option value="light">
              {{ $t('SettingsPage.theme.light') }}
            </ion-select-option>
            <ion-select-option value="system">
              {{ $t('SettingsPage.theme.system') }}
            </ion-select-option>
          </ion-select>
        </ion-item>
      </ion-list>
    </ion-content>
  </ion-page>
</template>

<script setup lang = "ts" >
import {
  IonBackButton,
  IonButtons,
  IonContent,
  IonHeader,
  IonPage,
  IonTitle,
  IonToolbar,
  isPlatform,
  IonList,
  IonItem,
  IonToggle,
  IonLabel,
  IonSelect,
  IonSelectOption
} from '@ionic/vue';

import SettingsOption from '@/components/SettingsOption.vue';
import { ref, inject, toRaw } from 'vue';
import { useI18n } from 'vue-i18n';
import { onMounted } from '@vue/runtime-core';
import { toggleDarkMode } from '@/states/darkMode';
import { Config, StorageManager } from '@/services/storageManager';

const { locale } = useI18n();
const storageManager: StorageManager = inject('storageManager')!;
const config = ref<Config>(structuredClone(StorageManager.DEFAULT_CONFIG));

async function changeLang(selectedLang: string): Promise<void> {
  config.value.locale = selectedLang;
  locale.value = selectedLang;
  await storageManager.storeConfig(toRaw(config.value));
}

async function changeTheme(selectedTheme: string): Promise<void> {
  config.value.theme = selectedTheme;
  toggleDarkMode(selectedTheme);
  await storageManager.storeConfig(toRaw(config.value));
}

onMounted(async (): Promise<void> => {
  config.value = await storageManager.retrieveConfig();

  if (!config.value.theme) {
    config.value.theme = 'system';
  }
});
</script>

<style scoped>
h5 {
    margin-bottom: 12px;
}
</style>
