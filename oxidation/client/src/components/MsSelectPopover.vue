<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-list>
    <ion-item
      class="option"
      :class="{selected: selectedOption?.key === option.key}"
      button
      lines="none"
      v-for="option in options"
      :key="option.key"
      detail="false"
      @click="onOptionClick(option)"
    >
      {{ option.label }}
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import { defineProps, onMounted, ref, Ref } from 'vue';
import {
  IonList,
  IonItem,
  popoverController
} from '@ionic/vue';
import { MsSelectOption } from '@/components/MsSelectOption.ts';

const props = defineProps<{
  options: MsSelectOption[],
  defaultOption?: string
}>();

const selectedOption: Ref<MsSelectOption> = ref(props.options.find((option) => {
  return option.key === props.defaultOption;
}));

onMounted((): void => {
  console.log(selectedOption.value);
  if (props.defaultOption) {
    console.log(props.defaultOption);
    selectedOption.value = props.options.find((option) => {
      console.log(option);
      return option.key === props.defaultOption;
    });
    console.log(selectedOption.value);
  }
});

function onOptionClick(option: MsSelectOption): void {
  selectedOption.value = option;
  popoverController.dismiss(option);
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
