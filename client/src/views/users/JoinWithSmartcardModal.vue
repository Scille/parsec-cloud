<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="join-smartcard-modal">
    <ms-modal
      title="JoinOrganization.joinPkiModal.title"
      :close-button="{ visible: true }"
      :confirm-button="{
        disabled: !areInfoValid,
        label: smartcardAvailable ? 'JoinOrganization.joinPkiModal.buttons.join' : 'JoinOrganization.joinPkiModal.buttons.close',
        onClick: onJoinClicked,
        queryingSpinner: sendingRequest,
      }"
      :cancel-button="
        smartcardAvailable
          ? {
              label: 'JoinOrganization.joinPkiModal.buttons.cancel',
              disabled: false,
              onClick: cancel,
            }
          : undefined
      "
    >
      <div
        v-if="smartcardAvailable"
        class="modal-content"
      >
        <user-information
          @field-update="onFieldUpdate"
          ref="userInformation"
        />
        <div
          class="certificate"
          :class="{ 'certificate-selected': certificate }"
        >
          <ion-text class="certificate-title title-h5">
            {{ $msTranslate('JoinOrganization.joinPkiModal.certificate.label') }}
          </ion-text>
          <div class="certificate-button-container">
            <div
              class="certificate-valid"
              v-if="certificate"
            >
              <ion-icon
                :icon="checkmarkCircle"
                class="certificate-valid__icon"
              />
              <ion-text class="certificate-valid__text button-large">
                {{ $msTranslate('JoinOrganization.joinPkiModal.certificate.selected') }}
              </ion-text>
              <ion-button
                @click="chooseCertificate"
                class="certificate-valid__button button-default button-medium"
                fill="clear"
              >
                {{ $msTranslate('JoinOrganization.joinPkiModal.certificate.updateButton') }}
              </ion-button>
            </div>
            <ion-button
              v-else
              @click="chooseCertificate"
              class="certificate-button button-default button-medium"
            >
              <ion-icon
                slot="start"
                :icon="documentOutline"
                class="certificate-button__icon"
              />
              {{ $msTranslate('JoinOrganization.joinPkiModal.certificate.addButton') }}
            </ion-button>
          </div>
          <ion-text class="certificate-description body">
            {{ $msTranslate('JoinOrganization.joinPkiModal.certificate.description') }}
          </ion-text>
        </div>
      </div>
      <div v-else>
        <ms-report-text :theme="MsReportTheme.Info">
          {{ $msTranslate('JoinOrganization.joinPkiModal.notAvailable') }}
        </ms-report-text>
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
import { IonButton, IonIcon, IonPage, modalController } from '@ionic/vue';
import { checkmarkCircle, documentOutline } from 'ionicons/icons';
import { MsModal, MsModalResult, MsReportText, MsReportTheme } from 'megashark-lib';
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

async function cancel(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Cancel);
}
</script>

<style scoped lang="scss">
.modal-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.certificate {
  display: flex;
  flex-direction: column;
  background: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  box-shadow: var(--parsec-shadow-input);
  padding: 0.75rem;
  border-radius: var(--parsec-radius-8);

  &-title {
    margin-bottom: 1rem;
    color: var(--parsec-color-light-secondary-soft-text);
  }

  &-button {
    width: 100%;

    &::part(native) {
      --background: var(--parsec-color-light-secondary-white);
      --background-hover: var(--parsec-color-light-secondary-medium);
      border: 1px dashed var(--parsec-color-light-secondary-light);
      color: var(--parsec-color-light-secondary-text);
    }

    &__icon {
      font-size: 1rem;
      margin-right: 0.5rem;
    }
  }

  &-description {
    margin-top: 0.5rem;
    color: var(--parsec-color-light-secondary-hard-grey);
  }

  &-selected {
    border-color: var(--parsec-color-light-success-500);
    background-color: var(--parsec-color-light-success-50);
    position: relative;

    &__icon {
      font-size: 1.125rem;
      position: absolute;
      top: 0.75rem;
      right: 0.75rem;
      color: var(--parsec-color-light-success-700);
    }

    .certificate-button::part(native) {
      --background: var(--parsec-color-light-primary-50);
      --background-hover: var(--parsec-color-light-primary-100);
      border: 1px solid var(--parsec-color-light-primary-600);
      color: var(--parsec-color-light-primary-600);
      align-items: left;
    }

    .certificate-button-container {
      border-radius: var(--parsec-radius-12);
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }

    .certificate-valid {
      display: flex;
      align-items: center;
      gap: 0.5rem;
      width: 100%;

      &__icon {
        font-size: 1.125rem;
        color: var(--parsec-color-light-success-700);
      }

      &__text {
        color: var(--parsec-color-light-success-700);
      }

      &__button {
        &::part(native) {
          --background: none;
          --background-hover: none;
          color: var(--parsec-color-light-primary-500);
        }

        &:hover {
          text-decoration: underline;
        }
      }
    }
  }
}
</style>
