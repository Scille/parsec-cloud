<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="login-popup">
    <!-- login -->
    <div class="login-header">
      <ion-title class="login-header__title title-h1">
        {{ $t('HomePage.organizationLogin.login') }}
      </ion-title>
    </div>
    <ion-card class="login-card">
      <ion-card-header class="login-card-header">
        <organization-card :device="device" />
      </ion-card-header>
      <ion-card-content class="login-card-content">
        <ms-input
          :label="$t('HomePage.organizationLogin.emailLabel')"
          :placeholder="device.humanHandle.email"
          id="ms-input"
          :disabled="true"
        />
        <div class="login-card-content__password">
          <ms-password-input
            :label="$t('HomePage.organizationLogin.passwordLabel')"
            ref="passwordInputRef"
            v-model="password"
            @on-enter-keyup="onLoginClick()"
            id="password-input"
            @change="onPasswordChange"
            :error-message="errorMessage"
            :password-is-invalid="passwordIsInvalid"
          />
          <ion-button
            fill="clear"
            @click="$emit('forgottenPasswordClick', device)"
            id="forgotten-password-button"
          >
            {{ $t('HomePage.organizationLogin.forgottenPassword') }}
          </ion-button>
        </div>
      </ion-card-content>
      <ion-footer class="login-card-footer">
        <ion-button
          @click="onLoginClick()"
          size="large"
          :disabled="password.length == 0"
          class="login-button"
        >
          <ion-icon
            slot="start"
            :icon="logIn"
          />
          {{ $t('HomePage.organizationLogin.login') }}
        </ion-button>
      </ion-footer>
    </ion-card>
    <!-- end of login -->
  </div>
</template>

<script setup lang="ts">
import { MsInput, MsPasswordInput } from '@/components/core';
import OrganizationCard from '@/components/organizations/OrganizationCard.vue';
import { AvailableDevice, ClientStartError } from '@/parsec';
import { translate } from '@/services/translation';
import { IonButton, IonCard, IonCardContent, IonCardHeader, IonFooter, IonIcon, IonTitle } from '@ionic/vue';
import { logIn } from 'ionicons/icons';
import { onMounted, ref } from 'vue';

const props = defineProps<{
  device: AvailableDevice;
}>();

const emits = defineEmits<{
  (e: 'loginClick', device: AvailableDevice, password: string): void;
  (e: 'forgottenPasswordClick', device: AvailableDevice): void;
}>();

const passwordInputRef = ref();
const password = ref('');
const errorMessage = ref('');
const passwordIsInvalid = ref(false);

onMounted(async () => {
  await passwordInputRef.value.setFocus();
});

async function onLoginClick(): Promise<void> {
  emits('loginClick', props.device, password.value);
}

async function setLoginError(_error?: ClientStartError): Promise<void> {
  errorMessage.value = translate('HomePage.organizationLogin.passwordError');
  passwordIsInvalid.value = true;
}

async function onPasswordChange(value: string): Promise<void> {
  if (value.length === 0) {
    passwordIsInvalid.value = false;
  }
  errorMessage.value = '';
}

defineExpose({
  setLoginError,
});
</script>

<style lang="scss" scoped>
.login-popup {
  height: auto;
  width: 100%;
  max-width: 25rem;
  margin: auto;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  box-shadow: none;

  .login-header {
    display: flex;
    flex-direction: column;
    align-items: center;

    &__title {
      padding: 0;
      margin-bottom: 2rem;
      color: var(--parsec-color-light-secondary-white);
    }
  }

  .login-card {
    background: var(--parsec-color-light-secondary-white);
    border: 1px solid var(--parsec-color-light-secondary-medium);
    padding: 2em;
    margin: 0;
    border-radius: var(--parsec-radius-8);
    box-shadow: none;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    width: 100%;
    transition: box-shadow 150ms ease-in-out;

    &:has(.has-focus) {
      box-shadow: var(--parsec-shadow-card);
    }

    &-header {
      padding: 0;
    }

    &-content {
      display: flex;
      flex-direction: column;
      gap: 1rem;
      padding: 0;

      &__password {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        margin: 0;
      }

      #password-input {
        margin: 0;
      }

      #forgotten-password-button {
        margin: 0;
        position: relative;
        width: fit-content;
        color: var(--parsec-color-light-secondary-grey);

        &::part(native) {
          --background-hover: none;
          padding: 0 0 0 2px;
        }

        &::after {
          content: '';
          position: absolute;
          bottom: 0.25rem;
          width: 0%;
          height: 1px;
          left: 0;
          transition: all 200ms ease-in-out;
        }

        &:hover {
          color: var(--parsec-color-light-primary-500);

          &::after {
            background: var(--parsec-color-light-primary-500);
            width: 100%;
          }
        }
      }
    }

    &-footer {
      padding: 0;
      width: 100%;
      display: flex;

      .login-button {
        width: 100%;
      }
    }
  }
}
</style>
