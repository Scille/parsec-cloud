<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="list invitations-container-list">
    <ion-list-header
      class="invitations-list-header"
      lines="full"
      v-if="isLargeDisplay"
    >
      <ion-text class="invitations-list-header__label cell-title label-email">
        {{ $msTranslate('InvitationsPage.emailInvitation.listDisplayTitles.email') }}
      </ion-text>
      <ion-text class="invitations-list-header__label cell-title label-sentOn">
        {{ $msTranslate('InvitationsPage.emailInvitation.listDisplayTitles.sentOn') }}
      </ion-text>
      <ion-text class="invitations-list-header__label cell-title label-space" />
    </ion-list-header>
    <invitation-item
      v-for="invitation in invitations"
      class="request"
      :key="invitation.token"
      :invitation="invitation"
      @greet-click="$emit('greetClick', invitation)"
      @copy-link-click="$emit('copyLinkClick', invitation)"
      @send-email-click="$emit('sendEmailClick', invitation)"
      @delete-click="$emit('deleteClick', invitation)"
    />
  </ion-list>
</template>

<script setup lang="ts">
import InvitationItem from '@/components/invitations/InvitationItem.vue';
import { UserInvitation } from '@/parsec';
import { IonList, IonListHeader, IonText } from '@ionic/vue';
import { useWindowSize } from 'megashark-lib';

defineProps<{
  invitations: Array<UserInvitation>;
}>();

const { isLargeDisplay } = useWindowSize();

defineEmits<{
  (e: 'greetClick', invitation: UserInvitation): void;
  (e: 'copyLinkClick', invitation: UserInvitation): void;
  (e: 'sendEmailClick', invitation: UserInvitation): void;
  (e: 'deleteClick', invitation: UserInvitation): void;
}>();
</script>

<style scoped lang="scss"></style>
