<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="'JOIN ORG WITH SMARTCARD'"
      :close-button="{ visible: true }"
      :confirm-button="{
        disabled: !areInfoValid,
        label: 'SEND A REQUEST TO JOIN',
        onClick: onJoinClicked,
        queryingSpinner: sendingRequest,
      }"
    >
      <div v-if="smartcardAvailable">
        <user-information
          @field-update="onFieldUpdate"
          ref="userInformation"
        />
        <ion-button @click="chooseCertificate">
          {{ certificate ? 'UPDATE' : 'CHOOSE CERTIFICATE' }}
        </ion-button>
        <div v-show="certificate">
          {{ 'CERTIFICATE SELECTED' }}
        </div>
      </div>
      <div v-else>
        {{ 'SMARTCARD NOT AVAILABLE ON THIS SYSTEM' }}
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { UserInformation } from '@/components/users';
import {
  isSmartcardAvailable,
  ParsecPkiEnrollmentAddr,
  ParsedParsecAddrPkiEnrollment,
  ParsedParsecAddrTag,
  parseParsecAddr,
  selectCertificate,
  X509CertificateReference,
} from '@/parsec';
import { IonButton, IonPage, modalController } from '@ionic/vue';
import { MsModal, MsModalResult } from 'megashark-lib';
import { onMounted, ref, toRaw, useTemplateRef } from 'vue';

const userInformationRef = useTemplateRef<typeof UserInformation>('userInformation');
const sendingRequest = ref(false);
const areInfoValid = ref(false);
const certificate = ref<X509CertificateReference | undefined>(undefined);
const joinAddr = ref<ParsedParsecAddrPkiEnrollment | undefined>(undefined);
const smartcardAvailable = ref(false);

const props = defineProps<{
  addr: ParsecPkiEnrollmentAddr;
}>();

onMounted(async () => {
  const result = await parseParsecAddr(props.addr);
  if (result.ok && result.value.tag === ParsedParsecAddrTag.PkiEnrollment) {
    joinAddr.value = result.value;
  }
  smartcardAvailable.value = await isSmartcardAvailable();
});

async function onFieldUpdate(): Promise<void> {
  areInfoValid.value = userInformationRef.value && (await userInformationRef.value.areFieldsCorrect()) && certificate.value;
}

async function chooseCertificate(): Promise<void> {
  const result = await selectCertificate();
  if (result.ok) {
    certificate.value = result.value;
  }
  await onFieldUpdate();
}

async function onJoinClicked(): Promise<boolean> {
  if (!areInfoValid.value || !userInformationRef.value) {
    return false;
  }
  return await modalController.dismiss(
    {
      certificate: toRaw(certificate.value),
      humanHandle: { label: userInformationRef.value.getFullName(), email: userInformationRef.value.getEmail() },
    },
    MsModalResult.Confirm,
  );
}
</script>

<style scoped lang="scss"></style>
