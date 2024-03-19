<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="ms-grid-list-toggle"
    @update:modelValue="updateModelValue"
  >
    <ion-button
      fill="clear"
      id="button-view"
      class="view-button"
      @click="openPopover($event)"
    >
      <ion-icon
        class="view-icon"
        :icon="modelValue === DisplayState.Grid ? grid : list"
      />
      <ion-icon
        class="chevron-icon"
        :icon="chevronDown"
      />
    </ion-button>
  </div>
</template>

<script setup lang="ts">
import { IonButton, IonIcon, popoverController } from '@ionic/vue';
import { grid, list, chevronDown } from 'ionicons/icons';
import { DisplayState, MsGridListTogglePopover } from '@/components/core';
import { defineProps, defineExpose, defineEmits } from 'vue';

const props = defineProps<{
  modelValue: DisplayState;
}>();

async function openPopover(event: Event): Promise<void> {
  const popover = await popoverController.create({
    component: MsGridListTogglePopover,
    cssClass: 'ms-grid-list-toggle-popover',
    componentProps: {
      modelValue: props.modelValue,
    },
    event: event,
    alignment: 'end',
    showBackdrop: false,
  });
  await popover.present();
  await popover.onDidDismiss();
  await popover.dismiss();
}

const emit = defineEmits<{
  (e: 'update:modelValue', value: DisplayState): void;
}>();

function updateModelValue(value: any) : void{
  emit('update:modelValue', value);
}

defineExpose({
  DisplayState,
});
</script>

<style scoped lang="scss">
.ms-grid-list-toggle {
  display: flex;
  align-items: center;
  padding: 0;
  gap: 0.25rem;
}

.view-button {
  --background: var(--parsec-color-light-secondary-white);
  --background-hover: var(--parsec-color-light-secondary-medium);
  --color: var(--parsec-color-light-secondary-text);

  &::part(native) {
    padding: 0.375rem 0.625rem;
  }

  &:hover ion-icon {
    color: var(--parsec-color-light-primary-700);
  }
}

.view-icon {
  color: var(--parsec-color-light-primary-700);
  font-size: 1rem;
  margin-right: 0.5rem;
}

.chevron-icon {
  color: var(--parsec-color-light-secondary-grey);
  font-size: 1rem;
}
</style>
