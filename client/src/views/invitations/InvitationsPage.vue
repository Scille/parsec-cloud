<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-page class="invitations-page">
    <div
      v-if="isSmallDisplay"
      class="switch-view"
    >
      <ion-button
        v-if="view === InvitationView.EmailInvitation"
        class="switch-view-button email-button"
        size="large"
        @click="openInvitationSwitchModal"
      >
        <ion-icon
          :icon="invitations.length > 0 ? mailUnread : mail"
          class="switch-view-button__icon button-icon-left"
        />
        {{ $msTranslate('InvitationsPage.emailInvitation.tab') }}
        <span
          class="switch-view-button__count"
          :class="{ empty: invitations.length === 0 }"
        >
          {{ invitations.length }}
        </span>
        <ion-icon
          :icon="caretDown"
          class="switch-view-button__toggle button-icon-right"
        />
      </ion-button>

      <ion-button
        v-else
        class="switch-view-button pki-button"
        size="large"
        @click="openInvitationSwitchModal"
      >
        <ion-icon
          :icon="link"
          class="switch-view-button__icon button-icon-left"
        />
        {{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.tab') }}
        <span
          class="switch-view-button__count"
          :class="{ empty: asyncEnrollmentRequests.length === 0 }"
        >
          {{ asyncEnrollmentRequests.length }}
        </span>
        <ion-icon
          :icon="caretDown"
          class="switch-view-button__toggle button-icon-right"
        />
      </ion-button>
    </div>

    <ion-content class="content-scroll">
      <div class="toggle-view-container">
        <div
          class="toggle-view"
          role="tablist"
          aria-label="Invitation view toggle"
          aria-orientation="horizontal"
          v-if="isLargeDisplay"
          @keydown="onTabKeydown"
        >
          <ion-button
            role="tab"
            id="invitation-tab-email"
            class="toggle-view-button email-button"
            :class="{ active: view === InvitationView.EmailInvitation }"
            :aria-selected="view === InvitationView.EmailInvitation"
            aria-controls="invitation-panel"
            :tabindex="view === InvitationView.EmailInvitation ? 0 : -1"
            @click="switchView(InvitationView.EmailInvitation)"
          >
            <ion-icon
              :icon="invitations.length > 0 ? mailUnread : mail"
              class="toggle-view-button__icon button-icon-left"
              aria-hidden="true"
            />
            <ion-text class="toggle-view-button__label">
              <span v-if="windowWidth > WindowSizeBreakpoints.MD">{{ $msTranslate('InvitationsPage.emailInvitation.tabLabel') }}</span>
              <span v-else>{{ $msTranslate('InvitationsPage.emailInvitation.tabShort') }}</span>
            </ion-text>
            <span
              class="toggle-view-button__count"
              :class="{ empty: invitations.length === 0 }"
            >
              {{ invitations.length }}
            </span>
          </ion-button>
          <ion-button
            role="tab"
            id="invitation-tab-pki"
            class="toggle-view-button pki-button"
            :class="{ active: view === InvitationView.AsyncEnrollmentRequest }"
            :aria-selected="view === InvitationView.AsyncEnrollmentRequest"
            aria-controls="invitation-panel"
            :tabindex="view === InvitationView.AsyncEnrollmentRequest ? 0 : -1"
            @click="switchView(InvitationView.AsyncEnrollmentRequest)"
          >
            <ion-icon
              :icon="link"
              class="toggle-view-button__icon button-icon-left"
              aria-hidden="true"
            />
            <ion-text class="toggle-view-button__label">
              <span>{{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.tabLabel') }}</span>
            </ion-text>
            <span
              class="toggle-view-button__count"
              :class="{ empty: asyncEnrollmentRequests.length === 0 }"
            >
              {{ asyncEnrollmentRequests.length }}
            </span>
          </ion-button>
        </div>

        <div class="toggle-view-header">
          <div class="toggle-view-header-text">
            <ion-text class="toggle-view-header__title">
              {{
                view === InvitationView.EmailInvitation
                  ? $msTranslate('InvitationsPage.emailInvitation.tabLabel')
                  : $msTranslate('InvitationsPage.asyncEnrollmentRequest.tabLabel')
              }}
            </ion-text>
          </div>
          <ion-button
            v-if="view === InvitationView.AsyncEnrollmentRequest"
            @click="onCopyJoinLinkClicked"
            id="copy-link-pki-request-button"
            ref="copyJoinRequestLink"
          >
            <ion-icon
              :icon="copy"
              class="button-icon button-icon-left"
            />
            <span>{{ $msTranslate('InvitationsPage.asyncEnrollmentRequest.copyLink') }}</span>
          </ion-button>

          <ion-button
            v-if="view === InvitationView.EmailInvitation"
            @click="inviteUser"
            id="invite-user-button"
          >
            <ion-icon
              :icon="personAdd"
              class="button-icon button-icon-left"
            />
            <span>{{ $msTranslate('InvitationsPage.emailInvitation.inviteUser') }}</span>
          </ion-button>
        </div>

        <div
          class="invitations-container scroll"
          v-if="view === InvitationView.EmailInvitation"
          id="invitation-panel"
          role="tabpanel"
          aria-labelledby="invitation-tab-email"
        >
          <div
            v-show="invitations.length === 0"
            class="no-active"
          >
            <div class="no-active-content">
              <ms-image :image="isDarkTheme ? NoInvitationDark : NoInvitation" />
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
          id="invitation-panel"
          role="tabpanel"
          aria-labelledby="invitation-tab-pki"
        >
          <div
            v-show="asyncEnrollmentRequests.length === 0 && !asyncListError"
            class="no-active"
          >
            <div class="no-active-content">
              <ms-image :image="isDarkTheme ? NoInvitationDark : NoInvitation" />
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
  AcceptFinalizeAsyncEnrollmentIdentityStrategyTag,
  AsyncEnrollmentIdentitySystemTag,
  AsyncEnrollmentUntrusted,
  cancelInvitation,
  ClientAcceptAsyncEnrollmentErrorTag,
  ClientCancelInvitationErrorTag,
  ClientNewUserInvitationError,
  ClientNewUserInvitationErrorTag,
  closeCertificate,
  getAsyncEnrollmentAddr,
  getCurrentAvailableDevice,
  getServerConfig,
  InvitationEmailSentStatus,
  isSmartcardAvailable,
  listAsyncEnrollments,
  listUserInvitations,
  makeAcceptOpenBaoIdentityStrategy,
  makeAcceptPkiIdentityStrategy,
  openCertificate,
  ParsecAddr,
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
import { caretDown, copy, link, mail, mailUnread, personAdd } from 'ionicons/icons';
import {
  Answer,
  askQuestion,
  attachMouseOverTooltip,
  MsImage,
  MsModalResult,
  MsReportText,
  MsReportTheme,
  NoInvitation,
  NoInvitationDark,
  Translatable,
  useTheme,
  useWindowSize,
  WindowSizeBreakpoints,
} from 'megashark-lib';
import { inject, onMounted, onUnmounted, Ref, ref, useTemplateRef, watch } from 'vue';

const { isLargeDisplay, isSmallDisplay, windowWidth } = useWindowSize();
const { isDarkTheme } = useTheme();
const view = ref(InvitationView.EmailInvitation);
const informationManager: Ref<InformationManager> = inject(InformationManagerKey)!;
const eventDistributor: Ref<EventDistributor> = inject(EventDistributorKey)!;
const invitations = ref<Array<UserInvitation>>([]);
const asyncEnrollmentRequests = ref<Array<AsyncEnrollmentUntrusted>>([]);
const serverAddr = ref<ParsecAddr | undefined>(undefined);
const serverConfig = ref<ServerConfig | undefined>(undefined);
const pkiAvailable = ref(false);
const openBaoClient = ref<OpenBaoClient | undefined>(undefined);
const certificate = ref<X509CertificateReference | undefined>(undefined);
const asyncListError = ref('');

const copyJoinRequestLinkRef = useTemplateRef<InstanceType<typeof IonButton>>('copyJoinRequestLink');

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
  const deviceResult = await getCurrentAvailableDevice();
  if (deviceResult.ok) {
    serverAddr.value = deviceResult.value.serverAddr;
    const configResult = await getServerConfig(serverAddr.value);
    if (configResult.ok) {
      serverConfig.value = configResult.value;
    }
    pkiAvailable.value = await isSmartcardAvailable();
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

  watch(copyJoinRequestLinkRef, (button) => {
    if (button?.$el) {
      setTimeout(() => {
        attachMouseOverTooltip(button.$el, 'InvitationsPage.asyncEnrollmentRequest.tooltips.copyLinkInfo');
      }, 500);
    }
  });
});

onUnmounted(async () => {
  if (eventCbId) {
    await eventDistributor.value.removeCallback(eventCbId);
  }
  routeWatchCancel();
});

async function onTabKeydown(event: KeyboardEvent): Promise<void> {
  const tabs = [
    { id: 'invitation-tab-email', view: InvitationView.EmailInvitation },
    { id: 'invitation-tab-pki', view: InvitationView.AsyncEnrollmentRequest },
  ];
  const current = tabs.findIndex((tab) => tab.view === view.value);
  let next: number;
  switch (event.key) {
    case 'ArrowRight':
      next = (current + 1) % tabs.length;
      break;
    case 'ArrowLeft':
      next = (current - 1 + tabs.length) % tabs.length;
      break;
    case 'Home':
      next = 0;
      break;
    case 'End':
      next = tabs.length - 1;
      break;
    default:
      return;
  }
  event.preventDefault();
  await switchView(tabs[next].view);
  document.getElementById(tabs[next].id)?.focus();
}

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
    window.nativeAPI.log('error', `Failed to create invitations: ${lastError.tag} (${lastError.error})`);
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
      window.nativeAPI.log('error', 'PKI not available');
      return;
    }
    const pkiModal = await modalController.create({
      component: AsyncEnrollmentPkiAuthModal,
      cssClass: 'async-enrollment-pki-modal',
    });
    await pkiModal.present();
    const { role, data } = await pkiModal.onWillDismiss();
    await pkiModal.dismiss();
    if (role !== MsModalResult.Confirm || !data.signCertificate || !data.encryptCertificate || !serverAddr.value) {
      return;
    }
    const signCertResult = await openCertificate(data.signCertificate);
    if (!signCertResult.ok) {
      return;
    }
    const encryptCertResult = await openCertificate(data.encryptCertificate);
    if (!encryptCertResult.ok) {
      await closeCertificate(signCertResult.value);
      return;
    }
    strategy = makeAcceptPkiIdentityStrategy(signCertResult.value, encryptCertResult.value);
  } else if (request.identitySystem.tag === AsyncEnrollmentIdentitySystemTag.OpenBao && !openBaoClient.value) {
    if (!serverConfig.value?.openbao || serverConfig.value.openbao.auths.length === 0) {
      return;
    }
    const ssoModal = await modalController.create({
      component: AsyncEnrollmentOpenBaoAuthModal,
      cssClass: 'async-enrollment-openbao-modal',
      componentProps: {
        serverConfig: serverConfig.value,
        action: 'accept',
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
    window.nativeAPI.log('error', `Unknown identity system ${request.identitySystem.tag}`);
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
  if (strategy.tag === AcceptFinalizeAsyncEnrollmentIdentityStrategyTag.PKI) {
    await closeCertificate(strategy.pkiSignPrivateKeyHandle);
    await closeCertificate(strategy.pkiEncryptPrivateKeyHandle);
  }
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
    window.nativeAPI.log('error', `Failed to list invitations: ${result.error.tag} (${result.error.error})`);
  }
}

async function refreshAsyncEnrollmentRequestList(): Promise<void> {
  const result = await listAsyncEnrollments();
  if (result.ok) {
    asyncEnrollmentRequests.value = result.value;
    asyncListError.value = '';
  } else {
    asyncListError.value = 'InvitationsPage.asyncEnrollmentRequest.errors.failToListRequests';
    window.nativeAPI.log('error', `Failed to list pki join requests: ${result.error.tag} (${result.error.error})`);
  }
}

async function refreshAll(): Promise<void> {
  await Promise.allSettled([refreshInvitationList(), refreshAsyncEnrollmentRequestList()]);
}
</script>

<style scoped lang="scss">
.invitations-page {
  padding: ms.spacing('padding-4xl');

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-none');
  }

  .content-scroll {
    --background: transparent;
    display: flex;
  }
}

.toggle-view-container {
  display: flex;
  flex-direction: column;
  width: 100%;
  height: 100%;
  background: ms.color('surface-base-default');
  border: ms.border('thin') solid ms.color('border-base-default');
  border-radius: ms.radius('2xl');
  overflow: hidden;

  @include ms.responsive-breakpoint('sm') {
    border: none;
    border-radius: ms.radius('2xl') ms.radius('2xl') 0 0;
    box-shadow: ms.shadow('strong');
  }

  .toggle-view {
    display: flex;
    flex-shrink: 0;
    gap: ms.spacing('gap-2xl');
    padding: ms.spacing('padding-none') ms.spacing('padding-3xl');
    box-shadow: inset 0 calc(-1 * ms.border('thin')) 0 0 ms.color('border-base-default');
    overflow-x: auto;
    overflow-y: hidden;

    @include ms.responsive-breakpoint('sm') {
      padding: ms.spacing('padding-md') ms.spacing('padding-2xl') ms.spacing('padding-none');
    }

    &-button {
      --background: transparent;
      --background-hover: transparent;
      --color: #{ms.color('text-base-description')};
      --color-hover: #{ms.color('text-base-body')};
      position: relative;
      transition: opacity 150ms ease;

      &::part(native) {
        padding: ms.spacing('padding-3xl') ms.spacing('padding-sm');
        gap: ms.spacing('gap-md');
      }

      &::after {
        content: '';
        position: absolute;
        left: 0;
        right: 0;
        bottom: 0;
        z-index: 1;
        height: ms.border('thick');
        border-radius: ms.radius('full') ms.radius('full') 0 0;
      }

      &__icon {
        flex-shrink: 0;
        font-size: 1.125rem;
      }

      &__label {
        white-space: nowrap;
      }

      &__count {
        @include ms.font('label-xs-medium');
        flex-shrink: 0;
        min-width: 1rem;
        min-height: 1rem;
        width: fit-content;
        margin-left: 0.5rem;
        padding: ms.spacing('padding-none') ms.spacing('padding-sm');
        border: ms.border('thin') solid transparent;
        background: ms.color('surface-error-default');
        border-radius: ms.radius('full');
        color: ms.color('text-on-color-label');
        text-align: center;

        &.empty {
          background: ms.color('surface-neutral-default-subtle');
          color: ms.color('text-base-description');
        }
      }

      &.active {
        --color: #{ms.color('text-brand-default')};
        --color-hover: #{ms.color('text-brand-default')};
        opacity: ms.opacity('10');
        cursor: default;

        &::after {
          background: ms.color('border-brand-default');
        }

        .toggle-view-button__count {
          border-color: transparent;
        }
      }
    }
  }

  .toggle-view-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: ms.spacing('gap-lg');
    flex-shrink: 0;
    padding: ms.spacing('padding-xl') ms.spacing('padding-3xl');
    background: ms.color('surface-base-page');

    @include ms.responsive-breakpoint('sm') {
      padding: ms.spacing('padding-xl') ms.spacing('padding-2xl') ms.spacing('padding-md');
    }

    &__title {
      @include ms.font('heading-h4');
      color: ms.color('text-base-heading');
    }

    #copy-link-pki-request-button,
    #invite-user-button {
      margin: 0 0 0 auto;
      flex-shrink: 0;
    }
  }

  .invitations-container {
    flex: 1 1 0;
    min-height: 0;
    height: auto;
    padding: ms.spacing('padding-none');
  }
}

