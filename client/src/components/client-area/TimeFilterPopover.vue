<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="time-container">
    <ion-list class="time-list">
      <ion-text
        class="time-list-item ion-no-padding button-medium"
        :class="{'selected': selected !== undefined && selected === time.key}"
        v-for="time in times.set"
        :key="time.key"
        @click="select(time.key)"
      >
        <!-- No ideal, should translate the month properly -->
        {{
          $msTranslate({
            key: "common.date.asIs",
            data: { date: I18n.translate(time.label) },
          })
        }}
      </ion-text>
    </ion-list>
  </div>
</template>

<script setup lang="ts">
import { IonList, IonText, popoverController } from '@ionic/vue';
import { MsModalResult, MsOptions, I18n } from 'megashark-lib';

defineProps<{
  times: MsOptions;
  selected?: any,
}>();

// if not selected, selected else deselect
async function select(selected: any): Promise<void> {
  await popoverController.dismiss({selected: selected}, MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped>
* {
  transition: all 0.150s ease;
}

.time-container {
  display: flex;
  --background: var(--parsec-color-light-secondary-inversed-contrast);
  padding: 0.25rem;

  .time-list {
    display: flex;
    flex-wrap: wrap;
    padding: 0;

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
