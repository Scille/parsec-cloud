<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item class="invitation-card-item">
    <div class="invitation-card-item__waiting">
      <ion-icon :icon="time" />
      <ion-text class="caption-caption">
        {{ translateInvitationStatus($t, $props.invitation.status) }}
      </ion-text>
    </div>
    <ion-label class="body invitation-card-item__label">
      {{ invitation.claimerEmail }}
    </ion-label>
    <ion-buttons class="invitation-card-item__buttons">
      <ion-button
        fill="clear"
        class="danger"
        @click.stop="$emit('rejectUser', invitation)"
      >
        {{ $t('UsersPage.invitation.rejectUser') }}
      </ion-button>
      <ion-button
        class="button-default"
        fill="solid"
        @click.stop="$emit('greetUser', invitation)"
      >
        {{ $t('UsersPage.invitation.greetUser') }}
      </ion-button>
    </ion-buttons>
  </ion-item>
</template>

<script setup lang="ts">
import { translateInvitationStatus } from '@/common/translations';
import { UserInvitation } from '@/parsec';
import { IonButton, IonButtons, IonIcon, IonItem, IonLabel, IonText } from '@ionic/vue';
import { time } from 'ionicons/icons';
import { defineProps } from 'vue';

defineProps<{
  invitation: UserInvitation;
}>();

defineEmits<{
  (e: 'rejectUser', invitation: UserInvitation): void;
  (e: 'greetUser', invitation: UserInvitation): void;
}>();
</script>

<style scoped lang="scss">
.invitation-card-item {
  width: 20rem;
  padding: 1rem;
  border: var(--parsec-color-light-secondary-disabled) 1px solid;
  --background: var(--parsec-color-light-secondary-background);
  background: var(--parsec-color-light-secondary-background);
  border-radius: var(--parsec-radius-6);
  position: relative;
  z-index: 2;

  &__waiting {
    display: flex;
    justify-content: end;
    width: 100%;
    align-items: center;
    gap: 0.5rem;
    padding: 0.2rem;
    color: var(--parsec-color-light-secondary-grey);
    font-style: italic;
  }

  &__label {
    color: var(--parsec-color-light-secondary-text);
    margin: 1rem 0;
    width: 100%;
  }

  &__buttons {
    display: flex;
    gap: 0.5rem;
    margin-left: auto;
    width: 100%;

    ion-button {
      width: 100%;
    }
  }
}
</style>
