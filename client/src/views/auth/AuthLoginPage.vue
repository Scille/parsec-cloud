<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="saas-login">
    <div class="saas-login-container">
      <ion-button @click="onSkipClicked"> SKIP </ion-button>
      <div class="saas-login-content">
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
            :disabled="!validEmail || !password.length || querying"
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

      <ion-footer class="saas-login-footer">
        <div class="create-account">
          <ion-text class="create-account__text body">{{ $msTranslate('clientArea.app.noAccount') }}</ion-text>
          <a
            class="create-account__link button-medium"
            target="_blank"
            @click="$event.stopPropagation()"
            :href="Env.getSignUrl()"
          >
            {{ $msTranslate('clientArea.app.createAccount') }}
            <ion-icon :icon="arrowForward" />
          </a>
        </div>
      </ion-footer>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonButton, IonText, IonFooter, IonIcon } from '@ionic/vue';
import { MsInput, MsPasswordInput, Translatable, Validity, MsSpinner } from 'megashark-lib';
import { emailValidator } from '@/common/validators';
import { warning, arrowForward } from 'ionicons/icons';
import { computed, onMounted, ref } from 'vue';
import { Env } from '@/services/environment';
import { AuthHandle, ParsecAuth } from '@/parsec';

const emits = defineEmits<{
  (e: 'loginSuccess', handle: AuthHandle): void;
  (e: 'skipClick'): void;
}>();

const email = ref<string>('');
const password = ref<string>('');
const emailInputRef = ref();
const passwordInputRef = ref();
const querying = ref(false);
const loginError = ref<Translatable>('');

const validEmail = computed(() => {
  return Boolean(email.value.length > 0 && emailInputRef.value && emailInputRef.value.validity === Validity.Valid);
});

onMounted(async () => {
  querying.value = false;

  if (email.value.length > 0) {
    await emailInputRef.value.validate(email.value);
    await passwordInputRef.value.setFocus();
  } else {
    await emailInputRef.value.setFocus();
  }
});

async function onLoginClicked(): Promise<void> {
  if (email.value.length === 0 || password.value.length === 0) {
    return;
  }
  querying.value = true;
  try {
    const result = await ParsecAuth.login(email.value, password.value);

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
  ParsecAuth.markSkipped();
  emits('skipClick');
}
</script>

<style scoped lang="scss"></style>
