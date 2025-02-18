<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="file-controls-dropdown-item"
    :class="isActive ? 'file-controls-dropdown-item-active' : ''"
    @click="emits('click', item)"
  >
    <div class="left">
      <ion-icon
        v-if="item.icon"
        :icon="item.icon"
        class="left-icon"
      />
      <ms-image
        v-if="item.image"
        :image="item.image"
        class="left-icon"
      />
      <ion-text class="left-text button-medium">{{ $msTranslate(item.label) }}</ion-text>
    </div>
    <ion-icon
      class="checkmark"
      v-if="isActive"
      :icon="checkmark"
    />
    <div
      class="right-text button-medium"
      v-if="item.children"
    >
      {{ $msTranslate(getActiveChildLabel()) }}
      <ion-icon :icon="chevronForward" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { FileControlsDropdownItemContent } from '@/components/viewers';
import { IonIcon, IonText } from '@ionic/vue';
import { checkmark, chevronForward } from 'ionicons/icons';
import { MsImage, Translatable } from 'megashark-lib';

const emits = defineEmits<{
  (e: 'click', option: FileControlsDropdownItemContent): void;
}>();

const props = defineProps<{
  item: FileControlsDropdownItemContent;
  isActive?: boolean;
}>();

function getActiveChildLabel(): Translatable {
  if (props.item.children) {
    // Display active child if only one is active
    const activeChildren = props.item.children.filter((item) => item.isActive === true);
    if (activeChildren.length === 1) {
      return activeChildren[0].label;
    }
  }
  return '';
}
</script>

<style lang="scss" scoped>
.file-controls-dropdown-item {
  display: flex;
  cursor: pointer;
  padding: 0.5rem 0.75rem;
  justify-content: space-between;
  align-items: center;
  border-radius: var(--parsec-radius-6);

  &-active {
    background-color: var(--parsec-color-light-primary-50);
  }

  &:hover:not(.dropdown-item-active) {
    color: var(--parsec-color-light-secondary-text);
    background-color: var(--parsec-color-light-secondary-background);
  }

  .left {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--parsec-color-light-secondary-text);

    &-icon {
      font-size: 1.125rem;
      width: 1.125rem;
      height: 1.125rem;
      color: var(--parsec-color-light-secondary-text);
      --fill-color: var(--parsec-color-light-secondary-text);
    }
  }

  .right-text {
    display: flex;
    align-items: center;
    color: var(--parsec-color-light-secondary-grey);
    gap: 0.5rem;
  }

  .checkmark {
    color: var(--parsec-color-light-primary-600);
    font-size: 1rem;
  }
}
</style>
