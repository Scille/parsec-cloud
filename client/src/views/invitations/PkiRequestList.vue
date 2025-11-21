<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="list pkiRequests-container-list">
    <div
      v-for="request in requests.filter((req) => req.tag === PkiEnrollmentListItemTag.Invalid)"
      :key="request.enrollmentId"
      class="invalid-request"
    >
      <ion-text class="invalid-request-title title-h4">
        {{ $msTranslate('InvitationsPage.pkiRequests.invalidRequest') }}
      </ion-text>
      <div class="invalid-request-text">
        <ion-text class="invalid-request-text__description subtitles-normal">
          {{ 'Il aurait pu ranger sa chambre avant de faire sa demande' }}
        </ion-text>
        <ion-text class="invalid-request-text__time body">
          {{ $msTranslate(formatTimeSince(request.submittedOn, '--', 'short')) }}
        </ion-text>
      </div>
      <ion-button
        @click="$emit('rejectClick', request)"
        class="invalid-request__button button-medium button-default"
      >
        <ion-icon
          :icon="closeCircle"
          class="button-icon"
        />
      </ion-button>
    </div>
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
      v-for="request in requests.filter((req) => req.tag === PkiEnrollmentListItemTag.Valid)"
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
import { PkiEnrollmentListItem, PkiEnrollmentListItemTag } from '@/parsec';
import { IonButton, IonIcon, IonList, IonListHeader, IonText } from '@ionic/vue';
import { closeCircle } from 'ionicons/icons';
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

<style scoped lang="scss">
.invalid-request {
  background: var(--parsec-color-light-danger-50);
  border: 1px solid var(--parsec-color-light-danger-100);
  border-radius: var(--parsec-radius-12);
  margin: 1rem;
  padding: 1rem;
  display: flex;
  max-width: 60rem;
  gap: 1rem;
  align-items: center;
  position: relative;

  @include ms.responsive-breakpoint('md') {
    flex-direction: column;
    align-items: start;
  }

  @include ms.responsive-breakpoint('sm') {
    margin: 0.75rem 0;
  }

  &-text {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;

    @include ms.responsive-breakpoint('md') {
      padding-right: 2rem;
    }

    @include ms.responsive-breakpoint('md') {
      padding-right: 3rem;
    }

    &__description {
      color: var(--parsec-color-light-secondary-text);
    }

    &__time {
      color: var(--parsec-color-light-secondary-text);
      opacity: 0.7;
    }
  }

  &-title {
    color: var(--parsec-color-light-danger-700);
    font-weight: 600;
  }

  &__button {
    margin-left: auto;

    @include ms.responsive-breakpoint('md') {
      position: absolute;
      top: 50%;
      right: 0;
      transform: translate(-50%, -50%);
      height: fit-content;
    }

    &::part(native) {
      --background-hover: var(--parsec-color-light-danger-100);
      --background: none;
      padding: 0.5rem;
    }

    .button-icon {
      font-size: 1.25rem;
      color: var(--parsec-color-light-danger-500);
    }
  }
}
</style>
