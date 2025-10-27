<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="account-login">
    <ion-text class="account-login__title title-h1">{{ $msTranslate('loginPage.title') }}</ion-text>
    <div class="account-login-container">
      <div class="account-login-content">
        <div class="account-login-content-list">
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
            class="login-server-input account-login-content__input"
            :class="{ 'login-server-input-disabled': !isEditingServer }"
            ref="serverInput"
            v-model="server"
            label="loginPage.inputFields.server"
            @on-enter-keyup="onLoginClicked()"
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
          <!-- email -->
          <ms-input
            class="account-login-content__input"
            ref="emailInput"
            v-model="email"
            label="loginPage.inputFields.email"
            @on-enter-keyup="onLoginClicked()"
            :validator="emailValidator"
          />
          <!-- password -->
          <div class="input-password">
            <ms-password-input
              class="account-login-content__input"
              ref="passwordInput"
              v-model="password"
              label="loginPage.inputFields.password"
              @on-enter-keyup="onLoginClicked()"
            />
            <ion-button
              class="input-password__forgot-password button-medium"
              @click="recoverAccount"
            >
              {{ $msTranslate('loginPage.forgotPassword') }}
            </ion-button>
          </div>
        </div>
        <!-- server -->

        <!-- back and login buttons -->
        <div class="account-login-button">
          <ion-button
            :disabled="disabled || !validInfo"
            @click="onLoginClicked"
            class="account-login-button__item"
            size="large"
          >
            {{ $msTranslate('loginPage.login') }}
            <ms-spinner
              class="account-login-button__spinner"
              v-show="querying"
            />
          </ion-button>
          <!-- error -->
          <ion-text
            class="form-error body login-button-error"
            v-show="loginError"
          >
            <ion-icon
              class="form-error-icon"
              :icon="warning"
            />{{ $msTranslate(loginError) }}
          </ion-text>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { emailValidator, parsecAddrValidator } from '@/common/validators';
import { AccountHandle, ParsecAccount, ParsecAccountAccess } from '@/parsec';
import { navigateTo, Routes } from '@/router';
import { Env } from '@/services/environment';
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { warning } from 'ionicons/icons';
import { MsInput, MsPasswordInput, MsSpinner, Translatable, Validity } from 'megashark-lib';
import { computed, onMounted, ref, useTemplateRef } from 'vue';

defineProps<{
  disabled?: boolean;
}>();

const emits = defineEmits<{
  (e: 'loginSuccess', handle: AccountHandle): void;
}>();

const email = ref<string>('');
const password = ref<string>('');
const server = ref<string>(Env.getAccountServer());
const isEditingServer = ref(false);
const emailInputRef = useTemplateRef<InstanceType<typeof MsInput>>('emailInput');
const passwordInputRef = useTemplateRef<InstanceType<typeof MsPasswordInput>>('passwordInput');
const serverInputRef = useTemplateRef<InstanceType<typeof MsInput>>('serverInput');
const querying = ref(false);
const loginError = ref<Translatable>('');

const validInfo = computed(() => {
  return Boolean(
    email.value.length > 0 &&
      emailInputRef.value &&
      emailInputRef.value.validity === Validity.Valid &&
      !querying.value &&
      password.value.length > 0 &&
      serverInputRef.value &&
      serverInputRef.value.validity === Validity.Valid,
  );
});

onMounted(async () => {
  querying.value = false;

  await serverInputRef.value?.setFocus();
  if (email.value.length > 0) {
    await emailInputRef.value?.validate(email.value);
    await passwordInputRef.value?.setFocus();
  } else {
    await emailInputRef.value?.setFocus();
  }
  await serverInputRef.value?.validate(server.value);
});

async function toggleEditServer(): Promise<void> {
  isEditingServer.value = !isEditingServer.value;
  if (isEditingServer.value) {
    serverInputRef.value?.setFocus();
  }
  await serverInputRef.value?.validate(server.value);
}

async function recoverAccount(): Promise<void> {
  await navigateTo(Routes.RecoverAccount, { skipHandle: true });
}

async function onLoginClicked(): Promise<void> {
  if (email.value.length === 0 || password.value.length === 0 || server.value.length === 0) {
    return;
  }
  querying.value = true;
  try {
    const result = await ParsecAccount.login(ParsecAccountAccess.usePasswordForLogin(email.value, password.value), server.value);

    if (result.ok) {
      emits('loginSuccess', result.value);
    } else {
      loginError.value = 'loginPage.loginFailed';
    }
  } finally {
    querying.value = false;
  }
}
</script>

<style scoped lang="scss">
.account-login {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-top: auto;
  width: 100%;
  gap: 2rem;

  @include ms.responsive-breakpoint('sm') {
    gap: 1.5rem;
  }

  &__title {
    background: var(--parsec-color-light-gradient-background);
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    margin: 0.25rem 0;
  }

  &-container {
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

    @include ms.responsive-breakpoint('sm') {
      padding: 1.5rem;
    }
  }

  &-content {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.5rem;

    &-list {
      width: 100%;
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 1.25rem;
      position: relative;
    }

    &__input {
      width: 100%;
    }

    .input-password {
      width: 100%;
      display: flex;
      flex-direction: column;

      .account-login-content__input {
        width: 100%;
      }

      &__forgot-password {
        width: fit-content;
        text-align: right;
        --background: transparent;
        --background-hover: transparent;
        --color: var(--parsec-color-light-secondary-text);
        opacity: 0.5;
        position: relative;

        &::part(native) {
          padding: 0.375rem 0;
        }

        &:hover {
          opacity: 0.9;
        }
      }
    }
  }

  &-button {
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    &__spinner {
      width: 1rem;
      height: 1rem;
      margin-left: 0.5rem;
    }

    &__item {
      width: 100%;
    }
  }

  .login-button-error {
    display: flex;
    align-items: baseline;
    gap: 0.5rem;
    position: relative;

    ion-icon {
      position: relative;
      top: 0.125rem;
    }
  }
}
</style>
