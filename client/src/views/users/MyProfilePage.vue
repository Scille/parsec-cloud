<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="page profile-page">
    <ion-content class="profile-page-content">
      <ion-header
        v-if="clientInfo && isSmallDisplay"
        class="profile-page-header"
      >
        <profile-info-card
          v-if="myProfileTab === undefined"
          :label="clientInfo.humanHandle.label"
          :email="clientInfo.humanHandle.email"
          :current-profile="clientInfo.currentProfile"
          :center-content="true"
        />
        <div
          v-else
          class="header-selected-item"
        >
          <ion-icon
            :icon="chevronBack"
            @click="goBack"
            class="header-selected-item__back"
          />
          <ion-text class="header-selected-item__title title-h2">
            {{ $msTranslate(getTitleProfileMenu().tab) }}
          </ion-text>
          <ion-text class="header-selected-item__subtitle body">
            {{ $msTranslate(getTitleProfileMenu().subtitle) }}
          </ion-text>
        </div>
      </ion-header>
      <div class="profile-page-container">
        <ion-radio-group
          v-if="isLargeDisplay || (isSmallDisplay && myProfileTab === undefined)"
          v-model="myProfileTab"
          class="profile-menu"
          @ion-change="switchPage"
        >
          <div class="menu-content">
            <ion-text class="menu-content__title subtitles-sm">
              {{ $msTranslate('MyProfilePage.tabs.account.title') }}
            </ion-text>
            <div class="menu-list">
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
                  <ion-icon
                    v-if="isSmallDisplay"
                    :icon="chevronForward"
                  />
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
                  <ion-icon
                    v-if="isSmallDisplay"
                    :icon="chevronForward"
                  />
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
                  <ion-icon
                    v-if="isSmallDisplay"
                    :icon="chevronForward"
                  />
                </div>
              </ion-radio>
              <!-- Recovery file -->
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
                  <ion-icon
                    v-if="isSmallDisplay"
                    :icon="chevronForward"
                  />
                </div>
              </ion-radio>
            </div>
          </div>

          <div class="menu-content">
            <ion-text class="menu-content__title subtitles-sm">
              {{ $msTranslate('MyProfilePage.tabs.support.title') }}
            </ion-text>
            <div class="menu-list">
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
                  <ms-image
                    class="item-container__open-icon"
                    :image="OpenIcon"
                  />
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
                  <ms-image
                    class="item-container__open-icon"
                    :image="OpenIcon"
                  />
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
                  <ion-icon
                    v-if="isSmallDisplay"
                    :icon="chevronForward"
                  />
                </div>
              </ion-radio>
            </div>
          </div>

          <!-- log out -->
          <div class="logout-container">
            <div
              class="logout"
              @click="logout"
            >
              <ion-icon
                :icon="logOut"
                slot="start"
              />
              <ion-text class="body logout-text">
                {{ $msTranslate('HomePage.topbar.logout') }}
              </ion-text>
            </div>
          </div>
        </ion-radio-group>
        <div
          class="profile-content"
          v-if="myProfileTab !== undefined || isLargeDisplay"
        >
          <!-- profile info -->
          <div
            class="profile-content-infos"
            v-if="clientInfo && isLargeDisplay"
          >
            <profile-info-card
              :label="clientInfo.humanHandle.label"
              :email="clientInfo.humanHandle.email"
              :current-profile="clientInfo.currentProfile"
            />
          </div>
          <!-- Settings tab -->
          <div
            v-if="myProfileTab === ProfilePages.Settings || (isLargeDisplay && myProfileTab === undefined)"
            class="profile-content-item"
            id="settings-profile-content"
          >
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
          <!-- recovery file tab -->
          <div
            v-if="myProfileTab === ProfilePages.Recovery"
            class="profile-content-item recovery"
          >
            <div class="item-header">
              <ion-text class="item-header__title title-h3">{{ $msTranslate('OrganizationRecovery.title') }}</ion-text>
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
import ProfileInfoCard from '@/components/profile/ProfileInfoCard.vue';
import SettingsList from '@/components/settings/SettingsList.vue';
import { ClientInfo, getClientInfo, getCurrentAvailableDevice } from '@/parsec';
import { getCurrentRouteName, getCurrentRouteQuery, navigateTo, ProfilePages, Routes, watchRoute } from '@/router';
import { Env } from '@/services/environment';
import { EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import AboutView from '@/views/about/AboutView.vue';
import DevicesPage from '@/views/devices/DevicesPage.vue';
import AuthenticationPage from '@/views/profile/AuthenticationPage.vue';
import OrganizationRecoveryPage from '@/views/profile/OrganizationRecoveryPage.vue';
import { IonContent, IonHeader, IonIcon, IonPage, IonRadio, IonRadioGroup, IonText } from '@ionic/vue';
import {
  chatbubbles,
  chevronBack,
  chevronForward,
  cog,
  documentText,
  fingerPrint,
  idCard,
  information,
  logOut,
  phonePortrait,
} from 'ionicons/icons';
import { Answer, askQuestion, MsImage, OpenIcon, Translatable, useWindowSize } from 'megashark-lib';
import { inject, onMounted, onUnmounted, Ref, ref } from 'vue';

const clientInfo: Ref<ClientInfo | null> = ref(null);
const informationManager: InformationManager = inject(InformationManagerKey)!;
const { isSmallDisplay, isLargeDisplay } = useWindowSize();

const eventDistributor: EventDistributor = inject(EventDistributorKey)!;
const myProfileTab = ref(isSmallDisplay ? undefined : ProfilePages.Settings);

const routeUnwatch = watchRoute(async () => {
  if (getCurrentRouteName() !== Routes.MyProfile) {
    return;
  }
  await switchPageFromQuery();
});

async function logout(): Promise<void> {
  const answer = await askQuestion('HomePage.topbar.logoutConfirmTitle', 'HomePage.topbar.logoutConfirmQuestion', {
    yesText: 'HomePage.topbar.logoutYes',
    noText: 'HomePage.topbar.logoutNo',
  });
  if (answer === Answer.Yes) {
    await eventDistributor.dispatchEvent(Events.LogoutRequested);
  }
}

interface TitleProfile {
  tab: Translatable;
  subtitle: Translatable;
}

function getTitleProfileMenu(): TitleProfile {
  switch (myProfileTab.value) {
    case ProfilePages.Settings:
      return {
        tab: 'MyProfilePage.tabs.account.settings',
        subtitle: 'MyProfilePage.tabs.account.title',
      };
    case ProfilePages.Devices:
      return {
        tab: 'MyProfilePage.tabs.account.devices',
        subtitle: 'MyProfilePage.tabs.account.title',
      };
    case ProfilePages.Authentication:
      return {
        tab: 'MyProfilePage.tabs.account.authentication',
        subtitle: 'MyProfilePage.tabs.account.title',
      };
    case ProfilePages.Recovery:
      return {
        tab: 'MyProfilePage.tabs.account.recovery',
        subtitle: 'MyProfilePage.tabs.account.title',
      };
    case ProfilePages.About:
      return {
        tab: 'MyProfilePage.tabs.support.about',
        subtitle: 'MyProfilePage.tabs.support.title',
      };
    default:
      return {
        tab: '',
        subtitle: '',
      };
  }
}

async function openDocumentationPopover(): Promise<void> {
  await Env.Links.openDocumentationLink();
}

async function openFeedbackPopover(): Promise<void> {
  await Env.Links.openContactLink();
}

async function switchPageFromQuery(): Promise<void> {
  const page = getCurrentRouteQuery().profilePage;
  if (page && Object.values(ProfilePages).includes(page)) {
    myProfileTab.value = page;
  } else {
    if (isSmallDisplay) {
      myProfileTab.value = undefined;
    }
  }
}

async function switchPage(): Promise<void> {
  await navigateTo(Routes.MyProfile, { replace: true, query: { profilePage: myProfileTab.value } });
}

async function goBack(): Promise<void> {
  myProfileTab.value = undefined;
  await navigateTo(Routes.MyProfile, { replace: true });
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
* {
  transition: all 0.2s ease-in-out;
}

.profile-page-content {
  &::part(scroll) {
    display: flex;
    flex-direction: column;
  }

  &::part(background) {
    @include ms.responsive-breakpoint('sm') {
      background: var(--parsec-color-light-secondary-background);
    }
  }
}

.profile-page-header {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: 2rem 1rem 1.5rem;
  background: var(--parsec-color-light-secondary-background);
  z-index: 1;

  @include ms.responsive-breakpoint('sm') {
    background: transparent;
  }

  .header-selected-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    align-items: center;
    justify-content: center;
    width: 100%;
    position: relative;

    &__back {
      position: absolute;
      left: 1rem;
      color: var(--parsec-color-light-secondary-hard-grey);
      background: var(--parsec-color-light-secondary-premiere);
      border-radius: var(--parsec-radius-8);
      font-size: 1.5rem;
      padding: 0.375rem;
      cursor: pointer;
    }

    &__title {
      color: var(--parsec-color-light-primary-800);
      text-align: center;
    }

    &__subtitle {
      color: var(--parsec-color-light-secondary-hard-grey);
      text-align: center;
    }
  }
}

.profile-page-container {
  display: flex;
  width: 100%;
  height: 100%;
  position: relative;
  z-index: 10;

  @include ms.responsive-breakpoint('sm') {
    height: auto;
    position: sticky;
    background: var(--parsec-color-light-secondary-white);
    box-shadow: var(--parsec-shadow-strong);
    border-radius: var(--parsec-radius-18) var(--parsec-radius-18) 0 0;
  }

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

    @include ms.responsive-breakpoint('md') {
      max-width: 15rem;
      padding: 1rem;
    }

    @include ms.responsive-breakpoint('sm') {
      border: none;
      overflow: visible;
      gap: 0rem;
      max-width: 100%;
      padding: 0;
    }
  }

  .profile-content {
    width: 100%;
    height: -webkit-fill-available;
    background: var(--parsec-color-light-secondary-background);
    display: flex;
    flex-direction: column;
    align-self: flex-start;
    gap: 2rem;
    padding: 2rem 2.5rem;
    overflow: auto;

    @include ms.responsive-breakpoint('lg') {
      padding: 2rem 1.5rem;
    }

    @include ms.responsive-breakpoint('sm') {
      padding: 0;
    }

    &-item {
      display: flex;
      flex-direction: column;
      max-width: 37.5rem;
      height: fit-content;
      gap: 1.5rem;
      border-radius: var(--parsec-radius-8);
      position: relative;

      &:not(#settings-profile-content) {
        padding: 2rem;
        background: var(--parsec-color-light-secondary-white);
      }

      @include ms.responsive-breakpoint('sm') {
        max-width: 100%;
        height: -webkit-fill-available;
      }

      @include ms.responsive-breakpoint('xs') {
        padding: 1.5rem;
      }

      .item-header {
        display: flex;
        flex-direction: column;
        gap: 0.75rem;

        @include ms.responsive-breakpoint('sm') {
          display: none;
        }

        &__title {
          color: var(--parsec-color-light-primary-700);
        }

        &__description {
          color: var(--parsec-color-light-secondary-hard-grey);
          display: flex;
          flex-direction: column;
        }
      }
    }
  }
}

