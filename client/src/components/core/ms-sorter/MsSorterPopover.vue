<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="container">
    <ion-item
      v-if="sorterLabels"
      id="sort-order-button"
      class="option body-small"
      button
      @click="onOptionClick()"
    >
      {{ sortByAsc ? sorterLabels.asc : sorterLabels.desc }}
      <ion-icon
        :icon="sortByAsc ? arrowUp : arrowDown"
        slot="end"
      />
    </ion-item>
    <ion-item
      class="option body"
      :class="{ selected: selectedOption?.key === option.key }"
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
        :class="{ selected: selectedOption?.key === option.key }"
        v-if="selectedOption?.key === option.key"
      />
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import { ref, Ref } from 'vue';
import { IonList, IonItem, IonIcon, popoverController } from '@ionic/vue';
import { arrowUp, arrowDown, checkmark } from 'ionicons/icons';
import { MsSorterOption, MsSorterLabels, getMsOptionByKey } from '@/components/core/ms-types';

const props = defineProps<{
  defaultOption?: any;
  options: MsSorterOption[];
  sorterLabels?: MsSorterLabels;
  sortByAsc: boolean;
}>();

const sortByAsc: Ref<boolean> = ref(props.sortByAsc);
const selectedOption = ref(props.defaultOption ? getMsOptionByKey(props.options, props.defaultOption) : props.options[0]);

function onOptionClick(option?: MsSorterOption): void {
  if (option) {
    selectedOption.value = option;
  } else {
    sortByAsc.value = !sortByAsc.value;
  }
  popoverController.dismiss({
    option: selectedOption.value,
    sortByAsc: sortByAsc.value,
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

  ion-icon {
    margin-left: 1em;
  }
}
#sort-order-button {
  --background: var(--parsec-color-light-secondary-medium);
  --color: var(--parsec-color-light-secondary-text);
  --color-hover: var(--parsec-color-light-secondary-text);
  --border-radius: 25px;
  width: fit-content;
  padding-right: 0.5rem;
  margin-left: auto;
  margin-bottom: 0.5rem;
  --min-height: 2rem;

  ion-icon {
    margin: 0;
    padding-left: 0.5rem;
    font-size: 1.25rem;
  }
}
</style>
