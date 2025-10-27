<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      class="create-account-page-container"
    >
      <div
        class="create-account-page"
        :key="refreshKey"
      >
        <div
          v-show="step !== Steps.Created"
          class="create-account-page-header"
          :class="{
            'create-account-page-header--creating': step === Steps.Creating,
          }"
        >
          <div class="header-menu">
            <ion-icon
              class="back-button header-menu__back"
              :icon="arrowBack"
              @click="back()"
              v-if="showBackButton() && windowWidth < WindowSizeBreakpoints.SM"
            />

            <ion-menu
              side="end"
              content-id="id-content"
              class="menu-secondary-collapse"
              v-if="(windowWidth < WindowSizeBreakpoints.MD || windowHeight < 900) && !onLeavePage"
            >
              <home-page-secondary-menu-collapse
                :show-customer-area-button="false"
                @settings-click="openSettingsModal"
              />
            </ion-menu>

            <home-page-secondary-menu
              v-else-if="!onLeavePage"
              class="homepage-menu-secondary"
              @settings-click="openSettingsModal"
            />

            <ion-menu-button
              v-if="windowWidth < WindowSizeBreakpoints.MD || windowHeight < 900"
              slot="end"
              id="id-content"
              class="header-menu-button"
            >
              <ion-icon
                :icon="menu"
                class="header-menu-button__icon"
              />
            </ion-menu-button>
          </div>

          <ion-text
            class="create-account-page-header__title title-h1"
            v-if="TITLES[step].title"
          >
            <ion-icon
              class="back-button create-account-page-header__back"
              :icon="arrowBack"
              @click="back()"
              v-if="showBackButton() && windowWidth >= WindowSizeBreakpoints.SM"
            />
            {{ $msTranslate(TITLES[step].title) }}
          </ion-text>
          <ion-text
            class="create-account-page-header__subtitle body-lg"
            v-if="TITLES[step].subtitle"
          >
            {{ $msTranslate(TITLES[step].subtitle) }}
          </ion-text>
          <ms-report-text
            v-if="error"
            :theme="MsReportTheme.Error"
            class="recover-account-text__error"
          >
            {{ $msTranslate(error) }}
          </ms-report-text>
        </div>

        <!-- User Information Step -->
        <div
          v-show="step === Steps.UserInformation"
          class="step-content user-information-step"
        >
          <account-user-information
            class="user-information-step__information"
            :creation-stepper="creationStepper"
            @creation-started="onCreationStarted"
          />
          <div class="user-information-step-login">
            <ion-text class="user-information-step-login__description subtitles-normal">
              {{ $msTranslate('loginPage.createAccount.alreadyHaveAnAccount') }}
            </ion-text>
            <ion-button
              class="user-information-step-login__button button-large"
              @click="goToLogin"
            >
              {{ $msTranslate('loginPage.login') }}
            </ion-button>
          </div>
        </div>

        <!-- Code Validation Step -->
        <div
          v-show="step === Steps.Code"
          class="step-content validation-email-step"
        >
          <div class="validation-email-step__code">
            <ms-code-validation-input
              ref="codeValidationInput"
              :code-length="6"
              :allowed-input="AllowedInput.UpperAlphaNumeric"
              @code-complete="onCodeComplete"
              :disabled="querying"
            />

            <div class="validation-email-step-footer">
              <ion-button
                class="primary-button button-large"
                @click="validateCode"
                :disabled="code.length === 0"
              >
                {{ $msTranslate('loginPage.createAccount.nextButton') }}
              </ion-button>
              <ion-button
                class="validation-email-step-footer__resend button-large"
                @click="resendCode"
                :disabled="querying"
              >
                {{ $msTranslate('loginPage.createAccount.resendCode') }}
              </ion-button>
              <ms-spinner
                v-show="querying"
                :size="14"
              />
            </div>
          </div>
        </div>

        <!-- Authentication Step -->
        <div
          v-show="step === Steps.Authentication"
          class="step-content authentication-step"
        >
          <div class="authentication-step-content">
            <ms-choose-password-input
              ref="choosePassword"
              class="choose-password"
              @on-enter-keyup="createAccount"
            />
            <ion-button
              class="primary-button button-large"
              @click="createAccount"
              :disabled="!validAuth"
            >
              {{ $msTranslate('loginPage.createAccount.createAccountButton') }}
            </ion-button>
          </div>
        </div>

        <!-- Creating Account Step -->
        <div
          class="step-content creating-step"
          v-show="step === Steps.Creating"
        >
          <ms-spinner class="creating-step__spinner" />
        </div>

        <!-- Account Created Step -->
        <div
          class="step-content created-step"
          v-show="step === Steps.Created"
        >
          <div class="created-step-welcome">
            <ion-text class="created-step-welcome__subtitle title-h4">
              {{ $msTranslate('loginPage.createAccount.welcome') }}
            </ion-text>
            <ion-text class="created-step-welcome__name title-h1-xl">
              {{ creationStepper.firstName }} {{ creationStepper.lastName }}
            </ion-text>
          </div>
          <ion-text class="created-step-welcome__description body-lg">
            {{ $msTranslate('loginPage.createAccount.welcomeDescription') }}
          </ion-text>
          <div class="created-step-welcome-login">
            <ion-button
              v-if="!error"
              class="created-step-welcome-login__button button-large button-default"
              @click="goToHome"
            >
              {{ $msTranslate('loginPage.createAccount.accessToOrganizationButton') }}
            </ion-button>
            <ion-button
              v-if="error"
              class="created-step-welcome-login__button button-default"
              @click="goToLogin"
            >
              {{ $msTranslate('loginPage.login') }}
            </ion-button>
            <ms-report-text
              v-if="error"
              :theme="MsReportTheme.Warning"
            >
              {{ $msTranslate(error) }}
            </ms-report-text>
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import AccountUserInformation from '@/components/account/AccountUserInformation.vue';
import HomePageSecondaryMenu from '@/components/header/HomePageSecondaryMenu.vue';
import HomePageSecondaryMenuCollapse from '@/components/header/HomePageSecondaryMenuCollapse.vue';
import { AccountCreateErrorTag, AccountCreationStepper, ParsecAccount, ParsecAccountAccess } from '@/parsec';
import { wait } from '@/parsec/internals';
import { getCurrentRouteParams, getCurrentRouteQuery, navigateTo, Routes } from '@/router';
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
import { arrowBack, menu } from 'ionicons/icons';
import { DateTime } from 'luxon';
import {
  AllowedInput,
  asyncComputed,
  MsChoosePasswordInput,
  MsCodeValidationInput,
  MsReportText,
  MsReportTheme,
  MsSpinner,
  Translatable,
  useWindowSize,
  WindowSizeBreakpoints,
} from 'megashark-lib';
import { ref, useTemplateRef } from 'vue';

