<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    class="file-controls-dropdown button-medium"
    size="default"
    @click="openPopover($event)"
  >
    <span
      class="file-controls-dropdown-label"
      v-if="title"
    >
      {{ $msTranslate(title) }}
    </span>
    <ion-icon
      class="file-controls-dropdown-icon"
      :class="{ 'popover-is-open': isPopoverOpen }"
      v-if="icon"
      :icon="icon"
    />
    <ms-image
      class="file-controls-dropdown-icon"
      :class="{ 'popover-is-open': isPopoverOpen }"
      v-if="!icon && image"
      :image="image"
    />
  </ion-button>
</template>

<script setup lang="ts">
import { FileControlsDropdownPopover, FileControlsDropdownItemContent } from '@/components/viewers';
import { Ref, ref } from 'vue';
import { IonButton, IonIcon, popoverController } from '@ionic/vue';
import { MsImage, Translatable } from 'megashark-lib';

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

<style lang="scss" scoped>
ion-icon {
  transition: transform ease-out 300ms;
  font-size: 1.125rem;

  &.popover-is-open {
    transform: rotate(180deg);
  }
}

.file-controls-dropdown {
  --background: none !important;
  --background-hover: none !important;
  margin-inline: 0px;
  margin-top: 0px;
  margin-bottom: 0px;
  --padding-top: 0;
  --padding-end: 0;
  --padding-bottom: 0;
  --padding-start: 0;
  height: 3em;
  width: fit-content;
  border-radius: 100%;
  color: var(--parsec-color-light-primary-600);
  opacity: 0.6;
  scale: 1;
  transition: all 0.2s ease-in-out;

  &.button-disabled {
    opacity: 0.3;
  }

  &:hover {
    scale: 1.1;
    opacity: 1;
  }

  &-icon,
  &-label {
    margin-inline: 0.625rem;
  }
}
</style>
