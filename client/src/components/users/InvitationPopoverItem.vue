<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    class="invitation-list-item-container ion-no-padding"
    lines="full"
    :detail="false"
  >
    <div class="invitation-list-item">
      <!-- invitation email -->
      <div class="invitation-email">
        <ion-label class="cell invitation-label">
          {{ invitation.claimerEmail }}
        </ion-label>
      </div>

      <!-- invitation action -->
      <div class="invitation-actions">
        <div class="invitation-actions-date">
          <span class="default-state body-sm">{{ formatTimeSince(invitation.createdOn, '', 'short') }}</span>
          <ion-button
            fill="clear"
            class="hover-state copy-link"
            @click.stop="copyLink(invitation)"
          >
            {{ $t('UsersPage.invitation.copyLink') }}
          </ion-button>
        </div>

        <ion-buttons class="invitation-actions-buttons">
          <ion-button
            fill="clear"
            class="danger"
            @click.stop="$emit('cancel', invitation)"
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
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { writeTextToClipboard } from '@/common/clipboard';
import { formatTimeSince } from '@/common/date';
import { UserInvitation } from '@/parsec';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { translate } from '@/services/translation';
import { IonButton, IonButtons, IonItem, IonLabel } from '@ionic/vue';
import { inject } from 'vue';

defineProps<{
  invitation: UserInvitation;
}>();

defineEmits<{
  (e: 'cancel', invitation: UserInvitation): void;
  (e: 'greetUser', invitation: UserInvitation): void;
}>();

const informationManager: InformationManager = inject(InformationManagerKey)!;

async function copyLink(invitation: UserInvitation): Promise<void> {
  const result = await writeTextToClipboard(invitation.addr);
  if (result) {
    informationManager.present(
      new Information({
        message: translate('UsersPage.invitation.linkCopiedToClipboard'),
        level: InformationLevel.Info,
      }),
      PresentationMode.Toast,
    );
  } else {
    informationManager.present(
      new Information({
        message: translate('UsersPage.invitation.linkNotCopiedToClipboard'),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}
</script>

<style scoped lang="scss">
.invitation-list-item-container {
  --inner-padding-end: 0;
}
.invitation-list-item {
  padding: 1rem 1rem 1rem 1.75rem;
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
  gap: 0.75rem;

  .hover-state {
    display: none;
  }

  &:hover,
  &:focus {
    background: var(--parsec-color-light-primary-30);
    color: var(--parsec-color-light-secondary-text);

    .default-state {
      display: none;
    }

    .hover-state {
      display: block;
    }

    .invitation-actions-buttons {
      opacity: 1;
    }
  }
}

.invitation-email {
  width: 100%;
  white-space: nowrap;
  overflow: hidden;
  color: var(--parsec-color-light-secondary-text);
}

.invitation-actions {
  display: flex;
  align-items: center;
  width: 100%;

  &-date {
    color: var(--parsec-color-light-secondary-grey);
  }

  .copy-link {
    background: none;
    color: var(--parsec-color-light-secondary-text);
    --padding-end: 0;
    --padding-start: 0;
    position: relative;
    cursor: pointer;

    &::after {
      content: ' ';
      position: absolute;
      width: 0%;
      left: 0;
      bottom: 4px;
      height: 1px;
      background: var(--parsec-color-light-secondary-text);
      transition: width 150ms ease-in-out;
    }

    &:hover {
      --background-hover: none;
      color: var(--parsec-color-light-secondary-text);

      &::after {
        content: ' ';
        position: absolute;
        width: 100%;
      }
    }
  }

  &-buttons {
    opacity: 0;
    gap: 1rem;
    margin-left: auto;

    ion-button {
      width: 100%;
    }
  }
}
</style>
