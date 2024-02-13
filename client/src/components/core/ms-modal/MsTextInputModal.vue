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
      ref="msInputRef"
      :placeholder="placeholder || ''"
      v-model="text"
      @on-enter-keyup="confirm()"
      :validator="validator"
    />
  </ms-modal>
</template>

<script setup lang="ts">
import { asyncComputed } from '@/common/asyncComputed';
import { Validity } from '@/common/validators';
import { MsInput } from '@/components/core/ms-input';
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import { GetTextOptions, MsModalResult } from '@/components/core/ms-modal/types';
import { modalController } from '@ionic/vue';
import { onMounted, ref } from 'vue';

const props = defineProps<GetTextOptions>();
const msInputRef = ref();

const text = ref(props.defaultValue || '');
const textIsValid = asyncComputed(async () => {
  return text.value && (!props.validator || (props.validator && (await props.validator(text.value)).validity === Validity.Valid));
});

onMounted(async () => {
  msInputRef.value.setFocus();
  if (props.selectionRange) {
    await msInputRef.value.selectText(props.selectionRange);
  }
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

<style scoped lang="scss"></style>
