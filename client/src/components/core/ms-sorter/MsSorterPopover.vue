<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="sorter-container">
    <ion-item
      v-if="sorterLabels"
      id="sort-order-button"
      class="order-button body"
      button
      @click="onOptionClick()"
    >
      {{ sortByAsc ? sorterLabels.asc : sorterLabels.desc }}
      <ion-icon
        :icon="sortByAsc ? arrowUp : arrowDown"
        slot="end"
      />
    </ion-item>
    <div
      class="sorter-list"
      id="sort-item-list"
    >
      <ion-label class="sorter-list__title body-sm">
        {{ $t('common.sortBy') }}
      </ion-label>
      <ion-item
        class="sorter-list-item body"
        :class="{ selected: selectedOption?.key === option.key }"
        :disabled="option.disabled"
        button
        lines="none"
        v-for="option in options.set"
        :key="option.key"
        @click="onOptionClick(option)"
      >
        <span class="body">{{ option.label }}</span>
        <ion-icon
          slot="end"
          :icon="checkmark"
          class="checked"
          :class="{ selected: selectedOption?.key === option.key }"
          v-if="selectedOption?.key === option.key"
        />
      </ion-item>
    </div>
  </ion-list>
</template>

<script setup lang="ts">
import { MsSorterLabels } from '@/components/core/ms-sorter/types';
import { MsOption, MsOptions } from '@/components/core/ms-types';
import { IonIcon, IonItem, IonLabel, IonList, popoverController } from '@ionic/vue';
import { arrowDown, arrowUp, checkmark } from 'ionicons/icons';
import { Ref, ref } from 'vue';

const props = defineProps<{
  defaultOption?: any;
  options: MsOptions;
  sorterLabels?: MsSorterLabels;
  sortByAsc: boolean;
}>();

const sortByAsc: Ref<boolean> = ref(props.sortByAsc);
const selectedOption = ref(props.defaultOption ? props.options.get(props.defaultOption) : props.options.at(0));

function onOptionClick(option?: MsOption): void {
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
.sorter-container {
  padding: 0;
  display: flex;
  flex-direction: column;
}

#sort-order-button {
  --background: none;
  --color: var(--parsec-color-light-secondary-grey);
  --border-radius: var(--parsec-radius-4);
  --padding-top: 0.375rem;
  --padding-bottom: 0.375rem;
  --padding-start: 0.75rem;
  --padding-end: 0.75rem;
  --inner-padding-end: 0;
  padding: 0.25rem;
  width: 100%;
  margin-left: auto;
  transition: transform 0.2s ease-in-out;
  border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
  cursor: pointer;

  &::part(native) {
    width: auto;
    margin-left: auto;
  }

  &:hover {
    background: var(--parsec-color-light-secondary-background);
    --background-hover: none;
    --color-hover: var(--parsec-color-light-secondary-text);

    ion-icon {
      color: var(--parsec-color-light-secondary-text);
    }
  }

  ion-icon {
    color: var(--parsec-color-light-secondary-grey);
    margin: 0;
    padding-left: 0.375rem;
    font-size: 1rem;
  }
}

.sorter-list {
  padding: 0.75rem;
  display: flex;
  flex-direction: column;

  &__title {
    color: var(--parsec-color-light-secondary-grey);
    opacity: 0.7;
    margin-bottom: 0.5rem;
  }

  &-item {
    --background-hover: var(--parsec-color-light-primary-30);
    --background-hover-opacity: 1;
    --color: var(--parsec-color-light-secondary-grey);
    --color-hover: var(--parsec-color-light-primary-600);
    --border-radius: var(--parsec-radius-4);
    --padding-top: 0.375rem;
    --padding-bottom: 0.375rem;
    --padding-start: 0.5rem;
    --padding-end: 0.5rem;
    --inner-padding-end: 0;

    &.selected {
      color: var(--parsec-color-light-primary-700);
      --color-hover: var(--parsec-color-light-primary-700);
      --background-hover: none;

      span {
        font-weight: 600;
      }
    }
    .checked.selected {
      color: var(--parsec-color-light-primary-700);
    }

    ion-icon {
      margin: 0 0 0 1em;
      font-size: 1.25rem;
    }
  }
}
</style>
