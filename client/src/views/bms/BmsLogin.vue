<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <div>
      <ms-input
        ref="emailInput"
        v-model="email"
        :validator="emailValidator"
      />
      <ms-password-input v-model="password" />
      <!-- TODO: UPDATE THE LINK -->
      <!-- TODO: CHECK THAT ELECTRON ALLOWS THE LINK TO BE OPENED -->
      <a
        class="link"
        target="_blank"
        @click="$event.stopPropagation()"
        href="FORGOT_PASSWORD_LINK"
      >
        FORGOT MY PASSWORD
      </a>
      <ion-button
        :disabled="!emailInput || emailInput.validity !== Validity.Valid || !password.length"
        @click="onLoginClicked"
      >
        LOG IN
      </ion-button>
      <ms-spinner v-show="querying" />

      <div
        class="form-error"
        v-show="loginError"
      >
        {{ $msTranslate(loginError) }}
      </div>

      <!-- TODO: UPDATE THE LINK -->
      <!-- TODO: CHECK THAT ELECTRON ALLOWS THE LINK TO BE OPENED -->
      <a
        class="link"
        target="_blank"
        @click="$event.stopPropagation()"
        href="ORG_CREATION_LINK"
      >
        CREATE MY ACCOUNT
      </a>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonButton } from '@ionic/vue';
import { MsInput, MsPasswordInput, Translatable, Validity, MsSpinner } from 'megashark-lib';
import { emailValidator } from '@/common/validators';
import { ref } from 'vue';
import { AuthenticationToken, BmsApi, DataType } from '@/services/bmsApi';

const props = defineProps<{
  email?: string;
}>();

const emits = defineEmits<{
  (e: 'loginSuccess', token: AuthenticationToken): void;
}>();

const email = ref<string>(props.email ?? '');
const password = ref<string>('');
const emailInput = ref();
const querying = ref(false);
const loginError = ref<Translatable>('');

async function onLoginClicked(): Promise<void> {
  if (email.value.length === 0 || password.value.length === 0) {
    return;
  }
  querying.value = true;
  const response = await BmsApi.login({ email: email.value, password: password.value });

  if (response.isError() || !response.data || response.data.type !== DataType.Login) {
    loginError.value = 'FAILED';
  } else {
    emits('loginSuccess', response.data.token);
  }
  querying.value = false;
}
</script>

<style scoped lang="scss"></style>
