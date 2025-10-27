<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="profile-header-popover-container">
    <div class="header-list">
      <ion-text class="body-sm header-list-email">
        {{ email }}
      </ion-text>
    </div>
    <div class="main-list">
      <download-parsec
        v-if="!isMobile() && isWeb() && !hideParsecDownloadContent"
        @hide-parsec-download="hideParsecDownload"
      />
      <ion-item
        class="main-list__item"
        @click="onOptionClick(ProfilePopoverHomepageOption.Settings)"
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
        @click="onOptionClick(ProfilePopoverHomepageOption.Account)"
      >
        <ion-icon
          :icon="person"
          slot="start"
          class="item-icon"
        />
        <ion-text class="body item-label">
          {{ $msTranslate('HomePage.topbar.account') }}
        </ion-text>
      </ion-item>
      <ion-item
        class="main-list__item"
        @click="onOptionClick(ProfilePopoverHomepageOption.Authentication)"
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
        class="main-list__item logout"
        @click="logOutParsecAccount"
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
        @click="onOptionClick(ProfilePopoverHomepageOption.Documentation)"
      >
        <ion-text class="body-sm"> {{ $msTranslate('HomePage.topbar.documentation') }} </ion-text>
      </ion-item>
      <ion-item
        class="footer-list__item"
        @click="onOptionClick(ProfilePopoverHomepageOption.Feedback)"
      >
        <ion-text class="body-sm"> {{ $msTranslate('HomePage.topbar.feedback') }} </ion-text>
      </ion-item>
      <ion-item
        class="footer-list__item"
        @click="onOptionClick(ProfilePopoverHomepageOption.About)"
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
    </div>
  </ion-list>
</template>

<script lang="ts">
export enum ProfilePopoverHomepageOption {
  Settings = 0,
  Account = 1,
  Authentication = 2,
  Documentation = 3,
  LogOut = 4,
  Feedback = 5,
  About = 6,
}
</script>

<script setup lang="ts">
import DownloadParsec from '@/components/misc/DownloadParsec.vue';
import { isMobile, isWeb } from '@/parsec';
import { ParsecAccount } from '@/parsec/account';
import { navigateTo, Routes } from '@/router';
import { APP_VERSION, Env } from '@/services/environment';
import { StorageManager, StorageManagerKey } from '@/services/storageManager';
import { AccountSettingsTabs } from '@/views/account/types';
import { popoverController } from '@ionic/core';
import { IonIcon, IonItem, IonList, IonText } from '@ionic/vue';
import { cog, fingerPrint, logOut, person } from 'ionicons/icons';
import { inject, onMounted, ref } from 'vue';

defineProps<{
  email: string;
}>();

const storageManager: StorageManager = inject(StorageManagerKey)!;
const hideParsecDownloadContent = ref(false);

async function onOptionClick(option: ProfilePopoverHomepageOption): Promise<void> {
  let tab = null;
  if (option === ProfilePopoverHomepageOption.Settings) {
    tab = AccountSettingsTabs.Settings;
  } else if (option === ProfilePopoverHomepageOption.Account) {
    tab = AccountSettingsTabs.Account;
  } else if (option === ProfilePopoverHomepageOption.Authentication) {
    tab = AccountSettingsTabs.Authentication;
  }
  await popoverController.dismiss({
    option,
    tab,
  });
}

async function hideParsecDownload(): Promise<void> {
  hideParsecDownloadContent.value = true;
  await storageManager.updateConfig({ hideParsecDownload: true });
}

async function logOutParsecAccount(): Promise<void> {
  await popoverController.dismiss();
  await ParsecAccount.logout();
  await navigateTo(Routes.Account, { skipHandle: true });
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

      .item-icon {
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
