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
        <ion-text class="invitation-date body-sm">{{ $msTranslate(formatTimeSince(invitation.createdOn, '', 'short')) }}</ion-text>
      </div>

      <!-- invitation action -->
      <div class="invitation-actions">
        <ion-button
          fill="clear"
          class="copy-link"
          @click.stop="copyLink(invitation)"
          :class="copyLinkActive ? 'active' : ''"
          :title="I18n.translate('UsersPage.invitation.copyLink')"
        >
          <ion-icon
            :icon="copyLinkActive ? checkmarkCircle : link"
            class="button-icon"
          />
        </ion-button>

        <ion-button
          fill="clear"
          class="send-email"
          :disabled="sendEmailDisabled"
          @click.stop="sendEmail(invitation)"
          :class="sendEmailDisabled ? 'active' : ''"
          :title="I18n.translate('UsersPage.invitation.sendEmail')"
        >
          <ion-icon
            :icon="mail"
            class="button-icon"
          />
        </ion-button>

        <ion-buttons class="invitation-actions-buttons">
          <ion-button
            size="default"
            fill="clear"
            class="danger button-medium"
            @click.stop="$emit('cancel', invitation)"
          >
            {{ $msTranslate('UsersPage.invitation.rejectUser') }}
          </ion-button>
          <ion-button
            size="default"
            fill="default"
            class="button-medium"
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
import { ClientNewUserInvitationErrorTag, InvitationEmailSentStatus, inviteUser, UserInvitation } from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { IonIcon, IonButton, IonButtons, IonItem, IonLabel, IonText } from '@ionic/vue';
import { formatTimeSince, Clipboard, I18n, askQuestion, Answer, Translatable } from 'megashark-lib';
import { checkmarkCircle, link, mail } from 'ionicons/icons';
import { ref } from 'vue';

const props = defineProps<{
  invitation: UserInvitation;
  informationManager: InformationManager;
}>();

const copyLinkActive = ref(false);

defineEmits<{
  (e: 'cancel', invitation: UserInvitation): void;
  (e: 'greetUser', invitation: UserInvitation): void;
}>();

const sendEmailDisabled = ref(false);

async function copyLink(invitation: UserInvitation): Promise<void> {
  const result = await Clipboard.writeText(invitation.addr);
  copyLinkActive.value = true;
  setTimeout(() => {
    copyLinkActive.value = false;
  }, 5000);
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

async function sendEmail(invitation: UserInvitation): Promise<void> {
  sendEmailDisabled.value = true;
  const answer = await askQuestion('UsersPage.invitation.sendEmailTitle', 'UsersPage.invitation.sendEmailMessage', {
    yesText: 'UsersPage.invitation.sendEmail',
  });

  if (answer === Answer.No) {
    sendEmailDisabled.value = false;
    return;
  }
  const result = await inviteUser(invitation.claimerEmail);
  if (result.ok && result.value.emailSentStatus === InvitationEmailSentStatus.Success) {
    props.informationManager.present(
      new Information({
        message: {
          key: 'UsersPage.invitation.inviteSuccessMailSent',
          data: {
            email: invitation.claimerEmail,
          },
        },
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    setTimeout(() => {
      sendEmailDisabled.value = false;
    }, 5000);
  } else {
    let message: Translatable = '';
    if (result.ok) {
      message = 'UsersPage.invitation.inviteSuccessNoMail';
    } else {
      switch (result.error.tag) {
        case ClientNewUserInvitationErrorTag.Offline:
          message = 'UsersPage.invitation.inviteFailedOffline';
          break;
        case ClientNewUserInvitationErrorTag.NotAllowed:
          message = 'UsersPage.invitation.inviteFailedNotAllowed';
          break;
        default:
          message = {
            key: 'UsersPage.invitation.inviteFailedUnknown',
            data: {
              reason: result.error.tag,
            },
          };
          break;
      }
    }
    props.informationManager.present(
      new Information({
        message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    setTimeout(() => {
      sendEmailDisabled.value = false;
    }, 2000);
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

  &:hover,
  &:focus {
    color: var(--parsec-color-light-secondary-text);
  }
}

.invitation-email {
  width: 100%;
  overflow: hidden;
  display: flex;
  justify-content: space-between;
  align-items: center;
  color: var(--parsec-color-light-secondary-text);

  .invitation-label {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .invitation-date {
    color: var(--parsec-color-light-secondary-grey);
    padding: 0 0.5rem;
  }
}

.invitation-actions {
  display: flex;
  align-items: center;
  width: 100%;

  .copy-link,
  .send-email {
    background: none;
    cursor: pointer;
    margin-right: 1rem;

    &::part(native) {
      padding: 0;
    }

    .button-icon {
      font-size: 1.25rem;
      padding: 0.5rem;
      background: var(--parsec-color-light-secondary-premiere);
      border-radius: var(--parsec-radius-8);
      color: var(--parsec-color-light-secondary-soft-text);
      transition: all 0.15s ease-in-out;
    }
    &:hover {
      .button-icon {
        color: var(--parsec-color-light-secondary-text);
        background: var(--parsec-color-light-secondary-medium);
      }
    }

    &.active {
      .button-icon {
        color: var(--parsec-color-light-success-700);
        background: var(--parsec-color-light-success-50);
      }
    }
  }

  &-buttons {
    gap: 1rem;
    margin-left: auto;

    ion-button {
      width: 100%;

      &:last-child {
        color: var(--parsec-color-light-secondary-white);
      }
    }
  }
}
</style>
