<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="container">
    <div class="header-list">
      <ion-text class="body-sm header-list-email">
        {{ email }}
      </ion-text>
      <tag-profile :profile="profile" />
    </div>
    <div class="main-list">
      <div
        button
        class="main-list__item update-item"
        @click="onOptionClick(ProfilePopoverOption.Update)"
        v-show="updateAvailability.updateAvailable"
      >
        <ion-text class="update-item-text subtitles-normal">
          {{ $msTranslate('HomePage.topbar.newVersionAvailable') }}
        </ion-text>
        <ion-text class="update-item-version body">
          <span>{{ $msTranslate('app.name') }}</span>
          <span>{{ $msTranslate({ key: 'formatter.version', data: { version: updateAvailability.version } }) }}</span>
        </ion-text>
      </div>
      <ion-item
        class="main-list__item"
        @click="onOptionClick(ProfilePopoverOption.Settings)"
      >
        <ion-icon
          :icon="cog"
          slot="start"
        />
        <ion-label class="body item-label">
          {{ $msTranslate('HomePage.topbar.settings') }}
        </ion-label>
      </ion-item>
      <ion-item
        class="main-list__item"
        @click="onOptionClick(ProfilePopoverOption.Device)"
      >
        <ion-icon
          :icon="phonePortrait"
          slot="start"
        />
        <ion-label class="body item-label">
          {{ $msTranslate('HomePage.topbar.devices') }}
        </ion-label>
      </ion-item>
      <ion-item
        class="main-list__item"
        @click="onOptionClick(ProfilePopoverOption.Authentication)"
      >
        <ion-icon
          :icon="fingerPrint"
          slot="start"
        />
        <ion-label class="body item-label">
          {{ $msTranslate('HomePage.topbar.authentication') }}
        </ion-label>
      </ion-item>
      <ion-item
        class="main-list__item"
        @click="onOptionClick(ProfilePopoverOption.Recovery)"
      >
        <ion-icon
          :icon="idCard"
          slot="start"
        />
        <ion-label class="body item-label">
          {{ $msTranslate('HomePage.topbar.recovery') }}
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
          {{ $msTranslate('HomePage.topbar.logout') }}
        </ion-label>
      </ion-item>
    </div>
    <div class="footer-list">
      <ion-item
        class="footer-list__item"
        @click="onOptionClick(ProfilePopoverOption.Documentation)"
      >
        <ion-text class="body-sm"> {{ $msTranslate('HomePage.topbar.documentation') }} </ion-text>
      </ion-item>
      <ion-item
        class="footer-list__item"
        @click="onOptionClick(ProfilePopoverOption.Feedback)"
      >
        <ion-text class="body-sm"> {{ $msTranslate('HomePage.topbar.feedback') }} </ion-text>
      </ion-item>
      <ion-item
        class="footer-list__item"
        @click="onOptionClick(ProfilePopoverOption.About)"
      >
        <ion-text class="body-sm version"> {{ $msTranslate('MenuPage.about') }} (v{{ APP_VERSION }}) </ion-text>
      </ion-item>
    </div>
  </ion-list>
</template>

<script lang="ts">
export enum ProfilePopoverOption {
  Settings = 0,
  Device = 1,
  Authentication = 2,
  Recovery = 3,
  Documentation = 4,
  LogOut = 5,
  Feedback = 6,
  About = 7,
  Update = 8,
}
</script>

<script setup lang="ts">
import { APP_VERSION } from '@/services/environment';
import TagProfile from '@/components/users/TagProfile.vue';
import { UserProfile } from '@/parsec';
import { UpdateAvailabilityData } from '@/services/eventDistributor';
import { popoverController } from '@ionic/core';
import { IonIcon, IonItem, IonLabel, IonList, IonText } from '@ionic/vue';
import { cog, fingerPrint, idCard, logOut, phonePortrait } from 'ionicons/icons';

defineProps<{
  email: string;
  profile: UserProfile;
  updateAvailability: UpdateAvailabilityData;
}>();

async function onOptionClick(option: ProfilePopoverOption): Promise<void> {
  await popoverController.dismiss({
    option: option,
  });
}
</script>

<style lang="scss" scoped>
.container {
  padding: 0;
}

.header-list {
  padding: 1rem;
  color: var(--parsec-color-light-secondary-grey);
  display: flex;
  flex-direction: column;
  gap: 0.5rem;

  &-email {
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
  }
}

.main-list {
  padding: 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  border-top: 1px solid var(--parsec-color-light-secondary-medium);

  &__item {
    --background: none;
    user-select: none;
    cursor: pointer;
    color: var(--parsec-color-light-secondary-text);
    margin-inline-end: 2px;
    padding: 0.5rem 0.75rem;
    --min-height: 1rem;
    width: 100%;
    border-radius: var(--parsec-radius-6);

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

  .update-item {
    background: var(--parsec-color-light-gradient);
    color: var(--parsec-color-light-secondary-white);
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: 1rem 0.875rem;
    position: relative;

    &::after {
      content: url('@/assets/images/background/logo-icon-white.svg');
      opacity: 0.1;
      width: 100%;
      max-width: 80px;
      max-height: 32px;
      position: absolute;
      bottom: 30px;
      right: -16px;
      z-index: 0;
    }

    &-text {
      flex: 1;
    }

    &-version {
      opacity: 0.8;
      display: flex;
      gap: 0.375rem;
      z-index: 1;
    }

    &:hover {
      opacity: 0.9;
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
