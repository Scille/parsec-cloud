<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <div class="account-login-content-list">
      <ms-input
        class="account-login-content__input"
        ref="serverInput"
        v-model="server"
        label="loginPage.inputFields.server"
        @on-enter-keyup="submit"
        :validator="parsecAddrValidator"
        :disabled="querying"
      />
      <ms-input
        class="account-login-content__input"
        v-model="firstName"
        label="loginPage.inputFields.firstname"
        @on-enter-keyup="submit"
        :disabled="querying"
      />
      <ms-input
        class="account-login-content__input"
        v-model="lastName"
        label="loginPage.inputFields.lastname"
        @on-enter-keyup="submit"
        :disabled="querying"
      />
      <ms-input
        class="account-login-content__input"
        ref="emailInput"
        v-model="email"
        label="loginPage.inputFields.email"
        @on-enter-keyup="submit()"
        :validator="emailValidator"
        :disabled="querying"
      />
    </div>
    <div class="account-login-content-button">
      <ion-button
        class="account-login-content-button__item button-large"
        @click="submit"
        :disabled="querying || !validInfo"
      >
        {{ $msTranslate('loginPage.createAccount.nextButton') }}
        <ms-spinner
          class="account-login-content-button__spinner"
          v-show="querying"
        />
      </ion-button>
      <!-- error -->
      <ms-informative-text v-show="error"> {{ $msTranslate(error) }} </ms-informative-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Env } from '@/services/environment';
import { IonButton } from '@ionic/vue';
import { computed, onMounted, ref, useTemplateRef } from 'vue';
import { parsecAddrValidator, emailValidator } from '@/common/validators';
import { MsInput, Validity, MsInformativeText, MsSpinner } from 'megashark-lib';
import { AccountCreationStepper, AccountCreateSendValidationEmailErrorTag } from '@/parsec';

const props = defineProps<{
  creationStepper: AccountCreationStepper;
}>();

const emits = defineEmits<{
  (e: 'creationStarted'): void;
}>();

const server = ref<string>(Env.getAccountServer());
const email = ref<string>('');
const lastName = ref<string>('');
const firstName = ref<string>('');
const emailInputRef = useTemplateRef<InstanceType<typeof MsInput>>('emailInput');
const serverInputRef = useTemplateRef<InstanceType<typeof MsInput>>('serverInput');
const querying = ref(false);
const error = ref<string>('');

const validInfo = computed(() => {
  return Boolean(
    email.value.length > 0 &&
      emailInputRef.value !== undefined &&
      emailInputRef.value?.validity === Validity.Valid &&
      !querying.value &&
      lastName.value.length > 0 &&
      firstName.value.length > 0 &&
      serverInputRef.value !== undefined &&
      serverInputRef.value?.validity === Validity.Valid,
  );
});

onMounted(async () => {
  if (serverInputRef.value) {
    await serverInputRef.value.validate(server.value);
  }
});

async function submit(): Promise<void> {
  querying.value = true;
  error.value = '';
  try {
    const result = await props.creationStepper.start(firstName.value, lastName.value, email.value, server.value);
    if (!result.ok) {
      switch (result.error.tag) {
        case AccountCreateSendValidationEmailErrorTag.Offline:
          error.value = 'loginPage.createAccount.errors.offline';
          break;
        case AccountCreateSendValidationEmailErrorTag.EmailRecipientRefused:
          error.value = 'loginPage.createAccount.errors.invalidEmail';
          break;
        default:
          error.value = 'loginPage.createAccount.errors.generic';
          window.electronAPI.log('error', `Create account start error: ${result.error.tag} (${result.error.error})`);
          break;
      }
    } else {
      emits('creationStarted');
    }
  } finally {
    querying.value = false;
  }
}
</script>

<style scoped lang="scss">
.account-login-content-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  width: 100%;
}
.account-login-content-button {
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
</style>
