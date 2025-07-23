<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div id="page">
        <!-- sidebar -->
        <home-page-sidebar class="homepage-sidebar" />
        <!-- main content -->
        <div class="homepage-content">
          <div class="homepage-parsec-account account">
            <account-login-page
              :key="refreshKey"
              @login-success="onLoginSuccess"
              :disabled="disableGoTo"
            />

            <div class="account-create">
              <ion-text class="account-create__description body">{{ $msTranslate('loginPage.createAccount.description') }}</ion-text>
              <ion-button
                class="account-create__button"
                @click="goToCreateAccount"
                :disabled="disableGoTo"
              >
                {{ $msTranslate('loginPage.createAccount.createAccountButton') }}
              </ion-button>
            </div>
          </div>

          <!-- Client Area -->
          <div class="homepage-client-area">
            <ion-text class="homepage-client-area__title title-h4">
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
                class="homepage-skip__icon"
              />
            </ion-button>
          </div>

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
              {{ $msTranslate('MenuPage.documentation') }}
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
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import HomePageSidebar from '@/views/home/HomePageSidebar.vue';
import AccountLoginPage from '@/views/account/AccountLoginPage.vue';
import { getCurrentRouteParams, getCurrentRouteQuery, navigateTo, Routes } from '@/router';
import { chevronForward, cog, informationCircle, open } from 'ionicons/icons';
import { IonContent, IonPage, IonButton, IonIcon, IonText } from '@ionic/vue';
import { ParsecAccount } from '@/parsec';
import { onMounted, ref } from 'vue';
import { openAboutModal } from '@/views/about';
import { openSettingsModal } from '@/views/settings';
import { Env } from '@/services/environment';

const disableGoTo = ref(false);
const refreshKey = ref(0);

onMounted(async () => {
  if (ParsecAccount.isLoggedIn()) {
    await onLoginSuccess();
  }
  disableGoTo.value = false;
});

async function onLoginSuccess(): Promise<void> {
  disableGoTo.value = true;
  await navigateTo(Routes.Home, { skipHandle: true, params: getCurrentRouteParams(), query: getCurrentRouteQuery() });
  refreshKey.value += 1;
  disableGoTo.value = false;
}

async function onSkipClicked(): Promise<void> {
  disableGoTo.value = true;
  await navigateTo(Routes.Home, { skipHandle: true, params: getCurrentRouteParams(), query: getCurrentRouteQuery() });
  refreshKey.value += 1;
  disableGoTo.value = false;
}

async function goToCreateAccount(): Promise<void> {
  await navigateTo(Routes.CreateAccount, { skipHandle: true });
}

async function goToCustomerArea(): Promise<void> {
  disableGoTo.value = true;
  const query = getCurrentRouteQuery();
  query.bmsLogin = true;
  await navigateTo(Routes.Home, { skipHandle: true, params: getCurrentRouteParams(), query: query });
  disableGoTo.value = false;
}
</script>

<style lang="scss" scoped>
#page {
  position: relative;
  height: 100vh;
  display: flex;
  overflow: hidden;
  background: linear-gradient(
    117deg,
    var(--parsec-color-light-secondary-inversed-contrast, #fcfcfc) 0%,
    var(--parsec-color-light-secondary-background, #f9f9fb) 100%
  );

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
    gap: 2rem;
    padding: 2rem 0;
    width: 100%;
    max-width: 28rem;
    align-items: center;
    margin: auto;

    @include ms.responsive-breakpoint('lg') {
      padding: 2rem 0;
    }

    @include ms.responsive-breakpoint('sm') {
      max-width: 30rem;
    }

    @include ms.responsive-breakpoint('xs') {
      padding: 1.5rem 1.5rem 0;
      max-width: 100%;
    }
  }

  .homepage-skip {
    border-top: 1px solid var(--parsec-color-light-secondary-medium);
    width: 100%;
    display: flex;
    padding-top: 1rem;
    justify-content: center;

    &__icon {
      font-size: 1rem;
      margin-left: 0.375rem;
      color: var(--parsec-color-light-secondary-grey);
    }

    &__button {
      width: fit-content;
      color: var(--parsec-color-light-secondary-grey);
      font-size: 0.875rem;

      &::part(native) {
        --background: transparent;
        --background-hover: transparent;
        padding: 0.125rem;
      }

      &:hover {
        color: var(--parsec-color-light-secondary-contrast);

        .homepage-skip__icon {
          color: var(--parsec-color-light-secondary-contrast);
        }
      }
    }
  }

  .homepage-parsec-account {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.5rem;

    .account-create {
      gap: 0.5rem;
      display: flex;
      align-items: center;

      &__button {
        color: var(--parsec-color-light-secondary-text);
        font-weight: 600;

        &::part(native) {
          --background-hover: transparent;
          --background: transparent;
          padding: 0.125rem;
        }

        &:hover {
          color: var(--parsec-color-light-secondary-text-hover);
          text-decoration: underline;
        }
      }

      &__description {
        color: var(--parsec-color-light-secondary-hard-grey);
      }
    }
  }

  .homepage-client-area {
    display: flex;
    width: 100%;
    height: fit-content;
    justify-content: space-between;
    border-radius: var(--parsec-radius-12);
    box-shadow: var(--parsec-shadow-soft);
    background: var(--parsec-color-light-primary-30);
    align-items: center;
    padding: 0.75rem 1rem 0.75rem 1.5rem;
    overflow: hidden;
    position: relative;

    &__title {
      color: var(--parsec-color-light-primary-700);
    }

    &__button {
      color: var(--parsec-color-light-primary-700);
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
}

.menu-secondary-buttons {
  display: flex;
  margin-top: auto;
  position: absolute;
  bottom: 2rem;

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

    &::part(native) {
      --background: transparent;
      --background-hover: transparent;
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
}
</style>
