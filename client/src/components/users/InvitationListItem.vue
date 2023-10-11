<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

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
        {{ invitation.claimerEmail }}
      </ion-label>
    </div>

    <!-- invitation date -->
    <div class="invitation-date">
      <ion-label
        class="cell invitation-label"
      >
        <span>{{ timeSince(invitation.createdOn, '--', 'short') }}</span>
      </ion-label>
    </div>

    <!-- invitation status -->
    <div class="invitation-status">
      <ion-text class="cell">
        {{ $t('UsersPage.invitation.waiting') }}
      </ion-text>
    </div>

    <!-- invitation action -->
    <div class="invitation-action">
      <ion-buttons class="invitation-action-buttons">
        <ion-button
          fill="clear"
          @click.stop="copyLink(invitation)"
        >
          {{ $t('UsersPage.invitation.copyLink') }}
        </ion-button>

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
import { FormattersKey, Formatters } from '@/common/injectionKeys';
import { defineProps, inject } from 'vue';
import { UserInvitation } from '@/parsec';
import { Notification, NotificationCenter, NotificationKey, NotificationLevel } from '@/services/notificationCenter';
import { useI18n } from 'vue-i18n';

defineProps<{
  invitation: UserInvitation,
}>();

defineEmits<{
  (e: 'rejectUser', invitation: UserInvitation) : void,
  (e: 'greetUser', invitation: UserInvitation) : void,
}>();

// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const { timeSince } = inject(FormattersKey)! as Formatters;
// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const notificationCenter: NotificationCenter = inject(NotificationKey)!;
const { t } = useI18n();

async function copyLink(invitation: UserInvitation): Promise<void> {
  await navigator.clipboard.writeText(invitation.addr);
  notificationCenter.showToast(new Notification({
    message: t('UsersPage.invitation.linkCopiedToClipboard'),
    level: NotificationLevel.Info,
  }));
}
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