.no-active {
  @include ms.font('body-lg-regular');
  width: 100%;
  height: 100%;
  max-width: 50rem;
  color: ms.color('text-base-description');
  display: flex;
  margin: auto;
  align-items: center;

  &__image {
    @include ms.responsive-breakpoint('sm') {
      max-width: 5rem;
    }
  }

  &-content {
    display: flex;
    height: fit-content;
    width: 100%;
    text-align: center;
    flex-direction: column;
    gap: ms.spacing('gap-3xl');
    align-items: center;
    padding: ms.spacing('padding-5xl') ms.spacing('padding-3xl');
  }
}

.switch-view {
  display: flex;
  justify-content: center;
  padding: ms.spacing('padding-lg') ms.spacing('padding-3xl') ms.spacing('padding-4xl');

  .switch-view-button {
    width: 100%;
    --background: #{ms.color('surface-base-default')};
    --background-hover: #{ms.color('surface-neutral-default-subtle-hover')};
    --color: #{ms.color('text-brand-default')};
    box-shadow: ms.shadow('input');
    border-radius: ms.radius('lg');

    &::part(native) {
      border: ms.border('thin') solid ms.color('border-base-default');
      padding: ms.spacing('padding-xl') ms.spacing('padding-3xl');
    }

    &__icon {
      font-size: 1.125rem;
      margin-right: 0.5rem;
    }

    &__count {
      @include ms.font('label-sm-medium');
      background: ms.color('surface-error-default');
      color: ms.color('text-on-color-label');
      border-radius: ms.radius('lg');
      padding: 0.1rem 0.35rem;
      margin-left: 0.5rem;

      &.empty {
        background: ms.color('surface-neutral-default-subtle');
        color: ms.color('text-base-description');
      }
    }

    &__toggle {
      font-size: 1rem;
      margin-left: auto;
      color: ms.color('text-base-description');
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
