<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    fill="clear"
    @click="openPopover($event)"
    id="select-filter-popover-button"
    class="filter-button button-small"
  >
    <ion-icon :icon="filter" />
    <span :class="{ 'missing-filters': missingFilters }">{{ $msTranslate('WorkspacesPage.filter.title') }}</span>
  </ion-button>
</template>

<script setup lang="ts">
import WorkspaceFilterPopover from '@/components/workspaces/WorkspaceFilterPopover.vue';
import { WorkspacesPageFilters } from '@/components/workspaces/utils';
import { IonButton, IonIcon, popoverController } from '@ionic/vue';
import { filter } from 'ionicons/icons';
import { computed } from 'vue';

const props = defineProps<{
  filters: WorkspacesPageFilters;
}>();

const emits = defineEmits<{
  (event: 'change'): void;
}>();

const missingFilters = computed(() => {
  return !props.filters.owner || !props.filters.manager || !props.filters.contributor || !props.filters.reader;
});

async function openPopover(event: Event): Promise<void> {
  const popover = await popoverController.create({
    component: WorkspaceFilterPopover,
    cssClass: 'filter-popover',
    componentProps: {
      filters: props.filters,
    },
    event: event,
    alignment: 'end',
    showBackdrop: false,
  });
  await popover.present();
  await popover.onDidDismiss();
  await popover.dismiss();
  emits('change');
}
</script>

<style lang="scss" scoped>
#select-filter-popover-button {
  position: relative;
  --background: transparent;
  --color: var(--parsec-color-light-secondary-text);
  --background-hover: transparent;
  border-radius: var(--parsec-radius-8);

  &::part(native) {
    padding: 0.375rem 0.625rem;
    overflow: visible;
  }

  &:hover {
    --background: var(--parsec-color-light-secondary-medium);
  }

  ion-icon {
    color: var(--parsec-color-light-secondary-grey);
    margin-right: 0.5rem;
  }

  &:hover ion-icon {
    color: var(--parsec-color-light-primary-700);
  }

  &:has(.missing-filters) {
    --background: var(--parsec-color-light-secondary-medium);
  }
}

.missing-filters::after {
  content: '';
  position: absolute;
  right: -11px;
  top: -7px;
  width: 0.625rem;
  height: 0.625rem;
  background: var(--parsec-color-light-primary-500);
  border-radius: var(--parsec-radius-8);
}
</style>
