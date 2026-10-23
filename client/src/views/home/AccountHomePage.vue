<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div id="page">
        <!-- sidebar -->
        <home-page-sidebar class="homepage-sidebar" />
        <div class="homepage-scroll">
          <!-- main content -->
          <div class="homepage-content">
            <ion-menu
              side="end"
              content-id="main-content"
              class="menu-secondary-collapse"
              v-if="(windowWidth < WindowSizeBreakpoints.MD || windowHeight < 900) && !onLeavePage"
            >
              <home-page-secondary-menu-collapse
                :show-customer-area-button="true"
                @customer-area-click="goToCustomerArea"
                @settings-click="openSettingsModal"
              />
            </ion-menu>

            <home-page-secondary-menu
              v-else-if="!onLeavePage"
              class="homepage-menu-secondary"
              @customer-area-click="goToCustomerArea"
              @settings-click="openSettingsModal"
            />

            <ion-menu-button
              v-if="windowWidth < WindowSizeBreakpoints.MD || windowHeight < 900"
              slot="end"
              id="main-content"
              class="menu-button"
            >
              <ion-icon
                :icon="menu"
                class="menu-button__icon"
              />
            </ion-menu-button>

            <div class="homepage-parsec-account account">
              <account-login-page
                :key="refreshKey"
                @login-success="onLoginSuccess"
                :disabled="disableGoTo"
              />

              <div class="account-create">
                <ion-text class="account-create__description">
                  {{ $msTranslate('loginPage.createAccount.description') }}
                </ion-text>
                <ion-button
                  class="account-create__button button-large"
                  @click="goToCreateAccount"
                  :disabled="disableGoTo"
                >
                  {{ $msTranslate('loginPage.createAccount.createAccountButton') }}
                </ion-button>
              </div>
            </div>

            <!-- Client Area -->
            <div class="homepage-client-area">
              <ion-text class="homepage-client-area__title">
                {{ $msTranslate('loginPage.clientArea.title') }}
              </ion-text>
              <ion-button
                @click="goToCustomerArea"
                :disabled="disableGoTo"
                fill="clear"
                class="homepage-client-area__button"
              >
                {{ $msTranslate('loginPage.clientArea.button') }}
              </ion-button>
              <img
                src="@/assets/images/background/background-shapes-small.svg"
                class="homepage-client-area__blob"
              />
            </div>

            <!-- Skip button -->
            <div class="homepage-skip">
              <ion-button
                class="homepage-skip__button"
                @click="onSkipClicked"
                :disabled="disableGoTo"
              >
                {{ $msTranslate('loginPage.skip') }}
                <ion-icon
                  :icon="chevronForward"
                  class="homepage-skip__icon button-icon-right"
                />
              </ion-button>
            </div>
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import HomePageSecondaryMenu from '@/components/header/HomePageSecondaryMenu.vue';
import HomePageSecondaryMenuCollapse from '@/components/header/HomePageSecondaryMenuCollapse.vue';
import { ParsecAccount } from '@/parsec';
import { getCurrentRouteParams, getCurrentRouteQuery, navigateTo, Routes } from '@/router';
import AccountLoginPage from '@/views/account/AccountLoginPage.vue';
import HomePageSidebar from '@/views/home/HomePageSidebar.vue';
import { openSettingsModal } from '@/views/settings';
import {
  IonButton,
  IonContent,
  IonIcon,
  IonMenu,
  IonMenuButton,
  IonPage,
  IonText,
  onIonViewWillEnter,
  onIonViewWillLeave,
} from '@ionic/vue';
import { chevronForward, menu } from 'ionicons/icons';
import { useWindowSize, WindowSizeBreakpoints } from 'megashark-lib';
import { ref } from 'vue';

const disableGoTo = ref(false);
const refreshKey = ref(0);
const { windowHeight, windowWidth } = useWindowSize();
const onLeavePage = ref(false);

onIonViewWillEnter(async () => {
  onLeavePage.value = false;
  if (ParsecAccount.isLoggedIn()) {
    await onLoginSuccess();
  }
  refreshKey.value += 1;
  disableGoTo.value = false;
});

onIonViewWillLeave(async () => {
  onLeavePage.value = true;
});

async function onLoginSuccess(): Promise<void> {
  disableGoTo.value = true;
  await navigateTo(Routes.Home, { skipHandle: true, params: getCurrentRouteParams(), query: getCurrentRouteQuery() });
}

async function onSkipClicked(): Promise<void> {
  disableGoTo.value = true;
  await navigateTo(Routes.Home, { skipHandle: true, params: getCurrentRouteParams(), query: getCurrentRouteQuery() });
}

async function goToCreateAccount(): Promise<void> {
  await navigateTo(Routes.CreateAccount, { skipHandle: true });
}

async function goToCustomerArea(): Promise<void> {
  disableGoTo.value = true;
  const query = getCurrentRouteQuery();
  query.bmsLogin = true;
  await navigateTo(Routes.Home, { skipHandle: true, params: getCurrentRouteParams(), query: query });
}
</script>

