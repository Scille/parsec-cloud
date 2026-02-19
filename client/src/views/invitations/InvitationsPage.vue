<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="invitations-page">
    <div
      v-if="isSmallDisplay"
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
          :icon="link"
          class="switch-view-button__icon"
        />
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.tab') }}
        <span class="switch-view-button__count button-small">{{ asyncEnrollmentRequests.length }}</span>
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
          v-if="isLargeDisplay"
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
            :class="{
              active: view === InvitationView.AsyncEnrollmentRequest,
            }"
            @click="switchView(InvitationView.AsyncEnrollmentRequest)"
            ref="switchToAsyncEnrollmentButton"
            :disabled="view === InvitationView.AsyncEnrollmentRequest"
          >
            <ion-icon
              :icon="link"
              class="toggle-view-button__icon"
            />
            <ion-text class="toggle-view-button__label">
              <span>{{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.tab') }}</span>
            </ion-text>
            <span class="toggle-view-button__count">
              {{ asyncEnrollmentRequests.length }}
            </span>
          </ion-button>
        </div>

        <ion-button
          v-if="view === InvitationView.AsyncEnrollmentRequest"
          @click="onCopyJoinLinkClicked"
          id="copy-link-pki-request-button"
          class="button-medium button-default"
        >
          <ion-icon
            :icon="copy"
            class="button-icon"
          />
          <span>{{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.copyLink') }}</span>
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
        v-if="view === InvitationView.AsyncEnrollmentRequest"
      >
        <div
          v-show="asyncEnrollmentRequests.length === 0 && !asyncListError"
          class="no-active body-lg"
        >
          <div class="no-active-content">
            <ms-image :image="NoInvitation" />
            <ion-text>
              {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.emptyState') }}
            </ion-text>
          </div>
        </div>
        <ms-report-text
          class="invitations-error-message"
          v-show="asyncEnrollmentRequests.length === 0 && asyncListError"
          :theme="MsReportTheme.Error"
        >
          {{ $msTranslate(asyncListError) }}
        </ms-report-text>
        <div v-show="asyncEnrollmentRequests.length > 0">
          <async-enrollment-request-list
            :requests="asyncEnrollmentRequests"
            :pki-available="pkiAvailable"
            :server-config="serverConfig"
            @accept-click="onAcceptAsyncEnrollmentRequestClicked"
            @reject-click="onRejectAsyncEnrollmentRequestClicked"
          />
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { copyToClipboard } from '@/common/clipboard';
import InviteModal from '@/components/invitations/InviteModal.vue';
import {
  acceptAsyncEnrollment,
  AcceptFinalizeAsyncEnrollmentIdentityStrategy,
  AsyncEnrollmentIdentitySystemTag,
  AsyncEnrollmentUntrusted,
  cancelInvitation,
  ClientAcceptAsyncEnrollmentErrorTag,
  ClientCancelInvitationErrorTag,
  ClientNewUserInvitationError,
  ClientNewUserInvitationErrorTag,
  getAsyncEnrollmentAddr,
  getCurrentServerConfig,
  InvitationEmailSentStatus,
  isSmartcardAvailable,
  listAsyncEnrollments,
  listUserInvitations,
  makeAcceptOpenBaoIdentityStrategy,
  makeAcceptPkiIdentityStrategy,
  inviteUser as parsecInviteUser,
  rejectAsyncEnrollment,
  ServerConfig,
  UserInvitation,
  X509CertificateReference,
} from '@/parsec';
import { currentRouteIs, getCurrentRouteQuery, navigateTo, Routes, watchRoute } from '@/router';
import { EventData, EventDistributor, EventDistributorKey, Events } from '@/services/eventDistributor';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { OpenBaoClient } from '@/services/openBao';
import AsyncEnrollmentRequestList from '@/views/invitations/AsyncEnrollmentRequestList.vue';
import InvitationList from '@/views/invitations/InvitationList.vue';
import InvitationSwitchModal from '@/views/invitations/InvitationSwitchModal.vue';
import SelectProfileModal from '@/views/invitations/SelectProfileModal.vue';
import { InvitationView } from '@/views/invitations/types';
import AsyncEnrollmentOpenBaoAuthModal from '@/views/users/AsyncEnrollmentOpenBaoAuthModal.vue';
import AsyncEnrollmentPkiAuthModal from '@/views/users/AsyncEnrollmentPkiAuthModal.vue';
import GreetUserModal from '@/views/users/GreetUserModal.vue';
import { IonButton, IonContent, IonIcon, IonPage, IonText, modalController } from '@ionic/vue';
import { caretDown, copy, link, mailUnread, personAdd } from 'ionicons/icons';
import {
  Answer,
  askQuestion,
  MsImage,
  MsModalResult,
  MsReportText,
  MsReportTheme,
  NoInvitation,
  Translatable,
  useWindowSize,
  WindowSizeBreakpoints,
} from 'megashark-lib';
import { inject, onMounted, onUnmounted, Ref, ref, toRaw } from 'vue';

const { isLargeDisplay, isSmallDisplay, windowWidth } = useWindowSize();
const view = ref(InvitationView.EmailInvitation);
const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
const invitations = ref<Array<UserInvitation>>([]);
const asyncEnrollmentRequests = ref<Array<AsyncEnrollmentUntrusted>>([]);
const serverConfig = ref<ServerConfig | undefined>(undefined);
const pkiAvailable = ref(false);
const openBaoClient = ref<OpenBaoClient | undefined>(undefined);
const certificate = ref<X509CertificateReference | undefined>(undefined);
const asyncListError = ref('');

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
  eventCbId = await eventDistributor.value.registerCallback(
    [Events.InvitationUpdated, Events.AsyncEnrollmentUpdated],
    async (event: Events, data?: EventData) => {
      if (event === Events.InvitationUpdated && data) {
        await refreshInvitationList();
      } else if (event === Events.AsyncEnrollmentUpdated) {
        await refreshAsyncEnrollmentRequestList();
      }
    },
  );
  pkiAvailable.value = await isSmartcardAvailable();
  const configResult = await getCurrentServerConfig();
  if (configResult.ok) {
    serverConfig.value = configResult.value;
  }

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

onUnmounted(async () => {
  if (eventCbId) {
    await eventDistributor.value.removeCallback(eventCbId);
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
      informationManager.value.present(
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
      informationManager.value.present(
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
    informationManager.value.present(
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
      asyncEnrollmentRequestsCount: asyncEnrollmentRequests.value.length,
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
      informationManager: informationManager.value,
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
    informationManager.value.present(
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
    informationManager.value.present(
      new Information({
        message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

async function onCopyInvitationLinkClicked(inv: UserInvitation): Promise<void> {
  const [_, invitationAddrAsHttpRedirection] = inv.addr;
  await copyToClipboard(
    invitationAddrAsHttpRedirection,
    informationManager.value,
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
    informationManager.value.present(
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
      informationManager.value.present(
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
  const result = await getAsyncEnrollmentAddr();
  if (result.ok) {
    const [_, invitationAddrAsHttpRedirection] = result.value;
    await copyToClipboard(
      invitationAddrAsHttpRedirection,
      informationManager.value,
      'InvitationsPage.asyncEnrollmentRequest.linkCopiedToClipboard.success',
      'InvitationsPage.asyncEnrollmentRequest.linkCopiedToClipboard.failed',
    );
  } else {
    informationManager.value.present(
      new Information({
        message: 'InvitationsPage.asyncEnrollmentRequest.linkCopiedToClipboard.failed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

async function onAcceptAsyncEnrollmentRequestClicked(request: AsyncEnrollmentUntrusted): Promise<void> {
  let strategy!: AcceptFinalizeAsyncEnrollmentIdentityStrategy;

  if (request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.PKI && !certificate.value) {
    if (!pkiAvailable.value) {
      window.electronAPI.log('error', 'PKI not available');
      return;
    }
    const pkiModal = await modalController.create({
      component: AsyncEnrollmentPkiAuthModal,
      cssClass: 'async-enrollment-pki-modal',
    });
    await pkiModal.present();
    const { role, data } = await pkiModal.onWillDismiss();
    await pkiModal.dismiss();
    if (role !== MsModalResult.Confirm || !data.certificate) {
      return;
    }
    strategy = makeAcceptPkiIdentityStrategy(toRaw(data.certificate) as X509CertificateReference);
  } else if (request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.OpenBao && !openBaoClient.value) {
    if (!serverConfig.value?.openbao || serverConfig.value.openbao.auths.length === 0) {
      return;
    }
    const ssoModal = await modalController.create({
      component: AsyncEnrollmentOpenBaoAuthModal,
      cssClass: 'async-enrollment-openbao-modal',
      componentProps: {
        serverConfig: serverConfig.value,
      },
    });
    await ssoModal.present();
    const { role, data } = await ssoModal.onWillDismiss();
    await ssoModal.dismiss();
    if (role !== MsModalResult.Confirm || !data.openBaoClient) {
      return;
    }
    strategy = makeAcceptOpenBaoIdentityStrategy(data.openBaoClient as OpenBaoClient);
  } else {
    window.electronAPI.log('error', `Unknown identity system ${request.identitySystem.tag}`);
    return;
  }

  const modal = await modalController.create({
    component: SelectProfileModal,
    componentProps: {
      email: request.untrustedRequestedHumanHandle.email,
      name: request.untrustedRequestedHumanHandle.label,
    },
    cssClass: 'select-profile-modal',
  });
  await modal.present();
  const { role, data } = await modal.onWillDismiss();
  await modal.dismiss();

  if (role !== MsModalResult.Confirm || !data.profile) {
    return;
  }

  const result = await acceptAsyncEnrollment(request, data.profile, strategy);
  if (result.ok) {
    informationManager.value.present(
      new Information({
        message: 'InvitationsPage.asyncEnrollmentRequest.validationModal.acceptedSuccess',
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    await refreshAsyncEnrollmentRequestList();
  } else {
    let message!: Translatable;
    switch (result.error.tag) {
      case ClientAcceptAsyncEnrollmentErrorTag.Offline: {
        message = 'InvitationsPage.asyncEnrollmentRequest.errors.offline';
        break;
      }
      case ClientAcceptAsyncEnrollmentErrorTag.OpenBaoBadServerResponse:
      case ClientAcceptAsyncEnrollmentErrorTag.OpenBaoNoServerResponse:
      case ClientAcceptAsyncEnrollmentErrorTag.OpenBaoBadURL: {
        message = 'InvitationsPage.asyncEnrollmentRequest.errors.couldNotContactServer';
        break;
      }
      case ClientAcceptAsyncEnrollmentErrorTag.PKICannotOpenCertificateStore:
      case ClientAcceptAsyncEnrollmentErrorTag.PKIServerInvalidX509Trustchain:
      case ClientAcceptAsyncEnrollmentErrorTag.PKIUnusableX509CertificateReference: {
        message = 'InvitationsPage.asyncEnrollmentRequest.errors.problemWithRequestCertificate';
        break;
      }
      default: {
        message = 'InvitationsPage.asyncEnrollmentRequest.validationModal.acceptedFailed';
      }
    }
    informationManager.value.present(
      new Information({
        message: message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

async function onRejectAsyncEnrollmentRequestClicked(request: AsyncEnrollmentUntrusted): Promise<void> {
  const result = await rejectAsyncEnrollment(request);
  if (result.ok) {
    informationManager.value.present(
      new Information({
        message: 'InvitationsPage.asyncEnrollmentRequest.validationModal.rejectedSuccess',
        level: InformationLevel.Info,
      }),
      PresentationMode.Toast,
    );
  } else {
    informationManager.value.present(
      new Information({
        message: 'InvitationsPage.asyncEnrollmentRequest.validationModal.rejectedFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
  await refreshAsyncEnrollmentRequestList();
}

async function refreshInvitationList(): Promise<void> {
  const result = await listUserInvitations();
  if (result.ok) {
    invitations.value = result.value;
  } else {
    window.electronAPI.log('error', `Failed to list invitations: ${result.error.tag} (${result.error.error})`);
  }
}

async function refreshAsyncEnrollmentRequestList(): Promise<void> {
  const result = await listAsyncEnrollments();
  if (result.ok) {
    asyncEnrollmentRequests.value = result.value;
    asyncListError.value = '';
  } else {
    asyncListError.value = 'InvitationsPage.asyncEnrollmentRequest.errors.failToListRequests';
    window.electronAPI.log('error', `Failed to list pki join requests: ${result.error.tag} (${result.error.error})`);
  }
}

async function refreshAll(): Promise<void> {
  await Promise.allSettled([refreshInvitationList(), refreshAsyncEnrollmentRequestList()]);
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
    background: var(--parsec-color-light-secondary-premiere);
    padding: 3px;
    border: 1px solid var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-8);
    box-shadow: 0 1px 2px 0 rgba(0, 0, 0, 0.04);
    position: relative;

    @include ms.responsive-breakpoint('lg') {
      border: 1px solid var(--parsec-color-light-secondary-medium);
      box-shadow: var(--parsec-shadow-input);
      cursor: pointer;
    }

    &-button {
      --color: var(--parsec-color-light-secondary-hard-grey);
      --background: none;
      --background-hover: var(--parsec-color-light-secondary-disabled);
      --border-radius: var(--parsec-radius-6);
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
        display: flex;
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

      &.disabled {
        .toggle-view-button__icon,
        .toggle-view-button__label {
          opacity: 0.8;
        }

        .toggle-view-button__count {
          background: var(--parsec-color-light-secondary-soft-text);
          color: var(--parsec-color-light-secondary-white);
          padding: 3px 0.4rem;
        }
      }
    }

    &-unavailable {
      top: -0.65rem;
      right: -2rem;
      color: var(--parsec-color-light-secondary-white);
      align-self: center;
      padding: 0.125rem 0.5rem;
      margin-right: 0.25rem;
      background: var(--parsec-color-light-secondary-text);
      border-radius: var(--parsec-radius-8);
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

.invitations-error-message {
  max-width: 30rem;
  width: 100%;
  margin: 1rem;

  @include ms.responsive-breakpoint('md') {
    max-width: 100%;
  }
}
</style>