enum Steps {
  UserInformation = 0,
  Code = 1,
  Authentication = 2,
  Creating = 3,
  Created = 4,
}

const { windowHeight, windowWidth } = useWindowSize();
const step = ref<Steps>(Steps.UserInformation);
const codeValidationInputRef = useTemplateRef<InstanceType<typeof MsCodeValidationInput>>('codeValidationInput');
const creationStepper = new AccountCreationStepper();
const error = ref('');
const querying = ref(false);
const choosePasswordRef = useTemplateRef<InstanceType<typeof MsChoosePasswordInput>>('choosePassword');
const refreshKey = ref(0);
const code = ref<Array<string>>([]);

// We are encountering an issue when opening the menu from LoginPage or CreateAccountPage.
// If we open from one of these pages and move to the other page, clicking on the menu button opens the menu on the previous page.
// To avoid this, we set a variable to track when leaving the page.
const onLeavePage = ref(false);

const TITLES: Array<{ title?: Translatable; subtitle?: Translatable }> = [
  {
    title: 'loginPage.createAccount.title.default',
    subtitle: 'loginPage.createAccount.subtitle.default',
  },
  {
    title: 'loginPage.createAccount.title.codeTitle',
    subtitle: 'loginPage.createAccount.subtitle.codeSubtitle',
  },
  {
    title: 'loginPage.createAccount.title.authenticationTitle',
    subtitle: 'loginPage.createAccount.subtitle.authenticationSubtitle',
  },
  {
    title: 'loginPage.createAccount.title.creatingTitle',
  },
  {},
];

const validAuth = asyncComputed(async () => {
  return step.value === Steps.Authentication && choosePasswordRef.value && (await choosePasswordRef.value.areFieldsCorrect());
});

onIonViewWillEnter(async () => {
  onLeavePage.value = false;
  step.value = Steps.UserInformation;
  await creationStepper.reset();
  error.value = '';
  querying.value = false;
  refreshKey.value += 1;
});

onIonViewWillLeave(async () => {
  onLeavePage.value = true;
});

function showBackButton(): boolean {
  return step.value === Steps.UserInformation || step.value === Steps.Code || step.value === Steps.Authentication;
}

