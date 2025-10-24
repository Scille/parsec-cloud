<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <div
      v-if="smartcardAvailable"
      class="choose-certificate"
    >
      <ion-text class="choose-certificate-instructions body">
        {{ $msTranslate('Authentication.method.smartcard.selectInstructions') }}
      </ion-text>
      <div class="choose-certificate-controls">
        <div
          class="choose-certificate-selected"
          v-if="certificate"
        >
          <ion-icon
            :icon="checkmarkCircle"
            class="choose-certificate-selected__icon"
          />
          <ion-text class="choose-certificate-selected__text subtitles-normal">
            {{ $msTranslate('Authentication.method.smartcard.certificateSelected') }}
          </ion-text>
        </div>
        <ion-button
          @click="selectCertificate"
          class="button-default button-normal choose-certificate-button"
          :class="{
            'choose-certificate-button--choose': !certificate,
            'choose-certificate-button--selected': certificate,
          }"
        >
          {{ $msTranslate(certificate ? 'Authentication.method.smartcard.selectOther' : 'Authentication.method.smartcard.selectButton') }}
        </ion-button>
      </div>
    </div>
    <ms-report-text
      v-else
      :theme="MsReportTheme.Error"
    >
      {{ $msTranslate('Authentication.method.smartcard.unavailable') }}
    </ms-report-text>
  </div>
</template>

<script setup lang="ts">
import { isSmartcardAvailable, selectCertificate as parsecSelectCertificate, X509CertificateReference } from '@/parsec';
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { MsReportText, MsReportTheme } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const certificate = ref<X509CertificateReference | undefined>(undefined);
const smartcardAvailable = ref(false);

const emits = defineEmits<{
  (e: 'certificateSelected', certificate: X509CertificateReference | undefined): void;
}>();

defineExpose({
  getCertificate,
});

onMounted(async () => {
  smartcardAvailable.value = await isSmartcardAvailable();
});

function getCertificate(): X509CertificateReference | undefined {
  return certificate.value;
}

async function selectCertificate(): Promise<void> {
  const result = await parsecSelectCertificate();
  if (!result.ok) {
    window.electronAPI.log('error', `Can't select certificate: ${result.error.tag} (${result.error.error})`);
    certificate.value = undefined;
  } else {
    certificate.value = result.value ?? undefined;
    emits('certificateSelected', certificate.value);
  }
}
</script>

<style scoped lang="scss">
.choose-certificate {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;

  &-instructions {
    color: var(--parsec-color-light-secondary-hard-grey);
  }

  &-controls {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  &-button {
    &--choose {
      &::part(native) {
        background-color: var(--parsec-color-light-secondary-text);
        color: var(--parsec-color-light-secondary-white);
        --background-hover: var(--parsec-color-light-secondary-contrast);
      }
    }

    &--selected {
      width: fit-content;

      &::part(native) {
        --background-hover: var(--parsec-color-light-secondary-medium);
        background-color: var(--parsec-color-light-secondary-premiere);
        color: var(--parsec-color-light-secondary-text);
      }
    }
  }

  &-selected {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    color: var(--parsec-color-light-secondary-text);

    &__icon {
      font-size: 1rem;
    }
  }
}
</style>
