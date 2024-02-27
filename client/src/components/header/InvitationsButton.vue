<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    v-show="invitations.length > 0"
    @click="openInvitationsPopover($event)"
  >
    {{ $t('HeaderPage.invitations.title', { count: invitations.length }, invitations.length) }}
    <ion-icon :icon="mailUnread" />
  </ion-button>
</template>

<script setup lang="ts">
import { emailValidator } from '@/common/validators';
import { Answer, MsModalResult, askQuestion, getTextInputFromUser } from '@/components/core';
import {
  ClientCancelInvitationErrorTag,
  ClientNewUserInvitationErrorTag,
  InvitationEmailSentStatus,
  UserInvitation,
  cancelInvitation,
  listUserInvitations,
  inviteUser as parsecInviteUser,
} from '@/parsec';
import { Information, InformationKey, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { translate } from '@/services/translation';
import GreetUserModal from '@/views/users/GreetUserModal.vue';
import InvitationsListPopover from '@/views/users/InvitationsListPopover.vue';
import { IonButton, IonIcon, modalController, popoverController } from '@ionic/vue';
import { mailUnread } from 'ionicons/icons';
import { Ref, inject, onMounted, ref } from 'vue';

const informationManager: InformationManager = inject(InformationKey)!;
const invitations: Ref<UserInvitation[]> = ref([]);

onMounted(async () => {
  await updateInvitations();
});

async function updateInvitations(): Promise<void> {
  const result = await listUserInvitations();
  if (result.ok) {
    invitations.value = result.value;
  }
}

async function openInvitationsPopover(event: Event): Promise<void> {
  event.stopPropagation();
  const popover = await popoverController.create({
    component: InvitationsListPopover,
    alignment: 'center',
    event: event,
    cssClass: 'invitations-list-popover',
    showBackdrop: false,
  });
  await popover.present();
  const { role, data } = await popover.onDidDismiss();
  await popover.dismiss();
  if (role === MsModalResult.Confirm && data.action) {
    if (data.action === 'greet') {
      await greetUser(data.invitation);
    } else if (data.action === 'cancel') {
      await cancelUserInvitation(data.invitation);
    } else if (data.action === 'invite') {
      await inviteUser();
    }
  }
}

async function cancelUserInvitation(invitation: UserInvitation): Promise<void> {
  const answer = await askQuestion(
    translate('UsersPage.invitation.cancelInvitation.title'),
    translate('UsersPage.invitation.cancelInvitation.message', { email: invitation.claimerEmail }),
    {
      yesText: translate('UsersPage.invitation.cancelInvitation.yes'),
      noText: translate('UsersPage.invitation.cancelInvitation.no'),
      yesIsDangerous: true,
    },
  );

  if (answer === Answer.No) {
    return;
  }

  const result = await cancelInvitation(invitation.token);

  if (result.ok) {
    informationManager.present(
      new Information({
        message: translate('UsersPage.invitation.cancelSuccess'),
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    await updateInvitations();
  } else {
    // In both those cases we can just refresh the list and the invitation should disappear, no need
    // to warn the user.
    if (
      result.error.tag === ClientCancelInvitationErrorTag.NotFound ||
      result.error.tag === ClientCancelInvitationErrorTag.AlreadyDeleted
    ) {
      await updateInvitations();
    } else {
      informationManager.present(
        new Information({
          message: translate('UsersPage.invitation.cancelFailed'),
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  }
}

async function greetUser(invitation: UserInvitation): Promise<void> {
  const modal = await modalController.create({
    component: GreetUserModal,
    canDismiss: true,
    backdropDismiss: false,
    cssClass: 'greet-organization-modal',
    componentProps: {
      invitation: invitation,
    },
  });
  await modal.present();
  await modal.onWillDismiss();
  await modal.dismiss();
  await updateInvitations();
}

async function inviteUser(): Promise<void> {
  const email = await getTextInputFromUser({
    title: translate('UsersPage.CreateUserInvitationModal.pageTitle'),
    trim: true,
    validator: emailValidator,
    inputLabel: translate('UsersPage.CreateUserInvitationModal.label'),
    placeholder: translate('UsersPage.CreateUserInvitationModal.placeholder'),
    okButtonText: translate('UsersPage.CreateUserInvitationModal.create'),
  });

  if (!email) {
    return;
  }

  const result = await parsecInviteUser(email);

  if (result.ok) {
    await updateInvitations();
    if (result.value.emailSentStatus === InvitationEmailSentStatus.Success) {
      informationManager.present(
        new Information({
          message: translate('UsersPage.invitation.inviteSuccessMailSent', {
            email: email,
          }),
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    } else {
      informationManager.present(
        new Information({
          message: translate('UsersPage.invitation.inviteSuccessNoMail', {
            email: email,
          }),
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    let message = '';
    switch (result.error.tag) {
      case ClientNewUserInvitationErrorTag.AlreadyMember:
        message = translate('UsersPage.invitation.inviteFailedAlreadyMember');
        break;
      case ClientNewUserInvitationErrorTag.Offline:
        message = translate('UsersPage.invitation.inviteFailedOffline');
        break;
      case ClientNewUserInvitationErrorTag.NotAllowed:
        message = translate('UsersPage.invitation.inviteFailedNotAllowed');
        break;
      default:
        message = translate('UsersPage.invitation.inviteFailedUnknown', {
          reason: result.error.tag,
        });
        break;
    }

    informationManager.present(
      new Information({
        message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}
</script>

<style scoped lang="scss"></style>
