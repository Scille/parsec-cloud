<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
          <ion-text class="header-selected-item__title">
            {{ $msTranslate(profileMenuTitle.tab) }}
          </ion-text>
          <ion-text class="header-selected-item__subtitle">
            {{ $msTranslate(profileMenuTitle.subtitle) }}
          </ion-text>
        </div>
      </ion-header>
      <div class="main-container profile-page-container">
        <div
          v-if="isLargeDisplay || (isSmallDisplay && myProfileTab === undefined)"
          class="profile-menu-wrapper"
        >
          <ion-radio-group
            v-model="myProfileTab"
            class="profile-menu"
            @ion-change="switchPage"
          >
            <div class="menu-content">
              <ion-text class="menu-content__title">
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
                    <ion-text class="item-container__text">
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
                    <ion-text class="item-container__text">
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
                    <ion-text class="item-container__text">
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
                    <ion-text class="item-container__text">
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
              <ion-text class="menu-content__title">
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
                    <ion-text class="item-container__text">
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
                    <ion-icon :icon="help" />
                    <ion-text class="item-container__text">
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
                    <ion-text class="item-container__text">
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
                  :icon="logIn"
                  slot="start"
                />
                <ion-text class="logout-text">
                  {{ $msTranslate('HomePage.topbar.logout') }}
                </ion-text>
              </div>
            </div>
          </ion-radio-group>
        </div>
        <div
          class="profile-content"
          v-if="myProfileTab !== undefined || isLargeDisplay"
        >
          <!-- Settings tab -->
          <div
            v-if="myProfileTab === ProfilePages.Settings || (isLargeDisplay && myProfileTab === undefined)"
            class="profile-content-item"
            id="settings-profile-content"
          >
            <!-- profile info -->
            <div
              class="profile-content-infos"
              v-if="clientInfo && isLargeDisplay"
            >
              <div class="item-header">
                <ion-text class="item-header__title">{{ $msTranslate('MyProfilePage.settings.title') }}</ion-text>
                <ion-text class="item-header__description">{{ $msTranslate('MyProfilePage.settings.description') }}</ion-text>
              </div>
              <profile-info-card
                class="profile-infos-container"
                :label="clientInfo.humanHandle.label"
                :email="clientInfo.humanHandle.email"
                :current-profile="clientInfo.currentProfile"
              />
            </div>
            <settings-list />
          </div>
          <!-- devices tab -->
          <div
            v-if="myProfileTab === ProfilePages.Devices"
            class="profile-content-item"
          >
            <div class="item-header">
              <ion-text class="item-header__title">{{ $msTranslate('DevicesPage.title') }}</ion-text>
              <ion-text class="item-header__description">{{ $msTranslate('DevicesPage.description') }}</ion-text>
            </div>
            <devices-page class="devices" />
          </div>
          <!-- authentication tab -->
          <div
            v-if="myProfileTab === ProfilePages.Authentication"
            class="profile-content-item"
          >
            <div class="item-header">
              <ion-text class="item-header__title">{{ $msTranslate('Authentication.title') }}</ion-text>
              <ion-text class="item-header__description">{{ $msTranslate('Authentication.description') }}</ion-text>
            </div>
            <authentication-page />
          </div>
          <!-- recovery file tab -->
          <div
            v-if="myProfileTab === ProfilePages.Recovery"
            class="profile-content-item recovery"
            id="recovery-profile-content"
          >
            <div class="item-header">
              <ion-text class="item-header__title">{{ $msTranslate('OrganizationRecovery.title') }}</ion-text>
              <ion-text class="item-header__description">{{ $msTranslate('OrganizationRecovery.description') }}</ion-text>
            </div>
            <organization-recovery-page />
          </div>
          <!-- About tab -->
          <div
            v-if="myProfileTab === ProfilePages.About"
            class="profile-content-item"
          >
            <div class="item-header">
              <ion-text class="item-header__title item-header__title--sm">{{ $msTranslate('AboutPage.title') }}</ion-text>
              <ion-text class="item-header__description">{{ $msTranslate('AboutPage.description') }}</ion-text>
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
  chevronBack,
  chevronForward,
  cog,
  documentText,
  fingerPrint,
  help,
  idCard,
  information,
  logIn,
  phonePortrait,
} from 'ionicons/icons';
import { Answer, askQuestion, MsImage, OpenIcon, Translatable, useWindowSize } from 'megashark-lib';
import { computed, inject, onMounted, onUnmounted, Ref, ref } from 'vue';

