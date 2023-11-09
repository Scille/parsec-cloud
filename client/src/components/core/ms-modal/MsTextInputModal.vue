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

<script lang="ts">
import MsTextInputModal from '@/components/core/ms-modal/MsTextInputModal.vue';

export interface GetTextOptions {
  title: string;
  subtitle?: string;
  trim?: boolean;
  validator?: IValidator;
  inputLabel?: string;
  placeholder?: string;
  okButtonText?: string;
  defaultValue?: string;
}

export async function getTextInputFromUser(options: GetTextOptions): Promise<string | null> {
  const modal = await modalController.create({
    component: MsTextInputModal,
    canDismiss: true,
    cssClass: 'text-input-modal',
    componentProps: {
      title: options.title,
      subtitle: options.subtitle,
      trim: options.trim,
      validator: options.validator,
      inputLabel: options.inputLabel,
      placeholder: options.placeholder,
      okButtonText: options.okButtonText,
      defaultValue: options.defaultValue,
    },
  });
  await modal.present();
  const result = await modal.onWillDismiss();
  await modal.dismiss();
  return result.role === MsModalResult.Confirm ? result.data : null;
}
</script>

<script setup lang="ts">
import { modalController } from '@ionic/vue';
import { ref } from 'vue';
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import MsInput from '@/components/core/ms-input/MsInput.vue';
import { MsModalResult } from '@/components/core/ms-types';
import { IValidator, Validity } from '@/common/validators';
import { asyncComputed } from '@/common/asyncComputed';

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
