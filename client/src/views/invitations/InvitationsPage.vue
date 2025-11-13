<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="invitations-page">
    <div
      v-if="isSmallDisplay && false"
      class="switch-view"
    >
      <ion-button
        v-if="view === InvitationView.EmailInvitation"
        class="switch-view-button email-button"
        @click="openInvitationSwitchModal"
      >
        <ion-icon
          :icon="mailUnread"
          class="switch-view-button__icon"
        />
        {{ $msTranslate('InvitationsPage.emailInvitation.tab') }}
        <span class="switch-view-button__count">{{ invitations.length }}</span>
        <ion-icon
          :icon="caretDown"
          class="switch-view-button__toggle"
        />
      </ion-button>

      <ion-button
        v-else
        class="switch-view-button pki-button"
        @click="openInvitationSwitchModal"
      >
        <ion-icon
          :icon="idCard"
          class="switch-view-button__icon"
        />
        {{ $msTranslate('InvitationsPage.pkiRequests.tab') }}
        <span class="switch-view-button__count button-small">{{ pkiRequests.length }}</span>
        <ion-icon
          :icon="caretDown"
          class="switch-view-button__toggle"
        />
      </ion-button>
    </div>

    <ion-content class="content-scroll">
      <div class="toggle-view-container">
        <div
          class="toggle-view"
          role="tablist"
          aria-label="Invitation view toggle"
          v-if="isLargeDisplay && false"
        >
          <ion-button
            class="toggle-view-button email-button"
            :class="{ active: view === InvitationView.EmailInvitation }"
            @click="switchView(InvitationView.EmailInvitation)"
            :disabled="view === InvitationView.EmailInvitation"
          >
            <ion-icon
              :icon="mailUnread"
              class="toggle-view-button__icon"
            />
            <ion-text class="toggle-view-button__label">
              <span v-if="windowWidth > WindowSizeBreakpoints.MD">{{ $msTranslate('InvitationsPage.emailInvitation.tab') }}</span>
              <span v-else>{{ $msTranslate('InvitationsPage.emailInvitation.tabShort') }}</span>
            </ion-text>
            <span class="toggle-view-button__count">{{ invitations.length }}</span>
          </ion-button>
          <ion-button
            class="toggle-view-button pki-button"
            :class="{ active: view === InvitationView.PkiRequest }"
            @click="switchView(InvitationView.PkiRequest)"
            :disabled="view === InvitationView.PkiRequest"
          >
            <ion-icon
              :icon="idCard"
              class="toggle-view-button__icon"
            />
            <ion-text class="toggle-view-button__label">
              <span v-if="windowWidth > WindowSizeBreakpoints.MD">{{ $msTranslate('InvitationsPage.pkiRequests.tab') }}</span>
              <span v-else>{{ $msTranslate('InvitationsPage.pkiRequests.tabShort') }}</span>
            </ion-text>
            <span class="toggle-view-button__count">{{ pkiRequests.length }}</span>
          </ion-button>
        </div>

        <ion-button
          v-if="view === InvitationView.PkiRequest && rootCertificate"
          @click="selectRootCertificate"
          id="update-root-certificate-button"
          class="button-medium button-default certificate-button"
          fill="clear"
          ref="updateRootCertificateButton"
        >
          <ion-icon
            :icon="document"
            class="certificate-button__icon"
          />
          <ms-image
            :image="CertificateIcon"
            class="certificate-button__badge"
          />
        </ion-button>

        <ion-button
          v-if="view === InvitationView.PkiRequest"
          @click="onCopyJoinLinkClicked"
          id="copy-link-pki-request-button"
          class="button-medium button-default"
        >
          <ion-icon
            :icon="link"
            class="button-icon"
          />
          <span>{{ $msTranslate('InvitationsPage.pkiRequests.copyLink') }}</span>
        </ion-button>

        <ion-button
          v-if="view === InvitationView.EmailInvitation"
          @click="inviteUser"
          id="invite-user-button"
          class="button-medium button-default"
        >
          <ion-icon
            :icon="personAdd"
            class="button-icon"
          />
          <span>{{ $msTranslate('InvitationsPage.emailInvitation.inviteUser') }}</span>
        </ion-button>
      </div>

      <div
        class="invitations-container scroll"
        v-if="view === InvitationView.EmailInvitation"
      >
        <div
          v-show="invitations.length === 0"
          class="no-active body-lg"
        >
          <div class="no-active-content">
            <ms-image
              :image="NoInvitation"
              class="no-active__image"
            />
            <ion-text>
              {{ $msTranslate('InvitationsPage.emailInvitation.emptyState') }}
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
      <div
        class="invitations-container scroll"
        v-if="view === InvitationView.PkiRequest"
      >
        <input
          type="file"
          ref="input"
          hidden
          @change="onInputChange"
        />
        <div
          class="root-certificate"
          v-if="!rootCertificate"
        >
          <div class="root-certificate-content">
            <ms-image
              :image="CertificateIcon"
              class="root-certificate__icon"
            />
            <div class="root-certificate-text">
              <ion-text class="root-certificate-text__title title-h3">
                {{ $msTranslate('InvitationsPage.pkiRequests.rootCertificate.missingCertificateTitle') }}
              </ion-text>
              <ion-text class="root-certificate-text__description body-lg">
                {{ $msTranslate('InvitationsPage.pkiRequests.rootCertificate.missingCertificateDescription') }}
              </ion-text>
            </div>
            <ms-report-text
              class="root-certificate-text__report"
              :theme="MsReportTheme.Warning"
            >
              {{ $msTranslate('InvitationsPage.pkiRequests.pkiFeature') }}
            </ms-report-text>
            <ion-button
              class="button-large button-default"
              @click="selectRootCertificate"
            >
              <ms-image
                :image="DocumentImport"
                class="button-icon"
              />
              {{ $msTranslate('InvitationsPage.pkiRequests.rootCertificate.setRootCertificate') }}
            </ion-button>
          </div>
        </div>
        <div
          v-show="invitations.length === 0"
          class="no-active body-lg"
        >
          <div class="no-active-content">
            <ms-image :image="NoInvitation" />
            <ion-text>
              {{ $msTranslate('InvitationsPage.pkiRequests.emptyState') }}
            </ion-text>
          </div>
        </div>
        <div v-show="(invitations.length > 0 && isLargeDisplay) || (rootCertificate && pkiRequests.length > 0)">
          <pki-request-list
            :requests="pkiRequests"
            @accept-click="onAcceptPkiRequestClicked"
            @reject-click="onRejectPkiRequestClicked"
          />
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  Answer,
  askQuestion,
  DocumentImport,
  MsImage,
  MsModalResult,
  NoInvitation,
  Translatable,
  useWindowSize,
  WindowSizeBreakpoints,
  MsReportText,
  MsReportTheme,
  attachMouseOverTooltip,
} from 'megashark-lib';
import CertificateIcon from '@/assets/images/certificate-icon.svg?raw';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { IonContent, IonPage, IonText, modalController, IonIcon, IonButton } from '@ionic/vue';
import { caretDown, document, idCard, link, mailUnread, personAdd } from 'ionicons/icons';
import { inject, onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';
import { InvitationView } from '@/views/invitations/types';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import InvitationSwitchModal from '@/views/invitations/InvitationSwitchModal.vue';
import {
  acceptOrganizationJoinRequest,
  cancelInvitation,
  ClientCancelInvitationErrorTag,
  ClientNewUserInvitationErrorTag,
  getPkiJoinOrganizationLink,
  InvitationEmailSentStatus,
  inviteUser as parsecInviteUser,
  JoinRequestValidity,
  listOrganizationJoinRequests,
  listUserInvitations,
  OrganizationJoinRequest,
  rejectOrganizationJoinRequest,
  UserInvitation,
  ClientNewUserInvitationError,
} from '@/parsec';
import InvitationList from '@/views/invitations/InvitationList.vue';
import PkiRequestList from '@/views/invitations/PkiRequestList.vue';
import { copyToClipboard } from '@/common/clipboard';
import GreetUserModal from '@/views/users/GreetUserModal.vue';
import SelectProfileModal from '@/views/invitations/SelectProfileModal.vue';
import { currentRouteIs, getCurrentRouteQuery, navigateTo, Routes, watchRoute } from '@/router';
import InviteModal from '@/components/invitations/InviteModal.vue';

const { isLargeDisplay, isSmallDisplay, windowWidth } = useWindowSize();
const view = ref(InvitationView.EmailInvitation);
const informationManager: InformationManager = inject(InformationManagerKey)!;
const eventDistributor: EventDistributor = inject(EventDistributorKey)!;

const invitations = ref<Array<UserInvitation>>([]);
const pkiRequests = ref<Array<OrganizationJoinRequest>>([]);
const rootCertificate = ref<string | undefined>(undefined);
const inputRef = useTemplateRef<HTMLInputElement>('input');
const updateRootCertificateButtonRef = useTemplateRef<InstanceType<typeof IonButton>>('updateRootCertificateButton');

let eventCbId: string | null = null;

const routeWatchCancel = watchRoute(async () => {
  if (!currentRouteIs(Routes.Invitations)) {
    return;
  }
  const query = getCurrentRouteQuery();

  if (query.invitationView && Object.values(InvitationView).includes(query.invitationView)) {
    view.value = query.invitationView;
  }

  if (query.openInvite) {
    view.value = InvitationView.EmailInvitation;
    await inviteUser();
    await navigateTo(Routes.Invitations, { replace: true, query: { invitationView: InvitationView.EmailInvitation } });
  }
  await refreshAll();
});

onMounted(async (): Promise<void> => {
  eventCbId = await eventDistributor.registerCallback(Events.InvitationUpdated, async (event: Events, data?: EventData) => {
    if (event === Events.InvitationUpdated && data) {
      await refreshInvitationList();
    }
  });
  await refreshAll();
  const query = getCurrentRouteQuery();

  if (query.invitationView && Object.values(InvitationView).includes(query.invitationView)) {
    view.value = query.invitationView;
  }

  if (query.openInvite) {
    view.value = InvitationView.EmailInvitation;
    await inviteUser();
    await navigateTo(Routes.Invitations, { replace: true, query: { invitationView: InvitationView.EmailInvitation } });
  }
});

watch(updateRootCertificateButtonRef, (el) => {
  if (el && el.$el) {
    attachMouseOverTooltip(el.$el, 'InvitationsPage.pkiRequests.rootCertificate.updateRootCertificate');
  }
});

onUnmounted(async () => {
  if (eventCbId) {
    await eventDistributor.removeCallback(eventCbId);
  }
  routeWatchCancel();
});

async function switchView(newView: InvitationView): Promise<void> {
  if (newView === view.value) {
    return;
  }
  await navigateTo(Routes.Invitations, { replace: true, query: { invitationView: newView } });
}

async function inviteUser(): Promise<void> {
  const modal = await modalController.create({
    component: InviteModal,
    cssClass: 'invite-modal',
  });
  await modal.present();
  const { data, role } = await modal.onDidDismiss();
  await modal.dismiss();

  if (role !== MsModalResult.Confirm || !data.emails) {
    return;
  }

  let noEmailCount = 0;
  let errorCount = 0;
  let lastError: ClientNewUserInvitationError | undefined = undefined;
  for (const email of data.emails) {
    const result = await parsecInviteUser(email);
    if (result.ok && result.value.emailSentStatus !== InvitationEmailSentStatus.Success) {
      noEmailCount += 1;
    }
    if (!result.ok) {
      lastError = result.error;
      errorCount += 1;
    }
  }
  // Everything went according to plan, no error at all
  if (!lastError) {
    // All emails were sent
    if (noEmailCount === 0) {
      informationManager.present(
        new Information({
          message: {
            key: 'UsersPage.invitation.inviteSuccessMailSent',
            data: {
              email: data.emails[0],
              count: data.emails.length,
            },
            count: data.emails.length,
          },
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    } else {
      // All invitations were successful but not all emails were sent
      informationManager.present(
        new Information({
          message: {
            key: 'UsersPage.invitation.inviteSuccessNoMail',
            data: {
              email: data.emails[0],
              count: data.emails.length,
            },
            count: data.emails.length,
          },
          level: InformationLevel.Success,
        }),
        PresentationMode.Toast,
      );
    }
  } else {
    // An error occurred, we're checking the last one recorded.
    // If there's only one email, then the last error is the right one anyway and if there are multiple emails,
    // we're generalizing it: if the last error was due to connection issues, chances are they all failed due to
    // connection issues.
    let message: Translatable = '';
    if (lastError.tag === ClientNewUserInvitationErrorTag.AlreadyMember) {
      message = { key: 'UsersPage.invitation.inviteFailedAlreadyMember', data: { email: data.emails[0] }, count: data.emails.length };
    } else if (lastError.tag === ClientNewUserInvitationErrorTag.Offline) {
      message = { key: 'UsersPage.invitation.inviteFailedOffline', count: data.emails.length };
    } else if (lastError.tag === ClientNewUserInvitationErrorTag.NotAllowed) {
      message = { key: 'UsersPage.invitation.inviteFailedNotAllowed', count: data.emails.length };
    } else if (errorCount === data.emails.length) {
      message = 'UsersPage.invitation.inviteFailedAll';
    } else {
      message = 'UsersPage.invitation.inviteFailedSome';
    }
    window.electronAPI.log('error', `Failed to create invitations: ${lastError.tag} (${lastError.error})`);
    informationManager.present(
      new Information({
        message,
        level: lastError.tag === ClientNewUserInvitationErrorTag.AlreadyMember ? InformationLevel.Warning : InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

async function openInvitationSwitchModal(): Promise<void> {
  const modal = await modalController.create({
    component: InvitationSwitchModal,
    canDismiss: true,
    backdropDismiss: true,
    cssClass: 'invitation-switch-modal',
    showBackdrop: true,
    handle: true,
    breakpoints: [1],
    expandToScroll: false,
    componentProps: {
      invitationsCount: invitations.value.length,
      pkiRequestsCount: pkiRequests.value.length,
      defaultView: view.value,
    },
    initialBreakpoint: isLargeDisplay.value ? undefined : 1,
  });

  await modal.present();
  const { data, role } = await modal.onWillDismiss();
  await modal.dismiss();
  if (role === MsModalResult.Confirm && data) {
    switchView(data);
  }
}

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
  const result = await parsecInviteUser(inv.claimerEmail);
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
    await copyToClipboard(
      result.value,
      informationManager,
      'InvitationsPage.pkiRequests.linkCopiedToClipboard.success',
      'InvitationsPage.pkiRequests.linkCopiedToClipboard.failed',
    );
  }
}

async function onAcceptPkiRequestClicked(req: OrganizationJoinRequest): Promise<void> {
  if (req.validity === JoinRequestValidity.Unknown) {
    const answer = await askQuestion(
      'InvitationsPage.pkiRequests.validationModal.error.unknown.title',
      'InvitationsPage.pkiRequests.validationModal.error.unknown.description',
      {
        yesText: 'InvitationsPage.pkiRequests.validationModal.button.confirm',
        noText: 'InvitationsPage.pkiRequests.validationModal.button.cancel',
      },
    );
    if (answer !== Answer.Yes) {
      return;
    }
  } else if (req.validity === JoinRequestValidity.Invalid) {
    const answer = await askQuestion(
      'InvitationsPage.pkiRequests.validationModal.error.notValid.title',
      'InvitationsPage.pkiRequests.validationModal.error.notValid.description',
      {
        yesText: 'InvitationsPage.pkiRequests.validationModal.button.confirm',
        noText: 'InvitationsPage.pkiRequests.validationModal.button.cancel',
      },
    );
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
    cssClass: 'select-profile-modal',
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
        message: 'InvitationsPage.pkiRequests.validationModal.acceptedSuccess',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
  } else {
    informationManager.present(
      new Information({
        message: 'InvitationsPage.pkiRequests.validationModal.acceptedFailed',
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
        message: 'InvitationsPage.pkiRequests.validationModal.rejectedSuccess',
        level: InformationLevel.Info,
      }),
      PresentationMode.Toast,
    );
  } else {
    informationManager.present(
      new Information({
        message: 'InvitationsPage.pkiRequests.validationModal.rejectedFailed',
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
  if (result.ok) {
    pkiRequests.value = result.value;
  } else {
    window.electronAPI.log('error', `Failed to list pki join requests: ${result.error.tag} (${result.error.error})`);
  }
}

async function refreshAll(): Promise<void> {
  await Promise.allSettled([refreshInvitationList(), refreshPkiRequestList()]);
}
</script>

<style scoped lang="scss">
.invitations-page {
  padding: 1.5rem;
  background: var(--parsec-color-light-secondary-background);

  @include ms.responsive-breakpoint('sm') {
    padding: 0;
  }

  .content-scroll {
    display: flex;
    border-radius: var(--parsec-radius-12);
    overflow: hidden;
    box-shadow: var(--parsec-shadow-soft);

    @include ms.responsive-breakpoint('sm') {
      box-shadow: var(--parsec-shadow-strong);
      border-radius: var(--parsec-radius-12) var(--parsec-radius-12) 0 0;
    }
  }
}

.toggle-view-container {
  border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
  padding: 0.75rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;

  @include ms.responsive-breakpoint('sm') {
    padding: 0.75rem 1rem;
  }

  .toggle-view {
    display: flex;
    width: fit-content;
    background: var(--parsec-color-light-secondary-medium);
    border: 2px solid var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-8);
    overflow: hidden;
    box-shadow: 0 0 11px 0 rgba(0, 0, 0, 0.05) inset;

    @include ms.responsive-breakpoint('lg') {
      border: 1px solid var(--parsec-color-light-secondary-medium);
      box-shadow: var(--parsec-shadow-input);
      cursor: pointer;
    }

    &-button {
      --color: var(--parsec-color-light-secondary-hard-grey);
      --background: none;
      --background-hover: var(--parsec-color-light-secondary-disabled);
      --border-radius: 0;
      display: contents;

      &:hover {
        --color: var(--parsec-color-light-secondary-soft-text);
      }

      &::part(native) {
        display: flex;
        width: fit-content;
      }

      &__icon {
        font-size: 1.125rem;
        margin-right: 0.5rem;
      }

      &__label {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }

      &__count {
        background: var(--parsec-color-light-primary-50);
        color: var(--parsec-color-light-primary-500);
        border-radius: var(--parsec-radius-8);
        padding: 0.1rem 0.35rem;
        margin-left: 0.5rem;
      }

      &.active {
        --background: var(--parsec-color-light-secondary-white);
        --border-radius: var(--parsec-radius-6);
        --color: var(--parsec-color-light-primary-700);
        cursor: default;
        opacity: 1;
      }

      &:not(.active) {
        .toggle-view-button__count {
          background: var(--parsec-color-light-secondary-background);
          color: var(--parsec-color-light-secondary-grey);
        }
      }

      &:hover {
        @include ms.responsive-breakpoint('lg') {
          --background-hover: var(--parsec-color-light-secondary-medium);
        }
      }
    }
  }

  #copy-link-pki-request-button,
  #invite-user-button {
    --background: var(--parsec-color-light-secondary-inversed-contrast);
    --background-hover: var(--parsec-color-light-secondary-medium);
    color: var(--parsec-color-light-primary-700);
    box-shadow: var(--parsec-shadow-input);

    &::part(native) {
      border: 1px solid var(--parsec-color-light-secondary-medium);
      padding: 0.5rem 1rem;

      @include ms.responsive-breakpoint('sm') {
        padding: 0.5rem;
      }
    }

    .button-icon {
      font-size: 1.125rem;
      margin-right: 0.5rem;
    }
  }

  .certificate-button {
    display: flex;
    margin-left: auto;
    --background-hover: var(--parsec-color-light-secondary-medium);

    &::part(native) {
      padding: 0.5rem 0.5rem;
      position: relative;
    }

    &__badge {
      position: absolute;
      bottom: -0.375rem;
      right: -0.125rem;
      width: 1.25rem;
      height: 1.25rem;
      color: var(--parsec-color-light-secondary-white);
      background: var(--parsec-color-light-secondary-white);
      padding: 0.125rem;
      border-radius: var(--parsec-radius-circle);
    }

    &__icon {
      color: var(--parsec-color-light-primary-600);
      font-size: 1.375rem;
      margin-right: 0.5rem;
    }
  }
}

.no-active {
  width: 100%;
  height: 100%;
  color: var(--parsec-color-light-secondary-grey);
  display: flex;
  margin: auto;
  align-items: center;

  &__image {
    @include ms.responsive-breakpoint('sm') {
      max-width: 5rem;
    }
  }

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

.invitations-container {
  padding: 0;
  position: relative;
}

.root-certificate {
  position: absolute;
  z-index: 100;
  top: 0;
  width: 100%;
  height: 100%;
  background: #00000058;
  padding: 1rem;
  display: flex;
  align-items: center;
  justify-content: center;

  @include ms.responsive-breakpoint('sm') {
    padding: 0;
    background: var(--parsec-color-light-secondary-white);
    align-items: flex-start;
    height: -webkit-fill-available;
  }

  &-content {
    background: var(--parsec-color-light-secondary-white);
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    max-width: 30rem;
    border-radius: var(--parsec-radius-12);
    box-shadow: var(--parsec-shadow-soft);

    @include ms.responsive-breakpoint('sm') {
      border-radius: 0;
      padding: 2rem 1.5rem;
      box-shadow: none;
    }

    .root-certificate__icon {
      width: 2.5rem;
      margin: 0 auto;

      @include ms.responsive-breakpoint('sm') {
        margin: 0;
        width: 1.5rem;
        position: absolute;
        top: 1.875rem;
      }
    }

    .root-certificate-text {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
      text-align: center;

      @include ms.responsive-breakpoint('sm') {
        text-align: left;
      }

      &__title {
        color: var(--parsec-color-light-secondary-text);

        @include ms.responsive-breakpoint('sm') {
          margin-left: 2rem;
        }
      }

      &__description {
        color: var(--parsec-color-light-secondary-hard-grey);
      }
    }

    .button-icon {
      --fill-color: var(--parsec-color-light-secondary-white);
      width: 1.125rem;
      margin-right: 0.5rem;
    }
  }
}

.switch-view {
  display: flex;
  justify-content: center;
  padding: 0.5rem 1rem 1.5rem;

  .switch-view-button {
    width: 100%;
    --background: var(--parsec-color-light-secondary-white);
    --background-hover: var(--parsec-color-light-secondary-medium);
    --color: var(--parsec-color-light-primary-700);
    box-shadow: var(--parsec-shadow-input);
    border-radius: var(--parsec-radius-8);

    &::part(native) {
      border: 1px solid var(--parsec-color-light-secondary-medium);
      padding: 0.625rem 1rem;
    }

    &__icon {
      font-size: 1.125rem;
      margin-right: 0.5rem;
    }

    &__count {
      background: var(--parsec-color-light-primary-50);
      color: var(--parsec-color-light-primary-500);
      border-radius: var(--parsec-radius-8);
      padding: 0.1rem 0.35rem;
      margin-left: 0.5rem;
    }

    &__toggle {
      font-size: 1rem;
      margin-left: auto;
      color: var(--parsec-color-light-secondary-grey);
    }
  }
}
</style>
