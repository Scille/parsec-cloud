<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
      <ion-text class="left-text">{{ $msTranslate(item.label) }}</ion-text>
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
import { FileControlsDropdownItemContent } from '@/components/files/handler/viewer';
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
  padding: ms.spacing('padding-lg') ms.spacing('padding-2xl');
  justify-content: space-between;
  align-items: center;
  border-radius: ms.radius('md');

  &-active {
    background-color: ms.color('surface-brand-default-subtle');
  }

  &:hover:not(.dropdown-item-active) {
    color: ms.color('text-neutral-default');
    background-color: ms.color('surface-neutral-default-subtle-hover');
  }

  .left {
    display: flex;
    align-items: center;
    @include ms.font('label-md-medium');
    gap: ms.spacing('gap-lg');
    color: ms.color('text-neutral-default');

    &-icon {
      font-size: 1.125rem;
      width: 1.125rem;
      height: 1.125rem;
      color: ms.color('icon-neutral-default');
      --fill-color: #{ms.color('icon-neutral-default')};
    }
  }

  .right-text {
    display: flex;
    align-items: center;
    @include ms.font('label-md-medium');
    color: ms.color('text-base-description');
    gap: ms.spacing('gap-lg');
  }

  .checkmark {
    color: ms.color('icon-brand-default-hover');
    font-size: 1rem;
  }
}
</style>
