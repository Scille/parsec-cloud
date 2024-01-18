<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    fill="clear"
    @click="openPopover($event)"
    id="select-popover-button"
    class="filter-button button-small"
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
import MsSorterPopover from '@/components/core/ms-sorter/MsSorterPopover.vue';
import { MsSorterChangeEvent, MsSorterLabels } from '@/components/core/ms-sorter/types';
import { MsOption, MsOptions } from '@/components/core/ms-types';
import { IonButton, IonIcon, popoverController } from '@ionic/vue';
import { swapVertical } from 'ionicons/icons';
import { Ref, ref } from 'vue';

const props = defineProps<{
  defaultOption: any;
  label?: string;
  options: MsOptions;
  sorterLabels?: MsSorterLabels;
  disabled?: boolean;
}>();

const emits = defineEmits<{
  (e: 'change', value: MsSorterChangeEvent): void;
}>();

const selectedOption: Ref<MsOption | undefined> = ref(props.defaultOption ? props.options.get(props.defaultOption) : undefined);
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
  height: fit-content;
  margin-right: 1rem;
  --background-hover: var(--parsec-color-light-secondary-medium);

  &::part(native) {
    padding: 0.5rem;
  }
}
.ms-sorter-icon {
  margin-left: 0.5em;
}
</style>
