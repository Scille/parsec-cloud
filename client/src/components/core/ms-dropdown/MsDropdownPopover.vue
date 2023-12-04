<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="container">
    <ion-item
      class="option body"
      :class="{ selected: selectedOption?.key === option.key, 'item-disabled': option.disabled }"
      button
      lines="none"
      v-for="option in options"
      :key="option.key"
      @click="onOptionClick(option)"
    >
      <ion-label class="option-text">
        <span class="option-text__title body">{{ option.label }}</span>
        <span
          v-if="option.description"
          class="option-text__subtitle body-sm"
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
      <ion-icon
        v-if="option.disabled"
        slot="end"
        :icon="informationCircle"
        class="icon disabled-icon"
        @click="openTooltip($event, option.label)"
      />
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { IonList, IonItem, IonIcon, IonLabel, popoverController } from '@ionic/vue';
import { checkmark, informationCircle } from 'ionicons/icons';
import { MsDropdownOption, getMsOptionByKey } from '@/components/core/ms-types';
import MsDropdownTooltip from '@/components/core/ms-dropdown/MsDropdownTooltip.vue';

const props = defineProps<{
  defaultOption?: any;
  options: MsDropdownOption[];
}>();

const selectedOption = ref(props.defaultOption ? getMsOptionByKey(props.options, props.defaultOption) : props.options[0]);

function onOptionClick(option?: MsDropdownOption): void {
  if (option) {
    selectedOption.value = option;
  }
  popoverController.dismiss({
    option: selectedOption.value,
  });
}

function openTooltip(event: Event, text: string): void {
  event.stopPropagation();
  const popover = popoverController.create({
    component: MsDropdownTooltip,
    alignment: 'center',
    event: event,
    componentProps: {
      text,
    },
    cssClass: 'tooltip-popover',
    showBackdrop: false,
  });
  popover.then((popoverInstance) => popoverInstance.present());
}
</script>

<style lang="scss" scoped>
.container {
  padding: 0.5rem;
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.option {
  --background-hover: none;
  --color: var(--parsec-color-light-secondary-grey);
  padding: 0.75rem;
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
    --color-hover: var(--parsec-color-light-primary-700);
  }

  &::part(native) {
    padding: 0;
  }

  &-text {
    margin: 0;
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    &__subtitle {
      margin-bottom: 0.25rem;
    }
  }

  .icon {
    margin: 0;
  }

  &.selected {
    .option-text {
      &__title {
        color: var(--parsec-color-light-primary-700);
      }

      &__subtitle {
        color: var(--parsec-color-light-secondary-grey);
      }
    }
  }

  .checked.selected {
    color: var(--parsec-color-light-primary-700);
  }

  &.item-disabled {
    opacity: 1;
    pointer-events: none;

    .option-text {
      opacity: 0.5;

      &__title {
        --color: var(--parsec-color-light-secondary-text);
      }

      &__subtitle {
        --color: var(--parsec-color-light-secondary-grey);
      }
    }

    .disabled-icon {
      pointer-events: initial;
      opacity: 0.8;
      --color: var(--parsec-color-light-secondary-grey);
      position: relative;
      &:hover {
        color: var(--parsec-color-light-secondary-text);
      }
    }
  }
}
</style>
