<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-button
    fill="clear"
    @click="openPopover($event)"
    id="select-popover-button"
  >
    <ion-icon
      slot="end"
      :icon="chevronDownOutline"
    />
    {{ labelRef }}
  </ion-button>
</template>

<script setup lang="ts">
import { defineProps, defineEmits, Ref, ref } from 'vue';
import {
  IonButton,
  IonIcon,
  popoverController
} from '@ionic/vue';
import { chevronDownOutline } from 'ionicons/icons';
import MsSelectPopover from '../components/MsSelectPopover.vue';
import { MsSelectOption, MsSelectSortByLabels, getOptionByKey, MsSelectChangeEvent } from '../components/MsSelectOption';

const props = defineProps<{
  defaultOption: string,
  label?: string,
  options: MsSelectOption[],
  sortByLabels: MsSelectSortByLabels
}>();

const emits = defineEmits<{
  (e: 'change', value: MsSelectChangeEvent): void
}>();

const selectedOption: Ref<MsSelectOption | undefined> = ref(
  props.defaultOption ? getOptionByKey(props.options, props.defaultOption) : undefined
);
const sortByAsc: Ref<boolean> = ref(true);
const labelRef = ref(selectedOption.value?.label || props.label);

async function openPopover(ev: Event): Promise<void> {
  const popover = await popoverController.create({
    component: MsSelectPopover,
    componentProps: {
      options: props.options,
      defaultOption: selectedOption.value?.key,
      sortByLabels: props.sortByLabels,
      sortByAsc: sortByAsc.value
    },
    event: ev
  });
  await popover.present();
  onDidDismissPopover(popover);
}

async function onDidDismissPopover(popover: any): Promise<void> {
  const { data } = await popover.onDidDismiss();
  if (data) {
    labelRef.value = data.option.label;
    selectedOption.value = data.option;
    sortByAsc.value = data.sortByAsc;
    emits('change', {
      option: data.option,
      sortByAsc: sortByAsc.value
    });
  }
}

</script>

<style lang="scss" scoped>
.option {
  --background-hover: var(--parsec-color-light-primary-50);
  --background-hover-opacity: 1;
  --color-hover: var(--ion-color-tertiary);

  &.selected {
    color: var(--ion-color-tertiary) !important;
    font-weight: bold;
  }
}
</style>
