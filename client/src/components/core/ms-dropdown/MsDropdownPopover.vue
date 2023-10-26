<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="container">
    <ion-item
      class="option body"
      :class="{selected: selectedOption?.key === option.key}"
      :disabled="option.disabled"
      button
      lines="none"
      v-for="option in options"
      :key="option.key"
      @click="onOptionClick(option)"
    >
      {{ option.label }}
      <ion-icon
        slot="end"
        :icon="checkmark"
        class="checked"
        :class="{selected: selectedOption?.key === option.key}"
        v-if="selectedOption?.key === option.key"
      />
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import {
  IonList,
  IonItem,
  IonIcon,
  popoverController,
} from '@ionic/vue';
import {
  checkmark,
} from 'ionicons/icons';
import { MsDropdownOption, getMsOptionByKey } from '@/components/core/ms-types';

const props = defineProps<{
  defaultOption?: any
  options: MsDropdownOption[]
}>();

const selectedOption = ref(
  props.defaultOption ? getMsOptionByKey(props.options, props.defaultOption) : props.options[0],
);

function onOptionClick(option?: MsDropdownOption): void {
  if (option) {
    selectedOption.value = option;
  }
  popoverController.dismiss({
    option: selectedOption.value,
  });
}
</script>

<style lang="scss" scoped>
.option {
  --background-hover: var(--parsec-color-light-primary-50);
  --background-hover-opacity: 1;
  --color: var(--parsec-color-light-secondary-grey);
  --color-hover: var(--parsec-color-light-primary-700);

  &.selected {
    color: var(--parsec-color-light-primary-700);
  }
  .checked.selected {
    color: var(--parsec-color-light-primary-700);
  }
}
</style>
