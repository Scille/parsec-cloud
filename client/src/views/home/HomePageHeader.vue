<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="homepage-header">
    <div class="menu-secondary">
      <ion-button
        v-if="updateAvailability !== null"
        class="update-button form-label"
        id="trigger-update-button"
        fill="clear"
        @click="update()"
      >
        {{ $msTranslate('notificationCenter.newVersionAvailable') }}
      </ion-button>

      <ion-buttons class="menu-secondary-buttons">
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
          @click="openSettingsModal"
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
      </ion-buttons>
    </div>
    <div class="topbar">
      <div class="topbar-left">
        <ion-text
          class="topbar-left__title title-h1"
          v-if="!showBackButton"
        >
          {{ $msTranslate('HomePage.topbar.welcome') }}
        </ion-text>
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
          class="button-default"
          v-if="displayCreateJoin && !showBackButton"
        >
          <ion-icon :icon="add" />
          <span v-if="isLargeDisplay">{{ $msTranslate('HomePage.noExistingOrganization.createOrJoin') }}</span>
        </ion-button>

        <ion-button
          v-if="accountLoggedIn && !showBackButton"
          @click="logOutParsecAccount"
          class="logout-button"
        >
          {{ $msTranslate('loginPage.logOut') }}
        </ion-button>

        <ion-button
          v-else-if="Env.isAccountEnabled() && !showBackButton"
          @click="goToParsecAccountLogin"
          class="login-button"
        >
          <ion-icon :icon="personCircle" />
          {{ $msTranslate('loginPage.title') }}
        </ion-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonButton, IonButtons, IonIcon, modalController, IonText } from '@ionic/vue';
import { add, arrowBack, cog, informationCircle, open, personCircle } from 'ionicons/icons';
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
import { ParsecAccount } from '@/parsec';
import { navigateTo, Routes, watchRoute } from '@/router';

const { isSmallDisplay, isLargeDisplay } = useWindowSize();
const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const eventDistributor = injectionProvider.getDefault().eventDistributor;
let eventCbId: string | null = null;
const updateAvailability: Ref<UpdateAvailabilityData | null> = ref(null);
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;

let hotkeys: HotkeyGroup | null = null;
const accountLoggedIn = ref(false);

const routeWatchCancel = watchRoute(async () => {
  accountLoggedIn.value = ParsecAccount.isLoggedIn();
});

onMounted(async () => {
  hotkeys = hotkeyManager.newHotkeys();
  hotkeys.add({ key: ',', modifiers: Modifiers.Ctrl, platforms: Platforms.Desktop, disableIfModal: true }, openSettingsModal);
  hotkeys.add({ key: 'a', modifiers: Modifiers.Ctrl | Modifiers.Alt, platforms: Platforms.Desktop, disableIfModal: true }, openAboutModal);

  eventCbId = await eventDistributor.registerCallback(Events.UpdateAvailability, async (event: Events, data?: EventData) => {
    if (event === Events.UpdateAvailability) {
      updateAvailability.value = data as UpdateAvailabilityData;
    }
  });
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

async function logOutParsecAccount(): Promise<void> {
  await ParsecAccount.logout();
  await navigateTo(Routes.Account, { skipHandle: true });
}

async function goToParsecAccountLogin(): Promise<void> {
  if (ParsecAccount.isLoggedIn()) {
    await ParsecAccount.logout();
  }
  await navigateTo(Routes.Account, { skipHandle: true });
}

defineProps<{
  showBackButton: boolean;
  backButtonTitle?: Translatable;
  displayCreateJoin: boolean;
}>();

const emits = defineEmits<{
  (e: 'backClick'): void;
  (e: 'customerAreaClick'): void;
  (e: 'createOrJoinOrganizationClick', event: Event): void;
}>();
</script>

<style lang="scss" scoped>
.menu-secondary {
  display: flex;
  justify-content: flex-end;
  width: 100%;
  padding: 0 0 2rem;

  @include ms.responsive-breakpoint('md') {
    flex-direction: column;
    gap: 1rem;
  }

  .update-button {
    margin-right: auto;
    min-height: 1rem;
    background: var(--parsec-color-light-primary-50);
    color: var(--parsec-color-light-primary-700);
    min-height: 1rem;
    border: 1px solid var(--parsec-color-light-primary-100);
    padding: 0 0.325rem;
    border-radius: var(--parsec-radius-32);
    transition: all 150ms linear;

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
  }

  &-buttons {
    display: flex;

    @include ms.responsive-breakpoint('md') {
      margin-left: auto;
    }

    @include ms.responsive-breakpoint('xs') {
      margin-left: 0;
    }

    &__item {
      background: none;
      color: var(--parsec-color-light-secondary-hard-grey);
      transition: all 150ms linear;
      position: relative;
      padding: 0 0.5rem;
      --background-hover: none;

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

      &:not(:first-child)::after {
        content: '';
        position: absolute;
        top: 50%;
        transform: translateY(-50%);
        left: 0;
        height: 80%;
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
        left: 1rem;
        height: 1px;
        width: 0px;
        background: transparent;
        transition: all 150ms linear;
      }

      &:hover {
        color: var(--parsec-color-light-primary-600);

        &::before {
          width: calc(100% - 2rem);
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
  margin-bottom: 2rem;
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
    margin-left: auto;
  }
}

#create-organization-button {
  color: var(--parsec-color-light-secondary-soft-text);
  border-radius: var(--parsec-radius-32);
  transition: all 150ms linear;
  border: 1px solid transparent;
  height: fit-content;

  &::part(native) {
    --background: var(--parsec-color-light-secondary-premiere);
    --background-hover: none;
    border-radius: var(--parsec-radius-32);
    padding: 0.5rem 0.825rem;
    height: auto;
  }

  ion-icon {
    font-size: 1rem;
    margin-right: 0.5rem;

    @include ms.responsive-breakpoint('sm') {
      margin: 0;
    }
  }

  &:hover {
    border: 1px solid var(--parsec-color-light-secondary-light);
    color: var(--parsec-color-light-secondary-text);
  }
}

.login-button,
.logout-button {
  border-radius: var(--parsec-radius-32);
  transition: all 150ms linear;
  height: fit-content;
  min-height: 1rem;

  &::part(native) {
    --background-hover: none;
    border-radius: var(--parsec-radius-32);
    padding: 0.5rem 0.825rem;
  }

  ion-icon {
    font-size: 1rem;
    margin-right: 0.5rem;
  }
}

.login-button {
  color: var(--parsec-color-light-primary-700);
  border: 1px solid var(--parsec-color-light-primary-100);

  &::part(native) {
    --background: var(--parsec-color-light-primary-50);
  }

  &:hover {
    color: var(--parsec-color-light-primary-600);
    box-shadow: var(--parsec-shadow-light);
  }
}

.logout-button {
  color: var(--parsec-color-light-danger-500);
  border: 1px solid transparent;

  &::part(native) {
    --background: transparent;
  }

  &:hover {
    background: var(--parsec-color-light-danger-50);
  }
}
</style>
