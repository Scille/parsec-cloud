<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="certificate-container">
    <ion-toggle
      v-model="showExpired"
      v-if="certificates.length > 0"
      label-placement="end"
      class="subtitles-sm expired-toggle"
    >
      {{ $msTranslate('HomePage.organizationRequest.asyncEnrollmentModal.certificate.showExpired') }}
    </ion-toggle>
    <div
      v-if="certificates.length > 0"
      class="certificate-list"
    >
      <certificate-card
        v-for="cert in filteredCertificates"
        :key="cert.getSerial()"
        :certificate="cert"
        :is-selected="selected !== undefined && cert.getSerial() === selected.getSerial()"
        @clicked="onCertificateClicked"
      />
    </div>
    <div
      class="no-certificate"
      v-if="certificates.length === 0"
    >
      <ion-text class="subtitles-normal">
        {{ $msTranslate('HomePage.organizationRequest.asyncEnrollmentModal.certificate.noCertificates') }}
      </ion-text>
    </div>
    <ms-report-text
      :theme="MsReportTheme.Error"
      v-if="error"
    >
      {{ $msTranslate(error) }}
    </ms-report-text>
  </div>
</template>

<script setup lang="ts">
import CertificateCard from '@/components/misc/CertificateCard.vue';
import { CertificatePurpose, CertificateWithDetailsValid } from '@/parsec';
import { listCertificates } from '@/parsec/async_enrollment';
import { IonText, IonToggle } from '@ionic/vue';
import { MsReportText, MsReportTheme } from 'megashark-lib';
import { computed, onMounted, ref } from 'vue';

const props = defineProps<{
  purpose: CertificatePurpose;
}>();

const certificates = ref<Array<CertificateWithDetailsValid>>([]);
const selected = ref<CertificateWithDetailsValid | undefined>(undefined);
const error = ref('');
const showExpired = ref(false);

const filteredCertificates = computed(() => {
  return certificates.value.filter((cert) => {
    return !cert.isExpired() || showExpired.value === true;
  });
});

onMounted(async () => {
  const result = await listCertificates(props.purpose);

  if (!result.ok) {
    error.value = 'Authentication.method.smartcard.listCertificatesFailed';
  } else {
    certificates.value = result.value.sort((certA, certB) => Number(certA.isExpired()) - Number(certB.isExpired()));
  }
});

const emits = defineEmits<{
  (e: 'certificateSelected', certificate: CertificateWithDetailsValid): void;
}>();

defineExpose({
  getSelectedCertificate,
});

function getSelectedCertificate(): CertificateWithDetailsValid | undefined {
  return selected.value;
}

async function onCertificateClicked(cert: CertificateWithDetailsValid): Promise<void> {
  if (cert.isExpired()) {
    return;
  }
  selected.value = cert;
  emits('certificateSelected', selected.value);
}
</script>

<style scoped lang="scss">
.certificate-container {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  overflow-y: auto;
  max-height: 25.5rem;
  position: relative;
  width: 100%;

  .expired-toggle {
    margin-left: auto;
    position: sticky;
    top: 0;
    right: 0.5rem;
    padding-bottom: 0.5rem;
    background: var(--parsec-color-light-secondary-white);
    display: flex;
    width: 100%;
    color: var(--parsec-color-light-secondary-hard-grey);
    --handle-width: 18px;
    --handle-height: 18px;
    --handle-spacing: 3px;

    &::part(track) {
      height: 24px;
      width: 42px;
    }

    &::part(label) {
      margin-left: 0.75rem;
    }
  }

  .certificate-list {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    width: 100%;
  }

  .no-certificate {
    display: flex;
    justify-content: center;
    width: 100%;
  }
}
</style>
