<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="page">
    <!-- content -->
    <ion-content class="page-content">
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
              <ion-icon
                :icon="cog"
              />
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
              <ion-icon
                :icon="options"
              />
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
              <!-- dropdown needs to be change depending on this issue : https://github.com/Scille/parsec-cloud/issues/4527 -->
              <ion-item>
                <ion-select
                  interface="popover"
                  :selected-text="$t(`SettingsPage.lang.${$i18n.locale.replace('-', '')}`)"
                  :label="$t('SettingsPage.language')"
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
              <!-- change theme -->
              <!-- dropdown needs to be change depending on this issue : https://github.com/Scille/parsec-cloud/issues/4527 -->
              <ion-item>
                <ion-select
                  interface="popover"
                  :value="config.theme"
                  :label="$t('SettingsPage.theme.label')"
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
          </div>
          <!-- advanced -->
          <div
            v-if="settingTab === SettingsTabs.Advanced"
            class="settings-advanced"
          >
            <ion-list class="settings-list">
              <!-- send error report -->
              <settings-option
                :title="$t('SettingsPage.enableTelemetry')"
                :description="$t('SettingsPage.enableTelemetryDescription')"
                v-model="config.enableTelemetry"
              />
              <!-- minimise in status bar -->
              <settings-option
                v-if="isPlatform('electron')"
                :title="$t('SettingsPage.minimizeToSystemTray')"
                :description="$t('SettingsPage.minimizeToSystemTrayDescription')"
                v-model="config.minimizeToTray"
              />
              <!-- display unsync files -->
              <settings-option
                :title="$t('SettingsPage.unsyncFiles')"
                :description="$t('SettingsPage.unsyncFilesDescription')"
                v-model="config.unsyncFiles"
              />
              <!-- synchro wifi -->
              <settings-option
                v-if="false"
                :title="$t('SettingsPage.meteredConnection')"
                :description="$t('SettingsPage.meteredConnectionDescription')"
                v-model="config.meteredConnection"
              />
            </ion-list>
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang = "ts" >
import {
  IonPage,
  IonContent,
  IonList,
  IonRadioGroup,
  IonItem,
  IonRadio,
  IonText,
  IonIcon,
  IonSelect,
  IonSelectOption,
  isPlatform,
} from '@ionic/vue';

import {
  cog,
  options,
} from 'ionicons/icons';
import { ref, inject, toRaw, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { onMounted, onUnmounted } from '@vue/runtime-core';
import { toggleDarkMode } from '@/states/darkMode';
import { Config, StorageManager } from '@/services/storageManager';
import { StorageManagerKey } from '@/common/injectionKeys';
import SettingsOption from '@/components/settings/SettingsOption.vue';

const { locale } = useI18n();
// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const storageManager = inject(StorageManagerKey)! as StorageManager;
const config = ref<Config>(structuredClone(StorageManager.DEFAULT_CONFIG));

enum SettingsTabs {
  General = 'General',
  Advanced = 'Advanced',
}
const settingTab = ref(SettingsTabs.General);

const configUnwatch = watch(config, async (_, oldConfig) => {
  // No point in saving a config we just loaded
  if (JSON.stringify(toRaw(oldConfig)) !== JSON.stringify(StorageManager.DEFAULT_CONFIG)) {
    await storageManager.storeConfig(toRaw(config.value));
  }
}, { deep: true });

async function changeLang(selectedLang: string): Promise<void> {
  config.value.locale = selectedLang;
  locale.value = selectedLang;
}

async function changeTheme(selectedTheme: string): Promise<void> {
  config.value.theme = selectedTheme;
  toggleDarkMode(selectedTheme);
}

onMounted(async (): Promise<void> => {
  config.value = await storageManager.retrieveConfig();

  if (!config.value.theme) {
    config.value.theme = 'system';
  }
});

onUnmounted(async ():Promise<void> => {
  configUnwatch();
});
</script>

<style lang="scss" scoped>
.page {
  --border-radius: 8px;
  --background: none;
  background: var(--parsec-color-light-secondary-inversed-contrast);

  &-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    margin-bottom: 1.5rem;

    &__toolbar {
      --min-height: 1rem;
    }

    .title-h2 {
      color: var(--parsec-color-light-primary-700);
      padding-inline:0;
    }
  }

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

    &__item {
      color: var(--parsec-color-light-secondary-text);
      border-radius: 4px;

      .item-container {
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 0.75rem 0.5em;
        gap: .5rem;
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
}
</style>
