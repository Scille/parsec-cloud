<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    fill="clear"
    @click="openPopover($event)"
    id="select-filter-popover-button"
    class="filter-button button-small"
  >
    {{ $t('UsersPage.filter.title') }}
  </ion-button>
</template>

<script setup lang="ts">
import { UserCollection } from '@/components/users';
import UserFilterPopover from '@/components/users/UserFilterPopover.vue';
import { IonButton, popoverController } from '@ionic/vue';

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

<style lang="scss" scoped></style>
