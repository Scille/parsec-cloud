<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    v-for="request in requests"
    :key="request.enrollmentId"
  >
    <div v-if="request.tag === PkiEnrollmentListItemTag.Valid">
      {{ (request as PkiEnrollmentListItemValid).payload.humanHandle.label }}
      {{ (request as PkiEnrollmentListItemValid).payload.humanHandle.email }}
      {{ $msTranslate(formatTimeSince(request.submittedOn, '--', 'short')) }}
      <ion-button @click="$emit('rejectClick', request)">{{ 'REJECT' }}</ion-button>
      <ion-button @click="$emit('acceptClick', request)">{{ 'ACCEPT' }}</ion-button>
    </div>
    <div v-if="request.tag === PkiEnrollmentListItemTag.Invalid">
      {{ 'NOP' }}
      {{ $msTranslate(formatTimeSince(request.submittedOn, '--', 'short')) }}
      <ion-button @click="$emit('rejectClick', request)">{{ 'REJECT' }}</ion-button>
    </div>
  </div>

  <ion-list
    class="list pkiRequests-container-list"
    v-if="false"
  >
    <ion-list-header
      class="pkiRequests-list-header"
      lines="full"
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
      :key="request.enrollmentId"
      :request="request"
      @accept-click="$emit('acceptClick', request)"
      @reject-click="$emit('rejectClick', request)"
    />
  </ion-list>
</template>

<script setup lang="ts">
import PkiRequestItem from '@/components/invitations/PkiRequestItem.vue';
import { PkiEnrollmentListItem, PkiEnrollmentListItemTag, PkiEnrollmentListItemValid } from '@/parsec';
import { IonButton, IonList, IonListHeader, IonText } from '@ionic/vue';
import { formatTimeSince, useWindowSize } from 'megashark-lib';

const { isLargeDisplay } = useWindowSize();

defineProps<{
  requests: Array<PkiEnrollmentListItem>;
}>();

defineEmits<{
  (e: 'acceptClick', invitation: PkiEnrollmentListItem): void;
  (e: 'rejectClick', invitation: PkiEnrollmentListItem): void;
  (e: 'copyJoinLinkClick'): void;
}>();
</script>

<style scoped lang="scss"></style>
