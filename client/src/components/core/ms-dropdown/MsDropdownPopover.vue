<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="container">
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
      <ion-label class="option-text">
        <span class="option-text__title body">{{ option.label }}</span>
        <span
          v-if="option.sublabel"
          class="option-text__subtitle body-sm"
        >{{ option.sublabel }}</span>
      </ion-label>
      <ion-icon
        slot="end"
        :icon="checkmark"
        class="icon checked"
        :class="{ selected: selectedOption?.key === option.key }"
        v-if="selectedOption?.key === option.key"
      />
      <ion-icon
        slot="end"
        :icon="informationCircle"
        class="icon disabled-icon"
        v-if="option.disabled"
        @click="openTooltip($event, option.label)"
      />
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { IonList, IonItem, IonIcon, popoverController } from '@ionic/vue';
import { checkmark, informationCircle } from 'ionicons/icons';
import { MsDropdownOption, getMsOptionByKey } from '@/components/core/ms-types';
import MsDropdownTooltip from '@/components/core/ms-dropdown/MsDropdownTooltip.vue';

const props = defineProps<{
  defaultOption?: any;
  options: MsDropdownOption[];
}>();

const selectedOption = ref(props.defaultOption ? getMsOptionByKey(props.options, props.defaultOption) : props.options[0]);

function onOptionClick(option?: MsDropdownOption): void {
  console.log('BITE');
  if (option) {
    selectedOption.value = option;
  }
  popoverController.dismiss({
    option: selectedOption.value,
  });
}

function openTooltip(event: Event, text: string): void {
  event.stopPropagation();
  console.log(event);
  const popover = popoverController.create({
    component: MsDropdownTooltip,
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
  padding: .5rem;
}

// eslint-disable-next-line vue-scoped-css/no-unused-selector
.option {
  --background-hover: none;
  --color: var(--parsec-color-light-secondary-grey);
  padding: .75rem;
  --background: none;
  border-radius: var(--parsec-radius-6);
  --min-height: 0;
  --inner-padding-end: 0;
  position: relative;
  z-index: 2;

  &:hover:not(.item-disabled) {
    background: var(--parsec-color-light-primary-50);
    --background-hover: var(--parsec-color-light-primary-50);
    --color-hover: var(--parsec-color-light-primary-700);
  }

  &::part(native) {
    padding: 0;
  }

  &-text {
    // display: flex;
    // flex-direction: column;
    // align-items: center;
    &__subtitle {
      margin-bottom:  .25rem;
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
      opacity: .5;

      &__title {
        --color: var(--parsec-color-light-secondary-text);
      }

      &__subtitle {
        --color: var(--parsec-color-light-secondary-grey);
      }
    }

    .disabled-icon {
      pointer-events: initial;
      opacity: .8;
      --color: var(--parsec-color-light-secondary-grey);
      position: relative;
    }
  }
}
</style>
