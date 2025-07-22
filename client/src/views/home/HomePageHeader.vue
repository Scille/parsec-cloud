<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="homepage-header">
    <div
      v-if="updateAvailability !== null"
      class="update-container"
      @click="update()"
    >
      <ion-text class="update-text form-label">{{ $msTranslate('notificationCenter.newVersionAvailable') }}</ion-text>
      <ion-button
        class="update-button form-label"
        id="trigger-update-button"
        fill="clear"
      >
        {{ $msTranslate('notificationCenter.update') }}
        <ion-icon
          :icon="arrowForward"
          class="update-button-icon"
        />
      </ion-button>
    </div>
    <div
      class="menu-secondary"
      v-if="showSecondaryMenu"
    >
      <div class="menu-secondary-buttons">
        <!-- about button -->
        <ion-button
          id="trigger-version-button"
          class="menu-secondary-buttons__item"
          @click="openAboutModal"
        >
          <ion-icon
            :icon="informationCircle"
            class="menu-secondary-buttons__icon"
          />
          <span class="menu-secondary-buttons__text">{{ $msTranslate('MenuPage.about') }}</span>
        </ion-button>
        <!-- doc button -->
        <ion-button
          class="menu-secondary-buttons__item"
          @click="Env.Links.openDocumentationLink"
        >
          {{ isSmallDisplay ? $msTranslate('MenuPage.doc') : $msTranslate('MenuPage.documentation') }}
          <ion-icon :icon="open" />
        </ion-button>
        <!-- contact button -->
        <ion-button
          class="menu-secondary-buttons__item"
          @click="Env.Links.openContactLink"
        >
          {{ $msTranslate('MenuPage.contact') }}
          <ion-icon :icon="open" />
        </ion-button>
        <!-- settings button -->
        <ion-button
          id="trigger-settings-button"
          class="menu-secondary-buttons__item"
          @click="openSettings"
        >
          <ion-icon
            :icon="cog"
            class="menu-secondary-buttons__icon"
          />
          <span class="menu-secondary-buttons__text">
            {{ $msTranslate('MenuPage.settings') }}
          </span>
        </ion-button>
        <!-- customer area button -->
        <ion-button
          class="menu-secondary-buttons__item"
          v-show="!Env.isStripeDisabled()"
          id="trigger-customer-area-button"
          @click="$emit('customerAreaClick')"
          v-if="!showBackButton"
        >
          {{ $msTranslate('HomePage.topbar.customerArea') }}
        </ion-button>
      </div>
    </div>
    <div class="topbar">
      <div class="topbar-left">
        <div
          class="topbar-left-text"
          v-if="!showBackButton"
        >
          <ion-text class="topbar-left-text__title title-h1">
            {{ $msTranslate('HomePage.topbar.welcome') }}
          </ion-text>
          <ion-text class="topbar-left-text__subtitle subtitles-normal">
            {{ $msTranslate('HomePage.organizationList.title') }}
          </ion-text>
        </div>
        <!-- back button -->
        <ion-button
          @click="$emit('backClick')"
          v-if="showBackButton"
          class="topbar-left__back-button"
        >
          <ion-icon
            slot="start"
            :icon="arrowBack"
          />
          {{ $msTranslate(backButtonTitle ?? 'HomePage.topbar.backToList') }}
        </ion-button>
      </div>
      <div class="topbar-right">
        <ion-button
          @click="emits('createOrJoinOrganizationClick', $event)"
          size="default"
          id="create-organization-button"
          class="button-default topbar-right-button button-large"
          v-if="displayCreateJoin && !showBackButton"
        >
          <span class="topbar-right-button__text">{{ $msTranslate('HomePage.noExistingOrganization.createOrJoin') }}</span>
          <ion-icon :icon="caretDown" />
        </ion-button>

        <profile-header-homepage
          v-if="Env.isAccountEnabled() && accountLoggedIn && accountInfo"
          :name="accountInfo.label"
          :email="accountInfo.email"
          :is-online="true"
          class="profile-header-homepage"
          @change-tab="onChangeTab"
        />

        <ion-button
          v-else-if="Env.isAccountEnabled()"
          @click="goToParsecAccountLogin"
          class="login-button"
          fill="clear"
        >
          <ion-icon
            :icon="personCircle"
            class="login-button-icon"
          />
          <ion-text class="login-button-text subtitles-sm">{{ $msTranslate('loginPage.logIn') }}</ion-text>
        </ion-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonButton, IonIcon, modalController, IonText } from '@ionic/vue';
