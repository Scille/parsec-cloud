<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="settings-list-container">
    <ion-list class="settings-list">
      <div class="settings-list-group">
        <ion-text class="title-large settings-list-group__title title-h3">
          {{ $msTranslate('SettingsModal.titles.display') }}
        </ion-text>
        <!-- change lang -->
        <settings-option
          class="settings-list__item"
          title="SettingsModal.language.label"
          description="SettingsModal.language.description"
        >
          <ms-dropdown
            class="dropdown"
            :options="languageOptions"
            :default-option-key="I18n.getLocale()"
            @change="changeLang($event.option.key)"
          />
        </settings-option>
        <!-- change theme -->
        <!-- TODO: REMOVE "disabled=true' : " WHEN DARK MODE WILL BE HERE: https://github.com/Scille/parsec-cloud/issues/5427 -->
        <settings-option
          class="settings-list__item"
          title="SettingsModal.theme.label"
          description="SettingsModal.theme.description"
        >
          <ms-dropdown
            class="dropdown"
            :options="themeOptions"
            :default-option-key="config.theme"
            @change="changeTheme($event.option.key)"
            :disabled="true"
          />
        </settings-option>
      </div>

      <div class="settings-list-group">
        <ion-text class="title-large settings-list-group__title title-h3">
          {{ $msTranslate('SettingsModal.titles.config') }}
        </ion-text>
        <!-- minimize in status bar -->
        <settings-option
          v-if="isPlatform('electron') && !isMacOS()"
          class="settings-list__item"
          title="SettingsModal.minimizeToSystemTray.label"
          description="SettingsModal.minimizeToSystemTray.description"
        >
          <ion-toggle v-model="config.minimizeToTray" />
        </settings-option>
        <!-- MacOS-only toggle confirm -->
        <settings-option
          v-if="isPlatform('electron') && isMacOS()"
          class="settings-list__item"
          title="SettingsModal.confirmBeforeQuit.label"
          description="SettingsModal.confirmBeforeQuit.description"
        >
          <ion-toggle v-model="config.confirmBeforeQuit" />
        </settings-option>
        <!-- send error report -->
        <settings-option
          class="settings-list__item"
          title="SettingsModal.enableTelemetry.label"
          description="SettingsModal.enableTelemetry.description"
        >
          <ion-toggle v-model="config.enableTelemetry" />
        </settings-option>
        <!-- see logs -->
        <settings-option
          class="settings-list__item"
          title="SettingsModal.seeLogs.label"
          description="SettingsModal.seeLogs.description"
        >
          <ion-button
            @click="openLogs"
            fill="clear"
            class="see-logs-button"
          >
            {{ $msTranslate('SettingsModal.seeLogs.seeLogs') }}
          </ion-button>
        </settings-option>
        <!-- synchro wifi -->
        <settings-option
          v-if="false"
          class="settings-list__item"
          title="SettingsModal.meteredConnection.label"
          description="SettingsModal.meteredConnection.description"
        >
          <ion-toggle v-model="config.meteredConnection" />
        </settings-option>
        <!-- open config dir -->
        <settings-option
          v-if="isDesktop() && !usesTestbed()"
          class="settings-list__item"
          title="SettingsModal.configDir.label"
          description="SettingsModal.configDir.description"
        >
          <ion-button @click="openConfigDir">
            {{ $msTranslate('SettingsModal.configDir.open') }}
          </ion-button>
        </settings-option>
      </div>
    </ion-list>

    <ion-list class="settings-list">
      <div
        v-if="Env.isAccountEnabled()"
        class="settings-list-group"
      >
        <ion-text class="title-large settings-list-group__title title-h3">
          {{ $msTranslate('SettingsModal.titles.account') }}
        </ion-text>

        <settings-option
          class="settings-list__item"
          title="SettingsModal.skipAccount.label"
          description="SettingsModal.skipAccount.description"
        >
          <ion-toggle v-model="config.skipAccount" />
        </settings-option>
      </div>

      <div
        v-if="isDesktop()"
        class="settings-list-group"
      >
        <ion-text class="title-large settings-list-group__title title-h3">
          {{ $msTranslate('SettingsModal.titles.file') }}
        </ion-text>
        <!-- display unsync files -->
        <settings-option
          v-if="false"
          class="settings-list__item"
          title="SettingsModal.unsyncFiles.label"
          description="SettingsModal.unsyncFiles.description"
        >
          <ion-toggle v-model="config.unsyncFiles" />
        </settings-option>
        <!-- skip file viewers -->
        <settings-option
          v-if="isDesktop()"
          class="settings-list__item"
          title="SettingsModal.skipViewers.label"
          description="SettingsModal.skipViewers.description"
        >
          <ion-toggle v-model="config.skipViewers" />
        </settings-option>
      </div>
    </ion-list>
  </div>
