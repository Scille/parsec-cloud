<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-item
    class="invitation-list-item"
    lines="full"
    :detail="false"
  >
    <!-- invitation email -->
    <div class="invitation-email">
      <ion-label
        class="cell invitation-label"
      >
        {{ invitation.email }}
      </ion-label>
    </div>

    <!-- invitation date -->
    <div class="invitation-date">
      <ion-label
        class="cell invitation-label"
      >
        <span>{{ timeSince(invitation.date, '--', 'short') }}</span>
      </ion-label>
    </div>

    <!-- invitation status -->
    <div class="invitation-status">
      <ion-text class="cell">
        {{ t('UsersPage.invitation.waiting') }}
      </ion-text>
    </div>

    <!-- invitation action -->
    <div class="invitation-action">
      <ion-buttons class="invitation-action-buttons">
        <ion-button
          fill="clear"
          class="danger"
          @click.stop="$emit('reject-user', props.invitation)"
        >
          {{ t('UsersPage.invitation.rejectUser') }}
        </ion-button>
        <ion-button
          class="button-default"
          fill="solid"
          @click.stop="$emit('greet-user', props.invitation)"
        >
          {{ t('UsersPage.invitation.greetUser') }}
        </ion-button>
      </ion-buttons>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import {
  IonLabel,
  IonButtons,
  IonButton,
  IonItem,
  IonText,
} from '@ionic/vue';
import { MockInvitation } from '@/common/mocks';
import { FormattersKey, Formatters } from '@/common/injectionKeys';
import { useI18n } from 'vue-i18n';
import { defineProps, inject } from 'vue';

const { t } = useI18n();

const props = defineProps<{
  invitation: MockInvitation,
}>();

defineEmits<{
  (e: 'reject-user', invitation: MockInvitation) : void,
  (e: 'greet-user', invitation: MockInvitation) : void,
}>();

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const { timeSince } = inject(FormattersKey)! as Formatters;
</script>

<style scoped lang="scss">
.invitation-list-item>[class^="invitation-"] {
  padding: 0 1rem;
  display: flex;
  align-items: center;
  height: 4rem;
}

.invitation-list-item {
  align-items: center;
  border-radius: var(--parsec-radius-6);

  &::part(native) {
    display: flex;
    padding: 0;
    --inner-padding-end: 0px;
  }

  &:hover, &:focus {
    --background: var(--parsec-color-light-primary-30);
    color: var(--parsec-color-light-secondary-text);

    .invitation-action-buttons {
      opacity: 1;
    }
  }
}

.invitation-email {
  width: 100%;
  max-width: 30vw;
  white-space: nowrap;
  overflow: hidden;
  color: var(--parsec-color-light-secondary-text);
}

.invitation-date {
  width: 100%;
  max-width: 10vw;
  flex-grow: 0;
  color: var(--parsec-color-light-secondary-grey);
}

.invitation-status {
  width: 100%;
  max-width: 10vw;
  flex-grow: 0;
  color: var(--parsec-color-light-secondary-grey);
}

.invitation-action {
  width: 100%;

  &-buttons {
    opacity: 0;
    gap: 1rem;
    max-width: 17.25rem;
    width: 100%;

    ion-button {
      width: 100%;
    }
  }
}
</style>
