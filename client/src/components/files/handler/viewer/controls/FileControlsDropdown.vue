<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <file-controls-button
    @click="openPopover($event)"
    class="file-controls-dropdown"
    :class="{ 'dropdown-active': isPopoverOpen }"
    :icon="icon || image"
    :label="title"
  />
</template>

<script setup lang="ts">
import { FileControlsButton, FileControlsDropdownItemContent, FileControlsDropdownPopover } from '@/components/files/handler/viewer';
import { popoverController } from '@ionic/vue';
import { Translatable } from 'megashark-lib';
import { Ref, ref } from 'vue';

const isPopoverOpen = ref(false);
const selectedOption: Ref<FileControlsDropdownItemContent | undefined> = ref(undefined);

const props = defineProps<{
  items: FileControlsDropdownItemContent[];
  title?: Translatable;
  icon?: string;
  image?: string;
}>();

async function openPopover(event: Event): Promise<void> {
  const popover = await popoverController.create({
    component: FileControlsDropdownPopover,
    cssClass: 'dropdown-popover',
    componentProps: {
      items: props.items,
    },
    event: event,
    alignment: 'center',
    side: 'top',
    showBackdrop: false,
  });
  isPopoverOpen.value = true;
  await popover.present();
  await onDidDismissPopover(popover);
  isPopoverOpen.value = false;
  await popover.dismiss();
}

async function onDidDismissPopover(popover: any): Promise<void> {
  const { data } = await popover.onDidDismiss();
  if (data) {
    if (data.option !== selectedOption.value) {
      selectedOption.value = data.option;
    }
  }
}
</script>

<style lang="scss" scoped></style>
