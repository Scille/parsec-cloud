<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="account-settings-page-container">
    <ion-radio-group
      v-if="isLargeDisplay || isSmallDisplay"
      class="tabs-menu"
    >
      <div class="menu-content">
        <div class="menu-list">
          <!-- Settings -->
          <ion-radio
            :value="AccountSettingsTabs.Settings"
            :class="{ 'radio-checked': activeTab === AccountSettingsTabs.Settings }"
            class="menu-list-item"
            @click="onTabChange(AccountSettingsTabs.Settings)"
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
            @click="onTabChange(AccountSettingsTabs.Account)"
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
            @click="onTabChange(AccountSettingsTabs.Authentication)"
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
      >
        <div class="item-header">
          <ion-text class="item-header__title title-h3">{{ $msTranslate('SettingsModal.pageTitle') }}</ion-text>
          <ion-text class="item-header__description body">{{ $msTranslate('SettingsModal.description') }}</ion-text>
        </div>
        <settings-list />
      </div>
      <!-- account tab -->
      <div
        v-if="activeTab === AccountSettingsTabs.Account"
        class="profile-content-item"
      >
        <div class="item-header">
          <ion-text class="item-header__title title-h3">{{ $msTranslate('Account.title') }}</ion-text>
          <ion-text class="item-header__description body">{{ $msTranslate('Account.description') }}</ion-text>
        </div>
      </div>
      <!-- authentication tab -->
      <div
        v-if="activeTab === AccountSettingsTabs.Authentication"
        class="profile-content-item"
      >
        <div class="item-header">
          <ion-text class="item-header__title title-h3">{{ $msTranslate('Authentication.title') }}</ion-text>
          <ion-text class="item-header__description body">{{ $msTranslate('Authentication.description') }}</ion-text>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import SettingsList from '@/components/settings/SettingsList.vue';
import { IonRadio, IonRadioGroup, IonText } from '@ionic/vue';
import { Ref, ref } from 'vue';
import { useWindowSize } from 'megashark-lib';
import { AccountSettingsTabs } from '@/views/account/types';

const props = defineProps<{
  activeTab: AccountSettingsTabs;
}>();

const activeTab: Ref<AccountSettingsTabs> = ref(props.activeTab || AccountSettingsTabs.Settings);

const { isSmallDisplay, isLargeDisplay } = useWindowSize();

function onTabChange(tab: AccountSettingsTabs): void {
  activeTab.value = tab;
}
</script>

<style scoped lang="scss">
* {
  transition: all 0.2s ease-in-out;
}

.account-settings-page-container {
  background: var(--parsec-color-light-secondary-premiere);
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 2rem;
  padding: 2.5rem 5rem 0;
}

.menu-content,
.profile-content {
  display: flex;
  flex-direction: column;
  max-width: 37.5rem;

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
  gap: 0.75rem;
  padding: 0 2rem;

  .menu-list {
    display: flex;
    gap: 1.5rem;
    width: 100%;

    @include ms.responsive-breakpoint('sm') {
      gap: 1rem;
      padding: 0.5rem 0;
      background: var(--parsec-color-light-secondary-background);
      border-radius: var(--parsec-radius-12);
    }

    // eslint-disable-next-line vue-scoped-css/no-unused-selector
    &-item {
      color: var(--parsec-color-light-secondary-hard-grey);
      cursor: pointer;
      position: relative;
      border-bottom: 1px solid transparent;
      display: flex;
      align-items: center;
      gap: 0.375rem;
      width: fit-content;
      overflow: hidden;

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

      &:hover:not(.radio-checked) {
        background: var(--parsec-color-light-secondary-premiere);
        color: var(--parsec-color-light-secondary-text);
      }

      &:not(:last-child)::after {
        @include ms.responsive-breakpoint('sm') {
          content: '';
          position: absolute;
          right: 0;
          margin-top: 0.5rem;
          width: calc(100% - 2.25rem);
          height: 1px;
          background: var(--parsec-color-light-secondary-medium);
        }
      }
    }
  }
}

.profile-content {
  padding: 2rem;
  background: var(--parsec-color-light-secondary-white);
  border-radius: var(--parsec-radius-12);

  &-item {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    .item-header {
      display: flex;
      flex-direction: column;
      gap: 0.25rem;

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
