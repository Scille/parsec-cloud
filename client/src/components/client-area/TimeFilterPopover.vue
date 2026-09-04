<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="time-container">
    <ion-list class="time-list">
      <ion-text
        class="time-list-item ion-no-padding"
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
  --background: #{ms.color('surface-base-default')};
  padding: ms.spacing('padding-sm');

  .time-list {
    width: 100%;
    display: flex;
    flex-wrap: wrap;
    padding: ms.spacing('padding-none');
    gap: ms.spacing('gap-xs');

    &-item {
      @include ms.font('label-md-medium');
      display: flex;
      justify-content: center;
      color: ms.color('text-base-description');
      padding: ms.spacing('padding-lg');
      max-width: 4rem;
      width: 100%;
      border-radius: ms.radius('md');
      cursor: pointer;
      user-select: none;

      &:hover {
        background: ms.color('surface-neutral-default-subtle-hover');
        color: ms.color('text-brand-default-hover');
      }

      &.selected {
        background: ms.color('surface-brand-default-subtle');
        color: ms.color('text-brand-default');

        &:hover {
          background: ms.color('surface-brand-default-subtle-hover');
          color: ms.color('text-brand-default');
        }
      }
    }
  }
}
</style>
