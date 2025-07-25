<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="toolbar ion-margin-bottom secondary file-controls">
    <slot />
  </div>
</template>

<script setup lang="ts"></script>

<style scoped lang="scss">
.file-controls {
  margin: 0 1rem 1rem 1rem;
  padding: 0.75rem 1.5rem;
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 1.5rem;
  width: 100%;
  background: var(--parsec-color-light-secondary-white);
  border-radius: var(--parsec-radius-8);
  box-shadow: var(--parsec-shadow-soft);
  --inner-padding-end: 0;
  border-top: 1px solid var(--parsec-color-light-secondary-medium);
  position: relative;
  z-index: 200;
  min-height: 3.75rem;
  transition: all 0.2s ease-in-out;

  & > *:not(:last-child)::after {
    content: '';
    display: block;
    width: 2px;
    height: 1rem;
    background: var(--parsec-color-light-secondary-disabled);
  }

  &:hover {
    box-shadow: var(--parsec-shadow-light);
  }
}
</style>
