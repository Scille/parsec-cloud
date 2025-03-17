<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="saas-login">
    <div class="saas-login-container">
      <ion-button @click="onSkipClicked"> SKIP </ion-button>
      <div class="saas-login-content">
        <!-- server -->
        <ms-input
          class="saas-login-content__input"
          ref="serverInputRef"
          v-model="server"
          label="SERVER"
          @on-enter-keyup="onLoginClicked()"
          :validator="parsecAddrValidator"
        />
        <!-- email -->
        <ms-input
          class="saas-login-content__input"
          ref="emailInputRef"
          v-model="email"
          label="clientArea.app.emailLabel"
          @on-enter-keyup="onLoginClicked()"
          :validator="emailValidator"
        />
        <!-- password -->
        <div class="input-password">
          <ms-password-input
            class="saas-login-content__input"
            ref="passwordInputRef"
            v-model="password"
            label="clientArea.app.password"
            @on-enter-keyup="onLoginClicked()"
          />
        </div>

        <!-- back and login buttons -->
        <div class="saas-login-button">
          <ion-button
            :disabled="!validInfo"
            @click="onLoginClicked"
            class="saas-login-button__item"
            size="large"
          >
            {{ $msTranslate('clientArea.app.login') }}
          </ion-button>
          <ms-spinner v-show="querying" />
        </div>

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
</template>

<script setup lang="ts">
import { IonButton, IonText, IonIcon } from '@ionic/vue';
import { MsInput, MsPasswordInput, Translatable, Validity, MsSpinner } from 'megashark-lib';
import { emailValidator, parsecAddrValidator } from '@/common/validators';
import { warning } from 'ionicons/icons';
import { computed, onMounted, ref } from 'vue';
import { Env } from '@/services/environment';
import { AccountHandle, ParsecAccount } from '@/parsec';

const emits = defineEmits<{
  (e: 'loginSuccess', handle: AccountHandle): void;
  (e: 'skipClick'): void;
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

async function onSkipClicked(): Promise<void> {
  ParsecAccount.markSkipped();
  emits('skipClick');
}
</script>

<style scoped lang="scss"></style>
