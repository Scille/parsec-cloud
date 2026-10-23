<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-list class="profile-header-popover-container">
    <div class="header-list">
      <ion-text class="header-list-email">
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
        <ion-text class="update-item-text">
          {{ $msTranslate('HomePage.topbar.newVersionAvailable') }}
        </ion-text>
        <ion-text class="update-item-version">
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
        <ion-text class="item-label">
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
        <ion-text class="item-label">
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
        <ion-text class="item-label">
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
        <ion-text class="item-label">
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
        <ion-text class="item-label">
          {{ $msTranslate('HomePage.topbar.logout') }}
        </ion-text>
      </ion-item>
    </div>
    <div class="footer-list">
      <ion-item
        class="footer-list__item"
        @click="onOptionClick(ProfilePopoverOption.Documentation)"
      >
        <ion-text class="footer-list__text"> {{ $msTranslate('HomePage.topbar.documentation') }} </ion-text>
      </ion-item>
      <ion-item
        class="footer-list__item"
        @click="onOptionClick(ProfilePopoverOption.Feedback)"
      >
        <ion-text class="footer-list__text"> {{ $msTranslate('HomePage.topbar.feedback') }} </ion-text>
      </ion-item>
      <ion-item
        class="footer-list__item"
        @click="onOptionClick(ProfilePopoverOption.About)"
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
      <ion-item
        class="footer-list__item"
        @click="onOptionClick(ProfilePopoverOption.ReportBug)"
      >
        <ion-text class="footer-list__text"> {{ $msTranslate('MenuPage.reportBug') }} </ion-text>
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

      ion-icon {
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

      .item-icon {
        color: ms.color('icon-brand-default-hover');
      }
    }
  }

  .update-item {
    background: linear-gradient(ms.color('surface-gradient-from'), ms.color('surface-gradient-to'));
    color: ms.color('text-on-color-heading');
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    padding: ms.spacing('padding-3xl') 0.875rem;
    position: relative;

    &::after {
      content: url('@/assets/images/background/logo-icon-white.svg');
      opacity: ms.opacity('1');
      width: 100%;
      max-width: 80px;
      max-height: 32px;
      position: absolute;
      bottom: 30px;
      right: -16px;
      z-index: 0;
    }

    &-text {
      @include ms.font('label-md-medium');
      flex: 1;
    }

    &-version {
      @include ms.font('body-md-regular');
      opacity: ms.opacity('8');
      display: flex;
      gap: ms.spacing('gap-md');
      z-index: 1;
    }

    &:hover {
      opacity: ms.opacity('9');
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
