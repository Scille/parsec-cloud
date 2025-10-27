<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      class="recover-account"
    >
      <div class="recover-account-content">
        <div class="recover-account-text">
          <ion-text
            class="recover-account-text__title title-h1"
            v-if="TITLES[step].title"
          >
            {{ $msTranslate(TITLES[step].title) }}
          </ion-text>
          <ion-text
            class="recover-account-text__subtitle body-lg"
            v-if="TITLES[step].subtitle"
          >
            {{ $msTranslate(TITLES[step].subtitle) }}
          </ion-text>
          <div
            class="recover-account-text-email"
            v-if="step === Steps.Code && email"
          >
            <ion-text class="recover-account-text-email__address button-medium">{{ email }}</ion-text>
            <ion-text
              class="recover-account-text-email__button button-medium"
              @click="onFocusEmailInput"
            >
              {{ $msTranslate('loginPage.recoverAccount.step2.update') }}
            </ion-text>
          </div>
        </div>
        <ms-report-text
          v-if="error"
          :theme="MsReportTheme.Error"
          class="recover-account-text__error"
        >
          {{ $msTranslate(error) }}
        </ms-report-text>

        <div
          v-show="step === Steps.Email"
          class="recover-account-step"
        >
          <ion-text
            button
            class="input-edit-button button-small"
            :class="{ 'input-edit-button--active': isEditingServer }"
            @click="toggleEditServer()"
            :disabled="isEditingServer"
          >
            {{ isEditingServer ? $msTranslate('loginPage.inputFields.cancel') : $msTranslate('loginPage.inputFields.edit') }}
          </ion-text>
          <ms-input
            v-show="isEditingServer"
            class="login-server-input"
            :class="{ 'login-server-input-disabled': !isEditingServer }"
            label="loginPage.inputFields.server"
            v-model="server"
            ref="serverInput"
            :disabled="!isEditingServer"
            :validator="parsecAddrValidator"
          />
          <ms-input
            v-if="!isEditingServer"
            class="login-server-input account-login-content__input login-server-input-disabled"
            :value="server.replace('parsec3://', '')"
            label="loginPage.inputFields.server"
            :disabled="true"
          />
          <ms-input
            label="loginPage.inputFields.email"
            v-model="email"
            ref="emailInput"
            :validator="emailValidator"
          />
          <ion-button
            class="recover-account-step__button button-default button-large"
            @click="sendCode"
            :disabled="emailInputRef?.validity !== Validity.Valid || serverInputRef?.validity !== Validity.Valid || querying"
          >
            {{ $msTranslate('loginPage.recoverAccount.step2.nextButton') }}
          </ion-button>
        </div>

        <div
          v-show="step === Steps.Code"
          class="recover-account-step"
        >
          <ms-code-validation-input
            ref="codeInput"
            :allowed-input="AllowedInput.UpperAlphaNumeric"
            :code-length="6"
            @code-complete="onCodeComplete"
          />
          <ion-button
            @click="codeEntered"
            :disabled="code.length !== 6 || querying"
            class="recover-account-step__button button-default button-large"
          >
            {{ $msTranslate('loginPage.recoverAccount.step2.nextButton') }}
          </ion-button>
          <ion-text
            button
            @click="resendCode"
            :disabled="resendDisabled"
            class="send-code subtitles-sm"
          >
            {{ $msTranslate('loginPage.recoverAccount.step2.resendCode') }}
          </ion-text>
        </div>

        <div
          v-show="step === Steps.NewAuthentication"
          class="recover-account-step authentication-step"
        >
          <ms-choose-password-input
            ref="passwordInput"
            @on-enter-keyup="onPasswordChosen"
          />
          <ion-button
            class="recover-account-step__button button-default button-large"
            @click="onPasswordChosen"
            :disabled="!validAuth || querying"
          >
            {{ $msTranslate('loginPage.recoverAccount.step3.nextButton') }}
            <ms-spinner
              class="recover-account-step__button-spinner"
              v-show="querying"
              :size="14"
            />
          </ion-button>
        </div>

        <div
          v-show="step === Steps.Finished"
          class="recover-account-step"
        >
          <ion-text class="recover-account-step__text body">{{ $msTranslate('loginPage.recoverAccount.step4.description') }}</ion-text>
          <ion-button
            class="recover-account-step__button button-default button-large"
            @click="login"
            :disabled="querying"
          >
            {{ $msTranslate('loginPage.recoverAccount.step4.nextButton') }}
          </ion-button>
        </div>
        <ion-button
          class="recover-account__back-button button-medium"
          @click="backToLogin"
        >
          <ion-icon
            class="button-icon"
            slot="start"
            :icon="home"
          />
          {{ $msTranslate('loginPage.recoverAccount.backButton') }}
        </ion-button>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { emailValidator, parsecAddrValidator } from '@/common/validators';
