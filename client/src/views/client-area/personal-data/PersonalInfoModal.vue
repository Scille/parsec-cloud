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
        <ms-input
          v-model="firstnameRef"
          :maxlength="128"
          :label="`${translationPrefix}.firstname`"
          @on-enter-keyup="submit"
          ref="firstnameInput"
        />
        <ms-input
          v-model="lastnameRef"
          :maxlength="128"
          :label="`${translationPrefix}.lastname`"
          @on-enter-keyup="submit"
        />
        <div class="input-container">
          <ms-phone-number-input
            class="form-item-input"
            :label="`${translationPrefix}.phone`"
            v-model="phoneRef"
            @on-enter-keyup="submit"
          />
          <div
            class="form-error form-helperText body"
            v-if="fieldHasError(Fields.Phone)"
          >
            {{ $msTranslate(`${translationPrefix}.errors.phone`) }}
          </div>
        </div>
        <ms-report-text
          class="report-error"
          :theme="MsReportTheme.Error"
          v-if="errors.length > 0 && !fieldHasError(Fields.Phone)"
        >
          {{ $msTranslate('globalErrors.unexpected') }}
        </ms-report-text>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { MsModal, MsInput, MsPhoneNumberInput, MsModalResult, MsReportTheme, MsReportText } from 'megashark-lib';
import { IonPage, modalController } from '@ionic/vue';
import { onMounted, Ref, ref } from 'vue';
import { BmsAccessInstance, BmsError } from '@/services/bms';

const props = defineProps<{
  firstname: string;
  lastname: string;
  phone?: string;
}>();

const translationPrefix = 'clientArea.personalDataPage.modals.personalInfo';
const firstnameRef = ref(props.firstname);
const lastnameRef = ref(props.lastname);
const phoneRef = ref(props.phone ?? '');
const firstnameInput = ref();
const errors: Ref<BmsError[]> = ref([]);
const querying = ref(false);

enum Fields {
  Phone = 'client.phone',
}

onMounted(async () => {
  await firstnameInput.value.setFocus();
});

function fieldHasError(field: Fields): boolean {
  return errors.value.find((error) => error.attr === field) !== undefined;
}

async function submit(): Promise<boolean> {
  if (!isFormValid()) {
    return false;
  }
  try {
    querying.value = true;
    const response = await BmsAccessInstance.get().updatePersonalInformation({
      firstname: firstnameRef.value,
      lastname: lastnameRef.value,
      phone: props.phone === undefined && phoneRef.value.length === 0 ? undefined : phoneRef.value,
    });

    if (response.isError) {
      errors.value = response.errors ?? [];
      return false;
    }
    return await modalController.dismiss(null, MsModalResult.Confirm);
  } finally {
    querying.value = false;
  }
}

function isFormValid(): boolean {
  return !!firstnameRef.value && !!lastnameRef.value;
}
</script>

<style lang="scss" scoped>
.modal-container {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}
</style>
