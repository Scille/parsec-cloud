<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="async-authentication-modal">
    <ms-modal
      :close-button="{ visible: true }"
      title="InvitationsPage.asyncEnrollmentRequest.pkiRequests.modal.title"
    >
      <div class="modal-info">
        <div class="async-authentication-modal-header">
          <ms-image
            :image="CertificateIcon"
            alt="Certificate Icon"
            class="async-authentication-modal-header__icon"
          />
          <div class="async-authentication-modal-header-text">
            <ion-text class="async-authentication-modal-header-text__title title-h4">
              {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.pkiRequests.modal.info') }}
            </ion-text>
            <ion-text class="async-authentication-modal-header-text__description body">
              {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.pkiRequests.modal.description') }}
            </ion-text>
          </div>
        </div>
        <certificate-selection
          :purpose="CertificatePurpose.Sign"
          @certificate-selected="onCertificateSelected"
        />
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import CertificateIcon from '@/assets/images/certificate-icon.svg?raw';
import CertificateSelection from '@/components/misc/CertificateSelection.vue';
import { CertificatePurpose, CertificateWithDetailsValid, X509CertificateReference } from '@/parsec';
import { IonPage, IonText, modalController } from '@ionic/vue';
import { MsImage, MsModal, MsModalResult } from 'megashark-lib';
import { ref } from 'vue';

const certificate = ref<X509CertificateReference | undefined>(undefined);

async function onCertificateSelected(cert: CertificateWithDetailsValid): Promise<void> {
  certificate.value = cert.reference;
  await onOkClicked();
}

async function onOkClicked(): Promise<boolean> {
  if (!certificate.value) {
    return false;
  }
  return modalController.dismiss({ certificate: certificate.value }, MsModalResult.Confirm);
}
</script>

<style scoped lang="scss">
.modal-info {
  display: flex;
  flex-direction: column;
  align-items: center;
  overflow: hidden;
  gap: 1rem;
}
</style>
