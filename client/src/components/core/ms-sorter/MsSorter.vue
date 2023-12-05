<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    fill="clear"
    @click="openPopover($event)"
    id="select-popover-button"
    class="filter-button button-medium"
    :disabled="disabled"
  >
    <ion-icon
      class="ms-sorter-icon"
      slot="end"
      :icon="swapVertical"
    />
    {{ labelRef }}
  </ion-button>
</template>

<script setup lang="ts">
import { defineProps, defineEmits, Ref, ref } from 'vue';
import { IonButton, IonIcon, popoverController } from '@ionic/vue';
import { swapVertical } from 'ionicons/icons';
import MsSorterPopover from '@/components/core/ms-sorter/MsSorterPopover.vue';
import { MsSorterOption, MsSorterLabels, MsSorterChangeEvent, getMsOptionByKey } from '@/components/core/ms-types';

const props = defineProps<{
  defaultOption: any;
  label?: string;
  options: MsSorterOption[];
  sorterLabels?: MsSorterLabels;
  disabled?: boolean;
}>();

const emits = defineEmits<{
  (e: 'change', value: MsSorterChangeEvent): void;
}>();

const selectedOption: Ref<MsSorterOption | undefined> = ref(
  props.defaultOption ? getMsOptionByKey(props.options, props.defaultOption) : undefined,
);
const sortByAsc: Ref<boolean> = ref(true);
const labelRef = ref(selectedOption.value?.label || props.label);

async function openPopover(event: Event): Promise<void> {
  const popover = await popoverController.create({
    component: MsSorterPopover,
    cssClass: 'sorter-popover',
    componentProps: {
      options: props.options,
      defaultOption: selectedOption.value?.key,
      sorterLabels: props.sorterLabels,
      sortByAsc: sortByAsc.value,
    },
    event: event,
    alignment: 'end',
    showBackdrop: false,
  });
  await popover.present();
  await onDidDismissPopover(popover);
  await popover.dismiss();
}

async function onDidDismissPopover(popover: any): Promise<void> {
  const { data } = await popover.onDidDismiss();
  if (data) {
    labelRef.value = data.option.label;
    selectedOption.value = data.option;
    sortByAsc.value = data.sortByAsc;
    emits('change', {
      option: data.option,
      sortByAsc: sortByAsc.value,
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
.ms-sorter-icon {
  margin-left: 0.5em;
}
</style>
