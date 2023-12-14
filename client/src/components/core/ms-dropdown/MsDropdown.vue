<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="dropdown-container">
    <ion-text
      class="form-label"
      v-if="title && appearanceRef === MsAppearance.Outline"
    >
      {{ title }}
    </ion-text>
    <ion-button
      :fill="appearanceRef"
      @click="openPopover($event)"
      id="dropdown-popover-button"
      class="filter-button button-medium"
      :class="isPopoverOpen ? 'active' : ''"
      :disabled="disabled"
    >
      <ion-icon
        :class="{ 'popover-is-open': isPopoverOpen }"
        class="ms-dropdown-icon"
        slot="end"
        :icon="getIcon()"
      />
      <span class="input-text">{{ labelRef }}</span>
    </ion-button>
  </div>
</template>

<script setup lang="ts">
import MsDropdownPopover from '@/components/core/ms-dropdown/MsDropdownPopover.vue';
import { MsDropdownChangeEvent } from '@/components/core/ms-dropdown/types';
import { MsAppearance, MsOption, MsOptions } from '@/components/core/ms-types';
import { IonButton, IonIcon, IonText, popoverController } from '@ionic/vue';
import { caretDown, chevronDown } from 'ionicons/icons';
import { Ref, defineEmits, defineProps, ref } from 'vue';

const props = defineProps<{
  defaultOption?: any;
  label?: string;
  title?: string;
  description?: string;
  options: MsOptions;
  disabled?: boolean;
  appearance?: MsAppearance;
}>();

const emits = defineEmits<{
  (e: 'change', value: MsDropdownChangeEvent): void;
}>();

const selectedOption: Ref<MsOption | undefined> = ref(props.defaultOption ? props.options.get(props.defaultOption) : undefined);
const labelRef = ref(selectedOption.value?.label || props.label);
const isPopoverOpen = ref(false);
const appearanceRef = ref(props.appearance ?? MsAppearance.Outline);

async function openPopover(event: Event): Promise<void> {
  const popover = await popoverController.create({
    component: MsDropdownPopover,
    cssClass: 'dropdown-popover',
    componentProps: {
      options: props.options,
      defaultOption: selectedOption.value?.key,
    },
    event: event,
    alignment: 'end',
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

function getIcon(): string {
  switch (appearanceRef.value) {
    case MsAppearance.Outline:
      return chevronDown;
    case MsAppearance.Clear:
      return caretDown;
  }
}
</script>

<style lang="scss" scoped>
.filter-button {
  background: none;
  color: var(--parsec-color-light-primary-800);
  --border-color-hover: none;
  margin: 0;
  // height: 100%;

  .input-text {
    width: 100%;
    text-align: left;
    pointer-events: none;
  }
}

.option {
  --background-hover: var(--parsec-color-light-primary-50);
  --color-hover: var(--ion-color-tertiary);

  &.selected {
    color: var(--parsec-color-light-primary-700);
    font-weight: bold;
  }
}

.dropdown-container {
  // offset necessary to simulate border 3px on focus with outline (outline 2px + border 1px)
  --offset: 2px;
  --min-height: 3rem;
  padding: var(--offset);
  display: flex;
  flex-direction: column;
  gap: 0.5rem;

  .active {
    --background: var(--parsec-color-light-secondary-background);
    outline: var(--offset) solid var(--parsec-color-light-primary-300);
    border-radius: var(--parsec-radius-6);
  }

  .form-label {
    color: var(--parsec-color-light-primary-700);
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
