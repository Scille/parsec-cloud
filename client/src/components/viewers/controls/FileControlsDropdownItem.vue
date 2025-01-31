<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="dropdown-item"
    :class="isActive ? 'dropdown-item-active' : ''"
    @click="emits('click', item)"
  >
    <div class="left">
      <ion-icon
        v-if="item.icon"
        :icon="item.icon"
      />
      <ion-text class="left-text">
        {{ $msTranslate(item.label) }}
      </ion-text>
    </div>
    <ion-icon
      class="checkmark"
      v-if="isActive"
      :icon="checkmark"
    />
    <div
      class="right-text"
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
import { Translatable } from 'megashark-lib';

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
.dropdown-item {
  display: flex;
  margin: 0.25rem 0.5rem;
  cursor: pointer;
  height: 2em;
  border-radius: var(--parsec-radius-6);

  &-active {
    background-color: var(--parsec-color-light-secondary-medium);
  }

  &:hover:not(.dropdown-item-active) {
    color: var(--parsec-color-light-primary-700);
    background-color: var(--parsec-color-light-secondary-background);
  }

  .left {
    display: flex;
    align-items: center;
    margin-left: 0.5rem;

    &-text {
      margin-left: 0.5rem;
    }
  }

  .right-text {
    display: flex;
    align-items: center;
    color: var(--parsec-color-light-secondary-grey);
    margin-left: auto;
    margin-right: 0.5rem;
  }

  .checkmark {
    color: var(--parsec-color-light-primary-600);
    margin: auto 0.5rem auto auto;
  }
}
</style>
