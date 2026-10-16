<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="homepage-header">
    <div
      v-if="updateAvailability !== null"
      class="update-container"
      @click="update()"
    >
      <ion-text class="update-text">{{ $msTranslate('notificationCenter.newVersionAvailable') }}</ion-text>
      <ion-button
        class="update-button"
        id="trigger-update-button"
        fill="clear"
      >
        {{ $msTranslate('notificationCenter.update') }}
        <ion-icon
          :icon="arrowForward"
          class="update-button-icon button-icon-right"
        />
      </ion-button>
    </div>
    <ion-menu
      side="end"
      content-id="main-content"
      v-if="isSmallDisplay"
      class="menu-secondary-collapse"
    >
      <home-page-secondary-menu-collapse
        :show-customer-area-button="!showBackButton"
        @settings-click="openSettings"
        @customer-area-click="$emit('customerAreaClick')"
        @report-bug-click="openBugReport"
      />
    </ion-menu>

    <home-page-secondary-menu
      v-if="isLargeDisplay && showSecondaryMenu"
      class="homepage-menu-secondary"
      @customer-area-click="$emit('customerAreaClick')"
      @settings-click="openSettings"
      @report-bug-click="openBugReport"
    />

    <div class="topbar">
      <div class="topbar-left">
        <div
          class="topbar-left-text"
          v-if="!showBackButton"
        >
          <ion-text class="topbar-left-text__title">
            {{ $msTranslate('HomePage.topbar.welcome') }}
          </ion-text>
          <ion-text class="topbar-left-text__subtitle">
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
            class="button-icon-left"
          />
          {{ $msTranslate(backButtonTitle ?? 'HomePage.topbar.backToList') }}
        </ion-button>

        <ion-menu-button
          id="main-content"
          class="menu-button"
          v-if="windowWidth < WindowSizeBreakpoints.XS"
        >
          <ion-icon
            :icon="menu"
            class="menu-button__icon"
          />
        </ion-menu-button>
      </div>
      <div class="topbar-right">
        <ion-button
          @click="$emit('recoverClick')"
          class="topbar-right-button color-secondary"
          id="recover-button-header"
          fill="outline"
        >
          <span class="topbar-right-button__text">{{ $msTranslate('HomePage.lostDevice') }}</span>
        </ion-button>
        <ion-button
          @click="emits('createOrJoinOrganizationClick', $event)"
          id="create-organization-button"
          class="topbar-right-button color-secondary"
          v-if="displayCreateJoin && !showBackButton"
        >
          <span class="topbar-right-button__text">{{ $msTranslate('HomePage.noExistingOrganization.createOrJoin') }}</span>
          <ion-icon
            :icon="caretDown"
            class="button-icon-right"
          />
        </ion-button>

        <profile-header-homepage
          v-if="Env.isAccountEnabled() && accountLoggedIn && accountInfo"
          :name="accountInfo.humanHandle.label"
          :email="accountInfo.humanHandle.email"
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
            class="login-button-icon button-icon-left"
          />
          <ion-text class="login-button-text">{{ $msTranslate('loginPage.logIn') }}</ion-text>
        </ion-button>

        <ion-menu-button
          id="main-content"
          class="menu-button"
          v-if="windowWidth >= WindowSizeBreakpoints.XS && windowWidth < WindowSizeBreakpoints.SM"
        >
          <ion-icon
            :icon="menu"
            class="menu-button__icon"
          />
        </ion-menu-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import HomePageSecondaryMenu from '@/components/header/HomePageSecondaryMenu.vue';
