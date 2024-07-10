<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="saas-login">
    <div class="saas-login-container">
      <create-organization-modal-header
        @close-clicked="$emit('closeRequested')"
        title="ClientApplication.title"
      />

      <div class="saas-login-content">
        <!-- email -->
        <ms-input
          class="saas-login-content__input"
          ref="emailInput"
          v-model="email"
          label="ClientApplication.emailLabel"
          :validator="emailValidator"
        />
        <!-- password -->
        <div class="input-password">
          <ms-password-input
            class="saas-login-content__input"
            v-model="password"
            label="ClientApplication.password"
          />
          <!-- TODO: UPDATE THE LINK -->
          <!-- TODO: CHECK THAT ELECTRON ALLOWS THE LINK TO BE OPENED -->
          <ion-text
            class="saas-login-inputs__link button-small"
            target="_blank"
            @click="$event.stopPropagation()"
            href="FORGOT_PASSWORD_LINK"
          >
            {{ $msTranslate('ClientApplication.forgottenPassword') }}
          </ion-text>
        </div>
      </div>

      <ion-footer class="saas-login-footer">
        <div class="login-button">
          <ion-button
            :disabled="!emailInput || emailInput.validity !== Validity.Valid || !password.length"
            @click="onLoginClicked"
          >
            {{ $msTranslate('ClientApplication.login') }}
          </ion-button>
          <ms-spinner v-show="querying" />

          <!-- error -->
          <div
            class="form-error login-button-error"
            v-show="loginError"
          >
            {{ $msTranslate(loginError) }}
          </div>
        </div>

        <!-- TODO: UPDATE THE LINK -->
        <!-- TODO: CHECK THAT ELECTRON ALLOWS THE LINK TO BE OPENED -->
        <div class="create-account">
          <ion-text class="create-account__text body">{{ $msTranslate('ClientApplication.noAccount') }}</ion-text>
          <ion-button
            class="create-account__link"
            target="_blank"
            fill="clear"
            @click="$event.stopPropagation()"
            href="parsec.cloud/register"
          >
            {{ $msTranslate('ClientApplication.createAccount') }}
          </ion-button>
        </div>
      </ion-footer>
    </div>
    <div class="saas-login-mockup">
      <img
        src="@/assets/images/mockup-parsec-client.svg"
        alt="mockup"
      />
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonButton, IonText, IonFooter } from '@ionic/vue';
import { MsInput, MsPasswordInput, Translatable, Validity, MsSpinner } from 'megashark-lib';
import { emailValidator } from '@/common/validators';
import { ref } from 'vue';
import { AuthenticationToken, BmsApi, DataType } from '@/services/bmsApi';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';

const props = defineProps<{
  email?: string;
}>();

const emits = defineEmits<{
  (e: 'loginSuccess', token: AuthenticationToken): void;
  (e: 'closeRequested'): void;
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

<style scoped lang="scss">
.saas-login {
  display: flex;
  flex-direction: row;
  height: auto;
  background: var(--parsec-color-light-primary-50);
  position: relative;
  padding: 2.5rem;

  &::before {
    content: url('@/assets/images/background/background-shapes.svg');
    position: absolute;
    left: 20rem;
    top: -20rem;
    background: var(--parsec-color-light-primary-50);
    z-index: -1;
  }

  &-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    width: 22rem;
  }

  // include inputs
  &-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    .input-password {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;

      .saas-login-inputs__link {
        cursor: pointer;
        color: var(--parsec-color-light-secondary-hard-grey);

        &:hover {
          text-decoration: underline;
        }
      }
    }
  }

  // include buttons
  &-footer {
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    margin-top: 2rem;
    gap: 2rem;

    .login-button {
      display: flex;
      gap: 1rem;
      align-items: center;
    }

    .create-account {
      display: flex;
      gap: 0.5rem;
      align-items: center;

      &__text {
        color: var(--parsec-color-light-secondary-hard-grey);
      }
    }
  }

  &-mockup {
    position: absolute;
    right: -0.5rem;
    bottom: -2rem;
    display: flex;
    align-items: flex-end;
  }
}
</style>
