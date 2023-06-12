<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-list class="container">
    <ion-item class="container__item profile-email">
      <ion-text class="body-sm">
        {{ firstname }} {{ lastname }}
      </ion-text>
    </ion-item>
    <ion-item
      class="container__item"
      @click="onOptionClick(ProfilePopoverOption.MyDevices)"
    >
      <ion-icon
        :icon="phonePortrait"
        slot="start"
      />
      <ion-label class="body">
        {{ $t('HomePage.topbar.myDevices') }}
      </ion-label>
    </ion-item>
    <ion-item
      class="container__item"
      @click="onOptionClick(ProfilePopoverOption.Settings)"
    >
      <ion-icon
        :icon="cog"
        slot="start"
      />
      <ion-label class="body">
        {{ $t('HomePage.topbar.settings') }}
      </ion-label>
    </ion-item>
    <ion-item
      class="container__item"
      @click="onOptionClick(ProfilePopoverOption.Help)"
    >
      <ion-icon
        :icon="helpCircle"
        slot="start"
      />
      <ion-label class="body">
        {{ $t('HomePage.topbar.help') }}
      </ion-label>
    </ion-item>
    <ion-item
      class="container__item logout"
      @click="onOptionClick(ProfilePopoverOption.LogOut)"
    >
      <ion-icon
        :icon="logOut"
        slot="start"
      />
      <ion-label class="body">
        {{ $t('HomePage.topbar.logout') }}
      </ion-label>
    </ion-item>
    <ion-item class="container__item version">
      <ion-text class="body-sm">
        Parsec v2.8.3
      </ion-text>
    </ion-item>
  </ion-list>
</template>

<script lang="ts">
export enum ProfilePopoverOption {
  MyDevices = 0,
  Settings = 1,
  Help = 2,
  LogOut = 3
}
</script>

<script setup lang="ts">
import {
  IonList,
  IonItem,
  IonIcon,
  IonLabel,
  IonText
} from '@ionic/vue';
import {
  phonePortrait,
  cog,
  helpCircle,
  logOut
} from 'ionicons/icons';
import { useI18n } from 'vue-i18n';
import { popoverController } from '@ionic/core';

const { t } = useI18n();

defineProps<{
  firstname: string,
  lastname: string
}>();

function onOptionClick(option: ProfilePopoverOption): void {
  popoverController.dismiss({
    option: option
  });
}
</script>

<style lang="scss" scoped>
.container {
  padding: 0.5rem;
  border-radius: 0.5rem;
  overflow: hidden;
  border: 1px solid var(--parsec-color-light-secondary-light);
  background: var(--parsec-color-light-secondary-background);
}

.profile-email, .version {
  color: var(--parsec-color-light-secondary-grey);
  --padding-start: 0.5rem;
  --padding-end: 0.5rem;
  --padding-top: 0.5rem;
  --padding-bottom: 0.5rem;
  --min-height: 1rem;
}

.container__item {
  --background: none;
  width: 100%;
  --min-height: 1rem;
}

.container__item:not(:first-child):not(:last-child) {
  user-select: none;
  cursor: pointer;
  margin-inline-end: 2px;
  color: var(--parsec-color-light-secondary-text);
  border-radius: 0.25rem;

  &:hover{
    --background: var(--parsec-color-light-primary-30);
    color: var(--parsec-color-light-primary-600);

    ion-icon {
      color: var(--parsec-color-light-primary-600);
    }
  }
  &.logout {
    color: var(--parsec-color-light-danger-500);

    ion-icon {
      color: var(--parsec-color-light-danger-500);
    }

    &:hover {
      --background: var(--parsec-color-light-danger-100);
    }

    &::before{
      content: '';
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 1px;
      background-color: var(--parsec-color-light-secondary-disabled);
    }
  }

  ion-icon {
    color: var(--parsec-color-light-secondary-text);
    margin-inline-end: 0.75rem;
    margin-top: 1rem;
    margin-bottom: 1rem;
  }
}
</style>
