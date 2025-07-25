<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="account-login">
    <ion-text class="account-login__title title-h1">{{ $msTranslate('loginPage.title') }}</ion-text>
    <div class="account-login-container">
      <div class="account-login-content">
        <div class="account-login-content-list">
          <ms-input
            class="account-login-content__input"
            ref="serverInput"
            v-model="server"
            label="loginPage.inputFields.server"
            @on-enter-keyup="onLoginClicked()"
            :validator="parsecAddrValidator"
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
import { IonButton, IonText, IonIcon } from '@ionic/vue';
import { MsInput, MsPasswordInput, Translatable, Validity, MsSpinner } from 'megashark-lib';
import { emailValidator, parsecAddrValidator } from '@/common/validators';
import { warning } from 'ionicons/icons';
import { computed, onMounted, ref, useTemplateRef } from 'vue';
import { Env } from '@/services/environment';
import { AccountHandle, ParsecAccount, ParsecAccountAccess } from '@/parsec';

defineProps<{
  disabled?: boolean;
}>();

const emits = defineEmits<{
  (e: 'loginSuccess', handle: AccountHandle): void;
}>();

const email = ref<string>('');
const password = ref<string>('');
const server = ref<string>(Env.getAccountServer());
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

  if (email.value.length > 0) {
    await emailInputRef.value?.validate(email.value);
    await passwordInputRef.value?.setFocus();
  } else {
    await emailInputRef.value?.setFocus();
  }
  await serverInputRef.value?.validate(server.value);
});

async function onLoginClicked(): Promise<void> {
  if (email.value.length === 0 || password.value.length === 0 || server.value.length === 0) {
    return;
  }
  querying.value = true;
  try {
    const result = await ParsecAccount.login(ParsecAccountAccess.usePassword(email.value, password.value), server.value);

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
  }

  &-content {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 2rem;

    &-list {
      width: 100%;
      display: flex;
      flex-direction: column;
      align-items: center;
      gap: 1.25rem;
    }

    &__input {
      width: 100%;
    }

    .input-password {
      width: 100%;
      display: flex;
      flex-direction: column;
      align-items: center;

      .account-login-content__input {
        width: 100%;
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
