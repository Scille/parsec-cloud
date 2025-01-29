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
              v-show="isDesktop() && !usesTestbed()"
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
                    :default-option-key="I18n.getLocale()"
                    @change="changeLang($event.option.key)"
                  />
                </settings-option>
                <!-- change theme -->
                <!-- TODO: REMOVE "disabled=true' : " WHEN DARK MODE WILL BE HERE: https://github.com/Scille/parsec-cloud/issues/5427 -->
                <settings-option
                  :title="'SettingsModal.theme.label'"
                  :description="'SettingsModal.theme.description'"
                >
                  <ms-dropdown
                    class="dropdown"
                    :options="themeOptions"
                    :default-option-key="config.theme"
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
                <!-- send error report -->
                <settings-option
                  :title="'SettingsModal.enableTelemetry.label'"
                  :description="'SettingsModal.enableTelemetry.description'"
                >
                  <ion-toggle v-model="config.enableTelemetry" />
                </settings-option>
              </ion-list>
            </div>
            <!-- advanced -->
            <div
              v-if="settingTab === SettingsTabs.Advanced"
              class="settings-advanced"
            >
              <ion-list class="settings-list">
                <!-- open config dir -->
                <settings-option
                  v-if="isDesktop() && !usesTestbed()"
                  title="SettingsModal.configDir.label"
                  description="SettingsModal.configDir.description"
                >
                  <ion-button @click="openConfigDir">
                    {{ $msTranslate('SettingsModal.configDir.open') }}
                  </ion-button>
                </settings-option>

                <!-- skip file viewers -->
                <settings-option
                  v-if="isDesktop()"
                  title="SettingsModal.skipViewers.label"
                  description="SettingsModal.skipViewers.description"
                >
                  <ion-toggle v-model="config.skipViewers" />
                </settings-option>

                <!-- display unsync files -->
                <settings-option
                  :title="'SettingsModal.unsyncFiles.label'"
                  :description="'SettingsModal.unsyncFiles.description'"
                  v-show="false"
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
import SettingsOption from '@/components/settings/SettingsOption.vue';
import { isMacOS, isDesktop, usesTestbed } from '@/parsec/environment';
import { Config, StorageManager, StorageManagerKey, ThemeManagerKey } from '@/services/storageManager';
import { MsModal, MsOptions, MsDropdown, Locale, I18n, ThemeManager, Theme, LocaleOptions } from 'megashark-lib';
import { IonIcon, IonList, IonPage, IonRadio, IonRadioGroup, IonText, IonToggle, isPlatform, IonButton } from '@ionic/vue';
import { cog, options } from 'ionicons/icons';
import { inject, onMounted, onUnmounted, ref, toRaw, watch } from 'vue';
import { Sentry } from '@/services/sentry';

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

onMounted(async (): Promise<void> => {
  justLoaded = true;
  config.value = await storageManager.retrieveConfig();
  enableTelemetry = config.value.enableTelemetry;
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
