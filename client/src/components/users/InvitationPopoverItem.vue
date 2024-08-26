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
          <span class="default-state body-sm">{{ $msTranslate(formatTimeSince(invitation.createdOn, '', 'short')) }}</span>
          <ion-button
            fill="clear"
            class="hover-state copy-link"
            @click.stop="copyLink(invitation)"
          >
            {{ $msTranslate('UsersPage.invitation.copyLink') }}
          </ion-button>
        </div>

        <ion-buttons class="invitation-actions-buttons">
          <ion-button
            fill="clear"
            class="danger"
            @click.stop="$emit('cancel', invitation)"
          >
            {{ $msTranslate('UsersPage.invitation.rejectUser') }}
          </ion-button>
          <ion-button
            class="button-default"
            fill="solid"
            @click.stop="$emit('greetUser', invitation)"
          >
            {{ $msTranslate('UsersPage.invitation.greetUser') }}
          </ion-button>
        </ion-buttons>
      </div>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { UserInvitation } from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { IonButton, IonButtons, IonItem, IonLabel } from '@ionic/vue';
import { formatTimeSince, Clipboard } from 'megashark-lib';

const props = defineProps<{
  invitation: UserInvitation;
  informationManager: InformationManager;
}>();

defineEmits<{
  (e: 'cancel', invitation: UserInvitation): void;
  (e: 'greetUser', invitation: UserInvitation): void;
}>();

async function copyLink(invitation: UserInvitation): Promise<void> {
  const result = await Clipboard.writeText(invitation.addr);
  if (result) {
    props.informationManager.present(
      new Information({
        message: 'UsersPage.invitation.linkCopiedToClipboard',
        level: InformationLevel.Info,
      }),
      PresentationMode.Toast,
    );
  } else {
    props.informationManager.present(
      new Information({
        message: 'UsersPage.invitation.linkNotCopiedToClipboard',
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
  display: flex;
  flex-shrink: 0;
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
  }
}

.invitation-email {
  width: 100%;
  overflow: hidden;
  color: var(--parsec-color-light-secondary-text);

  .invitation-label {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }
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
    gap: 1rem;
    margin-left: auto;

    ion-button {
      width: 100%;
    }
  }
}
</style>
