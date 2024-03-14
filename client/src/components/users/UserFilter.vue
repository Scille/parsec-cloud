<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    fill="clear"
    @click="openPopover($event)"
    id="select-filter-popover-button"
    class="filter-button button-small"
  >
    <ion-icon :icon="filter" />
    {{ $t('UsersPage.filter.title') }}
  </ion-button>
</template>

<script setup lang="ts">
import { UserCollection } from '@/components/users';
import UserFilterPopover from '@/components/users/UserFilterPopover.vue';
import { IonButton, IonIcon, popoverController } from '@ionic/vue';
import { filter } from 'ionicons/icons';

const props = defineProps<{
  users: UserCollection;
}>();

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
}
</script>

<style lang="scss" scoped>
#select-filter-popover-button {
  --background: var(--parsec-color-light-secondary-white);
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
</style>