const clientInfo: Ref<ClientInfo | null> = ref(null);
const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const { isSmallDisplay, isLargeDisplay } = useWindowSize();

const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
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
    await eventDistributor.value.dispatchEvent(Events.LogoutRequested);
  }
}

const profileMenuTitle = computed((): { tab: Translatable; subtitle: Translatable } => {
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
});

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
    informationManager.value.present(
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
  border-top: 1px solid ms.color('border-base-default');

  &::part(scroll) {
    display: flex;
    flex-direction: column;
  }

  &::part(background) {
    --background: #{ms.color('surface-base-default')};
  }
}

.profile-page-header {
  display: flex;
  justify-content: center;
  align-items: center;
  padding: ms.spacing('padding-5xl') ms.spacing('padding-3xl') ms.spacing('padding-4xl');
  background: ms.color('surface-base-page');
  z-index: 1;

  @include ms.responsive-breakpoint('sm') {
    position: sticky;
    top: 0;
    z-index: 10;
  }

  .header-selected-item {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-sm');
    align-items: center;
    justify-content: center;
    width: 100%;
    position: relative;

    &__back {
      position: absolute;
      left: 1rem;
      color: ms.color('icon-neutral-default');
      background: ms.color('surface-neutral-default-subtle');
      border-radius: ms.radius('lg');
      font-size: 1.5rem;
      padding: ms.spacing('padding-md');
      cursor: pointer;
    }

    &__title {
      @include ms.font('heading-h3');

      color: ms.color('text-brand-default');
      text-align: center;
    }

    &__subtitle {
      @include ms.font('body-md-regular');

      color: ms.color('text-base-description');
      text-align: center;
    }
  }
}

.profile-page-container {
  flex-direction: row;
  position: relative;
  z-index: 10;

  @include ms.responsive-breakpoint('sm') {
    background: ms.color('surface-base-default');
    border-radius: ms.radius('4xl') ms.radius('4xl') ms.radius('none') ms.radius('none');
    height: stretch;
    z-index: 1;
  }

  .profile-menu-wrapper {
    display: flex;
    flex-shrink: 0;
    height: 100%;
    border-right: 1px solid ms.color('border-base-default');

    @include ms.responsive-breakpoint('sm') {
      width: 100%;
      height: auto;
      padding: ms.spacing('padding-none');
    }
  }

  .profile-menu {
    display: flex;
    flex-direction: column;
    width: 18.25rem;
    height: 100%;
    gap: ms.spacing('gap-3xl');
    // background: ms.color('surface-base-page');
    overflow-x: hidden;
    overflow-y: auto;

    @include ms.responsive-breakpoint('md') {
      width: 15rem;
    }

    @include ms.responsive-breakpoint('sm') {
      width: 100%;
      overflow: visible;
      gap: ms.spacing('gap-none');
      background: none;
      border-radius: ms.radius('none');
    }
  }

  .profile-content {
    width: 100%;
    height: -webkit-fill-available;
    display: flex;
    flex-direction: column;
    align-self: flex-start;
    gap: ms.spacing('gap-5xl');
    padding: ms.spacing('padding-6xl');
    overflow: auto;

    @include ms.responsive-breakpoint('lg') {
      padding: ms.spacing('padding-5xl') ms.spacing('padding-4xl');
    }

    @include ms.responsive-breakpoint('sm') {
      padding: ms.spacing('padding-none');
      overflow: visible;
    }

    &-item {
      display: flex;
      flex-direction: column;
      max-width: 37.5rem;
      height: fit-content;
      gap: ms.spacing('gap-4xl');
      border-radius: ms.radius('2xl');
      position: relative;

      @include ms.responsive-breakpoint('sm') {
        max-width: 100%;
        height: -webkit-fill-available;
        padding: ms.spacing('padding-4xl');
      }

      .item-header {
        display: flex;
        flex-direction: column;
        gap: ms.spacing('gap-2xl');

        @include ms.responsive-breakpoint('sm') {
          display: none;
        }

        &__title {
          @include ms.font('heading-h3');

          color: ms.color('text-brand-default');

          &--sm {
            @include ms.font('heading-h4');
          }
        }

        &__description {
          @include ms.font('body-lg-regular');

          color: ms.color('text-base-description');
          display: flex;
          flex-direction: column;
        }
      }
    }
  }
}

