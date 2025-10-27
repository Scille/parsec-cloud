<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="account-settings-page-container">
    <ion-radio-group class="tabs-menu">
      <div class="menu-content">
        <div class="menu-list">
          <!-- Settings -->
          <ion-radio
            :value="AccountSettingsTabs.Settings"
            :class="{ 'radio-checked': activeTab === AccountSettingsTabs.Settings }"
            class="menu-list-item button-medium"
            @click="$emit('tabChange', AccountSettingsTabs.Settings)"
          >
            <ion-text class="menu-list-item__title button-medium">
              {{ $msTranslate('HomePage.tabs.settings') }}
            </ion-text>
          </ion-radio>
          <!-- Account -->
          <ion-radio
            :value="AccountSettingsTabs.Account"
            :class="{ 'radio-checked': activeTab === AccountSettingsTabs.Account }"
            class="menu-list-item"
            @click="$emit('tabChange', AccountSettingsTabs.Account)"
          >
            <ion-text class="menu-list-item__title button-medium">
              {{ $msTranslate('HomePage.tabs.account') }}
            </ion-text>
          </ion-radio>
          <!-- Authentication -->
          <ion-radio
            :value="AccountSettingsTabs.Authentication"
            :class="{ 'radio-checked': activeTab === AccountSettingsTabs.Authentication }"
            class="menu-list-item"
            @click="$emit('tabChange', AccountSettingsTabs.Authentication)"
          >
            <ion-text class="menu-list-item__title button-medium">
              {{ $msTranslate('HomePage.tabs.authentication') }}
            </ion-text>
          </ion-radio>
        </div>
      </div>
    </ion-radio-group>
    <div class="profile-content">
      <!-- Settings tab -->
      <div
        v-if="activeTab === AccountSettingsTabs.Settings"
        class="profile-content-item"
        id="settings-profile-content"
      >
        <settings-list />
      </div>
      <!-- account tab -->
      <div
        v-if="activeTab === AccountSettingsTabs.Account"
        class="profile-content-item"
      >
        <div class="item-header">
          <ion-text class="item-header__title title-h3">{{ $msTranslate('HomePage.profile.account.title') }}</ion-text>
        </div>
        <manage-account-page />
      </div>
      <!-- authentication tab -->
      <div
        v-if="activeTab === AccountSettingsTabs.Authentication"
        class="profile-content-item"
      >
        <div class="item-header">
          <ion-text class="item-header__title title-h3">{{ $msTranslate('HomePage.profile.authentication.title') }}</ion-text>
          <ion-text class="item-header__description body">{{ $msTranslate('HomePage.profile.authentication.description') }}</ion-text>
        </div>
        <account-authentication-page />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import SettingsList from '@/components/settings/SettingsList.vue';
import AccountAuthenticationPage from '@/views/account/AccountAuthenticationPage.vue';
import ManageAccountPage from '@/views/account/ManageAccountPage.vue';
import { AccountSettingsTabs } from '@/views/account/types';
import { IonRadio, IonRadioGroup, IonText } from '@ionic/vue';

defineProps<{
  activeTab: AccountSettingsTabs;
}>();

defineEmits<{
  (e: 'tabChange', tab: AccountSettingsTabs): void;
}>();
</script>

<style scoped lang="scss">
* {
  transition: all 0.2s ease-in-out;
}

.account-settings-page-container {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.menu-content,
.profile-content {
  display: flex;
  flex-direction: column;
  max-width: 45rem;

  @include ms.responsive-breakpoint('lg') {
    padding: 0.5rem 0;
  }

  @include ms.responsive-breakpoint('sm') {
    padding: 1.5rem 2rem;
  }

  @include ms.responsive-breakpoint('xs') {
    padding: 1.5rem;
  }
}

.menu-content {
  .menu-list {
    display: flex;
    width: 100%;

    @include ms.responsive-breakpoint('sm') {
      gap: 1rem;
      padding: 0.5rem 0;
      border-radius: var(--parsec-radius-12);
    }

    // eslint-disable-next-line vue-scoped-css/no-unused-selector
    &-item {
      color: var(--parsec-color-light-secondary-hard-grey);
      cursor: pointer;
      position: relative;
      border-bottom: 2px solid transparent;
      display: flex;
      align-items: center;
      gap: 0.375rem;
      width: fit-content;
      overflow: hidden;
      padding-inline: 1rem;

      &__title {
        flex: 1;
        width: 100%;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }

      &::part(label) {
        width: 100%;
        margin: 0;
        padding: 0.5rem 0;
      }

      &::part(container) {
        display: none;
      }

      &.radio-checked {
        color: var(--parsec-color-light-primary-600);
        border-color: var(--parsec-color-light-primary-600);
      }

      &:hover {
        border-color: var(--parsec-color-light-secondary-disabled);
      }
    }
  }
}

.profile-content {
  border-radius: var(--parsec-radius-12);
  overflow: auto;
  padding-bottom: 2rem;

  &:not(:has(#settings-profile-content)) {
    padding: 2rem;
    background: var(--parsec-color-light-secondary-white);
  }

  &-item {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    .item-header {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      &__title {
        font-weight: 600;
        color: var(--parsec-color-light-primary-700);
      }

      &__description {
        color: var(--parsec-color-light-secondary-soft-text);
      }
    }
  }
}
</style>
