<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-list class="list requests-container-list">
    <ion-list-header
      class="requests-list-header"
      lines="full"
      v-if="isLargeDisplay"
    >
      <ion-text class="requests-list-header__label cell-title label-name">
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.listDisplayTitles.name') }}
      </ion-text>
      <ion-text class="requests-list-header__label cell-title label-email">
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.listDisplayTitles.email') }}
      </ion-text>
      <ion-text class="requests-list-header__label cell-title label-createdOn">
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.listDisplayTitles.createdOn') }}
      </ion-text>
      <ion-text class="requests-list-header__label cell-title label-type">
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.listDisplayTitles.type') }}
      </ion-text>
      <ion-text class="requests-list-header__label cell-title label-space" />
    </ion-list-header>
    <async-enrollment-request-item
      v-for="request in requests"
      class="request"
      :key="request.enrollmentId"
      :request="request"
      :pki-available="pkiAvailable"
      :server-config="serverConfig"
      @accept-click="$emit('acceptClick', request)"
      @reject-click="$emit('rejectClick', request)"
    />
  </ion-list>
</template>

<script setup lang="ts">
import AsyncEnrollmentRequestItem from '@/components/invitations/AsyncEnrollmentRequestItem.vue';
import { AsyncEnrollmentUntrusted, ServerConfig } from '@/parsec';
import { IonList, IonListHeader, IonText } from '@ionic/vue';
import { useWindowSize } from 'megashark-lib';

const { isLargeDisplay } = useWindowSize();

defineProps<{
  requests: Array<AsyncEnrollmentUntrusted>;
  pkiAvailable: boolean;
  serverConfig?: ServerConfig;
}>();

defineEmits<{
  (e: 'acceptClick', invitation: AsyncEnrollmentUntrusted): void;
  (e: 'rejectClick', invitation: AsyncEnrollmentUntrusted): void;
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
