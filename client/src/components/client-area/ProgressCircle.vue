<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="circle-container"
    :class="props.state"
  >
    <div class="circle-outer">
      <div class="circle-inner">
        <ion-text class="circle__amount title-h5">{{ $msTranslate(formatFileSize(data)) }}</ion-text>
      </div>
    </div>

    <ion-text class="circle__description title-h5">{{ $msTranslate(text) }}</ion-text>
  </div>
</template>

<script setup lang="ts">
import { IonText } from '@ionic/vue';
import { formatFileSize } from '@/common/file';
import { computed } from 'vue';
import { Translatable } from 'megashark-lib';

const props = defineProps<{
  data: number;
  amountValue: number;
  text?: Translatable;
  state?: string;
}>();

const amountValueComputed = computed(() => {
  return `${props.amountValue}`;
});
</script>

<style lang="scss" scoped>
.circle-container {
  --percentage: v-bind(amountValueComputed);
  --thickness: 0.5rem;
  --color: var(--parsec-color-light-secondary-text);
  --size: 5rem;

  &.warning {
    --color: var(--parsec-color-light-warning-500);
  }

  &.danger {
    --color: var(--parsec-color-light-danger-500);
  }

  &.success {
    --color: var(--parsec-color-light-success-500);
  }

  &.active {
    --color: var(--parsec-color-light-primary-500);
  }

  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  gap: 0.75rem;
  border-radius: var(--parsec-radius-circle);
  position: relative;

  .circle-outer {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: var(--size);
    height: var(--size);
    border-radius: var(--parsec-radius-circle);

    &::before {
      content: '';
      position: absolute;
      width: calc(100% - 1rem);
      height: calc(100% - 1rem);
      border-radius: var(--parsec-radius-circle);
      border: 0.5rem solid var(--parsec-color-light-secondary-text);
      opacity: 0.1;
    }
  }

  .circle-inner {
    width:var(--size);
    aspect-ratio:1/1;
    position:relative;
    display:inline-grid;
    place-content:center;

    &::before,
    &::after {
      content:"";
      position:absolute;
      border-radius:50%;
    }

    &::before {
      inset:0;
      background:
        radial-gradient(farthest-side,var(--color) 98%,#0000) top/var(--thickness) var(--thickness) no-repeat,
        conic-gradient(var(--color) calc(var(--percentage)*1%),#0000 0);
      -webkit-mask:radial-gradient(farthest-side,#0000 calc(99% - var(--thickness)),#000 calc(100% - var(--thickness)));
              mask:radial-gradient(farthest-side,#0000 calc(99% - var(--thickness)),#000 calc(100% - var(--thickness)));
    }

    &::after {
      inset:calc(50% - var(--thickness)/2);
      background: var(--color);
      transform: rotate(calc(var(--percentage)*3.6deg - 90deg)) translate(calc(var(--size)/2 - 50%));
    }
  }

  .circle__description {
    color: var(--parsec-color-light-secondary-grey);
  }
}
</style>