import HomePageSecondaryMenuCollapse from '@/components/header/HomePageSecondaryMenuCollapse.vue';
import { openBugReportModal } from '@/components/misc';
import { AccountInfo, ParsecAccount } from '@/parsec';
import { navigateTo, Routes, watchRoute } from '@/router';
import { Env } from '@/services/environment';
import { EventData, Events, UpdateAvailabilityData } from '@/services/eventDistributor';
import { HotkeyGroup, HotkeyManager, HotkeyManagerKey, Modifiers, Platforms } from '@/services/hotkeyManager';
import { Information, InformationLevel, PresentationMode } from '@/services/informationManager';
import { InjectionProvider, InjectionProviderKey } from '@/services/injectionProvider';
import { useUpdateManager } from '@/services/updateManager';
import { openAboutModal, openUpdateAppModal } from '@/views/about';
import { AccountSettingsTabs } from '@/views/account/types';
import ProfileHeaderHomepage from '@/views/header/ProfileHeaderHomePage.vue';
import { openSettingsModal } from '@/views/settings';
import { IonButton, IonIcon, IonMenu, IonMenuButton, IonText, modalController } from '@ionic/vue';
import { arrowBack, arrowForward, caretDown, menu, personCircle } from 'ionicons/icons';
import { Answer, MsModalResult, Translatable, useWindowSize, WindowSizeBreakpoints } from 'megashark-lib';
import { inject, onMounted, onUnmounted, ref, Ref } from 'vue';

const { isSmallDisplay, isLargeDisplay, windowWidth } = useWindowSize();
const { isUpdatePromptAllowed, suppressUpdatePrompt } = useUpdateManager();
const injectionProvider: InjectionProvider = inject(InjectionProviderKey)!;
const eventDistributor = injectionProvider.getDefault().eventDistributor;
const informationManager = injectionProvider.getDefault().informationManager;
const updateAvailability: Ref<UpdateAvailabilityData | null> = ref(null);
let eventCbId: string | null = null;
const hotkeyManager: HotkeyManager = inject(HotkeyManagerKey)!;

let hotkeys: HotkeyGroup | null = null;
const accountLoggedIn = ref(false);
const accountInfo = ref<AccountInfo | undefined>();
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

  eventCbId = await eventDistributor.registerCallback([Events.UpdateAvailability], async (event: Events, data?: EventData) => {
    if (event === Events.UpdateAvailability) {
      updateAvailability.value = data as UpdateAvailabilityData;
      if (isUpdatePromptAllowed()) {
        await update();
      }
    }
  });
  accountLoggedIn.value = ParsecAccount.isLoggedIn();
  const result = await ParsecAccount.getInfo();
  if (result.ok) {
    accountInfo.value = result.value;
  } else {
    accountInfo.value = undefined;
  }
  window.nativeAPI.getUpdateAvailability();
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
  const existingModal = await modalController.getTop();
  if (existingModal) {
    window.nativeAPI.log('debug', 'An existing modal is opened, skipping update prompt');
    return;
  }
  if (!updateAvailability.value) {
    window.nativeAPI.log('error', 'Missing update data when trying to update');
    return;
  }
  if (!updateAvailability.value.version) {
    window.nativeAPI.log('error', 'Version missing from update data');
    return;
  }

  const answer = await openUpdateAppModal(updateAvailability.value.version);

  if (answer === Answer.Yes) {
    window.nativeAPI.prepareUpdate();
  }
  suppressUpdatePrompt();
}

async function goToParsecAccountLogin(): Promise<void> {
  if (ParsecAccount.isLoggedIn()) {
    await ParsecAccount.logout();
  }
  await navigateTo(Routes.Account, { skipHandle: true });
}

async function openSettings(): Promise<void> {
  if (!Env.isAccountEnabled()) {
    await openSettingsModal();
  } else {
    emits('settingsClick');
  }
}

