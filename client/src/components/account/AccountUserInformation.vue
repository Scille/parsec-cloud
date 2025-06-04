<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <ms-input
      class="account-login-content__input"
      ref="serverInputRef"
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
      ref="emailInputRef"
      v-model="email"
      label="loginPage.inputFields.email"
      @on-enter-keyup="submit()"
      :validator="emailValidator"
      :disabled="querying"
    />
    <div class="account-login-content-button">
      <ion-button
        class="account-login-content-button__item"
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
      <ms-informative-text v-show="error"> {{ $msTranslate(error) }}; </ms-informative-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Env } from '@/services/environment';
import { IonButton } from '@ionic/vue';
import { computed, onMounted, ref } from 'vue';
import { parsecAddrValidator, emailValidator } from '@/common/validators';
import { MsInput, Validity, MsInformativeText, MsSpinner } from 'megashark-lib';
import { AccountCreationStepper } from '@/parsec';

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
const emailInputRef = ref<typeof MsInput>();
const serverInputRef = ref<typeof MsInput>();
const querying = ref(false);
const error = ref<string>('');

const validInfo = computed(() => {
  return Boolean(
    email.value.length > 0 &&
      emailInputRef.value !== undefined &&
      emailInputRef.value.validity === Validity.Valid &&
      !querying.value &&
      lastName.value.length > 0 &&
      firstName.value.length > 0 &&
      serverInputRef.value !== undefined &&
      serverInputRef.value.validity === Validity.Valid,
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
      // Check error (offline, invalid email, invalid server, ...)
      error.value = 'FAILED';
    } else {
      emits('creationStarted');
    }
  } finally {
    querying.value = false;
  }
}
</script>

<style scoped lang="scss">
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
