<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="'SettingsModal.pageTitle'"
    :close-button="{ visible: true }"
  >
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
                  {{ $msTranslate('SettingsModal.general') }}
                </ion-text>
              </div>
            </ion-radio>
            <ion-radio
              slot="start"
              :value="SettingsTabs.Advanced"
              class="menu-list__item"
              v-show="false"
            >
              <div class="item-container">
                <ion-icon :icon="options" />
                <ion-text class="body">
                  {{ $msTranslate('SettingsModal.advanced') }}
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
                  :title="'SettingsModal.language.label'"
                  :description="'SettingsModal.language.description'"
                >
                  <ms-dropdown
                    class="dropdown"
                    :options="languageOptions"
                    :default-option-key="getLocale()"
                    @change="changeLang($event.option.key)"
                  />
                </settings-option>
                <!-- change theme -->
                <!-- TODO: REMOVE "'light' ? 'light' : " WHEN DARK MODE WILL BE HERE: https://github.com/Scille/parsec-cloud/issues/5427 -->
                <settings-option
                  :title="'SettingsModal.theme.label'"
                  :description="'SettingsModal.theme.description'"
                >
                  <ms-dropdown
                    class="dropdown"
                    :options="themeOptions"
                    :default-option-key="'light' ? 'light' : config.theme"
                    @change="changeTheme($event.option.key)"
                    :disabled="true"
                  />
                </settings-option>
                <!-- minimize in status bar -->
                <settings-option
                  v-if="isPlatform('electron') && !isMacOS()"
                  :title="'SettingsModal.minimizeToSystemTray.label'"
                  :description="'SettingsModal.minimizeToSystemTray.description'"
                >
                  <ion-toggle v-model="config.minimizeToTray" />
                </settings-option>
                <!-- MacOS-only toggle confirm -->
                <settings-option
                  v-if="isPlatform('electron') && isMacOS()"
                  :title="'SettingsModal.confirmBeforeQuit.label'"
                  :description="'SettingsModal.confirmBeforeQuit.description'"
                >
                  <ion-toggle v-model="config.confirmBeforeQuit" />
                </settings-option>
              </ion-list>
            </div>
            <!-- advanced -->
            <div
              v-if="settingTab === SettingsTabs.Advanced"
              class="settings-advanced"
              v-show="false"
            >
              <ion-list class="settings-list">
                <!-- send error report -->
                <settings-option
                  :title="'SettingsModal.enableTelemetry.label'"
                  :description="'SettingsModal.enableTelemetry.description'"
                >
                  <ion-toggle v-model="config.enableTelemetry" />
                </settings-option>
                <!-- display unsync files -->
                <settings-option
                  :title="'SettingsModal.unsyncFiles.label'"
                  :description="'SettingsModal.unsyncFiles.description'"
                >
                  <ion-toggle v-model="config.unsyncFiles" />
                </settings-option>
                <!-- synchro wifi -->
                <settings-option
                  v-if="false"
                  :title="'SettingsModal.meteredConnection.label'"
                  :description="'SettingsModal.meteredConnection.description'"
                >
                  <ion-toggle v-model="config.meteredConnection" />
                </settings-option>
              </ion-list>
            </div>
          </div>
        </div>
      </div>
    </ion-page>
  </ms-modal>
</template>

<script setup lang="ts">
import { MsDropdown, MsModal, MsOptions } from '@/components/core';
import SettingsOption from '@/components/settings/SettingsOption.vue';
import { isMacOS } from '@/parsec/environment';
import { Config, StorageManager, StorageManagerKey } from '@/services/storageManager';
import { Locale, changeLocale, getLocale } from '@/services/translation';
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
    label: 'SettingsModal.language.values.enUS',
  },
  {
    key: 'fr-FR',
    label: 'SettingsModal.language.values.frFR',
  },
]);

const themeOptions: MsOptions = new MsOptions([
  {
    key: 'dark',
    label: 'SettingsModal.theme.values.dark',
  },
  {
    key: 'light',
    label: 'SettingsModal.theme.values.light',
  },
  {
    key: 'system',
    label: 'SettingsModal.theme.values.system',
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
  --border-radius: var(--parsec-radius-8);
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
      color: var(--parsec-color-light-secondary-grey);
      border-radius: var(--parsec-radius-6);

      .item-container {
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 0.5rem 0.75em;
        gap: 0.375rem;
      }

      &::part(container) {
        display: none;
      }

      &.radio-checked {
        color: var(--parsec-color-light-primary-600);
        background: var(--parsec-color-light-primary-30);
        box-shadow: inset 0px 0px 0px 1px var(--parsec-color-light-primary-600);
      }

      &:hover:not(.radio-checked) {
        background: var(--parsec-color-light-secondary-premiere);
        color: var(--parsec-color-light-secondary-text);
      }

      ion-icon {
        font-size: 1.25rem;
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
