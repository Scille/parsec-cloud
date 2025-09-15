<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="invitations-page">
    <ion-content class="content-scroll">
      <ion-button
        @click="view = Views.EmailInvitations"
        :disabled="view === Views.EmailInvitations"
      >
        EMAIL {{ invitations.length }}
      </ion-button>
      <ion-button
        @click="view = Views.PkiRequests"
        :disabled="view === Views.PkiRequests"
      >
        PKI {{ pkIRequests.length }}
      </ion-button>

      <div
        class="invitations-container scroll"
        v-if="view === Views.EmailInvitations"
      >
        <div
          v-show="invitations.length === 0"
          class="no-active body-lg"
        >
          <div class="no-active-content">
            <ms-image :image="NoActiveUser" />
            <ion-text>
              {{ $msTranslate('invitationsPage.emptyList') }}
            </ion-text>
          </div>
        </div>
        <div v-show="invitations.length > 0">
          <invitation-list
            :invitations="invitations"
            @greet-click="onInvitationGreetClicked"
            @send-email-click="onSendInvitationEmailClicked"
            @copy-link-click="onCopyInvitationLinkClicked"
            @delete-click="onDeleteInvitationClicked"
          />
        </div>
      </div>
      <div v-if="view === Views.PkiRequests">
        <ion-button @click="selectRootCertificate">
          <span v-show="rootCertificate">UPDATE ROOT CERTIFICATE</span>
          <span v-show="!rootCertificate">SELECT ROOT CERTIFICATE</span>
        </ion-button>
        <input
          type="file"
          ref="input"
          hidden
          @change="onInputChange"
        />
      </div>
      <div
        class="invitations-container scroll"
        v-if="view === Views.PkiRequests"
      >
        <div
          v-show="invitations.length === 0"
          class="no-active body-lg"
        >
          <div class="no-active-content">
            <ms-image :image="NoActiveUser" />
            <ion-text>
              {{ $msTranslate('invitationsPage.emptyList') }}
            </ion-text>
          </div>
        </div>
        <div v-show="invitations.length > 0">
          <pki-request-list
            :requests="pkIRequests"
            @accept-click="onAcceptPkiRequestClicked"
            @reject-click="onRejectPkiRequestClicked"
            @copy-join-link-click="onCopyJoinLinkClicked"
          />
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { Answer, askQuestion, MsImage, MsModalResult, NoActiveUser, Translatable, useWindowSize } from 'megashark-lib';
import { IonButton } from '@ionic/vue';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { IonContent, IonPage, IonText, modalController } from '@ionic/vue';
import { inject, onMounted, onUnmounted, ref, useTemplateRef } from 'vue';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import {
  acceptOrganizationJoinRequest,
  cancelInvitation,
  ClientCancelInvitationErrorTag,
  ClientNewUserInvitationErrorTag,
  getPkiJoinOrganizationLink,
  InvitationEmailSentStatus,
  inviteUser,
  JoinRequestValidity,
  listOrganizationJoinRequests,
  listUserInvitations,
  OrganizationJoinRequest,
  rejectOrganizationJoinRequest,
  UserInvitation,
} from '@/parsec';
import InvitationList from '@/views/invitations/InvitationList.vue';
import PkiRequestList from '@/views/invitations/PkiRequestList.vue';
import { copyToClipboard } from '@/common/clipboard';
import GreetUserModal from '@/views/users/GreetUserModal.vue';
import SelectProfileModal from '@/views/invitations/SelectProfileModal.vue';

enum Views {
  EmailInvitations = 'email-invitation',
  PkiRequests = 'pki-request',
}