import { arrowBack, arrowForward, caretDown, cog, informationCircle, open, personCircle } from 'ionicons/icons';
import ProfileHeaderHomepage from '@/views/header/ProfileHeaderHomePage.vue';
import { EventData, Events, UpdateAvailabilityData } from '@/services/eventDistributor';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { Translatable, MsModalResult, useWindowSize } from 'megashark-lib';
import { onMounted, onUnmounted, ref, inject, Ref } from 'vue';
import { Env } from '@/services/environment';
import UpdateAppModal from '@/views/about/UpdateAppModal.vue';
import { APP_VERSION } from '@/services/environment';
import { openSettingsModal } from '@/views/settings';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { openAboutModal } from '@/views/about';
import { ParsecAccount, HumanHandle } from '@/parsec';
import { AccountSettingsTabs } from '@/views/account/types';
import { navigateTo, Routes, watchRoute } from '@/router';

const { isSmallDisplay } = useWindowSize();
const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const eventDistributor = injectionProvider.getDefault().eventDistributor;
let eventCbId: string | null = null;
const updateAvailability: Ref<UpdateAvailabilityData | null> = ref(null);
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;

let hotkeys: HotkeyGroup | null = null;
const accountLoggedIn = ref(false);
const accountInfo = ref<HumanHandle | undefined>();
const activeTab = ref<AccountSettingsTabs>(AccountSettingsTabs.Settings);

const routeWatchCancel = watchRoute(async () => {
  accountLoggedIn.value = ParsecAccount.isLoggedIn();
  if (accountLoggedIn.value) {
    const result = await ParsecAccount.getInfo();
    if (result.ok) {
      accountInfo.value = result.value;
    } else {
      accountInfo.value = undefined;
    }
  }
});

async function onChangeTab(tab: AccountSettingsTabs): Promise<void> {
  activeTab.value = tab;
  emits('changeTab', tab);
}

onMounted(async () => {
  hotkeys = hotkeyManager.newHotkeys();
  hotkeys.add({ key: ',', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true }, openSettingsModal);
  hotkeys.add({ key: 'a', modifiers: Modifiers.Ctrl | Modifiers.Alt, platforms: Platforms.Desktop, disableIfModal: true }, openAboutModal);

  eventCbId = await eventDistributor.registerCallback(Events.UpdateAvailability, async (event: Events, data?: EventData) => {
    if (event === Events.UpdateAvailability) {
      updateAvailability.value = data as UpdateAvailabilityData;
    }
  });
  accountLoggedIn.value = ParsecAccount.isLoggedIn();
  const result = await ParsecAccount.getInfo();
  if (result.ok) {
    accountInfo.value = result.value;
  } else {
    accountInfo.value = undefined;
  }
  window.electronAPI.getUpdateAvailability();
  accountLoggedIn.value = ParsecAccount.isLoggedIn();
});

onUnmounted(async () => {
  if (eventCbId) {
    eventDistributor.removeCallback(eventCbId);
  }
  if (hotkeys) {
    hotkeyManager.unregister(hotkeys);
  }
  routeWatchCancel();
});

async function update(): Promise<void> {
  if (!updateAvailability.value) {
    window.electronAPI.log('error', 'Missing update data when trying to update');
    return;
  }
  if (!updateAvailability.value.version) {
    window.electronAPI.log('error', 'Version missing from update data');
    return;
  }

  const modal = await modalController.create({
    component: UpdateAppModal,
    canDismiss: true,
    cssClass: 'update-app-modal',
    backdropDismiss: false,
    componentProps: {
      currentVersion: APP_VERSION,
      targetVersion: updateAvailability.value.version,
    },
  });
  await modal.present();
  const { role } = await modal.onWillDismiss();
  await modal.dismiss();

  if (role === MsModalResult.Confirm) {
    window.electronAPI.prepareUpdate();
  }
}

