<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    fill="clear"
    @click="openPopover($event)"
    id="dropdown-popover-button"
    class="filter-button button-medium"
    :disabled="disabled"
  >
    <ion-icon
      :class="{ 'popover-is-open': isPopoverOpen }"
      class="ms-dropdown-icon"
      slot="end"
      :icon="caretDown"
    />
    {{ labelRef }}
  </ion-button>
</template>

<script setup lang="ts">
import { defineProps, defineEmits, Ref, ref } from 'vue';
import { IonButton, IonIcon, popoverController } from '@ionic/vue';
import { caretDown } from 'ionicons/icons';
import MsDropdownPopover from '@/components/core/ms-dropdown/MsDropdownPopover.vue';
import { MsDropdownOption, MsDropdownChangeEvent, getMsOptionByKey } from '@/components/core/ms-types';

const props = defineProps<{
  defaultOption?: any;
  label?: string;
  description?: string;
  options: MsDropdownOption[];
  disabled?: boolean;
}>();

const emits = defineEmits<{
  (e: 'change', value: MsDropdownChangeEvent): void;
}>();

const selectedOption: Ref<MsDropdownOption | undefined> = ref(
  props.defaultOption ? getMsOptionByKey(props.options, props.defaultOption) : undefined,
);
const labelRef = ref(selectedOption.value?.label || props.label);
const isPopoverOpen = ref(false);

async function openPopover(ev: Event): Promise<void> {
  const popover = await popoverController.create({
    component: MsDropdownPopover,
    componentProps: {
      options: props.options,
      defaultOption: selectedOption.value?.key,
    },
    event: ev,
    showBackdrop: false,
  });
  isPopoverOpen.value = true;
  await popover.present();
  await onDidDismissPopover(popover);
  isPopoverOpen.value = false;
  await popover.dismiss();
}

async function onDidDismissPopover(popover: any): Promise<void> {
  const { data } = await popover.onDidDismiss();
  if (data) {
    labelRef.value = data.option.label;
    selectedOption.value = data.option;
    emits('change', {
      option: data.option,
    });
  }
}
</script>

<style lang="scss" scoped>
.filter-button {
  background: none;
  color: var(--parsec-color-light-primary-700);
}

.option {
  --background-hover: var(--parsec-color-light-primary-50);
  --color-hover: var(--ion-color-tertiary);

  &.selected {
    color: var(--parsec-color-light-primary-700);
    font-weight: bold;
  }
}

.ms-dropdown-icon {
  margin-left: 0.5em;
  transition: transform ease-out 300ms;
  &.popover-is-open {
    transform: rotate(180deg);
  }
}
</style>