const { isLargeDisplay } = useWindowSize();
const view = ref(Views.EmailInvitations);
const informationManager: InformationManager = inject(InformationManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;

const invitations = ref<Array<UserInvitation>>([]);
const pkIRequests = ref<Array<OrganizationJoinRequest>>([]);
const rootCertificate = ref<string | undefined>(undefined);
const inputRef = useTemplateRef<HTMLInputElement>('input');

let eventCbId: string | null = null;

onMounted(async (): Promise<void> => {
  eventCbId = await eventDistributor.registerCallback(Events.InvitationUpdated, async (event: Events, data?: EventData) => {
    if (event === Events.InvitationUpdated && data) {
      await refreshInvitationList();
    }
  });
  await refreshAll();
});

onUnmounted(async () => {
  if (eventCbId) {
    await eventDistributor.removeCallback(eventCbId);
  }
});

async function onInvitationGreetClicked(inv: UserInvitation): Promise<void> {
  const modal = await modalController.create({
    component: GreetUserModal,
    canDismiss: true,
    backdropDismiss: false,
    cssClass: 'greet-organization-modal',
    showBackdrop: true,
    handle: false,
    breakpoints: isLargeDisplay.value ? undefined : [1],
    expandToScroll: false,
    initialBreakpoint: isLargeDisplay.value ? undefined : 1,
    componentProps: {
      invitation: inv,
      informationManager: informationManager,
    },
  });
  await modal.present();
  await modal.onWillDismiss();
  await modal.dismiss();
  await refreshInvitationList();
}

async function onSendInvitationEmailClicked(inv: UserInvitation): Promise<void> {
  const answer = await askQuestion('UsersPage.invitation.sendEmailTitle', 'UsersPage.invitation.sendEmailMessage', {
    yesText: 'UsersPage.invitation.sendEmail',
  });

  if (answer === Answer.No) {
    return;
  }
  const result = await inviteUser(inv.claimerEmail);
  if (result.ok && result.value.emailSentStatus === InvitationEmailSentStatus.Success) {
    informationManager.present(
      new Information({
        message: {
          key: 'UsersPage.invitation.inviteSuccessMailSent',
          data: {
            email: inv.claimerEmail,
          },
        },
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
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
    informationManager.present(
      new Information({
        message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

async function onCopyInvitationLinkClicked(inv: UserInvitation): Promise<void> {
  await copyToClipboard(
    inv.addr,
    informationManager,
    'UsersPage.invitation.linkCopiedToClipboard',
    'UsersPage.invitation.linkNotCopiedToClipboard',
  );
}

async function onDeleteInvitationClicked(inv: UserInvitation): Promise<void> {
  const answer = await askQuestion(
    'UsersPage.invitation.cancelInvitation.title',
    { key: 'UsersPage.invitation.cancelInvitation.message', data: { email: inv.claimerEmail } },
    {
      yesText: isLargeDisplay.value ? 'UsersPage.invitation.cancelInvitation.yes.long' : 'UsersPage.invitation.cancelInvitation.yes.short',
      noText: isLargeDisplay.value ? 'UsersPage.invitation.cancelInvitation.no.long' : 'UsersPage.invitation.cancelInvitation.no.short',
      yesIsDangerous: true,
    },
  );

  if (answer === Answer.No) {
    return;
  }

  const result = await cancelInvitation(inv.token);

  if (result.ok) {
    informationManager.present(
      new Information({
        message: 'UsersPage.invitation.cancelSuccess',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    await refreshInvitationList();
  } else {
    // In all those cases we can just refresh the list and the invitation should disappear, no need
    // to warn the user.
    if (
      result.error.tag === ClientCancelInvitationErrorTag.NotFound ||
      result.error.tag === ClientCancelInvitationErrorTag.NotAllowed ||
      result.error.tag === ClientCancelInvitationErrorTag.AlreadyCancelled ||
      result.error.tag === ClientCancelInvitationErrorTag.Completed
    ) {
      await refreshInvitationList();
    } else {
      informationManager.present(
        new Information({
          message: 'UsersPage.invitation.cancelFailed',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
    }
  }
}

async function onCopyJoinLinkClicked(): Promise<void> {
  const result = await getPkiJoinOrganizationLink();
  if (result.ok) {
    await copyToClipboard(result.value, informationManager, 'YEP YEP YEP', 'NOP NOP NOP');
  }
}

async function onAcceptPkiRequestClicked(req: OrganizationJoinRequest): Promise<void> {
  if (req.validity === JoinRequestValidity.Unknown) {
    const answer = await askQuestion(
      'NOT VALIDATED',
      'THIS REQUEST HASNT BEEN VALIDATED WITH THE ROOT CERTIFICATE. WE CANNOT VERIFY THE IDENTIFY. YOU SURE BRO?',
    );
    if (answer !== Answer.Yes) {
      return;
    }
  } else if (req.validity === JoinRequestValidity.Invalid) {
    const answer = await askQuestion('INVALID', 'THE CERTIFICATE PROVIDED BY THE USER DOESNT MATCH THE ROOT CERTIFICATE. YOU SURE BRO?');
    if (answer !== Answer.Yes) {
      return;
    }
  }

  const modal = await modalController.create({
    component: SelectProfileModal,
    componentProps: {
      email: req.humanHandle.email,
      name: req.humanHandle.label,
    },
  });
  await modal.present();
  const { role, data } = await modal.onWillDismiss();
  await modal.dismiss();

  if (role !== MsModalResult.Confirm || !data.profile) {
    return;
  }

  const result = await acceptOrganizationJoinRequest(req, data.profile);
  if (result.ok) {
    informationManager.present(
      new Information({
        message: 'REQUEST HAS BEEN ACCEPTED',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  } else {
    informationManager.present(
      new Information({
        message: 'CANNOT ACCEPT THE REQUEST',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

async function onRejectPkiRequestClicked(req: OrganizationJoinRequest): Promise<void> {
  const result = await rejectOrganizationJoinRequest(req);
  if (result.ok) {
    informationManager.present(
      new Information({
        message: 'REQUEST HAS BEEN REJECTED',
        level: InformationLevel.Info,
      }),
      PresentationMode.Toast,
    );
  } else {
    informationManager.present(
      new Information({
        message: 'CANNOT REJECT THE REQUEST',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

async function selectRootCertificate(): Promise<void> {
  inputRef.value?.click();
}

async function onInputChange(): Promise<void> {
  if (!inputRef.value?.files) {
    return;
  }
  const file = inputRef.value.files.item(0) as File;
  rootCertificate.value = await file.text();
  await refreshPkiRequestList();
}

async function refreshInvitationList(): Promise<void> {
  const result = await listUserInvitations();
  if (result.ok) {
    invitations.value = result.value;
  } else {
    window.electronAPI.log('error', `Failed to list invitations: ${result.error.tag} (${result.error.error})`);
  }
}

async function refreshPkiRequestList(): Promise<void> {
  const result = await listOrganizationJoinRequests(rootCertificate.value);
  console.log(result);
  if (result.ok) {
    pkIRequests.value = result.value;
  } else {
    window.electronAPI.log('error', `Failed to list pki join requests: ${result.error.tag} (${result.error.error})`);
  }
}

async function refreshAll(): Promise<void> {
  await Promise.allSettled([refreshInvitationList(), refreshPkiRequestList()]);
}
</script>

<style scoped lang="scss">
.no-active {
  width: 100%;
  height: 100%;
  color: var(--parsec-color-light-secondary-grey);
  display: flex;
  margin: auto;
  align-items: center;

  &-content {
    border-radius: var(--parsec-radius-8);
    display: flex;
    height: fit-content;
    width: 100%;
    text-align: center;
    flex-direction: column;
    gap: 1rem;
    align-items: center;
    padding: 2rem 1rem;
  }
}

.invitations-container > div {
  height: 100%;
}
</style>
