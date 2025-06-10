<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="time-container">
    <ion-list class="time-list">
      <ion-text
        class="time-list-item ion-no-padding button-medium"
        :class="{ selected: selected !== undefined && selected.includes(time.key) }"
        v-for="time in times.set"
        :key="time.key"
        @click="select(time.key)"
      >
        {{ $msTranslate(time.label) }}
      </ion-text>
    </ion-list>
  </div>
</template>

<script setup lang="ts">
import { IonList, IonText, popoverController } from '@ionic/vue';
import { MsModalResult, MsOptions } from 'megashark-lib';

const props = defineProps<{
  times: MsOptions;
  selected?: Array<any>;
  sortDesc?: boolean;
}>();

async function select(selected: any): Promise<void> {
  if (props.selected === undefined) {
    await popoverController.dismiss({ selected: selected }, MsModalResult.Confirm);
  }
  const index = (props.selected || []).findIndex((time) => time === selected);

  /* eslint-disable vue/no-mutating-props */
  // No proper way to do it with ionic popover, we can't use emits
  if (index === -1) {
    props.selected?.push(selected);
    props.selected?.sort((t1, t2) => (props.sortDesc ? t2 - t1 : t1 - t2));
  } else {
    props.selected?.splice(index, 1);
  }
  /* eslint-enable vue/no-mutating-props */
}
</script>

<style lang="scss" scoped>
* {
  transition: all 0.15s ease;
}

.time-container {
  display: flex;
  --background: var(--parsec-color-light-secondary-inversed-contrast);
  padding: 0.25rem;

  .time-list {
    display: flex;
    flex-wrap: wrap;
    padding: 0;
    gap: 0.125rem;

    &-item {
      display: flex;
      justify-content: center;
      color: var(--parsec-color-light-secondary-hard-grey);
      padding: 0.5rem;
      max-width: 4rem;
      width: 100%;
      border-radius: var(--parsec-radius-6);
      cursor: pointer;
      user-select: none;

      &:hover {
        background: var(--parsec-color-light-secondary-premiere);
        color: var(--parsec-color-light-primary-700);
      }

      &.selected {
        background: var(--parsec-color-light-primary-100);
        color: var(--parsec-color-light-primary-700);

        &:hover {
          background: var(--parsec-color-light-primary-50);
          color: var(--parsec-color-light-primary-700);
        }
      }
    }
  }
}
</style>
