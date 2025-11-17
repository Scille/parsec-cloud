<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="async-authentication-modal">
    <ms-modal
      :close-button="{ visible: true }"
      :confirm-button="
        openBaoClient
          ? { disabled: !openBaoClient, label: 'InvitationsPage.asyncEnrollmentRequest.ssoRequests.modal.next', onClick: onOkClicked }
          : undefined
      "
    >
      <div class="async-authentication-modal-header">
        <ms-image
          :image="CertificateIcon"
          alt="Certificate Icon"
          class="async-authentication-modal-header__icon"
        />
        <ion-text class="async-authentication-modal-header__title subtitles-lg">
          {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.ssoRequests.modal.title') }}
        </ion-text>
      </div>
      <ion-text class="async-authentication-modal-text body-lg">
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.ssoRequests.modal.description') }}
      </ion-text>
      <connect-sso
        class="async-authentication-modal-sso"
        :server-config="serverConfig"
        @open-bao-connected="onOpenBaoConnected"
      />
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import CertificateIcon from '@/assets/images/certificate-icon.svg?raw';
import ConnectSso from '@/components/devices/ConnectSso.vue';
import { ServerConfig } from '@/parsec';
import { OpenBaoClient } from '@/services/openBao';
import { IonPage, IonText, modalController } from '@ionic/vue';
import { MsImage, MsModal, MsModalResult } from 'megashark-lib';
import { ref } from 'vue';

const openBaoClient = ref<OpenBaoClient | undefined>(undefined);

defineProps<{
  serverConfig: ServerConfig;
}>();

async function onOpenBaoConnected(client: OpenBaoClient): Promise<void> {
  openBaoClient.value = client;
}

async function onOkClicked(): Promise<boolean> {
  if (!openBaoClient.value) {
    return false;
  }
  return modalController.dismiss({ openBaoClient: openBaoClient.value }, MsModalResult.Confirm);
}
</script>

<style scoped lang="scss">
.join-async-modal-sso {
  border: 1px solid var(--parsec-color-light-secondary-medium);
  background: var(--parsec-color-light-secondary-background);
  display: flex;
  padding: 1.25rem 1rem;
}
</style>