async function goToParsecAccountLogin(): Promise<void> {
  if (ParsecAccount.isLoggedIn()) {
    await ParsecAccount.logout();
  }
  await navigateTo(Routes.Account, { skipHandle: true });
}

async function openSettings(): Promise<void> {
  if (!Env.isAccountEnabled()) {
    return openSettingsModal();
  } else {
    emits('settingsClick');
  }
}

defineProps<{
  showBackButton: boolean;
  showSecondaryMenu?: boolean;
  backButtonTitle?: Translatable;
  displayCreateJoin: boolean;
}>();

const emits = defineEmits<{
  (e: 'backClick'): void;
  (e: 'customerAreaClick'): void;
  (e: 'createOrJoinOrganizationClick', event: Event): void;
  (e: 'changeTab', tab: AccountSettingsTabs): void;
  (e: 'settingsClick'): void;
}>();
</script>

<style lang="scss" scoped>
.update-container {
  margin-right: auto;
  min-height: 1rem;
  width: 100%;
  margin: 0 0.25rem 2rem;
  background: linear-gradient(90deg, var(--parsec-color-light-primary-700) 0%, var(--parsec-color-light-primary-600) 100%);
  color: var(--parsec-color-light-secondary-background);
  min-height: 2rem;
  padding: 0 0.325rem;
  border-radius: var(--parsec-radius-8);
  transition: all 150ms linear;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  position: relative;
  overflow: hidden;

  &:hover {
    --background-hover: none;
    box-shadow: var(--parsec-shadow-light);
  }

  @include ms.responsive-breakpoint('md') {
    flex-direction: column;
  }

  @include ms.responsive-breakpoint('xs') {
    width: 100%;
  }

  .update-text {
    color: var(--parsec-color-light-secondary-background);
    transition: bottom 150ms linear;
    position: relative;
    bottom: 0;
  }

  .update-button {
    --background-hover: none;
    color: var(--parsec-color-light-secondary-white);
    font-weight: 700;
    transition: top 150ms linear;
    position: absolute;
    top: 2rem;

    &:hover {
      color: var(--parsec-color-light-secondary-background);
    }

    &-icon {
      font-size: 1rem;
      margin-left: 0.25rem;
    }
  }

  &:hover {
    .update-text {
      bottom: 1.5rem;
    }

    .update-button {
      top: -0.125rem;

      @include ms.responsive-breakpoint('md') {
        top: -0.125rem;
      }
    }
  }
}

.menu-secondary {
  display: flex;
  width: 100%;
  padding: 0 0 2rem;
  justify-content: space-between;

  @include ms.responsive-breakpoint('md') {
    flex-direction: column;
    gap: 1rem;
    padding: 0 0 1rem;
  }

  &-buttons {
    display: flex;
    gap: 1rem;

    &__item {
      color: var(--parsec-color-light-secondary-hard-grey);
      transition: all 150ms linear;
      position: relative;
      --background-hover: none;
      display: flex;
      align-items: center;
      gap: 1rem;

      &::part(native) {
        padding: 0;
        border-radius: var(--parsec-radius-8);
        background: none;
        --background-hover: none;
      }

      @include ms.responsive-breakpoint('sm') {
        padding: 0;
      }

      &:nth-of-type(1) {
        @include ms.responsive-breakpoint('xs') {
          order: 3;
          margin-left: auto;
        }
      }

      &:nth-of-type(2) {
        @include ms.responsive-breakpoint('xs') {
          order: 1;
        }
      }

      &:nth-of-type(3) {
        @include ms.responsive-breakpoint('xs') {
          order: 2;
          margin-right: auto;
        }
      }

      &:nth-of-type(4) {
        @include ms.responsive-breakpoint('xs') {
          order: 4;
        }
      }

      ion-icon {
        margin-left: 0.5rem;
        font-size: 1rem;
        color: var(--parsec-color-light-secondary-soft-grey);
      }

      &:hover {
        color: var(--parsec-color-light-secondary-text);

        ion-icon {
          color: var(--parsec-color-light-secondary-hard-grey);
        }
      }

      &:not(:last-child)::after {
        content: '';
        position: relative;
        display: block;
        height: 1rem;
        width: 1px;
        background: var(--parsec-color-light-secondary-disabled);
        transition: all 150ms linear;

        @include ms.responsive-breakpoint('xs') {
          display: none;
        }
      }

      .menu-secondary-buttons__text {
        display: block;

        @include ms.responsive-breakpoint('xs') {
          display: none;
        }
      }

      .menu-secondary-buttons__icon {
        display: none;

        @include ms.responsive-breakpoint('xs') {
          display: block;
          background: var(--parsec-color-light-secondary-premiere);
          padding: 0.5rem;
          border-radius: var(--parsec-radius-8);
          font-size: 1.25rem;
        }
      }
    }

    #trigger-customer-area-button {
      color: var(--parsec-color-light-primary-500);
      border-radius: var(--parsec-radius-8);
      position: relative;

      &::before {
        content: '';
        position: absolute;
        bottom: 0.125rem;
        height: 1px;
        width: 0px;
        background: transparent;
        transition: all 150ms linear;
      }

      &:hover {
        color: var(--parsec-color-light-primary-600);

        &::before {
          width: 100%;
          background: var(--parsec-color-light-primary-600);
        }
      }
    }
  }
}

.topbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  padding: 0 0 2rem;
  gap: 1rem;
  position: relative;
  border-bottom: 1px solid var(--parsec-color-light-secondary-medium);

  @include ms.responsive-breakpoint('sm') {
    border: none;
    padding: 0;
    margin-inline: auto;
  }

  @include ms.responsive-breakpoint('xs') {
    flex-wrap: wrap;
  }

  .topbar-left {
    display: flex;
    align-items: center;
    width: 100%;
    gap: 1.5rem;

    &-text {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      &__title {
        background: var(--parsec-color-light-gradient-background);
        background-clip: text;
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        margin: 0.25rem 0;

        @include ms.responsive-breakpoint('sm') {
          max-width: 16rem;
        }
      }

      &__subtitle {
        color: var(--parsec-color-light-secondary-hard-grey);
        margin: 0;
      }
    }

    &__back-button {
      color: var(--parsec-color-light-secondary-soft-text);
      position: relative;
      --overflow: visible;
      margin: 1px 0;
      padding-left: 1rem;

      &::part(native) {
        background: none;
        --background-hover: none;
        border-radius: var(--parsec-radius-32);
        padding: 0.5rem 0.75rem;
      }

      ion-icon {
        font-size: 1rem;
        margin-right: 0.5rem;
        position: absolute;
        left: -1.5rem;
        transition: left 150ms linear;
      }

      &:hover {
        border-color: transparent;

        ion-icon {
          left: -2rem;
        }
      }
    }
  }

  .topbar-right {
    display: flex;
    gap: 1.25rem;
    justify-content: flex-end;
    width: fit-content;
    align-items: center;
    margin-left: auto;
  }
}

#create-organization-button {
  color: var(--parsec-color-light-secondary-soft-text);
  border-radius: var(--parsec-radius-12);
  transition: all 150ms linear;
  border: 1px solid var(--parsec-color-light-secondary-medium);

  & * {
    pointer-events: none;
  }

  &::part(native) {
    border-radius: var(--parsec-radius-12);
    padding: 0.625rem 0.75rem;
    --background: none;
    --background-hover: none;
    --background-focused: transparent;
    --background-activated: transparent;
  }

  ion-icon {
    font-size: 1rem;
    margin-left: 0.5rem;
  }

  &:hover {
    color: var(--parsec-color-light-secondary-text);
    border: 1px solid var(--parsec-color-light-secondary-light);
  }
}

.login-button {
  border-radius: var(--parsec-radius-12);
  border: 1px solid transparent;
  --background: var(--parsec-color-light-gradient-background);
  --background-hover: var(--parsec-color-light-primary-600);

  &::part(native) {
    border-radius: var(--parsec-radius-12);
  }

  &-icon {
    font-size: 1.25rem;
    color: var(--parsec-color-light-secondary-white);
    margin-right: 0.5rem;
  }

  &-text {
    color: var(--parsec-color-light-secondary-white);
    display: flex;
  }
}
</style>
