<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->
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
          :icon="mailUnread"
          class="switch-item__icon"
        />
        <ion-text class="subtitles-normal">{{ $msTranslate('InvitationsPage.emailInvitation.tab') }}</ion-text>
        <span class="switch-item__count button-small">{{ invitationsCount }}</span>

        <ion-icon
          v-if="invitationView === InvitationView.EmailInvitation"
          :icon="checkmarkCircle"
          class="switch-item__check-icon"
        />
      </ion-radio>

      <ion-radio
        class="switch-item"
        :value="InvitationView.PkiRequest"
        @click="$event.preventDefault()"
        :class="{ 'radio-checked': invitationView === InvitationView.PkiRequest }"
      >
        <ion-icon
          :icon="idCard"
          class="switch-item__icon"
        />
        <ion-text class="subtitles-normal">{{ $msTranslate('InvitationsPage.pkiRequests.tab') }}</ion-text>
        <span class="switch-item__count button-small">{{ pkiRequestsCount }}</span>

        <ion-icon
          v-if="invitationView === InvitationView.PkiRequest"
          :icon="checkmarkCircle"
          class="switch-item__check-icon"
        />
      </ion-radio>
    </ion-radio-group>

    <ion-button
      class="invitation-switch-button button-medium button-default"
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
import { checkmarkCircle, idCard, mailUnread } from 'ionicons/icons';
import { MsModalResult } from 'megashark-lib';
import { ref } from 'vue';

const props = defineProps<{
  invitationsCount: number;
  pkiRequestsCount: number;
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
  gap: 2rem;
  align-items: center;
  justify-content: center;
  padding: 2rem 1rem;
}

.invitation-switch-content {
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: 1rem;

  .switch-item {
    gap: 0.5rem;
    font-size: var(--parsec-font-size-md);
    color: var(--parsec-color-light-secondary-soft-text);
    padding: 1rem;
    border: 1px solid var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-12);
    width: 100%;
    margin-bottom: 1rem;

    &::part(label) {
      padding: 0;
      display: flex;
      align-items: center;
      gap: 0.5rem;
      margin: 0;
      width: 100%;
    }

    &::part(container) {
      display: none;
    }

    &__icon {
      font-size: 1.25rem;
      color: var(--parsec-color-light-secondary-soft-text);
    }

    &__count {
      background: var(--parsec-color-light-secondary-medium);
      color: var(--parsec-color-light-secondary-text);
      border-radius: 1rem;
      padding: 0.125rem 0.25rem;
      min-width: 1.25rem;
      text-align: center;
    }

    &__check-icon {
      font-size: 1.25rem;
      margin-left: auto;
      color: var(--parsec-color-light-primary-600);
    }

    &.radio-checked {
      border-color: var(--parsec-color-light-primary-600);
      background: var(--parsec-color-light-primary-50);
      color: var(--parsec-color-light-primary-600);

      .switch-item__icon {
        color: var(--parsec-color-light-primary-600);
      }

      .switch-item__count {
        background: var(--parsec-color-light-primary-100);
        color: var(--parsec-color-light-primary-600);
      }
    }
  }
}

.invitation-switch-button {
  width: 100%;
}
</style>
