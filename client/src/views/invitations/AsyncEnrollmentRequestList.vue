<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-list class="list-container requests-list-container">
    <ion-list-header
      class="list-header requests-list-header"
      lines="full"
      v-if="isLargeDisplay"
    >
      <ion-text class="list-header-label cell-title label-name">
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.listDisplayTitles.name') }}
      </ion-text>
      <ion-text class="list-header-label cell-title label-email">
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.listDisplayTitles.email') }}
      </ion-text>
      <ion-text class="list-header-label cell-title label-createdOn">
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.listDisplayTitles.createdOn') }}
      </ion-text>
      <ion-text class="list-header-label cell-title label-type">
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.listDisplayTitles.type') }}
      </ion-text>
      <ion-text class="list-header-label list-item-end cell-title label-space" />
    </ion-list-header>
    <async-enrollment-request-item
      v-for="request in requests"
      class="request"
      :key="request.enrollmentId"
      :request="request"
      :pki-available="pkiAvailable"
      :server-config="serverConfig"
      @accept-click="$emit('acceptClick', request)"
      @reject-click="onRejectClick(request)"
    />
  </ion-list>
</template>

<script setup lang="ts">
import AsyncEnrollmentRequestItem from '@/components/invitations/AsyncEnrollmentRequestItem.vue';
import { AsyncEnrollmentUntrusted, ServerConfig } from '@/parsec';
import { IonList, IonListHeader, IonText } from '@ionic/vue';
import { Answer, askQuestion, useWindowSize } from 'megashark-lib';

const { isLargeDisplay } = useWindowSize();

defineProps<{
  requests: Array<AsyncEnrollmentUntrusted>;
  pkiAvailable: boolean;
  serverConfig?: ServerConfig;
}>();

async function onRejectClick(request: AsyncEnrollmentUntrusted): Promise<void> {
  const answer = await askQuestion(
    'InvitationsPage.asyncEnrollmentRequest.rejectModal.title',
    'InvitationsPage.asyncEnrollmentRequest.rejectModal.message',
    {
      yesText: 'InvitationsPage.asyncEnrollmentRequest.rejectModal.actions.confirm',
      noText: 'InvitationsPage.asyncEnrollmentRequest.rejectModal.actions.cancel',
      yesIsDangerous: true,
    },
  );
  if (answer === Answer.Yes) {
    emits('rejectClick', request);
  }
}

const emits = defineEmits<{
  (e: 'acceptClick', invitation: AsyncEnrollmentUntrusted): void;
  (e: 'rejectClick', invitation: AsyncEnrollmentUntrusted): void;
  (e: 'copyJoinLinkClick'): void;
}>();
</script>

<style scoped lang="scss">
.invalid-request {
  background: ms.color('surface-error-default-subtle');
  border: ms.border('thin') solid ms.color('border-error-default-subtle-hover');
  border-radius: ms.radius('2xl');
  margin: 1rem;
  padding: ms.spacing('padding-3xl');
  display: flex;
  max-width: 60rem;
  gap: ms.spacing('gap-3xl');
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
    gap: ms.spacing('gap-lg');

    @include ms.responsive-breakpoint('md') {
      padding-right: ms.spacing('padding-5xl');
    }

    @include ms.responsive-breakpoint('md') {
      padding-right: ms.spacing('padding-7xl');
    }

    &__description {
      color: ms.color('text-base-body');
    }

    &__time {
      color: ms.color('text-base-body');
      opacity: ms.opacity('7');
    }
  }

  &-title {
    color: ms.color('text-error-default');
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
      --background-hover: #{ms.color('surface-error-default-subtle-hover')};
      --background: none;
      padding: ms.spacing('padding-lg');
    }

    .button-icon {
      font-size: 1.25rem;
      color: ms.color('icon-error-default');
    }
  }
}
</style>