.profile-infos-container {
  margin-top: 1.5rem;
  border-radius: ms.radius('2xl');
  padding: ms.spacing('padding-sm') ms.spacing('padding-none');
}

.menu-content {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-2xl');
  padding: ms.spacing('padding-3xl') ms.spacing('padding-3xl');

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-4xl') ms.spacing('padding-5xl');
  }

  @include ms.responsive-breakpoint('xs') {
    padding: ms.spacing('padding-4xl');
  }

  & + .menu-content {
    padding: ms.spacing('padding-3xl');

    @include ms.responsive-breakpoint('sm') {
      padding: ms.spacing('padding-4xl') ms.spacing('padding-5xl');
    }

    @include ms.responsive-breakpoint('xs') {
      padding: ms.spacing('padding-4xl');
    }
  }

  &__title {
    @include ms.font('body-sm-medium');

    color: ms.color('text-base-label');
    opacity: ms.opacity('description');
    padding-inline: ms.spacing('padding-lg');

    @include ms.responsive-breakpoint('sm') {
      padding-inline: ms.spacing('padding-none');
    }
  }

  .menu-list {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-sm');
    width: 100%;

    @include ms.responsive-breakpoint('sm') {
      gap: ms.spacing('gap-3xl');
      padding: ms.spacing('padding-lg') ms.spacing('padding-none');
      background: ms.color('surface-base-page');
      border-radius: ms.radius('2xl');
    }

    // eslint-disable-next-line vue-scoped-css/no-unused-selector
    &__item {
      color: ms.color('text-neutral-default');
      border-radius: ms.radius('lg');
      cursor: pointer;
      position: relative;

      &::part(label) {
        width: 100%;
        margin: 0;
      }

      .item-container {
        display: flex;
        align-items: center;
        padding: ms.spacing('padding-xl') ms.spacing('padding-2xl');
        gap: ms.spacing('gap-lg');
        width: 100%;
        overflow: hidden;

        &__text {
          @include ms.font('label-md-medium');

          flex: 1;
          width: 100%;
          overflow: hidden;
          text-overflow: ellipsis;
          white-space: nowrap;
        }

        &__open-icon {
          width: 1.125rem;
          height: 1.125rem;
          margin-left: auto;
          --fill-color: #{ms.color('icon-neutral-default')};
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
        color: ms.color('text-base-label');
        background: ms.color('surface-neutral-default-subtle-pressed');
      }

      &:hover:not(.radio-checked) {
        background: ms.color('surface-neutral-default-subtle-hover');
        color: ms.color('text-neutral-default');

        .item-container__open-icon {
          top: -0.125rem;
          right: -0.125rem;
        }
      }

      ion-icon {
        &:first-child {
          color: ms.color('icon-neutral-default');
          font-size: 1.25rem;
          opacity: ms.opacity('8');
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
          background: ms.color('border-base-default');
        }
      }
    }
  }
}

.logout-container {
  display: flex;
  padding: ms.spacing('padding-lg');

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-3xl') ms.spacing('padding-5xl') ms.spacing('padding-5xl');
  }

  @include ms.responsive-breakpoint('xs') {
    padding: ms.spacing('padding-3xl') ms.spacing('padding-4xl') ms.spacing('padding-5xl');
  }

  .logout-text {
    @include ms.font('label-md-medium');
  }

  .logout {
    display: flex;
    align-items: center;
    width: 100%;
    gap: ms.spacing('gap-lg');
    padding: ms.spacing('padding-xl') ms.spacing('padding-2xl');
    border-radius: ms.radius('lg');
    cursor: pointer;
    color: ms.color('text-error-default');

    ion-icon {
      color: ms.color('icon-error-default');
      font-size: 1rem;
      flex-shrink: 0;
    }

    @include ms.responsive-breakpoint('sm') {
      border: ms.border('thin') solid ms.color('border-base-default');
      box-shadow: ms.shadow('input');
      padding: ms.spacing('padding-2xl') ms.spacing('padding-3xl');
      justify-content: center;
    }

    &:hover {
      background: ms.color('surface-error-default-subtle-hover');
      color: ms.color('text-error-default-hover');

      ion-icon {
        color: ms.color('icon-error-default-hover');
      }
    }
  }
}
</style>
