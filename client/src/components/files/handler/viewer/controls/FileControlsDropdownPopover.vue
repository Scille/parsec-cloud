<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-list
    class="file-controls-dropdown-popover"
    ref="popoverElement"
  >
    <div
      v-if="parentHistory.length > 0"
      @click="onBackClicked"
      class="back-button"
    >
      <ion-icon :icon="chevronBack" />
      <ion-text class="back-button__text">
        {{ $msTranslate(parentHistory.slice(-1)[0].name) }}
      </ion-text>
    </div>
    <div class="dropdown-list-items">
      <file-controls-dropdown-item
        v-for="(item, index) in displayedItems"
        :key="index"
        :is-active="item.isActive"
        :item="item"
        @click="onOptionClick"
      />
    </div>
  </ion-list>
</template>

<script setup lang="ts">
import { FileControlsDropdownItem, FileControlsDropdownItemContent } from '@/components/files/handler/viewer';
import { IonIcon, IonList, IonText, popoverController } from '@ionic/vue';
import { chevronBack } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';
import { nextTick, ref, useTemplateRef, watch } from 'vue';

const props = defineProps<{
  items: FileControlsDropdownItemContent[];
}>();
const displayedItems = ref<FileControlsDropdownItemContent[]>(props.items);
const parentHistory = ref<Array<{ items: FileControlsDropdownItemContent[]; name: Translatable }>>([]);
const prevHeight = ref(0);
const popoverElementRef = useTemplateRef<InstanceType<typeof IonList>>('popoverElement');

watch(displayedItems, async () => {
  await nextTick();
  requestAnimationFrame(async () => {
    const popover = await popoverController.getTop();
    if (popover) {
      popover.style.setProperty('--offset-y', await getOffsetY());
    }
  });
});

async function onOptionClick(option: FileControlsDropdownItemContent): Promise<void> {
  if (option.children) {
    saveHeight();
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
    saveHeight();
    displayedItems.value = parentItem.items;
  }
}

async function getOffsetY(): Promise<string> {
  let offset = 0;
  if (popoverElementRef.value?.$el) {
    offset = popoverElementRef.value.$el.clientHeight - prevHeight.value;

    return offset > 0 ? `-${offset}px` : '0px';
  }
  return '0px';
}

function saveHeight(): void {
  prevHeight.value = popoverElementRef.value?.$el.clientHeight;
}
</script>

<style lang="scss" scoped>
.file-controls-dropdown-popover {
  overflow-y: auto;
  max-height: 20em;
  padding: ms.spacing('padding-none');

  .back-button {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
    padding: ms.spacing('padding-2xl');
    cursor: pointer;
    color: ms.color('text-base-description');
    border-bottom: ms.border('thin') solid ms.color('border-neutral-default-subtle-hover');

    &__text {
      @include ms.font('label-md-medium');
    }

    &:hover {
      color: ms.color('text-brand-default-hover');
    }

    ion-icon {
      font-size: 0.875rem;
    }
  }
}

.dropdown-list-items {
  padding: ms.spacing('padding-lg');
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-lg');
}
</style>
