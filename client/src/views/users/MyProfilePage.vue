<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="page">
    <ion-content :fullscreen="true">
      <div class="page-container">
        <ion-radio-group
          v-model="myProfileTab"
          class="profile-menu"
          @ion-change="switchPage"
        >
          <div class="menu-list">
            <ion-text class="menu-list__title subtitles-sm">
              {{ $msTranslate('MyProfilePage.tabs.account.title') }}
            </ion-text>
            <!-- Settings -->
            <ion-radio
              slot="start"
              :value="ProfilePages.Settings"
              class="menu-list__item"
            >
              <div class="item-container">
                <ion-icon :icon="cog" />
                <ion-text class="item-container__text body">
                  {{ $msTranslate('MyProfilePage.tabs.account.settings') }}
                </ion-text>
              </div>
            </ion-radio>
            <!-- Devices -->
            <ion-radio
              slot="start"
              :value="ProfilePages.Devices"
              class="menu-list__item"
            >
              <div class="item-container">
                <ion-icon :icon="phonePortrait" />
                <ion-text class="item-container__text body">
                  {{ $msTranslate('MyProfilePage.tabs.account.devices') }}
                </ion-text>
              </div>
            </ion-radio>
            <!-- Authentication -->
            <ion-radio
              slot="start"
              :value="ProfilePages.Authentication"
              class="menu-list__item"
            >
              <div class="item-container">
                <ion-icon :icon="fingerPrint" />
                <ion-text class="item-container__text body">
                  {{ $msTranslate('MyProfilePage.tabs.account.authentication') }}
                </ion-text>
              </div>
            </ion-radio>
            <!-- Organization Recovery -->
            <ion-radio
              slot="start"
              :value="ProfilePages.Recovery"
              class="menu-list__item"
            >
              <div class="item-container">
                <ion-icon :icon="idCard" />
                <ion-text class="item-container__text body">
                  {{ $msTranslate('MyProfilePage.tabs.account.recovery') }}
                </ion-text>
              </div>
            </ion-radio>
          </div>

          <div class="menu-list">
            <ion-text class="menu-list__title subtitles-sm">
              {{ $msTranslate('MyProfilePage.tabs.support.title') }}
            </ion-text>
            <!-- Documentation -->
            <ion-text
              @click="openDocumentationPopover"
              class="menu-list__item"
            >
              <div class="item-container">
                <ion-icon :icon="documentText" />
                <ion-text class="item-container__text body">
                  {{ $msTranslate('MyProfilePage.tabs.support.documentation') }}
                </ion-text>
                <ion-icon :icon="open" />
              </div>
            </ion-text>
            <!-- Help & comments -->
            <ion-text
              @click="openFeedbackPopover"
              class="menu-list__item"
            >
              <div class="item-container">
                <ion-icon :icon="chatbubbles" />
                <ion-text class="item-container__text body">
                  {{ $msTranslate('MyProfilePage.tabs.support.help') }}
                </ion-text>
                <ion-icon :icon="open" />
              </div>
            </ion-text>
            <!-- About -->
            <ion-radio
              slot="start"
              :value="ProfilePages.About"
              class="menu-list__item"
            >
              <div class="item-container">
                <ion-icon :icon="information" />
                <ion-text class="item-container__text body">
                  {{ $msTranslate('MyProfilePage.tabs.support.about') }}
                </ion-text>
              </div>
            </ion-radio>
          </div>
        </ion-radio-group>
        <div class="profile-content">
          <!-- profile info -->
          <div
            class="profile-content-infos"
            v-if="clientInfo"
          >
            <ion-text class="profile-content-infos__name title-h2">
              {{ clientInfo.humanHandle.label }}
            </ion-text>
            <ion-text class="profile-content-infos__email body">
              {{ clientInfo.humanHandle.email }}
            </ion-text>
            <tag-profile :profile="clientInfo.currentProfile" />
          </div>
          <!-- Settings tab -->
          <div
            v-if="myProfileTab === ProfilePages.Settings"
            class="profile-content-item"
          >
            <div class="item-header">
              <ion-text class="item-header__title title-h3">{{ $msTranslate('SettingsModal.pageTitle') }}</ion-text>
              <ion-text class="item-header__description body">{{ $msTranslate('SettingsModal.description') }}</ion-text>
            </div>
            <settings-list />
          </div>
          <!-- devices tab -->
          <div
            v-if="myProfileTab === ProfilePages.Devices"
            class="profile-content-item"
          >
            <div class="item-header">
              <ion-text class="item-header__title title-h3">{{ $msTranslate('DevicesPage.title') }}</ion-text>
              <ion-text class="item-header__description body">{{ $msTranslate('DevicesPage.description') }}</ion-text>
            </div>
            <devices-page class="devices" />
          </div>
          <!-- authentication tab -->
          <div
            v-if="myProfileTab === ProfilePages.Authentication"
            class="profile-content-item"
          >
            <div class="item-header">
              <ion-text class="item-header__title title-h3">{{ $msTranslate('Authentication.title') }}</ion-text>
              <ion-text class="item-header__description body">{{ $msTranslate('Authentication.description') }}</ion-text>
            </div>
            <authentication-page />
          </div>
          <!-- organization recovery tab -->
          <div
            v-if="myProfileTab === ProfilePages.Recovery"
            class="profile-content-item recovery"
          >
            <div class="item-header">
              <ion-text class="item-header__title title-h3">{{ $msTranslate('OrganizationRecovery.title') }}</ion-text>
              <ion-text class="item-header__description body">
                <span>{{ $msTranslate('OrganizationRecovery.done.subtitle') }}</span>
                <span>{{ $msTranslate('OrganizationRecovery.done.subtitle2') }}</span>
              </ion-text>
            </div>
            <organization-recovery-page />
          </div>
          <!-- About tab -->
          <div
            v-if="myProfileTab === ProfilePages.About"
            class="profile-content-item"
          >
            <div class="item-header">
              <ion-text class="item-header__title title-h3">{{ $msTranslate('AboutPage.title') }}</ion-text>
              <ion-text class="item-header__description body">{{ $msTranslate('AboutPage.description') }}</ion-text>
            </div>
            <about-view />
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import TagProfile from '@/components/users/TagProfile.vue';
import { ClientInfo, getClientInfo, getCurrentAvailableDevice } from '@/parsec';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import DevicesPage from '@/views/devices/DevicesPage.vue';
import SettingsList from '@/components/settings/SettingsList.vue';
import AboutView from '@/views/about/AboutView.vue';
import AuthenticationPage from '@/views/profile/AuthenticationPage.vue';
import OrganizationRecoveryPage from '@/views/profile/OrganizationRecoveryPage.vue';
import { IonContent, IonIcon, IonPage, IonRadio, IonRadioGroup, IonText } from '@ionic/vue';
import { open, phonePortrait, cog, fingerPrint, idCard, documentText, information, chatbubbles } from 'ionicons/icons';
import { Ref, inject, onMounted, onUnmounted, ref } from 'vue';
import { Env } from '@/services/environment';
import { getCurrentRouteName, getCurrentRouteQuery, navigateTo, Routes, watchRoute, ProfilePages } from '@/router';

