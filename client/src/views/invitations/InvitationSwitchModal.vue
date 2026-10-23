<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="invitation-switch-modal">
    <ion-radio-group
      v-model="invitationView"
      class="invitation-switch-content"
    >
      <ion-radio
        class="switch-item"
        :value="InvitationView.EmailInvitation"
        @click="$event.preventDefault()"
        :class="{ 'radio-checked': invitationView === InvitationView.EmailInvitation }"
      >
        <ion-icon
          :icon="invitationsCount > 0 ? mailUnread : mail"
          class="switch-item__icon"
        />
        <ion-text class="switch-item__label">{{ $msTranslate('InvitationsPage.emailInvitation.tab') }}</ion-text>
        <span
          class="switch-item__count"
          :class="{ empty: invitationsCount === 0 }"
        >
          {{ invitationsCount }}
        </span>

        <ion-icon
          v-if="invitationView === InvitationView.EmailInvitation"
          :icon="checkmarkCircle"
          class="switch-item__check-icon"
        />
      </ion-radio>

      <ion-radio
        class="switch-item"
        :value="InvitationView.AsyncEnrollmentRequest"
        @click="$event.preventDefault()"
        :class="{ 'radio-checked': invitationView === InvitationView.AsyncEnrollmentRequest }"
      >
        <ion-icon
          :icon="idCard"
          class="switch-item__icon"
        />
        <ion-text class="switch-item__label">{{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.tab') }}</ion-text>
        <span
          class="switch-item__count"
          :class="{ empty: asyncEnrollmentRequestsCount === 0 }"
        >
          {{ asyncEnrollmentRequestsCount }}
        </span>

        <ion-icon
          v-if="invitationView === InvitationView.AsyncEnrollmentRequest"
          :icon="checkmarkCircle"
          class="switch-item__check-icon"
        />
      </ion-radio>
    </ion-radio-group>

    <ion-button
      class="invitation-switch-button"
      size="default"
      fill="solid"
      @click="validate"
    >
      {{ $msTranslate('InvitationsPage.confirm') }}
    </ion-button>
  </div>
</template>

<script setup lang="ts">
import { InvitationView } from '@/views/invitations/types';
import { IonButton, IonIcon, IonRadio, IonRadioGroup, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle, idCard, mail, mailUnread } from 'ionicons/icons';
import { MsModalResult } from 'megashark-lib';
import { ref } from 'vue';

const props = defineProps<{
  invitationsCount: number;
  asyncEnrollmentRequestsCount: number;
  defaultView: InvitationView;
}>();

const invitationView = ref<InvitationView>(props.defaultView);

async function validate(): Promise<void> {
  modalController.dismiss(invitationView.value, MsModalResult.Confirm);
}
</script>

<style scoped lang="scss">
.invitation-switch-modal {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-5xl');
  align-items: center;
  justify-content: center;
  padding: ms.spacing('padding-5xl') ms.spacing('padding-3xl');
}

.invitation-switch-content {
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: ms.spacing('gap-3xl');

  .switch-item {
    gap: ms.spacing('gap-lg');
    font-size: var(--parsec-font-size-md);
    color: ms.color('text-neutral-default');
    padding: ms.spacing('padding-3xl');
    border: ms.border('thin') solid ms.color('border-base-default');
    border-radius: ms.radius('2xl');
    width: 100%;

    &::part(label) {
      padding: ms.spacing('padding-none');
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-lg');
      margin: 0;
      width: 100%;
    }

    &::part(container) {
      display: none;
    }

    &__label {
      @include ms.font('body-lg-medium');
    }

    &__icon {
      font-size: 1.25rem;
      color: ms.color('text-neutral-default');
    }

    &__count {
      @include ms.font('label-sm-medium');
      background: ms.color('surface-error-default');
      color: ms.color('text-on-color-label');
      border-radius: ms.radius('3xl');
      padding: ms.spacing('padding-xs') ms.spacing('padding-sm');
      min-width: 1.25rem;
      text-align: center;

      &.empty {
        background: ms.color('surface-neutral-default-subtle');
        color: ms.color('text-base-description');
      }
    }

    &__check-icon {
      font-size: 1.25rem;
      margin-left: auto;
      color: ms.color('text-brand-default');
    }

    &.radio-checked {
      border-color: ms.color('border-brand-default');
      background: ms.color('surface-brand-default-subtle');
      color: ms.color('text-brand-default');

      .switch-item__icon {
        color: ms.color('text-brand-default');
      }
    }
  }
}

.invitation-switch-button {
  @include ms.font('label-md-medium');
  width: 100%;
}
</style>
