<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      title="clientArea.personalDataPage.modals.professionalInfo.title"
      :close-button="{ visible: true }"
      :confirm-button="{
        label: 'clientArea.personalDataPage.modals.professionalInfo.nextButton',
        disabled: !isFormValid(),
        onClick: submit,
      }"
    >
      <div class="modal-container">
        <div class="input-container represent-company">
          <p class="form-label">
            {{ $msTranslate('clientArea.personalDataPage.modals.professionalInfo.representCompany') }}
          </p>
          <ms-boolean-toggle v-model="representCompanyRef" />
        </div>
        <ms-input
          v-show="representCompanyRef === Answer.Yes"
          v-model="companyRef"
          :maxlength="128"
          label="clientArea.personalDataPage.modals.professionalInfo.company"
          @on-enter-keyup="submit"
        />
        <ms-input
          v-show="representCompanyRef === Answer.Yes"
          v-model="jobRef"
          :maxlength="128"
          label="clientArea.personalDataPage.modals.professionalInfo.job"
          @on-enter-keyup="submit"
        />
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { Answer, MsBooleanToggle, MsModal, MsInput, MsModalResult } from 'megashark-lib';
import { IonPage, modalController } from '@ionic/vue';
import { Ref, ref } from 'vue';
import { BmsAccessInstance, BmsError } from '@/services/bms';

const props = defineProps<{
  company?: string;
  job?: string;
}>();

const companyRef = ref(props.company);
const jobRef = ref(props.job);
const representCompanyRef = ref(areFieldsFilled() ? Answer.Yes : Answer.No);
const errors: Ref<BmsError[]> = ref([]);

async function submit(): Promise<boolean> {
  if (!isFormValid()) {
    return false;
  }
  const data: any = {
    company: representCompanyRef.value === Answer.Yes ? companyRef.value : null,
    job: representCompanyRef.value === Answer.Yes ? jobRef.value : null,
  };
  const response = await BmsAccessInstance.get().updatePersonalInformation(data);

  if (response.isError) {
    errors.value = response.errors ?? [];
    return false;
  }
  return await modalController.dismiss({}, MsModalResult.Confirm);
}

function areFieldsFilled(): boolean {
  return !!companyRef.value && !!jobRef.value;
}

function isFormValid(): boolean {
  return representCompanyRef.value === Answer.No || (!!companyRef.value && !!jobRef.value);
}
</script>

<style lang="scss" scoped>
.modal-container {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;

  .represent-company {
    .form-label {
      margin: 0;
    }
  }
}
</style>
