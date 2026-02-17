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
        <div class="async-authentication-modal-header-text">
          <ion-text class="async-authentication-modal-header-text__title title-h4">
            {{ $msTranslate(texts.title) }}
          </ion-text>
          <ion-text class="async-authentication-modal-header-text__description body">
            {{ $msTranslate(texts.message) }}
          </ion-text>
        </div>
      </div>
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
import { computed, ref } from 'vue';

const openBaoClient = ref<OpenBaoClient | undefined>(undefined);

const props = defineProps<{
  serverConfig: ServerConfig;
  action: 'finalize' | 'accept';
}>();

const texts = computed(() => {
  if (props.action === 'accept') {
    return {
      title: 'InvitationsPage.asyncEnrollmentRequest.ssoRequests.modal.title',
      message: 'InvitationsPage.asyncEnrollmentRequest.ssoRequests.modal.description',
    };
  }
  return {
    title: 'HomePage.organizationRequest.asyncEnrollmentModal.sso.finalizeTitle',
    message: 'HomePage.organizationRequest.asyncEnrollmentModal.sso.finalizeMessage',
  };
});

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