<style lang="scss" scoped>
#page {
  position: relative;
  height: 100vh;
  display: flex;
  overflow: hidden;
  background: linear-gradient(117deg, ms.color('surface-base-default') 0%, ms.color('surface-base-default-secondary') 100%);

  .homepage-scroll {
    @media screen and (max-height: 600px) {
      padding: ms.spacing('padding-7xl') 0 ms.spacing('padding-3xl');
    }

    @include ms.responsive-breakpoint('sm') {
      padding: ms.spacing('padding-7xl') 0 ms.spacing('padding-3xl');
    }
  }

  // Should be edited later with responsive
  .homepage-sidebar {
    @include ms.responsive-breakpoint('xxl') {
      max-width: 35rem;
    }

    @include ms.responsive-breakpoint('xl') {
      max-width: 30rem;

      &:before {
        height: 560px;
        max-height: 50vh;
      }
    }

    @include ms.responsive-breakpoint('lg') {
      max-width: 22rem;
    }

    @include ms.responsive-breakpoint('md') {
      max-width: 17rem;
    }

    @include ms.responsive-breakpoint('sm') {
      display: none;
    }
  }

  .homepage-content {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: ms.spacing('gap-5xl');
    padding: ms.spacing('padding-5xl') 0;
    max-width: 28rem;
    align-items: center;
    margin: auto;

    @media screen and (max-height: 600px) {
      justify-content: flex-start !important;
    }

    @include ms.responsive-breakpoint('lg') {
      padding: ms.spacing('padding-5xl') 0;
    }

    @include ms.responsive-breakpoint('sm') {
      max-width: 30rem;
      justify-content: flex-start !important;
    }

    @include ms.responsive-breakpoint('xs') {
      padding: ms.spacing('padding-4xl') ms.spacing('padding-4xl') 0;
      max-width: 100%;
    }
  }

  .homepage-skip {
    border-top: ms.border('thin') solid ms.color('border-base-default');
    width: 100%;
    display: flex;
    padding-top: ms.spacing('padding-3xl');
    justify-content: center;

    &__icon {
      font-size: 1rem;
      margin-left: ms.spacing('padding-md');
      color: ms.color('icon-neutral-default');
    }

    &__button {
      width: fit-content;
      color: ms.color('text-base-description');

      &::part(native) {
        --background: transparent;
        --background-hover: transparent;
      }

      &:hover {
        color: ms.color('text-neutral-default-hover');

        .homepage-skip__icon {
          color: ms.color('icon-neutral-default-hover');
        }
      }
    }
  }

  .homepage-parsec-account {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: ms.spacing('gap-4xl');

    .account-create {
      gap: ms.spacing('gap-lg');
      display: flex;
      align-items: center;

      &__button {
        position: relative;

        &::part(native) {
          --background-hover: transparent;
          --background: transparent;
          padding: ms.spacing('padding-xs');
        }

        &::before {
          content: '';
          position: absolute;
          bottom: -0.25rem;
          left: 0.125rem;
          height: ms.border('thin');
          width: 0px;
          background: transparent;
          transition: all 150ms linear;
        }

        &:hover {
          color: ms.color('text-brand-default-hover');

          &::before {
            width: calc(100% - 0.25rem);
            background: ms.color('text-brand-default-hover');
          }
        }
      }

      &__description {
        @include ms.font('body-lg-medium');
        color: ms.color('text-base-description');
      }
    }
  }

  .homepage-client-area {
    display: flex;
    width: 100%;
    height: fit-content;
    justify-content: space-between;
    border-radius: ms.radius('2xl');
    box-shadow: var(--parsec-shadow-soft);
    background: ms.color('surface-brand-default-subtle');
    align-items: center;
    padding: ms.spacing('padding-2xl') ms.spacing('padding-3xl') ms.spacing('padding-2xl') ms.spacing('padding-4xl');
    overflow: hidden;
    position: relative;
    flex-shrink: 0;

    @include ms.responsive-breakpoint('sm') {
      padding: ms.spacing('padding-2xl') ms.spacing('padding-3xl');
    }

    &__title {
      @include ms.font('heading-h5');
      color: ms.color('text-brand-default');
    }

    &__button {
      --background-hover: transparent;
      z-index: 2;

      &:hover {
        text-decoration: underline;
      }
    }

    &__blob {
      position: absolute;
      width: 100%;
      max-width: 200px;
      right: -4rem;
      bottom: 1rem;
      z-index: 1;
    }
  }

  .homepage-menu-secondary {
    position: absolute;
    top: 3rem;
    right: 3rem;
  }

  &::before {
    content: '';
    position: absolute;
    height: 100%;
    width: 100%;
    max-width: 500px;
    max-height: 500px;
    bottom: 0;
    right: 0;
    background-image: url('@/assets/images/background/blob-shape.svg');
    background-size: contain;
    background-repeat: no-repeat;
    background-position: top center;
    opacity: 0.1;
    filter: blur(600px);
  }

  .menu-button {
    height: 2.5rem;
    width: 2.5rem;
    position: absolute;
    top: 1.5rem;
    right: 1.5rem;
    color: ms.color('text-neutral-default');
    cursor: pointer;

    &__icon {
      font-size: 1.75rem;
    }
  }
}
</style>
