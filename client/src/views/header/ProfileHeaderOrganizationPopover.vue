<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="profile-header-popover-container">
    <div class="header-list">
      <ion-text class="body-sm header-list-email">
        {{ email }}
      </ion-text>
      <user-profile-tag :profile="profile" />
    </div>
    <div class="main-list">
      <download-parsec
        v-if="!isMobile() && isWeb() && !hideParsecDownloadContent"
        @hide-parsec-download="hideParsecDownload"
      />
      <div
        button
        class="main-list__item update-item"
        @click="onOptionClick(ProfilePopoverOption.Update)"
        v-show="updateAvailability.updateAvailable"
      >
        <ion-text class="update-item-text button-medium">
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
          class="item-icon"
        />
        <ion-text class="body item-label">
          {{ $msTranslate('HomePage.topbar.settings') }}
        </ion-text>
      </ion-item>
      <ion-item
        class="main-list__item"
        @click="onOptionClick(ProfilePopoverOption.Device)"
      >
        <ion-icon
          :icon="phonePortrait"
          slot="start"
          class="item-icon"
        />
        <ion-text class="body item-label">
          {{ $msTranslate('HomePage.topbar.devices') }}
        </ion-text>
      </ion-item>
      <ion-item
        class="main-list__item"
        @click="onOptionClick(ProfilePopoverOption.Authentication)"
      >
        <ion-icon
          :icon="fingerPrint"
          slot="start"
          class="item-icon"
        />
        <ion-text class="body item-label">
          {{ $msTranslate('HomePage.topbar.authentication') }}
        </ion-text>
      </ion-item>
      <ion-item
        class="main-list__item"
        @click="onOptionClick(ProfilePopoverOption.Recovery)"
      >
        <ion-icon
          :icon="idCard"
          slot="start"
          class="item-icon"
        />
        <ion-text class="body item-label">
          {{ $msTranslate('HomePage.topbar.recovery') }}
        </ion-text>
      </ion-item>
      <ion-item
        class="main-list__item logout"
        @click="onOptionClick(ProfilePopoverOption.LogOut)"
      >
        <ion-icon
          :icon="logOut"
          slot="start"
          class="item-icon"
        />
        <ion-text class="body item-label">
          {{ $msTranslate('HomePage.topbar.logout') }}
        </ion-text>
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
      <ion-item
        class="footer-list__item"
        @click="Env.Links.openDownloadParsecLink"
        v-if="hideParsecDownloadContent"
      >
        <ion-text class="body-sm version"> {{ $msTranslate('MenuPage.downloadParsec') }} </ion-text>
      </ion-item>
      <ion-item
        v-if="!Env.isStripeDisabled()"
        class="footer-list__item"
        @click="onOptionClick(ProfilePopoverOption.ReportBug)"
      >
        <ion-text class="body-sm version"> {{ $msTranslate('MenuPage.reportBug') }} </ion-text>
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
  ReportBug = 9,
  DownloadParsec = 10,
}
</script>

<script setup lang="ts">
import DownloadParsec from '@/components/misc/DownloadParsec.vue';
import UserProfileTag from '@/components/users/UserProfileTag.vue';
import { isMobile, isWeb, UserProfile } from '@/parsec';
import { APP_VERSION, Env } from '@/services/environment';
import { UpdateAvailabilityData } from '@/services/eventDistributor';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { popoverController } from '@ionic/core';
import { IonIcon, IonItem, IonList, IonText } from '@ionic/vue';
import { cog, fingerPrint, idCard, logOut, phonePortrait } from 'ionicons/icons';
import { inject, onMounted, ref } from 'vue';

defineProps<{
  email: string;
  profile: UserProfile;
  updateAvailability: UpdateAvailabilityData;
}>();

const storageManager: StorageManager = inject(StorageManagerKey)!;
const hideParsecDownloadContent = ref(false);

async function hideParsecDownload(): Promise<void> {
  hideParsecDownloadContent.value = true;
  await storageManager.updateConfig({ hideParsecDownload: true });
}

async function onOptionClick(option: ProfilePopoverOption): Promise<void> {
  await popoverController.dismiss({
    option: option,
  });
}

onMounted(async () => {
  const config = await storageManager.retrieveConfig();
  hideParsecDownloadContent.value = config.hideParsecDownload;
});
</script>

<style lang="scss" scoped>
.profile-header-popover-container {
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
    padding: 0.375rem 0.5rem;
    --min-height: 1rem;
    width: 100%;
    border-radius: var(--parsec-radius-6);

    .item-label {
      margin: 0;
    }

    .item-icon {
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
        background: var(--parsec-color-light-danger-50);
        color: var(--parsec-color-light-danger-700);

        ion-icon {
          color: var(--parsec-color-light-danger-700);
        }
      }
    }

    &:hover {
      background: var(--parsec-color-light-primary-30);
      color: var(--parsec-color-light-primary-700);

      .item-icon {
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