const clientInfo: Ref<ClientInfo | null> = ref(null);
const informationManager: InformationManager = inject(InformationManagerKey)!;

const myProfileTab = ref(ProfilePages.Settings);

const routeUnwatch = watchRoute(async () => {
  if (getCurrentRouteName() !== Routes.MyProfile) {
    return;
  }
  await switchPageFromQuery();
});

async function openDocumentationPopover(): Promise<void> {
  await Env.Links.openDocumentationLink();
}

async function openFeedbackPopover(): Promise<void> {
  await Env.Links.openContactLink();
}

async function switchPageFromQuery(): Promise<void> {
  myProfileTab.value = ProfilePages.Settings;
  const page = getCurrentRouteQuery().profilePage;
  if (page && Object.values(ProfilePages).includes(page)) {
    myProfileTab.value = page;
  }
}

async function switchPage(): Promise<void> {
  await navigateTo(Routes.MyProfile, { replace: true, query: { profilePage: myProfileTab.value } });
}

onMounted(async () => {
  const deviceResult = await getCurrentAvailableDevice();
  const result = await getClientInfo();

  if (!result.ok || !deviceResult.ok) {
    informationManager.present(
      new Information({
        message: 'MyProfilePage.errors.failedToRetrieveInformation',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  } else {
    clientInfo.value = result.value;
  }
  await switchPageFromQuery();
});

onUnmounted(async () => {
  routeUnwatch();
});
</script>

<style scoped lang="scss">
.page-container {
  display: flex;
  width: 100%;
  height: 100%;

  .profile-menu {
    display: flex;
    flex-direction: column;
    width: 100%;
    max-width: 21.375rem;
    height: 100%;
    padding: 1.5rem;
    border-top: 1px solid var(--parsec-color-light-secondary-medium);
    border-right: 1px solid var(--parsec-color-light-secondary-medium);
    gap: 1rem;
    overflow: auto;
  }

  .profile-content {
    width: 100%;
    background: var(--parsec-color-light-secondary-background);
    display: flex;
    flex-direction: column;
    gap: 2rem;
    padding: 2rem 2.5rem;
    overflow: auto;

    &-item {
      display: flex;
      flex-direction: column;
      background: var(--parsec-color-light-secondary-white);
      max-width: 37.5rem;
      height: fit-content;
      padding: 2rem;
      gap: 1.5rem;
      border-radius: var(--parsec-radius-8);
    }

    &-infos {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      &__name {
        color: var(--parsec-color-light-secondary-text);
      }

      &__email {
        color: var(--parsec-color-light-secondary-soft-text);
      }
    }
  }
}

.item-header {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;

  &__title {
    color: var(--parsec-color-light-primary-700);
  }

  &__description {
    color: var(--parsec-color-light-secondary-hard-grey);
    display: flex;
    flex-direction: column;
  }
}

.menu-list {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  width: 100%;
  padding: 1rem 0.5rem;

  &__title {
    color: var(--parsec-color-light-secondary-hard-grey);
    margin: 0 0.5rem 0.75rem;
  }

  // eslint-disable-next-line vue-scoped-css/no-unused-selector
  &__item {
    color: var(--parsec-color-light-secondary-text);
    border-radius: var(--parsec-radius-8);
    cursor: pointer;

    &::part(label) {
      width: 100%;
      margin: 0;
    }

    .item-container {
      display: flex;
      align-items: center;
      justify-content: center;
      padding: 0.5rem 0.75em;
      gap: 0.375rem;
      width: 100%;

      &__text {
        flex: 1;
        width: 100%;
      }
    }

    &::part(container) {
      display: none;
    }

    &.radio-checked {
      color: var(--parsec-color-light-secondary-text);
      background: var(--parsec-color-light-secondary-medium);
    }

    &:hover:not(.radio-checked) {
      background: var(--parsec-color-light-secondary-premiere);
      color: var(--parsec-color-light-secondary-text);
    }

    ion-icon {
      font-size: 1.125rem;
      opacity: 0.8;
      flex-shrink: 0;
    }
  }
}

.recovery {
  position: relative;
}
</style>
