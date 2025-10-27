<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <!-- error -->
    <ms-report-text
      v-show="error"
      :theme="MsReportTheme.Error"
    >
      {{ $msTranslate(error) }}
    </ms-report-text>
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
        @on-enter-keyup="submit"
        :validator="parsecAddrValidator"
        :disabled="!isEditingServer || querying"
      />
      <ms-input
        v-if="!isEditingServer"
        class="login-server-input account-login-content__input login-server-input-disabled"
        :value="server.replace('parsec3://', '')"
        label="loginPage.inputFields.server"
        :disabled="true"
      />
      <ms-input
        class="account-login-content__input"
        v-model="firstName"
        ref="firstNameInput"
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
        {{ $msTranslate('loginPage.createAccount.sendCode') }}
        <ms-spinner
          class="account-login-content-button__spinner"
          v-show="querying"
        />
      </ion-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { emailValidator, parsecAddrValidator } from '@/common/validators';
import { AccountCreateSendValidationEmailErrorTag, AccountCreationStepper } from '@/parsec';
import { Env } from '@/services/environment';
import { IonButton, IonText } from '@ionic/vue';
import { MsInput, MsReportText, MsReportTheme, MsSpinner, Validity } from 'megashark-lib';
import { computed, onMounted, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  creationStepper: AccountCreationStepper;
}>();

const emits = defineEmits<{
  (e: 'creationStarted'): void;
}>();

const server = ref<string>(Env.getAccountServer());
const isEditingServer = ref(false);
const email = ref<string>('');
const lastName = ref<string>('');
const firstName = ref<string>('');
const emailInputRef = useTemplateRef<InstanceType<typeof MsInput>>('emailInput');
const serverInputRef = useTemplateRef<InstanceType<typeof MsInput>>('serverInput');
const firstNameInputRef = useTemplateRef<InstanceType<typeof MsInput>>('firstNameInput');
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
  await serverInputRef.value?.setFocus();
  await serverInputRef.value?.validate(server.value);
  await firstNameInputRef.value?.setFocus();
});

async function toggleEditServer(): Promise<void> {
  isEditingServer.value = !isEditingServer.value;
  if (isEditingServer.value) {
    serverInputRef.value?.setFocus();
  }
  await serverInputRef.value?.validate(server.value);
}

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
  position: relative;
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