async function openBugReport(): Promise<void> {
  const result = await openBugReportModal();
  if (result === MsModalResult.Confirm) {
    informationManager.present(
      new Information({
        message: 'bugReport.sent',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
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
  (e: 'recoverClick'): void;
}>();
</script>

<style lang="scss" scoped>
.update-container {
  margin-right: auto;
  min-height: 1rem;
  width: 100%;
  margin: 0 ms.spacing('padding-sm') ms.spacing('padding-5xl');
  background: linear-gradient(90deg, ms.color('surface-gradient-to') 0%, ms.color('surface-gradient-from-hover') 100%);
  color: ms.color('text-on-color-heading');
  min-height: 2rem;
  padding: ms.spacing('padding-none') ms.spacing('padding-md');
  border-radius: ms.radius('lg');
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
    @include ms.font('label-md-medium');
    color: ms.color('text-on-color-heading');
    transition: bottom 150ms linear;
    position: relative;
    bottom: 0;
  }

  .update-button {
    --background: none;
    --background-hover: none;
    color: ms.color('text-on-color-heading');
    transition: top 150ms linear;
    position: absolute;
    top: 2rem;

    &:hover {
      color: ms.color('text-on-color-heading');
    }

    &-icon {
      font-size: 1rem;
      margin-left: ms.spacing('padding-sm');
    }
  }

  &:hover {
    .update-text {
      bottom: 1.5rem;
    }

    .update-button {
      top: -0.25rem;

      @include ms.responsive-breakpoint('md') {
        top: 0;
      }
    }
  }
}

.topbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  padding: ms.spacing('padding-none') ms.spacing('padding-none') ms.spacing('padding-5xl');
  gap: ms.spacing('gap-3xl');
  position: relative;
  border-bottom: ms.border('thin') solid ms.color('border-base-default');

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
    gap: ms.spacing('gap-4xl');
    min-height: 4rem;
    user-select: none;

    @include ms.responsive-breakpoint('xs') {
      width: 100%;
      min-height: auto;
    }

    &-text {
      display: flex;
      flex-direction: column;
      gap: ms.spacing('gap-lg');

      @include ms.responsive-breakpoint('xs') {
        width: 100%;
        justify-content: space-around;
      }

      &__title {
        @include ms.font('heading-h2');
        background: linear-gradient(ms.color('surface-gradient-from'), ms.color('surface-gradient-to'));
        background-clip: text;
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        margin: ms.spacing('padding-sm') 0;
      }

      &__subtitle {
        @include ms.font('body-lg-medium');
        color: ms.color('text-base-description');
        margin: 0;

        @include ms.responsive-breakpoint('sm') {
          display: none;
        }
      }
    }

    &__back-button {
      color: ms.color('text-neutral-default');
      position: relative;
      --overflow: visible;
      margin: 1px 0;
      padding-left: ms.spacing('padding-3xl');

      @include ms.responsive-breakpoint('xs') {
        margin: 1px auto 1px 0;
      }

      &::part(native) {
        background: none;
        --background-hover: none;
        border-radius: ms.radius('6xl');
        padding: ms.spacing('padding-lg') ms.spacing('padding-2xl');
      }

      ion-icon {
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
    user-select: none;
    gap: 1.25rem;
    justify-content: flex-end;
    align-items: center;
    margin-left: auto;

    &-button {
      transition: all 150ms linear;

      & * {
        pointer-events: none;
      }
    }
  }

  .menu-button {
    height: 2.5rem;
    width: 2.5rem;
    color: ms.color('text-neutral-default');
    cursor: pointer;
    overflow-y: visible;
    -webkit-app-region: no-drag;

    &__icon {
      font-size: 1.75rem;
    }
  }
}

.login-button {
  border-radius: ms.radius('2xl');
  border: ms.border('thin') solid transparent;
  --background: linear-gradient(#{ms.color('surface-gradient-from')}, #{ms.color('surface-gradient-to')});
  --background-hover: #{ms.color('surface-brand-default-hover')};

  &::part(native) {
    padding: ms.spacing('padding-xl') ms.spacing('padding-2xl');
    border-radius: ms.radius('2xl');

    @include ms.responsive-breakpoint('xs') {
      font-size: 1.125rem;
    }
  }

  &-icon {
    font-size: 1rem;
    color: ms.color('icon-on-color-heading');
    margin-right: ms.spacing('gap-lg');

    @include ms.responsive-breakpoint('xs') {
      font-size: 1.125rem;
    }
  }

  &-text {
    @include ms.font('body-md-medium');
    color: ms.color('text-on-color-heading');
    display: flex;
  }
}
</style>
