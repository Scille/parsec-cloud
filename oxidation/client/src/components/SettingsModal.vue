<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page class="modal-container">
    <!-- top -->
    <ion-header class="ion-margin-bottom">
      <ion-toolbar>
        <ion-title>{{ $t('SettingsPage.pageTitle') }}</ion-title>
        <ion-buttons slot="end">
          <ion-button
            slot="icon-only"
            @click="closeModal()"
          >
            <ion-icon
              :icon="close"
              size="large"
            />
          </ion-button>
        </ion-buttons>
      </ion-toolbar>
    </ion-header>
    <!-- content -->
    <ion-content class="modal-content">
      <div class="menu">
        <!-- menu list -->
        <ion-radio-group
          v-model="showTOS"
          value="parsecServer"
          class="menu-list"
        >
          <ion-radio
            slot="start"
            value="parsecServer"
            class="menu-list__item"
          >
            <ion-icon
              :icon="cog"
            />
            <ion-text class="body">
              {{ $t('SettingsPage.general') }}
            </ion-text>
          </ion-radio>
          <ion-radio
            slot="start"
            value="myOwnServer"
            class="menu-list__item"
          >
            <ion-icon
              :icon="options"
            />
            <ion-text class="body">
              {{ $t('SettingsPage.advanced') }}
            </ion-text>
          </ion-radio>
        </ion-radio-group>
        <!-- list item content -->
        <div class="menu-item-content">
          <!-- general -->
          <div
            v-if="showTOS != 'myOwnServer'"
            class="settings-general"
          >
            <ion-list>
              <!-- synchro wifi -->
              <ion-item>
                <ion-toggle
                  class="toggle-settings"
                >
                  <div class="toggle-settings__content">
                    <ion-text class="body">
                      {{ $t('SettingsPage.synchroWifi.title') }}
                    </ion-text>
                    <ion-text class="body-sm">
                      {{ $t('SettingsPage.synchroWifi.description') }}
                    </ion-text>
                  </div>
                </ion-toggle>
              </ion-item>
              <!-- change lang -->
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
            </ion-list>
          </div>
          <!-- advanced -->
          <div
            v-else
            class="settings-advanced"
          >
            <ion-text>Avancé</ion-text>
          </div>
        </div>
      </div>
    </ion-content>
    <ion-footer>
      <ion-toolbar>
        <ion-buttons
          v-if="pageStep === 1"
          slot="primary"
        >
          <ion-button
            @click="nextStep()"
            :disabled="!firstPageIsFilled()"
          >
            {{ $t('CreateOrganization.next') }}
          </ion-button>
        </ion-buttons>
        <ion-buttons
          v-else
          slot="primary"
        >
          <ion-button
            @click="previousStep()"
            slot="start"
          >
            {{ $t('CreateOrganization.previous') }}
          </ion-button>
          <ion-button type="submit">
            {{ $t('CreateOrganization.done') }}
          </ion-button>
        </ion-buttons>
      </ion-toolbar>
    </ion-footer>
  </ion-page>
</template>

<script setup lang = "ts" >
import {
  IonPage,
  IonHeader,
  IonToolbar,
  IonTitle,
  IonButtons,
  IonButton,
  IonContent,
  IonList,
  IonRadioGroup,
  IonItem,
  IonRadio,
  IonText,
  IonFooter,
  IonIcon,
  IonToggle,
  IonSelect,
  IonSelectOption,
  modalController
} from '@ionic/vue';

import {
  close,
  cog,
  options
} from 'ionicons/icons';
import { ref, inject, toRaw } from 'vue';
import { useI18n } from 'vue-i18n';
import { onMounted } from '@vue/runtime-core';
import { toggleDarkMode } from '@/states/darkMode';
import { Config, StorageManager } from '@/services/storageManager';
import { storageManagerKey } from '@/main';

const { locale } = useI18n();
const storageManager = inject(storageManagerKey)!;
const config = ref<Config>(structuredClone(StorageManager.DEFAULT_CONFIG));
const ownServerUrl = ref('');
const showTOS = ref('parsecServer');

function closeModal(): Promise<boolean> {
  return modalController.dismiss(null, 'cancel');
}

async function changeLang(selectedLang: string): Promise<void> {
  config.value.locale = selectedLang;
  locale.value = selectedLang;
  await storageManager.storeConfig(toRaw(config.value));
}

onMounted(async (): Promise<void> => {
  config.value = await storageManager.retrieveConfig();

  if (!config.value.theme) {
    config.value.theme = 'system';
  }
});
</script>

<style lang="scss" scoped>

.menu {
  display: flex;
  background: beige;
  padding: 2.5rem;
  gap: 2rem;
}
.menu-list {
    display: flex;
    flex-direction: column;
    width: 11.25rem;
    gap: 0.5rem;

    &__item {
      display: flex;
      align-items: center;
      justify-content: center;
      padding: 0.75rem 0.5em;
      color: var(--parsec-color-light-secondary-text);
      border-radius: 4px;

      &::part(container) {
        display: none;
      }

      &.radio-checked {
        color: var(--parsec-color-light-primary-700);
        background: var(--parsec-color-light-primary-30);
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
  background: red;
  display: flex;
  flex-direction: column;
  flex-grow: 1;

  .toggle-settings {
    display: flex;

    &__content {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
    }
  }
}

.flex-row {
  @media screen and (min-width: 576px) {
    display: flex;
    flex-flow: row wrap;
    justify-content: space-between;

    .flex-row-item {
      width: 48%;
    }
  }
}
</style>
