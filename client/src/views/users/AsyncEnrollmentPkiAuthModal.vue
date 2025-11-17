<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="async-authentication-modal">
    <ms-modal :close-button="{ visible: true }">
      <div class="async-authentication-modal-header">
        <ms-image
          :image="CertificateIcon"
          alt="Certificate Icon"
          class="async-authentication-modal-header__icon"
        />
        <ion-text class="async-authentication-modal-header__title subtitles-lg">
          {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.pkiRequests.modal.title') }}
        </ion-text>
      </div>
      <ion-text class="async-authentication-modal-text body-lg">
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.pkiRequests.modal.description') }}
      </ion-text>
      <div>
        <choose-certificate @certificate-selected="onCertificateSelected" />
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import CertificateIcon from '@/assets/images/certificate-icon.svg?raw';
import ChooseCertificate from '@/components/devices/ChooseCertificate.vue';
import { X509CertificateReference } from '@/parsec';
import { IonPage, IonText, modalController } from '@ionic/vue';
import { MsImage, MsModal, MsModalResult } from 'megashark-lib';
import { ref } from 'vue';

const certificate = ref<X509CertificateReference | undefined>(undefined);

async function onCertificateSelected(certif: X509CertificateReference | undefined): Promise<void> {
  certificate.value = certif;
  await onOkClicked();
}

async function onOkClicked(): Promise<boolean> {
  if (!certificate.value) {
    return false;
  }
  return modalController.dismiss({ certificate: certificate.value }, MsModalResult.Confirm);
}
</script>

<style scoped lang="scss"></style>