import { AccountRecoverProceedErrorTag, AccountRecoverSendValidationEmailErrorTag, ParsecAccount, ParsecAccountAccess } from '@/parsec';
import { getCurrentRouteParams, getCurrentRouteQuery, navigateTo, Routes, watchRoute } from '@/router';
import { Env } from '@/services/environment';
import { IonButton, IonContent, IonIcon, IonPage, IonText } from '@ionic/vue';
import { home } from 'ionicons/icons';
import {
  AllowedInput,
  asyncComputed,
  MsChoosePasswordInput,
  MsCodeValidationInput,
  MsInput,
  MsReportText,
  MsReportTheme,
  MsSpinner,
  Translatable,
  Validity,
} from 'megashark-lib';
import { onMounted, onUnmounted, ref, useTemplateRef } from 'vue';

enum Steps {
  Email = 0,
  Code = 1,
  NewAuthentication = 2,
  Finished = 3,
}

const step = ref<Steps>(Steps.Email);
const email = ref('');
const code = ref<Array<string>>([]);
const password = ref('');
const server = ref(Env.getAccountServer());
const isEditingServer = ref(false);
const error = ref('');
const resendDisabled = ref(false);
const querying = ref(false);

const passwordInputRef = useTemplateRef<typeof MsChoosePasswordInput>('passwordInput');
const serverInputRef = useTemplateRef('serverInput');
const emailInputRef = useTemplateRef('emailInput');
const codeInputRef = useTemplateRef('codeInput');

const TITLES: Array<{ title?: Translatable; subtitle?: Translatable }> = [
  {
    title: 'loginPage.recoverAccount.step1.title',
    subtitle: 'loginPage.recoverAccount.step1.description',
  },
  {
    title: 'loginPage.recoverAccount.step2.title',
    subtitle: 'loginPage.recoverAccount.step2.description',
  },
  {
    title: 'loginPage.recoverAccount.step3.title',
    subtitle: 'loginPage.recoverAccount.step3.description',
  },
  {
    title: 'loginPage.recoverAccount.step4.title',
  },
  {},
];

async function toggleEditServer(): Promise<void> {
  isEditingServer.value = !isEditingServer.value;
  if (isEditingServer.value) {
    serverInputRef.value?.setFocus();
  }
  await serverInputRef.value?.validate(server.value);
}

// Should be updated
async function resendCode(): Promise<void> {
  if (!email.value || !server.value) {
    return;
  }
  resendDisabled.value = true;
  const result = await ParsecAccount.sendRecoveryEmail(email.value, server.value);
  if (!result.ok) {
    error.value = 'loginPage.recoverAccount.error.resendFailed';
  }
  setTimeout(() => {
    resendDisabled.value = false;
  }, 5000);
}

const validAuth = asyncComputed(async () => {
  return step.value === Steps.NewAuthentication && passwordInputRef.value && (await passwordInputRef.value.areFieldsCorrect());
});

const watchCancel = watchRoute(async (newRoute, oldRoute) => {
  if (oldRoute.name !== newRoute.name) {
    await reset();
  }
});

async function reset(): Promise<void> {
  step.value = Steps.Email;
  email.value = '';
  code.value = [];
  password.value = '';
  server.value = Env.getAccountServer();
  error.value = '';
  resendDisabled.value = false;
  querying.value = false;
  await serverInputRef.value?.setFocus();
  await serverInputRef.value?.validate(server.value);
  await emailInputRef.value?.validate(email.value);
  await emailInputRef.value?.setFocus();
  await codeInputRef.value?.clear();
  await passwordInputRef.value?.clear();
}

onMounted(async () => {
  await reset();
});

onUnmounted(() => {
  watchCancel();
});

function onFocusEmailInput(): void {
  step.value = Steps.Email;
  emailInputRef.value?.setFocus();
}

async function sendCode(): Promise<void> {
  if (!email.value || !server.value) {
    return;
  }
  querying.value = true;
  const result = await ParsecAccount.recoveryRequest(email.value, server.value);
  if (result.ok) {
    step.value = Steps.Code;
    await codeInputRef.value?.setFocus();
    error.value = '';
  } else {
    if (result.error.tag === AccountRecoverSendValidationEmailErrorTag.Offline) {
      error.value = 'loginPage.recoverAccount.error.offline';
    } else {
      error.value = 'loginPage.recoverAccount.error.generic';
    }
  }
  querying.value = false;
}

async function onCodeComplete(completeCode: Array<string>): Promise<void> {
  code.value = completeCode;
}

async function codeEntered(): Promise<void> {
  if (code.value.length !== 6) {
    return;
  }
  error.value = '';
  step.value = Steps.NewAuthentication;
}

