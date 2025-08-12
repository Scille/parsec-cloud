<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    fill="clear"
    @click="openPopover($event)"
    id="select-filter-popover-button"
    class="filter-button button-small"
  >
    <ion-icon
      :icon="filter"
      class="filter-button__icon"
    />
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

<style lang="scss" scoped></style>
