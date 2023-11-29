<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="container">
    <div class="main-list">
      <ion-item class="main-list__item profile-email">
        <ion-text class="body-sm">
          {{ name }}
        </ion-text>
      </ion-item>
      <ion-item
        class="main-list__item"
        @click="onOptionClick(ProfilePopoverOption.MyContactDetails)"
      >
        <ion-icon
          :icon="personCircle"
          slot="start"
        />
        <ion-label class="body item-label">
          {{ $t('HomePage.topbar.myContactDetails') }}
        </ion-label>
      </ion-item>
      <ion-item
        class="main-list__item"
        @click="onOptionClick(ProfilePopoverOption.MyDevices)"
      >
        <ion-icon
          :icon="phonePortrait"
          slot="start"
        />
        <ion-label class="body item-label">
          {{ $t('HomePage.topbar.myDevices') }}
        </ion-label>
      </ion-item>
      <ion-item
        class="main-list__item"
        @click="onOptionClick(ProfilePopoverOption.Settings)"
      >
        <ion-icon
          :icon="cog"
          slot="start"
        />
        <ion-label class="body item-label">
          {{ $t('HomePage.topbar.settings') }}
        </ion-label>
      </ion-item>
      <ion-item
        class="main-list__item logout"
        @click="onOptionClick(ProfilePopoverOption.LogOut)"
      >
        <ion-icon
          :icon="logOut"
          slot="start"
        />
        <ion-label class="body item-label">
          {{ $t('HomePage.topbar.logout') }}
        </ion-label>
      </ion-item>
    </div>
    <div class="footer-list">
      <ion-item
        class="footer-list__item"
        @click="onOptionClick(ProfilePopoverOption.Help)"
      >
        <ion-text class="body-sm"> {{ $t('HomePage.topbar.help') }} </ion-text>
      </ion-item>
      <ion-item
        class="footer-list__item"
        @click="onOptionClick(ProfilePopoverOption.App)"
      >
        <ion-text class="body-sm"> {{ $t('MenuPage.about') }} (v{{ getAppVersion() }}) </ion-text>
      </ion-item>
    </div>
  </ion-list>
</template>

<script lang="ts">
export enum ProfilePopoverOption {
  MyDevices = 0,
  Settings = 1,
  Help = 2,
  LogOut = 3,
  App = 4,
  MyContactDetails = 5,
}
</script>

<script setup lang="ts">
import { IonList, IonItem, IonIcon, IonLabel, IonText } from '@ionic/vue';
import { phonePortrait, cog, logOut, personCircle } from 'ionicons/icons';
import { popoverController } from '@ionic/core';
import { getAppVersion } from '@/common/mocks';

defineProps<{
  name: string;
}>();

function onOptionClick(option: ProfilePopoverOption): void {
  popoverController.dismiss({
    option: option,
  });
}
</script>

<style lang="scss" scoped>
.container {
  border-radius: var(--parsec-radius-8);
  padding: 0;
  overflow: hidden;
  border: 1px solid var(--parsec-color-light-secondary-medium);
  background: var(--parsec-color-light-secondary-inversed-contrast);
}

.main-list {
  padding: 0.5rem;
  gap: 0.5rem;
}

.profile-email {
  color: var(--parsec-color-light-secondary-grey);
}

.profile-email,
.footer-list__item {
  --padding-start: 0.5rem;
  --padding-end: 0.5rem;
  --padding-top: 0.5rem;
  --padding-bottom: 0.5rem;
  --min-height: 0;
}

.main-list__item {
  --background: none;

  &:not(.profile-email) {
    user-select: none;
    cursor: pointer;
    color: var(--parsec-color-light-secondary-text);
    margin-inline-end: 2px;
    --min-height: 1rem;
    width: 100%;
    border-radius: 0.25rem;

    .item-label {
      margin: 0;
    }

    &.logout {
      color: var(--parsec-color-light-danger-500);

      ion-icon {
        color: var(--parsec-color-light-danger-500);
      }

      &:hover {
        --background: var(--parsec-color-light-danger-100);
        color: var(--parsec-color-light-danger-700);

        ion-icon {
          color: var(--parsec-color-light-danger-700);
        }
      }
    }

    ion-icon {
      color: var(--parsec-color-light-secondary-text);
      margin-inline-end: 0.5rem;
      font-size: 1.125rem;
    }

    &:hover {
      background: var(--parsec-color-light-secondary-medium);
      color: var(--parsec-color-light-primary-600);

      ion-icon {
        color: var(--parsec-color-light-primary-600);
      }
    }
  }
}

.footer-list {
  background: var(--parsec-color-light-secondary-background);
  padding: 0.5rem 0.5rem;

  &__item {
    --background: none;
    color: var(--parsec-color-light-secondary-text);
    cursor: pointer;
    width: fit-content;
    display: flex;
    gap: 0.5rem;

    &:hover {
      text-decoration: underline;
    }

    ion-icon {
      font-size: 1.125rem;
      margin: 0;
      margin-right: 0.5rem;
    }
  }
}
</style>
