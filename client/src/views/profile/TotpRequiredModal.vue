<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-image
    :image="LoginMfaRequired"
    class="login-mfa-required"
  />
  <ms-modal
    :close-button="{ visible: false }"
    class="totp-required"
  >
    <div class="totp-required-content">
      <ion-text class="totp-required__title title-h2">
        {{ $msTranslate('Authentication.mfa.modalRequired.title') }}
      </ion-text>
      <div class="totp-required-main">
        <div class="totp-required-message">
          <ion-text class="totp-required-message__subtitle body-lg">
            {{ $msTranslate('Authentication.mfa.modalRequired.subtitle') }}
          </ion-text>
        </div>
      </div>
      <div class="totp-required-footer">
        <ion-button
          @click="dismiss(MsModalResult.Confirm)"
          class="totp-required-footer__button button-default button-large"
        >
          {{ $msTranslate('Authentication.mfa.modalRequired.configureButton') }}
        </ion-button>
        <ion-button
          @click="dismiss(MsModalResult.Cancel)"
          class="totp-required-footer__button button-default button-large"
        >
          {{ $msTranslate('Authentication.mfa.modalRequired.cancelButton') }}
        </ion-button>
      </div>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import LoginMfaRequired from '@/assets/images/login-mfa-required.svg?raw';
import { IonButton, IonText, modalController } from '@ionic/vue';
import { MsImage, MsModal, MsModalResult } from 'megashark-lib';

async function dismiss(role: MsModalResult): Promise<void> {
  await modalController.dismiss(undefined, role);
}
</script>

<style lang="scss" scoped>
.login-mfa-required {
  background: var(--parsec-color-light-primary-30);
  margin: -0.25rem;
}

.totp-required-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  .totp-required__title {
    text-align: center;
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-image: var(--parsec-color-light-gradient-background);
  }

  .totp-required-main {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;

    .totp-required-message {
      color: var(--parsec-color-light-secondary-hard-grey);
      display: flex;
      flex-direction: column;

      &__subtitle {
        text-align: center;
      }
    }
  }

  .totp-required-footer {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 0.5rem;
    margin-top: 0.5rem;

    &__button {
      margin: auto;
      width: 100%;

      &:nth-child(1) {
        --background: var(--parsec-color-light-gradient-background);
        --padding-end: 2rem;
        --padding-start: 2rem;
        --padding-bottom: 0.75rem;
        --padding-top: 0.75rem;
      }

      &:nth-child(2) {
        --background: transparent;
        --background-hover: var(--parsec-color-light-secondary-premiere);
        --color: var(--parsec-color-light-secondary-text);
        --padding-end: 0.5rem;
        --padding-start: 0.5rem;
      }
    }
  }
}
</style>
