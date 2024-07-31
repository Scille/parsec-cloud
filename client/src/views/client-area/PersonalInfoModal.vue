<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      title="clientArea.personalDataPage.modals.personalInfo.title"
      :close-button="{ visible: true }"
      :confirm-button="{
        label: 'clientArea.personalDataPage.modals.personalInfo.nextButton',
        disabled: !isFormValid(),
        onClick: submit,
      }"
    >
      <div class="modal-container">
        <ms-input
          v-model="firstnameRef"
          :maxlength="128"
          label="clientArea.personalDataPage.modals.personalInfo.firstname"
          @on-enter-keyup="submit"
        />
        <ms-input
          v-model="lastnameRef"
          :maxlength="128"
          label="clientArea.personalDataPage.modals.personalInfo.lastname"
          @on-enter-keyup="submit"
        />
        <div class="input-container">
          <ms-phone-number-input
            class="form-item-input"
            label="clientArea.personalDataPage.modals.personalInfo.phone"
            v-model="phoneRef"
            @on-enter-keyup="submit"
          />
          <div
            class="form-error body"
            v-if="fieldHasError(Fields.Phone)"
          >
            {{ $msTranslate('clientArea.personalDataPage.modals.personalInfo.errors.phone') }}
          </div>
        </div>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { MsModal, MsInput, MsPhoneNumberInput, MsModalResult } from 'megashark-lib';
import { IonPage, modalController } from '@ionic/vue';
import { Ref, ref } from 'vue';
import { BmsAccessInstance, BmsError } from '@/services/bms';

const props = defineProps<{
  firstname: string;
  lastname: string;
  phone: string;
}>();

const firstnameRef = ref(props.firstname);
const lastnameRef = ref(props.lastname);
const phoneRef = ref(props.phone);
const errors: Ref<BmsError[]> = ref([]);

enum Fields {
  Phone = 'client.phone',
}

function fieldHasError(field: Fields): boolean {
  return errors.value.find((error) => error.attr === field) !== undefined;
}

async function submit(): Promise<boolean> {
  if (!isFormValid()) {
    return false;
  }
  const response = await BmsAccessInstance.get().updatePersonalInformation({
    firstname: firstnameRef.value,
    lastname: lastnameRef.value,
    phone: phoneRef.value,
  });

  if (response.isError) {
    errors.value = response.errors ?? [];
    return false;
  }
  return await modalController.dismiss({}, MsModalResult.Confirm);
}

function isFormValid(): boolean {
  return !!firstnameRef.value && !!lastnameRef.value && !!phoneRef.value;
}
</script>

<style lang="scss" scoped>
.modal-container {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}
</style>
