<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div
    class="toolbar secondary file-controls"
    :class="{ 'file-controls-disabled': disabled }"
    :inert="disabled"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
defineProps<{
  disabled?: boolean;
}>();
</script>

<style scoped lang="scss">
.file-controls {
  padding: ms.spacing('padding-2xl') ms.spacing('padding-4xl') ms.spacing('padding-4xl');
  display: flex;
  justify-content: center;
  align-items: center;
  gap: ms.spacing('gap-4xl');
  width: 100%;
  background: ms.color('surface-base-default');
  box-shadow: ms.shadow('elevation-sm');
  --inner-padding-end: #{ms.spacing('padding-none')};
  border-top: ms.border('thin') solid ms.color('border-neutral-default-subtle-hover');
  position: relative;
  z-index: 200;
  min-height: 3.75rem;
  transition: all 0.2s ease-in-out;

  @include ms.responsive-breakpoint('lg') {
    margin: 0;
    padding: ms.spacing('padding-2xl') ms.spacing('padding-3xl') ms.spacing('padding-4xl');
    border-radius: ms.radius('lg') ms.radius('lg') 0 0;
    gap: ms.spacing('gap-3xl');
  }

  & > *:not(:last-child)::after {
    content: '';
    display: block;
    width: ms.border('thick');
    height: 1rem;
    background: ms.color('surface-disabled-default');
  }

  &:hover {
    box-shadow: ms.shadow('light');
  }

  &-disabled {
    opacity: ms.opacity('5');
  }
}
</style>
