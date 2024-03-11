<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="page">
    <!-- content -->
    <div class="page-content">
      <div class="menu">
        <!-- menu list -->
        <ion-radio-group
          v-model="settingTab"
          :value="SettingsTabs.General"
          class="menu-list"
        >
          <ion-radio
            slot="start"
            :value="SettingsTabs.General"
            class="menu-list__item"
          >
            <div class="item-container">
              <ion-icon :icon="cog" />
              <ion-text class="body">
                {{ $t('SettingsPage.general') }}
              </ion-text>
            </div>
          </ion-radio>
          <ion-radio
            slot="start"
            :value="SettingsTabs.Advanced"
            class="menu-list__item"
          >
            <div class="item-container">
              <ion-icon :icon="options" />
              <ion-text class="body">
                {{ $t('SettingsPage.advanced') }}
              </ion-text>
            </div>
          </ion-radio>
        </ion-radio-group>
        <!-- list item content -->
        <div class="menu-item-content">
          <!-- general -->
          <div
            v-if="settingTab === SettingsTabs.General"
            class="settings-general"
          >
            <ion-list class="settings-list">
              <!-- change lang -->
              <settings-option
                :title="$t('SettingsPage.language.label')"
                :description="$t('SettingsPage.language.description')"
              >
                <ms-dropdown
                  class="dropdown"
                  :options="languageOptions"
                  :default-option="$i18n.locale"
                  @change="changeLang($event.option.key)"
                />
              </settings-option>
              <!-- change theme -->
              <!-- TODO: REMOVE "'light' ? 'light' : " WHEN DARK MODE WILL BE HERE: https://github.com/Scille/parsec-cloud/issues/5427 -->
              <settings-option
                :title="$t('SettingsPage.theme.label')"
                :description="$t('SettingsPage.theme.description')"
              >
                <ms-dropdown
                  class="dropdown"
                  :options="themeOptions"
                  :default-option="'light' ? 'light' : config.theme"
                  @change="changeTheme($event.option.key)"
                  :disabled="true"
                />
              </settings-option>
              <!-- minimize in status bar -->
              <settings-option
                v-if="isPlatform('electron')"
                :title="$t('SettingsPage.minimizeToSystemTray.label')"
                :description="$t('SettingsPage.minimizeToSystemTray.description')"
              >
                <ion-toggle v-model="config.minimizeToTray" />
              </settings-option>
            </ion-list>
          </div>
          <!-- advanced -->
          <div
            v-if="settingTab === SettingsTabs.Advanced"
            class="settings-advanced"
          >
            <ion-list class="settings-list">
              <!-- send error report -->
              <settings-option
                :title="$t('SettingsPage.enableTelemetry.label')"
                :description="$t('SettingsPage.enableTelemetry.description')"
              >
                <ion-toggle v-model="config.enableTelemetry" />
              </settings-option>
              <!-- display unsync files -->
              <settings-option
                :title="$t('SettingsPage.unsyncFiles.label')"
                :description="$t('SettingsPage.unsyncFiles.description')"
              >
                <ion-toggle v-model="config.unsyncFiles" />
              </settings-option>
              <!-- synchro wifi -->
              <settings-option
                v-if="false"
                :title="$t('SettingsPage.meteredConnection.label')"
                :description="$t('SettingsPage.meteredConnection.description')"
              >
                <ion-toggle v-model="config.meteredConnection" />
              </settings-option>
            </ion-list>
          </div>
        </div>
      </div>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { MsDropdown, MsOptions } from '@/components/core';
import SettingsOption from '@/components/settings/SettingsOption.vue';
import { Config, StorageManager, StorageManagerKey } from '@/services/storageManager';
import { Locale, changeLocale, translate } from '@/services/translation';
import { toggleDarkMode } from '@/states/darkMode';
import { IonIcon, IonList, IonPage, IonRadio, IonRadioGroup, IonText, IonToggle, isPlatform } from '@ionic/vue';
import { cog, options } from 'ionicons/icons';
import { inject, onMounted, onUnmounted, ref, toRaw, watch } from 'vue';

const storageManager: StorageManager = inject(StorageManagerKey)!;
const config = ref<Config>(structuredClone(StorageManager.DEFAULT_CONFIG));
let justLoaded = false;

const languageOptions: MsOptions = new MsOptions([
  {
    key: 'en-US',
    label: translate('SettingsPage.language.values.enUS'),
  },
  {
    key: 'fr-FR',
    label: translate('SettingsPage.language.values.frFR'),
  },
]);

const themeOptions: MsOptions = new MsOptions([
  {
    key: 'dark',
    label: translate('SettingsPage.theme.values.dark'),
  },
  {
    key: 'light',
    label: translate('SettingsPage.theme.values.light'),
  },
  {
    key: 'system',
    label: translate('SettingsPage.theme.values.system'),
  },
]);

enum SettingsTabs {
  General = 'General',
  Advanced = 'Advanced',
}
const settingTab = ref(SettingsTabs.General);

const configUnwatch = watch(
  config,
  async (newConfig) => {
    // No point in saving a config we just loaded
    if (!justLoaded) {
      await storageManager.storeConfig(toRaw(newConfig));
    }
    justLoaded = false;
  },
  { deep: true },
);

async function changeLang(lang: Locale): Promise<void> {
  config.value.locale = lang;
  changeLocale(lang);
}

async function changeTheme(selectedTheme: string): Promise<void> {
  config.value.theme = selectedTheme;
  toggleDarkMode(selectedTheme);
}

onMounted(async (): Promise<void> => {
  justLoaded = true;
  config.value = await storageManager.retrieveConfig();

  if (!config.value.theme) {
    config.value.theme = 'system';
  }
});

onUnmounted(async (): Promise<void> => {
  configUnwatch();
});
</script>

<style lang="scss" scoped>
.page {
  --border-radius: 8px;
  --background: none;

  &-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .menu {
    display: flex;
    gap: 2rem;
  }

  .menu-list {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
    max-width: 11.25rem;

    // eslint-disable-next-line vue-scoped-css/no-unused-selector
    &__item {
      color: var(--parsec-color-light-secondary-text);
      border-radius: 4px;

      .item-container {
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 0.75rem 0.5em;
        gap: 0.5rem;
      }

      &::part(container) {
        display: none;
      }

      &.radio-checked {
        color: var(--parsec-color-light-primary-600);
        background: var(--parsec-color-light-primary-30);
        box-shadow: inset 0px 0px 0px 1px var(--parsec-color-light-primary-600);
      }

      &:hover {
        background: var(--parsec-color-light-primary-30);
      }

      ion-icon {
        font-size: 1.5rem;
      }
    }
  }

  .menu-item-content {
    display: flex;
    flex-direction: column;
    flex-grow: 1;
    max-width: 40rem;

    .settings-list {
      display: flex;
      flex-direction: column;
      gap: 1.5rem;
      padding-top: 0px;
      padding-bottom: 0px;
    }
  }

  .dropdown {
    width: var(--popover-dropdown-settings-width);
  }
}
</style>
