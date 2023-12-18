<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="container">
    <ion-item class="profile-email">
      <ion-text class="body-sm">
        {{ name }}
      </ion-text>
    </ion-item>
    <div class="main-list">
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
        <ion-text class="body-sm version"> {{ $t('MenuPage.about') }} (v{{ getAppVersion() }}) </ion-text>
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
import { getAppVersion } from '@/common/mocks';
import { popoverController } from '@ionic/core';
import { IonIcon, IonItem, IonLabel, IonList, IonText } from '@ionic/vue';
import { cog, logOut, personCircle, phonePortrait } from 'ionicons/icons';

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
  padding: 0;
}

.profile-email {
  color: var(--parsec-color-light-secondary-grey);
  --padding-start: 1rem;
  --padding-end: 1rem;
  --padding-top: 1rem;
  --padding-bottom: 0;
  --min-height: 0;
}

.main-list {
  padding: 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;

  &__item {
    --background: none;
    user-select: none;
    cursor: pointer;
    color: var(--parsec-color-light-secondary-text);
    margin-inline-end: 2px;
    padding: 0.5rem 0.75rem;
    --min-height: 1rem;
    width: 100%;
    border-radius: 0.25rem;

    .item-label {
      margin: 0;
    }

    ion-icon {
      color: var(--parsec-color-light-secondary-text);
      margin: 0;
      margin-inline-end: 0.5rem;
      font-size: 1.125rem;
    }

    &::part(native) {
      padding: 0;
    }

    &.logout {
      color: var(--parsec-color-light-danger-500);

      ion-icon {
        color: var(--parsec-color-light-danger-500);
      }

      &:hover {
        background: var(--parsec-color-light-danger-100);
        color: var(--parsec-color-light-danger-700);

        ion-icon {
          color: var(--parsec-color-light-danger-700);
        }
      }
    }

    &:hover {
      background: var(--parsec-color-light-primary-30);
      color: var(--parsec-color-light-primary-700);

      ion-icon {
        color: var(--parsec-color-light-primary-700);
      }
    }
  }
}

.footer-list {
  background: var(--parsec-color-light-secondary-background);
  padding: 0.5rem 0.5rem;

  &__item {
    --padding-start: 0.5rem;
    --padding-end: 0.5rem;
    --padding-top: 0.5rem;
    --padding-bottom: 0.5rem;
    --min-height: 0;
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
