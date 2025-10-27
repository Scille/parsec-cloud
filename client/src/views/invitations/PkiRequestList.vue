<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="list pkiRequests-container-list">
    <ion-list-header
      class="pkiRequests-list-header"
      lines="full"
      v-if="isLargeDisplay"
    >
      <ion-text class="pkiRequests-list-header__label cell-title label-name">
        {{ $msTranslate('InvitationsPage.pkiRequests.listDisplayTitles.name') }}
      </ion-text>
      <ion-text class="pkiRequests-list-header__label cell-title label-email">
        {{ $msTranslate('InvitationsPage.pkiRequests.listDisplayTitles.email') }}
      </ion-text>
      <ion-text class="pkiRequests-list-header__label cell-title label-createdOn">
        {{ $msTranslate('InvitationsPage.pkiRequests.listDisplayTitles.createdOn') }}
      </ion-text>
      <ion-text class="pkiRequests-list-header__label cell-title label-certificate">
        {{ $msTranslate('InvitationsPage.pkiRequests.listDisplayTitles.certificate') }}
      </ion-text>
      <ion-text class="pkiRequests-list-header__label cell-title label-space" />
    </ion-list-header>
    <pki-request-item
      v-for="request in requests"
      class="request"
      :key="request.certificate"
      :request="request"
      @accept-click="$emit('acceptClick', request)"
      @reject-click="$emit('rejectClick', request)"
    />
  </ion-list>
</template>

<script setup lang="ts">
import PkiRequestItem from '@/components/invitations/PkiRequestItem.vue';
import { OrganizationJoinRequest } from '@/parsec';
import { IonList, IonListHeader, IonText } from '@ionic/vue';
import { useWindowSize } from 'megashark-lib';

const { isLargeDisplay } = useWindowSize();

defineProps<{
  requests: Array<OrganizationJoinRequest>;
}>();

defineEmits<{
  (e: 'acceptClick', invitation: OrganizationJoinRequest): void;
  (e: 'rejectClick', invitation: OrganizationJoinRequest): void;
  (e: 'copyJoinLinkClick'): void;
}>();
</script>

<style scoped lang="scss"></style>