async function back(): Promise<void> {
  if (step.value === Steps.UserInformation) {
    await goToLogin();
  } else if (step.value === Steps.Code || step.value === Steps.Authentication) {
    await codeValidationInputRef.value?.clear();
    step.value = Steps.UserInformation;
    error.value = '';
  }
}

async function onCreationStarted(): Promise<void> {
  step.value = Steps.Code;
  setTimeout(async () => {
    await codeValidationInputRef.value?.setFocus();
  }, 300);
}

async function validateCode(): Promise<void> {
  if (!code.value.length) {
    return;
  }
  error.value = '';
  querying.value = true;
  try {
    const result = await creationStepper.validateCode(code.value);
    if (result.ok) {
      step.value = Steps.Authentication;
    } else {
      if (result.error.tag === AccountCreateErrorTag.InvalidValidationCode) {
        error.value = 'loginPage.createAccount.errors.invalidCode';
      } else if (result.error.tag === AccountCreateErrorTag.Offline) {
        error.value = 'loginPage.createAccount.errors.offlineOrIncorrectServer';
      } else {
        error.value = 'loginPage.createAccount.errors.offlineOrIncorrectServer';
        window.electronAPI.log('error', `Failed to validated the code: ${result.error.tag} (${result.error.error})`);
      }
    }
  } finally {
    querying.value = false;
  }
}

async function onCodeComplete(completeCode: Array<string>): Promise<void> {
  code.value = completeCode;
}

async function resendCode(): Promise<void> {
  querying.value = true;
  const result = await creationStepper.resendCode();
  if (!result.ok) {
    error.value = 'loginPage.createAccount.errors.resendCodeFailed';
  }
  // Limit the spam
  setTimeout(() => {
    querying.value = false;
  }, 10000);
}

async function goToHome(): Promise<void> {
  if (!ParsecAccount.isLoggedIn()) {
    return;
  }
  await navigateTo(Routes.Home, { skipHandle: true, params: getCurrentRouteParams(), query: getCurrentRouteQuery() });
}

async function goToLogin(): Promise<void> {
  await navigateTo(Routes.Account, { skipHandle: true, params: getCurrentRouteParams(), query: getCurrentRouteQuery() });
}

async function createAccount(): Promise<void> {
  if (!validAuth.value) {
    return;
  }
  error.value = '';
  querying.value = true;
  try {
    step.value = Steps.Creating;
    const start = DateTime.now().toMillis();
    if (!creationStepper.email || !choosePasswordRef.value) {
      error.value = 'loginPage.createAccount.errors.missingFields';
      return;
    }
    const email = creationStepper.email!;
    const password = choosePasswordRef.value?.password;
    const result = await creationStepper.createAccount(ParsecAccountAccess.usePasswordForCreate(password));
    const end = DateTime.now().toMillis();
    // Wait for a bit if it's too fast
    const diff = end - start;
    if (diff < 2000) {
      await wait(2000 - diff);
    }
    if (!result.ok) {
      if (result.error.tag === AccountCreateErrorTag.Offline) {
        error.value = 'loginPage.createAccount.errors.offline';
      } else {
        error.value = 'loginPage.createAccount.errors.accountCreationFailed';
      }
    } else {
      const loginResult = await ParsecAccount.login(ParsecAccountAccess.usePasswordForLogin(email, password), creationStepper.server!);
      if (!loginResult.ok) {
        error.value = 'loginPage.createAccount.errors.accountCreationOkLoginFailed';
      }
      step.value = Steps.Created;
    }
  } finally {
    querying.value = false;
  }
}
</script>

