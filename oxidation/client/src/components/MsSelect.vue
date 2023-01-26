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
import { defineProps, ref } from 'vue';
import {
  IonButton,
  IonIcon,
  popoverController
} from '@ionic/vue';
import { chevronDownOutline } from 'ionicons/icons';
import MsSelectPopover from '@/components/MsSelectPopover.vue';
import { MsSelectOption } from '@/components/MsSelectOption.ts';

const props = defineProps<{
  label: string
  options: MsSelectOption[],
  defaultOption?: string
}>();

const labelRef = ref(props.label);
async function openPopover(ev: Event): Promise<void> {
  const popover = await popoverController.create({
    component: MsSelectPopover,
    componentProps: {
      options: props.options,
      defaultOption: props.defaultOption
    },
    event: ev
  });
  await popover.present();

  const { data } = await popover.onDidDismiss();
  labelRef.value = data.label;
}

</script>

<style lang="scss" scoped>
.option {
  --background-hover: #E5F1FF;
  --background-hover-opacity: 1;
  --color-hover: var(--ion-color-tertiary);

  &.selected {
    color: var(--ion-color-tertiary) !important;
    font-weight: bold;
  }
}
</style>
