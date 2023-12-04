<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="tooltip-container">
    <svg
      width="20"
      height="8"
      viewBox="0 0 20 8"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        fill-rule="evenodd"
        clip-rule="evenodd"
        d="M10 0C13 0 15.9999 8 20 8L0 8C3.9749 8 7 0 10 0Z"
        fill="var(--parsec-color-light-primary-900)"
      />
    </svg>
    <div class="tooltip-content">
      <ion-text class="tooltip-text body-sm">
        {{ text }}
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { IonText } from '@ionic/vue';
import { defineProps } from 'vue';

defineProps<{
  text: string;
}>();
</script>

<style lang="scss" scoped>
.tooltip-container {
  display: flex;
  align-items: center;
  flex-direction: column;

  &.bottom {
    flex-direction: column-reverse;

    svg {
      transform: rotate(180deg);
    }
  }
}

.tooltip-content {
  background: var(--parsec-color-light-primary-900);
  color: var(--parsec-color-light-primary-30);
  max-width: 14.5rem;
  width: fit-content;
  padding: 0.75rem;
  position: relative;
  border-radius: var(--parsec-radius-6);
  box-shadow: var(--parsec-shadow-light);
  text-align: center;
}
</style>