async function onPasswordChosen(): Promise<void> {
  if (!validAuth.value || !passwordInputRef.value) {
    return;
  }
  password.value = await passwordInputRef.value.password;
  querying.value = true;
  const result = await ParsecAccount.recoveryProceed(
    email.value,
    ParsecAccountAccess.usePasswordForCreate(password.value),
    code.value,
    server.value,
  );
  if (!result.ok) {
    if (result.error.tag === AccountRecoverProceedErrorTag.InvalidValidationCode) {
      step.value = Steps.Code;
      error.value = 'loginPage.recoverAccount.error.invalidCode';
      await codeInputRef.value?.setFocus();
    } else if (result.error.tag === AccountRecoverProceedErrorTag.Offline) {
      error.value = 'loginPage.recoverAccount.error.offline';
    } else {
      error.value = 'loginPage.recoverAccount.error.generic';
    }
    code.value = [];
    await codeInputRef.value?.setFocus();
  } else {
    step.value = Steps.Finished;
  }
  querying.value = false;
}

async function login(): Promise<void> {
  const result = await ParsecAccount.login(ParsecAccountAccess.usePasswordForLogin(email.value, password.value), server.value);
  if (result.ok) {
    await navigateTo(Routes.Home, { skipHandle: true, params: getCurrentRouteParams(), query: getCurrentRouteQuery() });
  } else {
    await navigateTo(Routes.Account, { skipHandle: true, params: getCurrentRouteParams(), query: getCurrentRouteQuery() });
  }
}

async function backToLogin(): Promise<void> {
  await navigateTo(Routes.Account, { skipHandle: true, params: getCurrentRouteParams(), query: getCurrentRouteQuery() });
}
</script>

<style scoped lang="scss">
.recover-account {
  position: relative;
  height: 100vh;
  display: flex;
  flex-direction: column;
  overflow: hidden;

  &::part(background) {
    background: linear-gradient(
      117deg,
      var(--parsec-color-light-secondary-inversed-contrast, #fcfcfc) 0%,
      var(--parsec-color-light-secondary-background, #f9f9fb) 100%
    );
  }
}

.recover-account-content {
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 1.5rem;
  width: 100%;
  height: 100%;
  max-width: 28rem;
  align-items: center;
  margin: auto;
  transition: all 0.2s ease-in-out;

  @include ms.responsive-breakpoint('lg') {
    padding: 2rem 0;
  }

  @include ms.responsive-breakpoint('sm') {
    max-width: 30rem;
  }

  @include ms.responsive-breakpoint('xs') {
    padding: 2rem 1.5rem 1rem;
  }

  @media screen and (max-height: 800px) {
    padding: 2rem 1.5rem 1rem;
    margin: 2rem auto;
    justify-content: flex-start;
  }
}

.recover-account-text {
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: 0.75rem;

  &__title {
    background: var(--parsec-color-light-gradient-background);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
  }

  &__subtitle {
    color: var(--parsec-color-light-secondary-hard-grey);
  }

  &__error {
    width: 100%;
  }

  &-email {
    display: flex;
    gap: 0.5rem;
    margin-top: 0.5rem;

    &__address {
      color: var(--parsec-color-light-secondary-text);
      font-weight: 500;
      text-align: center;
    }

    &__button {
      color: var(--parsec-color-light-primary-500);
      text-align: center;
      cursor: pointer;

      &:hover {
        color: var(--parsec-color-light-primary-700);
      }
    }
  }
}

.recover-account-step {
  width: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  background: var(--parsec-color-light-secondary-white);
  border-radius: var(--parsec-radius-12);
  box-shadow: var(--parsec-shadow-soft);
  padding: 2rem;
  gap: 1rem;
  position: relative;

  .input-edit-button {
    right: 2.25rem;
  }

  &__text {
    width: 100%;
    color: var(--parsec-color-light-secondary-hard-grey);
  }

  &__button {
    flex-grow: 1;
    width: 100%;
    margin-top: 0.5rem;
  }

  &__button-spinner {
    margin-left: 0.5rem;
  }
}

// step validation code
.send-code {
  width: 100%;
  background: var(--parsec-color-light-secondary-background);
  text-align: center;
  padding: 0.625rem 0.75rem;
  border-radius: var(--parsec-radius-8);
  color: var(--parsec-color-light-secondary-text);
  cursor: pointer;

  &:hover {
    background: var(--parsec-color-light-secondary-premiere);
  }
}

// back button
.recover-account__back-button {
  padding-bottom: 2rem;

  .button-icon {
    font-size: 1.125rem;
    margin-right: 0.5rem;
  }

  &::part(native) {
    --background: transparent;
    --background-hover: var(--parsec-color-light-secondary-medium);
    color: var(--parsec-color-light-secondary-text);
  }
}
</style>
