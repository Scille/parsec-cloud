<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="container">
    <ion-item
      class="option body"
      :class="{ selected: selectedOption?.key === option.key, 'item-disabled': option.disabled }"
      button
      lines="none"
      v-for="option in options.set"
      :key="option.key"
      @click="onOptionClick(option)"
    >
      <ion-label class="option-text">
        <span class="option-text__label body">
          {{ option.label }}
        </span>
        <span
          v-if="option.description"
          class="option-text__description body-sm"
        >
          {{ option.description }}
        </span>
      </ion-label>
      <ion-icon
        slot="end"
        :icon="checkmark"
        class="icon checked"
        :class="{ selected: selectedOption?.key === option.key }"
        v-if="selectedOption?.key === option.key"
      />
      <ms-information-tooltip
        v-if="option.disabled && option.disabledReason"
        :text="option.disabledReason"
        class="icon disabled-icon"
        slot="end"
      />
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import { MsInformationTooltip, MsOption, MsOptions } from '@/components/core';
import { IonIcon, IonItem, IonLabel, IonList, popoverController } from '@ionic/vue';
import { checkmark } from 'ionicons/icons';
import { ref } from 'vue';

const props = defineProps<{
  defaultOption?: any;
  options: MsOptions;
}>();

const selectedOption = ref(props.defaultOption ? props.options.get(props.defaultOption) : props.options.at(0));

async function onOptionClick(option?: MsOption): Promise<void> {
  if (option) {
    selectedOption.value = option;
  }
  await popoverController.dismiss({
    option: selectedOption.value,
  });
}
</script>

<style lang="scss" scoped>
.container {
  padding: 0.5rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.option {
  --background-hover: none;
  --color: var(--parsec-color-light-secondary-grey);
  padding: 0.375rem 0.75rem;
  --background: none;
  border-radius: var(--parsec-radius-6);
  --min-height: 0;
  --inner-padding-end: 0;
  position: relative;
  z-index: 2;
  pointer-events: auto;

  &:hover:not(.item-disabled) {
    background: var(--parsec-color-light-primary-50);
    --background-hover: var(--parsec-color-light-primary-50);
  }

  &::part(native) {
    padding: 0;
  }

  &-text {
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.25rem;

    &__label {
      color: var(--parsec-color-light-secondary-text);
    }
  }

  .icon {
    margin: 0;
    margin-left: 1em;
  }

  &.selected {
    .option-text {
      &__label {
        color: var(--parsec-color-light-primary-700);
      }

      &__description {
        color: var(--parsec-color-light-secondary-grey);
      }
    }

    .icon {
      color: var(--parsec-color-light-primary-700);
    }
  }

  &.item-disabled {
    opacity: 1;
    pointer-events: none;

    .option-text {
      opacity: 0.5;

      &__label {
        --color: var(--parsec-color-light-secondary-text);
      }

      &__description {
        --color: var(--parsec-color-light-secondary-grey);
      }
    }

    .disabled-icon {
      pointer-events: initial;
      opacity: 0.8;
      position: relative;
    }
  }
}
</style>
