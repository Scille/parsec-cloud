<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-list class="profile-header-popover-container">
    <div class="header-list">
      <ion-text class="header-list-email">
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
        <ion-text class="item-label">
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
        <ion-text class="item-label">
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
        <ion-text class="item-label">
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
        <ion-text class="item-label">
          {{ $msTranslate('HomePage.topbar.logout') }}
        </ion-text>
      </ion-item>
    </div>
    <div class="footer-list">
      <ion-item
        class="footer-list__item"
        @click="onOptionClick(ProfilePopoverHomepageOption.Documentation)"
      >
        <ion-text class="footer-list__text"> {{ $msTranslate('HomePage.topbar.documentation') }} </ion-text>
      </ion-item>
      <ion-item
        class="footer-list__item"
        @click="onOptionClick(ProfilePopoverHomepageOption.Feedback)"
      >
        <ion-text class="footer-list__text"> {{ $msTranslate('HomePage.topbar.feedback') }} </ion-text>
      </ion-item>
      <ion-item
        class="footer-list__item"
        @click="onOptionClick(ProfilePopoverHomepageOption.About)"
      >
        <ion-text class="footer-list__text"> {{ $msTranslate('MenuPage.about') }} (v{{ APP_VERSION }}) </ion-text>
      </ion-item>
      <ion-item
        class="footer-list__item"
        @click="Env.Links.openDownloadParsecLink"
        v-if="hideParsecDownloadContent"
      >
        <ion-text class="footer-list__text"> {{ $msTranslate('MenuPage.downloadParsec') }} </ion-text>
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
  padding: ms.spacing('padding-none');
}

.header-list {
  padding: ms.spacing('padding-3xl');
  color: ms.color('text-base-description');
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-lg');

  &-email {
    @include ms.font('body-sm-regular');
    white-space: nowrap;
    text-overflow: ellipsis;
    overflow: hidden;
  }
}

.main-list {
  padding: ms.spacing('padding-lg');
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-sm');
  border-top: ms.border('thin') solid ms.color('border-base-default');

  &__item {
    --background: none;
    user-select: none;
    cursor: pointer;
    color: ms.color('text-neutral-default');
    margin-inline-end: 2px;
    padding: ms.spacing('padding-lg');
    --min-height: 1rem;
    width: 100%;
    border-radius: ms.radius('md');

    .item-label {
      @include ms.font('label-md-medium');
      margin: 0;
    }

    .item-icon {
      color: ms.color('icon-neutral-default');
      margin: 0;
      margin-inline-end: ms.spacing('padding-lg');
      font-size: 1.125rem;
    }

    &::part(native) {
      padding: ms.spacing('padding-none');
    }

    &.logout {
      color: ms.color('text-error-default');

      .item-icon {
        color: ms.color('icon-error-default');
      }

      &:hover {
        background: ms.color('surface-error-default-subtle-hover');
        color: ms.color('text-error-default-hover');

        ion-icon {
          color: ms.color('icon-error-default-hover');
        }
      }
    }

    &:hover {
      background: ms.color('surface-brand-default-subtle-hover');
      color: ms.color('text-brand-default-hover');

      ion-icon {
        color: ms.color('icon-brand-default-hover');
      }
    }
  }
}

.footer-list {
  background: ms.color('surface-base-default-secondary');
  padding: ms.spacing('padding-lg');

  &__item {
    --padding-start: #{ms.spacing('padding-lg')};
    --padding-end: #{ms.spacing('padding-lg')};
    --padding-top: #{ms.spacing('padding-lg')};
    --padding-bottom: #{ms.spacing('padding-lg')};
    --min-height: 0;
    --background: none;
    color: ms.color('text-neutral-default');
    cursor: pointer;
    width: fit-content;
    display: flex;
    gap: ms.spacing('gap-lg');

    &:hover {
      text-decoration: underline;
    }

    ion-icon {
      font-size: 1.125rem;
      margin: 0;
      margin-right: ms.spacing('padding-lg');
    }
  }

  &__text {
    @include ms.font('body-sm-regular');
  }
}
</style>
