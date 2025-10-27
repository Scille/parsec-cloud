<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="`${translationPrefix}.title`"
      :close-button="{ visible: true }"
      :confirm-button="{
        label: `${translationPrefix}.nextButton`,
        disabled: !isFormValid() || querying,
        onClick: submit,
      }"
    >
      <div class="modal-container">
        <div class="input-container represent-company">
          <p class="form-label">
            {{ $msTranslate(`${translationPrefix}.representCompany`) }}
          </p>
          <ms-boolean-toggle
            v-model="representCompanyRef"
            ref="representCompanyInput"
          />
        </div>
        <ms-input
          v-show="representCompanyRef === Answer.Yes"
          v-model="companyRef"
          :maxlength="128"
          :label="`${translationPrefix}.company`"
          @on-enter-keyup="submit"
        />
        <ms-input
          v-show="representCompanyRef === Answer.Yes"
          v-model="jobRef"
          :maxlength="128"
          :label="`${translationPrefix}.job`"
          @on-enter-keyup="submit"
        />
        <ms-report-text
          :theme="MsReportTheme.Error"
          v-if="errors.length > 0"
        >
          {{ $msTranslate('globalErrors.unexpected') }}
        </ms-report-text>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { BmsAccessInstance, BmsError } from '@/services/bms';
import { IonPage, modalController } from '@ionic/vue';
import { Answer, MsBooleanToggle, MsInput, MsModal, MsModalResult, MsReportText, MsReportTheme } from 'megashark-lib';
import { onMounted, Ref, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  company?: string;
  job?: string;
}>();

const translationPrefix = 'clientArea.personalDataPage.modals.professionalInfo';
const companyRef = ref(props.company);
const jobRef = ref(props.job);
const representCompanyRef = ref(areFieldsFilled() ? Answer.Yes : Answer.No);
const errors: Ref<BmsError[]> = ref([]);
const representCompanyInputRef = useTemplateRef<InstanceType<typeof MsBooleanToggle>>('representCompanyInput');
const querying = ref(false);

onMounted(async () => {
  await representCompanyInputRef.value?.setFocus();
});

async function submit(): Promise<boolean> {
  if (!isFormValid()) {
    return false;
  }
  try {
    querying.value = true;
    const data: any = {
      company: representCompanyRef.value === Answer.Yes ? companyRef.value : null,
      job: representCompanyRef.value === Answer.Yes ? jobRef.value : null,
    };
    const response = await BmsAccessInstance.get().updatePersonalInformation(data);

    if (response.isError) {
      errors.value = response.errors ?? [];
      return false;
    }
    return await modalController.dismiss(null, MsModalResult.Confirm);
  } finally {
    querying.value = false;
  }
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