</template>

<script setup lang="ts">
import { openLogDisplayModal } from '@/components/misc';
import SettingsOption from '@/components/settings/SettingsOption.vue';
import { isDesktop, isMacOS, usesTestbed } from '@/parsec/environment';
import { Env } from '@/services/environment';
import { Sentry } from '@/services/sentry';
import { Config, StorageManager, StorageManagerKey, ThemeManagerKey } from '@/services/storageManager';
import { IonButton, IonList, IonText, IonToggle, isPlatform } from '@ionic/vue';
import { I18n, Locale, LocaleOptions, MsDropdown, MsOptions, Theme, ThemeManager } from 'megashark-lib';
import { inject, onMounted, onUnmounted, ref, toRaw, watch } from 'vue';

const themeManager: ThemeManager = inject(ThemeManagerKey)!;
const storageManager: StorageManager = inject(StorageManagerKey)!;
const config = ref<Config>(structuredClone(StorageManager.DEFAULT_CONFIG));
let justLoaded = false;
// Local backup, since watch in deep mode gives the same object
// for both newValue and oldValue.
let enableTelemetry = config.value.enableTelemetry;

const languageOptions: MsOptions = new MsOptions(LocaleOptions);

const themeOptions: MsOptions = new MsOptions([
  {
    key: Theme.Dark,
    label: 'SettingsModal.theme.values.dark',
  },
  {
    key: Theme.Light,
    label: 'SettingsModal.theme.values.light',
  },
  {
    key: Theme.System,
    label: 'SettingsModal.theme.values.system',
  },
]);

const configUnwatch = watch(
  config,
  async (newConfig) => {
    // No point in saving a config we just loaded
    if (!justLoaded) {
      await storageManager.storeConfig(toRaw(newConfig));
    }
    justLoaded = false;
    if (newConfig.enableTelemetry !== enableTelemetry) {
      enableTelemetry = newConfig.enableTelemetry;
      if (window.isDev()) {
        console.log('Dev mode, not doing anything, Sentry stays disabled');
        Sentry.disable();
      } else {
        newConfig.enableTelemetry ? Sentry.enable() : Sentry.disable();
      }
    }
  },
  { deep: true },
);

async function changeLang(lang: Locale): Promise<void> {
  config.value.locale = lang;
  I18n.changeLocale(lang);
}

async function changeTheme(selectedTheme: Theme): Promise<void> {
  config.value.theme = selectedTheme as Theme;
  themeManager.use(selectedTheme as Theme);
}

async function openConfigDir(): Promise<void> {
  window.electronAPI.openConfigDir();
}

async function openLogs(): Promise<void> {
  await openLogDisplayModal();
}

onMounted(async (): Promise<void> => {
  justLoaded = true;
  config.value = await storageManager.retrieveConfig();
  enableTelemetry = config.value.enableTelemetry;
});

onUnmounted(async (): Promise<void> => {
  configUnwatch();
});
</script>

<style scoped lang="scss">
.settings-list {
  display: flex;
  flex-direction: column;
  background: none;
  padding-top: 0px;
  padding-bottom: 0px;
  gap: 2rem;

  &-container {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  &-group {
    background: var(--parsec-color-light-secondary-white);
    border-radius: var(--parsec-radius-12);
    padding: 1.5rem 0 0;

    &__title {
      color: var(--parsec-color-light-primary-700);
      padding-inline: 1.5rem;
    }
  }

  &__item {
    padding: 1.25rem 1.5rem;
    border-bottom: 1px solid var(--parsec-color-light-secondary-medium);

    &:last-child {
      border-bottom: none;
    }
  }
}

.dropdown {
  width: var(--popover-dropdown-settings-width);
}
</style>
