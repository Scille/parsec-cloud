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
import { UserCollection } from '@/components/users';
import UserFilterPopover from '@/components/users/UserFilterPopover.vue';
import { IonButton, IonIcon, popoverController } from '@ionic/vue';
import { filter } from 'ionicons/icons';
import { computed } from 'vue';

const props = defineProps<{
  users: UserCollection;
}>();

const emits = defineEmits<{
  (event: 'change'): void;
}>();

const missingFilters = computed(() => {
  const filters = props.users.getFilters();
  return !filters.profileAdmin || !filters.profileOutsider || !filters.profileStandard || !filters.statusActive || !filters.statusRevoked;
});

async function openPopover(event: Event): Promise<void> {
  const popover = await popoverController.create({
    component: UserFilterPopover,
    cssClass: 'filter-popover',
    componentProps: {
      users: props.users,
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
