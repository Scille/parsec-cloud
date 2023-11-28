<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="tooltip-container">
    <ion-text class="tooltip-text body-sm">
      {{ text }}
    </ion-text>
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
  background: var(--parsec-color-light-primary-900);
  color: var(--parsec-color-light-primary-30);
  max-width: 14.5rem;
  padding: .75rem;
  position: sticky;
  border-radius: var(--parsec-radius-6);
  box-shadow: var(--parsec-shadow-light);
  text-align: center;
}

/* Tooltip arrow */
.tooltip-container::after {
  content: "";
  position: absolute;
  top: 100%;
  left: 50%;
  margin-left: -10px;
  border-width: 10px;
  border-style: solid;
  border-color: var(--parsec-color-light-primary-900) transparent transparent transparent;
}
</style>