.menu-content {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  padding: 1rem 0.5rem;

  @include ms.responsive-breakpoint('lg') {
    padding: 0.5rem 0;
  }

  @include ms.responsive-breakpoint('sm') {
    padding: 1.5rem 2rem;
  }

  @include ms.responsive-breakpoint('xs') {
    padding: 1.5rem;
  }

  &__title {
    color: var(--parsec-color-light-secondary-hard-grey);
    margin: 0 0.5rem 0 1rem;

    @include ms.responsive-breakpoint('sm') {
      margin: 0;
    }
  }

  .menu-list {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    width: 100%;
    padding-inline: 0.5rem;

    @include ms.responsive-breakpoint('sm') {
      gap: 1rem;
      padding: 0.5rem 0;
      background: var(--parsec-color-light-secondary-background);
      border-radius: var(--parsec-radius-12);
    }

    // eslint-disable-next-line vue-scoped-css/no-unused-selector
    &__item {
      color: var(--parsec-color-light-secondary-text);
      border-radius: var(--parsec-radius-8);
      cursor: pointer;
      position: relative;

      &::part(label) {
        width: 100%;
        margin: 0;
      }

      .item-container {
        display: flex;
        align-items: center;
        padding: 0.5rem 0.75em;
        gap: 0.375rem;
        width: 100%;
        overflow: hidden;

        &__text {
          flex: 1;
          width: 100%;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
        }

        &__open-icon {
          width: 1.25rem;
          height: 1.25rem;
          margin-left: auto;
          --fill-color: var(--parsec-color-light-secondary-hard-grey);
          opacity: 0.8;
          flex-shrink: 0;
          position: relative;
          top: 0;
          right: 0;
          transition: all 0.2s ease-in-out;
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

        .item-container__open-icon {
          top: -0.125rem;
          right: -0.125rem;
        }
      }

      ion-icon {
        &:first-child {
          color: var(--parsec-color-light-secondary-text);
          font-size: 1.125rem;
          opacity: 0.8;
          flex-shrink: 0;
        }
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

.logout-container {
  display: flex;
  border-top: 1px solid var(--parsec-color-light-secondary-medium);
  padding: 1rem 0rem;

  @include ms.responsive-breakpoint('sm') {
    padding: 0;
    margin-top: 0;
    padding: 1rem 1.5rem 2rem;
  }

  .logout {
    display: flex;
    align-items: center;
    width: 100%;
    gap: 0.5rem;
    padding: 0.5rem 1rem;
    margin-inline: 0.5rem;
    border-radius: var(--parsec-radius-8);
    cursor: pointer;
    color: var(--parsec-color-light-danger-500);

    @include ms.responsive-breakpoint('sm') {
      justify-content: center;
      margin-inline: 0;
    }

    &:hover {
      background: var(--parsec-color-light-danger-50);
      color: var(--parsec-color-light-danger-700);

      ion-icon {
        color: var(--parsec-color-light-danger-700);
      }
    }
  }
}
</style>
