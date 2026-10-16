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
        <ion-label class="invitation-header__email">
          {{ invitation.claimerEmail }}
        </ion-label>
        <ion-button
          class="invitation-header__greet-button"
          @click.stop="$emit('greetUser', invitation)"
        >
          {{ $msTranslate('UsersPage.invitation.greetUser') }}
        </ion-button>
      </div>

      <!-- invitation action -->
      <div class="invitation-footer">
        <div class="invitation-footer-manage">
          <ion-button
            slot="icon-only"
            fill="outline"
            size="small"
            class="manage-button color-secondary copy-link"
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
            slot="icon-only"
            fill="outline"
            size="small"
            class="manage-button color-secondary send-email"
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
            slot="icon-only"
            fill="clear"
            size="small"
            class="manage-button color-danger cancel-button"
            @click.stop="$emit('cancel', invitation)"
          >
            <ion-icon
              :icon="trash"
              class="button-icon"
            />
          </ion-button>
        </div>
        <ion-text class="invitation-footer__date">{{ $msTranslate(formatTimeSince(invitation.createdOn, '', 'short')) }}</ion-text>
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
  const [_, invitationAddrAsHttpRedirection] = invitation.addr;
  const result = await Clipboard.writeText(invitationAddrAsHttpRedirection);
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
  --inner-padding-end: #{ms.spacing('padding-none')};
  display: flex;
  flex-shrink: 0;
}

.invitation-list-item {
  padding: ms.spacing('padding-xl') ms.spacing('padding-3xl');
  display: flex;
  flex-direction: column;
  align-items: center;
  width: 100%;
  gap: ms.spacing('gap-xl');
  border-bottom: 1px solid ms.color('border-neutral-default-subtle');

  &:hover,
  &:focus {
    color: ms.color('text-base-body');
  }
}

.invitation-header {
  display: flex;
  align-items: center;
  width: 100%;
  justify-content: space-between;
  gap: ms.spacing('gap-3xl');

  &__email {
    @include ms.font('label-md-medium');
    color: ms.color('text-base-body');
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
    gap: ms.spacing('gap-2xl');
    width: 100%;

    .manage-button {
      &.color-danger {
        position: relative;
        display: flex;
        margin-left: 1rem;
        --background: #{ms.color('surface-error-default-subtle')};
        --background-hover: #{ms.color('surface-error-default-subtle-hover')};
        color: ms.color('icon-error-default-hover');

        &::part(native) {
          border: 1px solid ms.color('border-error-default-subtle');
        }

        &::after {
          content: '';
          position: relative;
          display: block;
          left: -2.875rem;
          top: 0.5rem;
          width: 1px;
          height: 1.125rem;
          background: ms.color('border-neutral-default-subtle');
        }
      }

      &.active {
        .button-icon {
          color: ms.color('icon-success-default-hover');
          background: ms.color('surface-success-default-subtle');
        }
      }
    }
  }

  &__date {
    @include ms.font('body-sm-regular');
    color: ms.color('text-base-description');
    flex-shrink: 0;
  }
}
</style>
