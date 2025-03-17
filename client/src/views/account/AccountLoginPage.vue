<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="saas-login">
    <ion-text class="saas-login__title title-h1">{{ $msTranslate('loginPage.title') }}</ion-text>
    <div class="saas-login-container">
      <div class="saas-login-content">
        <div class="saas-login-content-list">
          <ms-input
            class="saas-login-content__input"
            ref="serverInputRef"
            v-model="server"
            label="loginPage.server"
            @on-enter-keyup="onLoginClicked()"
            :validator="parsecAddrValidator"
          />
          <!-- email -->
          <ms-input
            class="saas-login-content__input"
            ref="emailInputRef"
            v-model="email"
            label="loginPage.email"
            @on-enter-keyup="onLoginClicked()"
            :validator="emailValidator"
          />
          <!-- password -->
          <div class="input-password">
            <ms-password-input
              class="saas-login-content__input"
              ref="passwordInputRef"
              v-model="password"
              label="loginPage.password"
              @on-enter-keyup="onLoginClicked()"
            />
          </div>
        </div>
        <!-- server -->

        <!-- back and login buttons -->
        <div class="saas-login-button">
          <ion-button
            :disabled="disabled || !validInfo"
            @click="onLoginClicked"
            class="saas-login-button__item"
            size="large"
          >
            {{ $msTranslate('loginPage.login') }}
            <ms-spinner
              class="saas-login-button__spinner"
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
import { computed, onMounted, ref } from 'vue';
import { Env } from '@/services/environment';
import { AccountHandle, ParsecAccount } from '@/parsec';

defineProps<{
  disabled?: boolean;
}>();

const emits = defineEmits<{
  (e: 'loginSuccess', handle: AccountHandle): void;
}>();

const email = ref<string>('');
const password = ref<string>('');
const server = ref<string>(Env.getAccountServer());
const emailInputRef = ref();
const passwordInputRef = ref();
const serverInputRef = ref();
const querying = ref(false);
const loginError = ref<Translatable>('');

const validInfo = computed(() => {
  return Boolean(
    email.value.length > 0 &&
      emailInputRef.value !== undefined &&
      emailInputRef.value.validity === Validity.Valid &&
      !querying.value &&
      password.value.length > 0 &&
      serverInputRef.value !== undefined &&
      serverInputRef.value.validity === Validity.Valid,
  );
});

onMounted(async () => {
  querying.value = false;

  if (email.value.length > 0) {
    await emailInputRef.value.validate(email.value);
    await passwordInputRef.value.setFocus();
  } else {
    await emailInputRef.value.setFocus();
  }
  await serverInputRef.value.validate(server.value);
});

async function onLoginClicked(): Promise<void> {
  if (email.value.length === 0 || password.value.length === 0 || server.value.length === 0) {
    return;
  }
  querying.value = true;
  try {
    const result = await ParsecAccount.login(email.value, password.value, server.value);

    if (result.ok) {
      emits('loginSuccess', result.value);
    } else {
      loginError.value = 'FAILED TO LOG IN';
    }
  } finally {
    querying.value = false;
  }
}
</script>

<style scoped lang="scss">
.saas-login {
  display: flex;
  flex-direction: column;
  align-items: center;
  margin-top: auto;
  width: 100%;
  gap: 2rem;

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

      .saas-login-content__input {
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
}
</style>
