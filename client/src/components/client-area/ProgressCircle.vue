<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div
    class="circle-container"
    :class="state"
  >
    <div class="circle-outer">
      <div class="circle-inner">
        <ion-text
          class="circle__amount"
          :class="state"
        >
          {{ amountValue }}%
        </ion-text>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonText } from '@ionic/vue';
import { computed } from 'vue';

const props = defineProps<{
  amountValue: number;
  state?: string;
}>();

const amountValueComputed = computed(() => {
  return `${props.amountValue}`;
});
</script>

<style lang="scss" scoped>
.circle-container,
.circle__amount {
  color: var(--color);

  &.default {
    --color: #{ms.color('text-neutral-default')};
  }

  &.warning {
    --color: #{ms.color('text-warning-default')};
  }

  &.danger {
    --color: #{ms.color('text-error-default')};
  }

  &.success {
    --color: #{ms.color('text-success-default')};
  }

  &.active {
    --color: #{ms.color('text-brand-default')};
  }
}

.circle__amount {
  @include ms.font('heading-h6');
}

.circle-container {
  --percentage: v-bind(amountValueComputed);
  --thickness: 0.5rem;
  --color: #{ms.color('text-neutral-default')};
  --size: 5rem;

  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  gap: ms.spacing('gap-2xl');
  border-radius: ms.radius('full');
  position: relative;

  .circle-outer {
    position: relative;
    display: flex;
    align-items: center;
    justify-content: center;
    width: var(--size);
    height: var(--size);
    border-radius: ms.radius('full');

    &::before {
      content: '';
      position: absolute;
      width: calc(100% - 1rem);
      height: calc(100% - 1rem);
      border-radius: ms.radius('full');
      border: 0.5rem solid ms.color('border-neutral-default');
      opacity: ms.opacity('1');
    }
  }

  .circle-inner {
    width: var(--size);
    aspect-ratio: 1/1;
    position: relative;
    display: inline-grid;
    place-content: center;

    &::before,
    &::after {
      content: '';
      position: absolute;
      border-radius: 50%;
    }

    &::before {
      inset: 0;
      background:
        radial-gradient(farthest-side, var(--color) 98%, #0000) top/var(--thickness) var(--thickness) no-repeat,
        conic-gradient(var(--color) calc(var(--percentage) * 1%), #0000 0);
      -webkit-mask: radial-gradient(farthest-side, #0000 calc(99% - var(--thickness)), #000 calc(100% - var(--thickness)));
      mask: radial-gradient(farthest-side, #0000 calc(99% - var(--thickness)), #000 calc(100% - var(--thickness)));
    }

    &::after {
      inset: calc(50% - var(--thickness) / 2);
      background: var(--color);
      transform: rotate(calc(var(--percentage) * 3.6deg - 90deg)) translate(calc(var(--size) / 2 - 50%));
    }
  }

  .circle__description {
    color: ms.color('text-base-description');
  }
}
</style>
