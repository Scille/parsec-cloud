<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="certificate-container">
    <certificate-card
      v-for="cert in certificates"
      :key="cert.id"
      :certificate="cert"
      :is-selected="selected !== undefined && cert.id === selected.id"
      @clicked="onCertificateClicked"
    />
  </div>
</template>

<script setup lang="ts">
import CertificateCard from '@/components/misc/CertificateCard.vue';
import { Certificate, listCertificates } from '@/parsec/async_enrollment';
import {} from '@ionic/vue';
import { onMounted, ref } from 'vue';

const certificates = ref<Array<Certificate>>([]);
const selected = ref<Certificate | undefined>(undefined);

onMounted(async () => {
  certificates.value = await listCertificates();
  console.log(certificates.value);
});

defineEmits<{
  (e: 'certificateSelected', certificate: Certificate): void;
}>();

defineExpose({
  getSelectedCertificate,
});

function getSelectedCertificate(): Certificate | undefined {
  return selected.value;
}

async function onCertificateClicked(cert: Certificate): Promise<void> {
  if (cert.isExpired()) {
    return;
  }
  selected.value = cert;
}
</script>

<style scoped lang="scss">
.certificate-container {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}
</style>
