<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="toolbar ion-margin-bottom secondary file-controls">
    <slot />
  </div>
</template>

<script setup lang="ts"></script>

<style scoped lang="scss">
.file-controls {
  padding: 0.5rem 1rem;
  display: flex;
  gap: 0.5em;
  width: fit-content;
  background: var(--parsec-color-light-primary-100);
  border-radius: 5em;
  box-shadow: var(--parsec-shadow-light);
  border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
  --inner-padding-end: 0;

  & > *:not(:last-child)::after {
    content: '';
    display: block;
    width: 2px;
    height: 2em;
    background: var(--parsec-color-light-primary-200);
    opacity: 0.5;
  }
}
</style>
