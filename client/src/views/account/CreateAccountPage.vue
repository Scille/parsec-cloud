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
          <ion-text
            class="create-account-page-header__title title-h1"
            v-if="TITLES[step].title"
          >
            {{ $msTranslate(TITLES[step].title) }}
          </ion-text>
          <ion-text
            class="create-account-page-header__subtitle body"
            v-if="TITLES[step].subtitle"
          >
            {{ $msTranslate(TITLES[step].subtitle) }}
          </ion-text>
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
            <ion-text class="user-information-step-login__description body">
              {{ $msTranslate('loginPage.createAccount.alreadyHaveAnAccount') }}
            </ion-text>
            <ion-button
              class="user-information-step-login__button tertiary-button"
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
              ref="codeValidationInputRef"
              :code-length="6"
              :allowed-input="AllowedInput.UpperAlphaNumeric"
              @code-complete="onCodeComplete"
              :disabled="querying"
            />

            <ion-button
              class="primary-button"
              @click="validateCode"
              :disabled="code.length === 0"
            >
              {{ $msTranslate('loginPage.createAccount.createAccountButton') }}
            </ion-button>

            <div class="validation-email-step-footer">
              <ion-text
                v-show="error"
                class="form-error body validation-email-step-footer__error"
              >
                <ion-icon
                  class="form-error-icon"
                  :icon="warning"
                />
                {{ $msTranslate(error) }}
              </ion-text>

              <div class="footer-content">
                <ion-button
                  class="validation-email-step-footer__resend"
                  @click="resendCode"
                  :disabled="querying"
                >
                  {{ $msTranslate('loginPage.createAccount.resendCode') }}
                </ion-button>
                <ms-spinner v-show="querying" />
              </div>
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
              ref="choosePasswordRef"
              class="choose-password"
              @on-enter-keyup="createAccount"
            />
            <ion-button
              class="primary-button"
              @click="createAccount"
              :disabled="!validAuth"
            >
              {{ $msTranslate('loginPage.createAccount.nextButton') }}
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
              class="created-step-welcome-login__button"
              @click="goToHome"
            >
              {{ $msTranslate('loginPage.createAccount.accessToOrganizationButton') }}
            </ion-button>
            <ion-button
              v-if="error"
              class="created-step-welcome-login__button"
              size="large"
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
import { IonPage, IonContent, IonButton, IonText, IonIcon } from '@ionic/vue';
import {
  asyncComputed,
  MsCodeValidationInput,
  MsSpinner,
  Translatable,
  MsReportText,
  MsReportTheme,
  MsChoosePasswordInput,
  AllowedInput,
} from 'megashark-lib';
import { onUnmounted, ref } from 'vue';
import AccountUserInformation from '@/components/account/AccountUserInformation.vue';
import { AccountCreateErrorTag, AccountCreationStepper, ParsecAccount, ParsecAccountAccess } from '@/parsec';
import { wait } from '@/parsec/internals';
import { getCurrentRouteParams, getCurrentRouteQuery, navigateTo, Routes, watchRoute } from '@/router';
import { DateTime } from 'luxon';
import { warning } from 'ionicons/icons';
import { RouteLocationNormalizedLoaded } from 'vue-router';

enum Steps {
  UserInformation = 0,
  Code = 1,
  Authentication = 2,
  Creating = 3,
  Created = 4,
}

const step = ref<Steps>(Steps.UserInformation);
const codeValidationInputRef = ref<typeof MsCodeValidationInput>();
const creationStepper = new AccountCreationStepper();
const error = ref('');
const querying = ref(false);
const choosePasswordRef = ref<typeof MsChoosePasswordInput>();
const refreshKey = ref(0);
const code = ref<Array<string>>([]);

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

// As always with Vue, you cannot trust mounted/on mounted
const watchRouteCancel = watchRoute(async (newRoute: RouteLocationNormalizedLoaded, oldRoute: RouteLocationNormalizedLoaded) => {
  if (newRoute.name === Routes.CreateAccount && oldRoute.name !== Routes.CreateAccount) {
    step.value = Steps.UserInformation;
    await creationStepper.reset();
    error.value = '';
    querying.value = false;
    refreshKey.value += 1;
  }
});

onUnmounted(async () => {
  watchRouteCancel();
});

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
  error.value = '';
  querying.value = true;
  try {
    step.value = Steps.Creating;
    const start = DateTime.now().toMillis();
    const access = ParsecAccountAccess.usePassword(creationStepper.email!, choosePasswordRef.value?.password);
    const result = await creationStepper.createAccount(access);
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
      const loginResult = await ParsecAccount.login(access, creationStepper.server!);
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
.create-account-page {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  gap: 2rem;
  background: linear-gradient(
    117deg,
    var(--parsec-color-light-secondary-inversed-contrast, #fcfcfc) 0%,
    var(--parsec-color-light-secondary-background, #f9f9fb) 100%
  );

  &-header {
    display: flex;
    flex-direction: column;
    gap: 1rem;
    width: 100%;
    padding-inline: 2rem;
    max-width: calc(28rem + 4rem);
    margin: 0 auto;

    &__title {
      background: var(--parsec-color-light-gradient-background);
      background-clip: text;
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
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
    gap: 1rem;
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
    padding: 2rem 0;
  }

  @include ms.responsive-breakpoint('sm') {
    max-width: 30rem;
  }

  @include ms.responsive-breakpoint('xs') {
    padding: 1.5rem 1.5rem 0;
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
    gap: 1.5rem;
  }

  .primary-button {
    width: 100%;
  }

  .tertiary-button {
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
    flex-direction: column-reverse;

    .footer-content {
      width: 100%;
      display: flex;
      justify-content: space-between;
      align-items: center;
      gap: 1rem;
    }

    &__resend {
      align-self: flex-end;
      width: 100%;

      &::part(native) {
        --background: var(--parsec-color-light-secondary-background);
        width: 100%;
        --background-hover: transparent;
        color: var(--parsec-color-light-secondary-text);
        padding: 0.125rem;
      }

      &:hover {
        &::part(native) {
          --background-hover: var(--parsec-color-light-secondary-premiere);
        }
      }
    }

    &__error {
      flex-shrink: 0;
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
  gap: 2.5rem;

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
      }
    }
  }
}
</style>
