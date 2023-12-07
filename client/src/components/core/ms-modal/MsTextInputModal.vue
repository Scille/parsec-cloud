<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="title"
    :subtitle="subtitle"
    :close-button="{ visible: true }"
    :cancel-button="{
      label: $t('TextInputModal.cancel'),
      disabled: false,
      onClick: cancel,
    }"
    :confirm-button="{
      label: okButtonText || $t('TextInputModal.ok'),
      disabled: !textIsValid,
      onClick: confirm,
    }"
  >
    <ms-input
      :label="inputLabel || ''"
      :placeholder="placeholder || ''"
      v-model="text"
      @on-enter-keyup="confirm()"
    />
  </ms-modal>
</template>

<script setup lang="ts">
import { modalController } from '@ionic/vue';
import { ref } from 'vue';
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import { MsInput } from '@/components/core/ms-input';
import { Validity } from '@/common/validators';
import { asyncComputed } from '@/common/asyncComputed';
import { GetTextOptions, MsModalResult } from '@/components/core/ms-modal/types';

const props = defineProps<GetTextOptions>();

const text = ref(props.defaultValue || '');
const textIsValid = asyncComputed(async () => {
  return text.value && (!props.validator || (props.validator && (await props.validator(text.value)) === Validity.Valid));
});

async function confirm(): Promise<boolean> {
  if (!textIsValid.value) {
    return false;
  }
  return await modalController.dismiss(props.trim ? text.value.trim() : text.value, MsModalResult.Confirm);
}

function cancel(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Cancel);
}
</script>

<style scoped lang="scss">
.text-input-modal {
}
</style>
