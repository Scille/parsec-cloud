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
    <span :class="{ 'missing-filters': missingFilters }">{{ $msTranslate('UsersPage.filter.title') }}</span>
  </ion-button>
</template>

<script setup lang="ts">
import { UserFilterLabels } from '@/components/users';
import UserFilterPopover from '@/components/users/UserFilterPopover.vue';
import { IonButton, IonIcon, popoverController } from '@ionic/vue';
import { filter } from 'ionicons/icons';
import { computed } from 'vue';

const props = defineProps<{
  filters: UserFilterLabels;
}>();

const emits = defineEmits<{
  (event: 'update:filters', filters: UserFilterLabels): void;
}>();

const missingFilters = computed(() => {
  return (
    !props.filters.statusActive ||
    !props.filters.statusRevoked ||
    !props.filters.statusFrozen ||
    !props.filters.profileAdmin ||
    !props.filters.profileStandard ||
    !props.filters.profileOutsider
  );
});

async function openPopover(event: Event): Promise<void> {
  const popover = await popoverController.create({
    component: UserFilterPopover,
    cssClass: 'filter-popover',
    componentProps: {
      filters: props.filters,
      // Popovers are not created from a template, so we can't use `v-model`.
      'onUpdate:filters': (filters: UserFilterLabels): void => emits('update:filters', filters),
    },
    event: event,
    alignment: 'end',
    showBackdrop: false,
  });
  await popover.present();
  await popover.onDidDismiss();
  await popover.dismiss();
}
</script>

<style lang="scss" scoped></style>
