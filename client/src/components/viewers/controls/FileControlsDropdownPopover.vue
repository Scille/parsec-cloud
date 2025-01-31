<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="file-controls-dropdown-popover">
    <div
      v-if="parentHistory.length > 0"
      @click="onBackClicked"
      class="back-button"
    >
      <ion-icon :icon="chevronBack" />
      <ion-text class="title">
        {{ parentHistory.slice(-1)[0].name }}
      </ion-text>
    </div>
    <div
      class="divider"
      v-if="parentHistory.length > 0"
    />
    <file-controls-dropdown-item
      v-for="(item, index) in displayedItems"
      :key="index"
      :is-active="item.isActive"
      :item="item"
      @click="onOptionClick"
    />
  </ion-list>
</template>

<script setup lang="ts">
import { FileControlsDropdownItem, FileControlsDropdownItemContent } from '@/components/viewers';
import { IonIcon, IonList, IonText, popoverController } from '@ionic/vue';
import { ref } from 'vue';
import { chevronBack } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';

const props = defineProps<{
  items: FileControlsDropdownItemContent[];
}>();

const displayedItems = ref<FileControlsDropdownItemContent[]>(props.items);
const parentHistory = ref<Array<{ items: FileControlsDropdownItemContent[]; name: Translatable }>>([]);

async function onOptionClick(option: FileControlsDropdownItemContent): Promise<void> {
  if (option.children) {
    parentHistory.value.push({ items: displayedItems.value, name: option.label });
    displayedItems.value = option.children;
  }
  if (option.callback) {
    await option.callback();
  }
  if (option.dismissToParent) {
    await onBackClicked();
  } else if (option.dismissPopover) {
    await popoverController.dismiss({
      option: option,
    });
  }
}

async function onBackClicked(): Promise<void> {
  const parentItem = parentHistory.value.pop();
  if (parentItem) {
    displayedItems.value = parentItem.items;
  }
}
</script>

<style lang="scss" scoped>
.file-controls-dropdown-popover {
  overflow-y: auto;
  max-height: 20em;

  .back-button {
    display: flex;
    align-items: center;
    margin: 0.5rem;
    cursor: pointer;
    color: var(--parsec-color-light-secondary-hard-grey);

    .title {
      margin-left: 0.5rem;
    }

    &:hover {
      color: var(--parsec-color-light-primary-700);
    }
  }

  .divider {
    background-color: var(--parsec-color-light-secondary-background);
    height: 2px;
    width: 100%;
    margin: 0.8rem 0;
  }
}
</style>
