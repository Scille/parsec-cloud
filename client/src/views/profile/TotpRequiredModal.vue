<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
      <ion-text class="totp-required__title">
        {{ $msTranslate('Authentication.mfa.modalRequired.title') }}
      </ion-text>
      <div class="totp-required-main">
        <div class="totp-required-message">
          <ion-text class="totp-required-message__subtitle">
            {{ $msTranslate('Authentication.mfa.modalRequired.subtitle') }}
          </ion-text>
        </div>
      </div>
      <div class="totp-required-footer">
        <ion-button
          @click="dismiss(MsModalResult.Confirm)"
          class="totp-required-footer__button color-secondary"
          fill="outline"
        >
          {{ $msTranslate('Authentication.mfa.modalRequired.configureButton') }}
        </ion-button>
        <ion-button
          @click="dismiss(MsModalResult.Cancel)"
          class="totp-required-footer__button"
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
  background: ms.color('surface-brand-default-subtle');
  margin: -0.25rem;
}

.totp-required-content {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');

  .totp-required__title {
    @include ms.font('heading-h3');
    text-align: center;
    background-clip: text;
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-image: linear-gradient(ms.color('surface-gradient-from'), ms.color('surface-gradient-to'));
  }

  .totp-required-main {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-4xl');

    .totp-required-message {
      color: ms.color('text-base-description');
      display: flex;
      flex-direction: column;

      &__subtitle {
        @include ms.font('body-lg-regular');
        text-align: center;
      }
    }
  }

  .totp-required-footer {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: ms.spacing('gap-lg');
    margin-top: 0.5rem;

    &__button {
      @include ms.font('label-lg-medium');
      margin: auto;
      width: 100%;

      &:nth-child(1) {
        --background: linear-gradient(#{ms.color('surface-gradient-from')}, #{ms.color('surface-gradient-to')});
        --padding-end: #{ms.spacing('padding-5xl')};
        --padding-start: #{ms.spacing('padding-5xl')};
        --padding-bottom: #{ms.spacing('padding-2xl')};
        --padding-top: #{ms.spacing('padding-2xl')};
      }

      &:nth-child(2) {
        --background: transparent;
        --background-hover: #{ms.color('surface-neutral-default-subtle-hover')};
        --color: #{ms.color('text-base-body')};
        --padding-end: #{ms.spacing('padding-lg')};
        --padding-start: #{ms.spacing('padding-lg')};
      }
    }
  }
}
</style>