<style scoped lang="scss">
.create-account-page-container {
  &::part(background) {
    background: linear-gradient(
      117deg,
      var(--parsec-color-light-secondary-inversed-contrast, #fcfcfc) 0%,
      var(--parsec-color-light-secondary-background, #f9f9fb) 100%
    );
  }
}

.create-account-page {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  gap: 2rem;

  @media screen and (max-height: 600px) {
    padding: 2rem 0;
    justify-content: flex-start !important;
  }

  @include ms.responsive-breakpoint('sm') {
    padding: 3rem 0 1rem;
    justify-content: flex-start !important;
  }

  &-header {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    width: 100%;
    padding-inline: 2rem;
    max-width: calc(28rem + 4rem);
    margin: 0 auto;

    .back-button {
      color: var(--parsec-color-light-secondary-grey);
      font-size: 1.125rem;
      border-radius: var(--parsec-radius-circle);
      padding: 0.25rem;
      background-color: var(--parsec-color-light-secondary-white);
      border: 1px solid var(--parsec-color-light-secondary-premiere);
      box-shadow: var(--parsec-shadow-soft);
      cursor: pointer;
      transition: all 150ms linear;

      &:hover {
        background: var(--parsec-color-light-secondary-disabled);
        color: var(--parsec-color-light-secondary-grey);
      }
    }

    .header-menu {
      display: flex;
      justify-content: space-between;
      align-items: center;

      .homepage-menu-secondary {
        position: absolute;
        top: 2rem;
        right: 2rem;

        @include ms.responsive-breakpoint('sm') {
          position: relative;
          right: -0.5rem;
          top: auto;
        }
      }

      &-button {
        position: absolute;
        top: 2rem;
        right: 2rem;
        color: var(--parsec-color-light-secondary-text);
        height: 2.5rem;
        width: 2.5rem;
        cursor: pointer;

        @include ms.responsive-breakpoint('sm') {
          position: relative;
          right: -0.5rem;
          top: auto;
        }
      }
    }

    &__title {
      background: var(--parsec-color-light-gradient-background);
      background-clip: text;
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
      position: relative;
    }

    &__back {
      position: absolute;
      left: -2.5rem;
      top: 50%;
      transform: translateY(-50%);
    }

    &__subtitle {
      color: var(--parsec-color-light-secondary-hard-grey);
    }

    &--creating {
      .create-account-page-header__title {
        text-align: center;
      }
    }
  }

  &:has(.creating-step) {
    gap: 1.5rem;
  }
}

.step-content {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  width: 100%;
  max-width: 28rem;
  align-items: center;
  margin: 0 auto;

  @include ms.responsive-breakpoint('lg') {
    padding: 0 0 2rem;
  }

  @include ms.responsive-breakpoint('sm') {
    max-width: 30rem;
  }

  @include ms.responsive-breakpoint('xs') {
    padding: 0 1.5rem;
    max-width: 100%;
  }

  .user-information-step__information,
  .validation-email-step__code,
  .authentication-step-content {
    width: 100%;
    padding: 2rem;
    background: var(--parsec-color-light-secondary-white);
    border-radius: var(--parsec-radius-12);
    border: 1px solid var(--parsec-color-light-secondary-premiere);
    box-shadow: var(--parsec-shadow-soft);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2rem;

    @include ms.responsive-breakpoint('sm') {
      padding: 1.5rem;
    }
  }

  .primary-button {
    width: 100%;
  }
}

.user-information-step {
  &-login {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    width: 100%;

    &__description {
      color: var(--parsec-color-light-secondary-hard-grey);
    }

    &__button {
      color: var(--parsec-color-light-primary-500);
      position: relative;

      &::part(native) {
        --background-hover: transparent;
        --background: transparent;
        padding: 0.125rem;
      }

      &::before {
        content: '';
        position: absolute;
        bottom: -0.25rem;
        left: 0.125rem;
        height: 1px;
        width: 0px;
        background: transparent;
        transition: all 150ms linear;
      }

      &:hover {
        color: var(--parsec-color-light-primary-600);

        &::before {
          width: calc(100% - 0.25rem);
          background: var(--parsec-color-light-primary-600);
        }
      }
    }
  }
}

.validation-email-step {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;

  &__code {
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 1rem;
  }

  &-footer {
    width: 100%;
    gap: 1rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    flex-direction: column;

    &__resend {
      align-self: flex-end;
      width: 100%;

      &::part(native) {
        --background: var(--parsec-color-light-secondary-background);
        width: 100%;
        --background-hover: transparent;
        color: var(--parsec-color-light-secondary-text);
      }

      &:hover {
        &::part(native) {
          --background-hover: var(--parsec-color-light-secondary-premiere);
        }
      }
    }
  }
}

.creating-step {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 0;

  &__spinner {
    background: var(--parsec-color-light-secondary-white);
    border-radius: var(--parsec-radius-circle);
    padding: 0.25rem;
    box-shadow: var(--parsec-shadow-soft);
  }
}

.created-step {
  display: flex;
  flex-direction: column;
  gap: 2rem;

  &-welcome {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.25rem;

    &__subtitle {
      color: var(--parsec-color-light-secondary-hard-grey);
    }

    &__name {
      color: var(--parsec-color-light-secondary-text);
    }

    &__description {
      color: var(--parsec-color-light-secondary-hard-grey);
      text-align: center;
    }

    &-login {
      display: flex;
      flex-direction: column;
      gap: 1.5rem;
      width: 100%;

      &__button {
        width: 100%;
        max-width: 17rem;
        margin: auto;
        --box-shadow: var(--parsec-shadow-light);
      }
    }
  }
}
</style>
