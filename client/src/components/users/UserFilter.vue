<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    fill="clear"
    @click="openPopover($event)"
    id="select-filter-popover-button"
    class="filter-button button-small"
    :class="{ 'missing-filters': missingFilters }"
  >
    <ion-icon :icon="filter" />
    {{ $msTranslate('UsersPage.filter.title') }}
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

<style lang="scss" scoped>
#select-filter-popover-button {
  --background: transparent;
  --background-hover: var(--parsec-color-light-secondary-medium);
  --color: var(--parsec-color-light-secondary-text);

  &::part(native) {
    padding: 0.375rem 0.625rem;
  }

  ion-icon {
    color: var(--parsec-color-light-secondary-grey);
    margin-right: 0.5rem;
  }

  &:hover ion-icon {
    color: var(--parsec-color-light-primary-700);
  }
}

.missing-filters::after {
  position: absolute;
  right: 28px;
  top: 5px;
  content: '';
  width: 6px;
  height: 6px;
  background: var(--parsec-color-light-primary-500);
}
</style>
