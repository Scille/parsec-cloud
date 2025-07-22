<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div v-show="step === Steps.Email">
        <ms-input
          v-model="server"
          ref="serverInput"
          :validator="parsecAddrValidator"
        />
        <ms-input
          v-model="email"
          ref="emailInput"
          :validator="emailValidator"
        />
        <ion-button
          @click="sendCode"
          :disabled="emailInputRef?.validity !== Validity.Valid || serverInputRef?.validity !== Validity.Valid || querying"
        >
          SEND CODE
        </ion-button>
      </div>
      <div v-show="step === Steps.Code">
        {{ `EMAIL SENT TO ${email}` }}
        <ms-code-validation-input
          ref="codeInput"
          :allowed-input="AllowedInput.UpperAlphaNumeric"
          :code-length="6"
          @code-complete="onCodeComplete"
        />
        <ion-button
          @click="codeEntered"
          :disabled="code.length !== 6 || querying"
        >
          NEXT
        </ion-button>
      </div>
      <div v-show="step === Steps.NewAuthentication">
        <ms-choose-password-input
          ref="passwordInput"
          @on-enter-keyup="onPasswordChosen"
        />
        <ion-button
          @click="onPasswordChosen"
          :disabled="!validAuth || querying"
        >
          GO GO
        </ion-button>
      </div>
      <div v-show="step === Steps.Finished">
        SUCCESS
        <ion-button
          @click="login"
          :disabled="querying"
        >
          LOG IN
        </ion-button>
      </div>
      <ms-spinner v-show="querying" />
      <div v-if="error">
        {{ $msTranslate(error) }}
      </div>
      <ion-button @click="backToLogin"> BACK TO LOG IN </ion-button>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonContent, IonButton } from '@ionic/vue';
import { onMounted, ref, useTemplateRef } from 'vue';
import { emailValidator, parsecAddrValidator } from '@/common/validators';
import { MsInput, Validity, MsCodeValidationInput, AllowedInput, MsChoosePasswordInput, asyncComputed, MsSpinner } from 'megashark-lib';
import { AccountRecoverProceedErrorTag, AccountRecoverSendValidationEmailErrorTag, ParsecAccount, ParsecAccountAccess } from '@/parsec';
import { Env } from '@/services/environment';
import { getCurrentRouteParams, getCurrentRouteQuery, navigateTo, Routes } from '@/router';

enum Steps {
  Email = 'email',
  Code = 'code',
  NewAuthentication = 'new-authentication',
  Finished = 'finished',
}

const step = ref<Steps>(Steps.Email);
const email = ref('');
const code = ref<Array<string>>([]);
const password = ref('');
const server = ref(Env.getAccountServer());
const error = ref('');
const querying = ref(false);

const passwordInputRef = useTemplateRef<typeof MsChoosePasswordInput>('passwordInput');
const serverInputRef = useTemplateRef('serverInput');
const emailInputRef = useTemplateRef('emailInput');
const codeInputRef = useTemplateRef('codeInput');

const validAuth = asyncComputed(async () => {
  return step.value === Steps.NewAuthentication && passwordInputRef.value && (await passwordInputRef.value.areFieldsCorrect());
});

onMounted(async () => {
  await serverInputRef.value?.validate(server.value);
});

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
      error.value = 'OFFLINE';
    } else {
      error.value = 'GENERIC';
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
    ParsecAccountAccess.usePassword(email.value, password.value),
    code.value,
    server.value,
  );
  if (!result.ok) {
    if (result.error.tag === AccountRecoverProceedErrorTag.InvalidValidationCode) {
      step.value = Steps.Code;
      error.value = 'INVALID CODE';
    } else if (result.error.tag === AccountRecoverProceedErrorTag.Offline) {
      error.value = 'OFFLINE';
    } else {
      error.value = 'GENERIC ERROR';
    }
  } else {
    step.value = Steps.Finished;
  }
  querying.value = false;
}

async function login(): Promise<void> {
  const result = await ParsecAccount.login(ParsecAccountAccess.usePassword(email.value, password.value), server.value);
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

<style scoped lang="scss"></style>
