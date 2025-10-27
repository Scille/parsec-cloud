<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    class="invitation-list-item-container ion-no-padding"
    lines="full"
    :detail="false"
  >
    <div class="invitation-list-item">
      <div class="invitation-header">
        <!-- invitation email -->
        <ion-label class="cell invitation-header__email">
          {{ invitation.claimerEmail }}
        </ion-label>
        <ion-button
          size="default"
          class="invitation-header__greet-button button-medium"
          @click.stop="$emit('greetUser', invitation)"
        >
          {{ $msTranslate('UsersPage.invitation.greetUser') }}
        </ion-button>
      </div>

      <!-- invitation action -->
      <div class="invitation-footer">
        <div class="invitation-footer-manage">
          <ion-text class="manage-text button-medium">{{ $msTranslate('UsersPage.invitation.manageInvitation') }}</ion-text>
          <ion-button
            fill="clear"
            class="manage-button copy-link"
            :class="copyLinkActive ? 'active' : ''"
            @click.stop="copyLink(invitation)"
            ref="copyLinkButton"
          >
            <ion-icon
              :icon="copyLinkActive ? checkmarkCircle : link"
              class="button-icon"
            />
          </ion-button>

          <ion-button
            fill="clear"
            class="manage-button send-email"
            :disabled="sendEmailDisabled"
            @click.stop="sendEmail(invitation)"
            :class="sendEmailDisabled ? 'active' : ''"
            ref="sendEmailButton"
          >
            <ion-icon
              :icon="mail"
              class="button-icon"
            />
          </ion-button>

          <ion-button
            size="default"
            fill="clear"
            class="manage-button cancel-button"
            @click.stop="$emit('cancel', invitation)"
          >
            <ion-icon
              :icon="trash"
              class="button-icon"
            />
          </ion-button>
        </div>
        <ion-text class="invitation-footer__date body-sm">{{ $msTranslate(formatTimeSince(invitation.createdOn, '', 'short')) }}</ion-text>
      </div>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { ClientNewUserInvitationErrorTag, InvitationEmailSentStatus, UserInvitation, inviteUser } from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { IonButton, IonIcon, IonItem, IonLabel, IonText } from '@ionic/vue';
import { checkmarkCircle, link, mail, trash } from 'ionicons/icons';
import { Answer, Clipboard, Translatable, askQuestion, attachMouseOverTooltip, formatTimeSince } from 'megashark-lib';
import { onMounted, ref, useTemplateRef } from 'vue';

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
const sendEmailButtonRef = useTemplateRef<InstanceType<typeof IonButton>>('sendEmailButton');
const copyLinkButtonRef = useTemplateRef<InstanceType<typeof IonButton>>('copyLinkButton');

onMounted(async () => {
  attachMouseOverTooltip(sendEmailButtonRef.value?.$el, 'UsersPage.invitation.sendEmail');
  attachMouseOverTooltip(copyLinkButtonRef.value?.$el, 'UsersPage.invitation.copyLink');
});

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
      message = 'UsersPage.invitation.sendEmailFailed';
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

.invitation-header {
  display: flex;
  align-items: center;
  width: 100%;
  justify-content: space-between;
  gap: 1rem;

  &__email {
    color: var(--parsec-color-light-secondary-text);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
}

.invitation-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  width: 100%;

  &-manage {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;

    .manage-text {
      color: var(--parsec-color-light-secondary-grey);
    }

    .manage-button {
      background: none;
      cursor: pointer;

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

      &:hover {
        &.cancel-button .button-icon {
          background: var(--parsec-color-light-danger-100);
          color: var(--parsec-color-light-danger-700);
        }
      }

      &.cancel-button {
        position: relative;
        display: flex;
        margin-left: 1rem;

        &::after {
          content: '';
          position: relative;
          display: block;
          left: -3.125rem;
          top: 0.5rem;
          width: 1px;
          height: 1.125rem;
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
  }

  &__date {
    color: var(--parsec-color-light-secondary-grey);
    flex-shrink: 0;
  }
}
</style>
