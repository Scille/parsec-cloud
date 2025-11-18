<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page
    class="modal-stepper"
    :class="UserJoinOrganizationStep[pageStep]"
  >
    <ms-wizard-stepper
      v-if="pageStep > UserJoinOrganizationStep.WaitForHost && isLargeDisplay"
      :current-index="pageStep - 1"
      :titles="[
        'JoinOrganization.stepTitles.GetHostCode',
        'JoinOrganization.stepTitles.ProvideGuestCode',
        'JoinOrganization.stepTitles.ContactDetails',
        'JoinOrganization.stepTitles.Authentication',
      ]"
    />
    <ion-button
      slot="icon-only"
      @click="cancelModal()"
      v-if="pageStep !== UserJoinOrganizationStep.Finish && pageStep !== UserJoinOrganizationStep.Authentication && isLargeDisplay"
      class="closeBtn"
    >
      <ion-icon
        :icon="close"
        class="closeBtn__icon"
      />
    </ion-button>
    <!-- v-if pageStep === WaitForHost so add class wizardTrue -->
    <div
      class="modal"
      :class="{
        wizardTrue: pageStep > 1 && isLargeDisplay,
      }"
    >
      <ion-header
        class="modal-header"
        v-if="isLargeDisplay"
      >
        <ion-title class="modal-header__title title-h3">
          {{ $msTranslate(steps[pageStep]?.title) }}
        </ion-title>
        <ion-text
          v-if="steps[pageStep]?.subtitle"
          class="modal-header__text body"
        >
          {{ $msTranslate(steps[pageStep]?.subtitle) }}
        </ion-text>
      </ion-header>

      <template v-else>
        <small-display-modal-header
          v-if="pageStep === UserJoinOrganizationStep.WaitForHost || pageStep === UserJoinOrganizationStep.Finish"
          @close-clicked="cancelModal()"
          :hide-close-button="pageStep === UserJoinOrganizationStep.Finish"
          :title="steps[pageStep]?.title"
        />
        <small-display-step-modal-header
          v-else
          @close-clicked="cancelModal()"
          :title="'HomePage.noExistingOrganization.joinOrganization'"
          :icon="personAdd"
          :steps="steps.slice(1, steps.length - 1)"
          :current-step="pageStep - 1"
        />
      </template>
      <!-- modal content: create component for each part-->
      <div class="modal-content inner-content">
        <!-- part 1 (wait for host)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.WaitForHost"
          class="step organization-name"
        >
          <div class="organization-name-content">
            <ms-informative-text>
              {{ $msTranslate('JoinOrganization.instructions.start.first') }}
            </ms-informative-text>
            <ms-informative-text v-if="!claimer.preferredGreeter && !claimer.greeter">
              {{
                $msTranslate({
                  key: 'JoinOrganization.instructions.start.secondWithoutPreferredGreeter',
                  data: {
                    organizationName: organizationName,
                  },
                })
              }}
            </ms-informative-text>
            <ms-informative-text v-if="claimer.preferredGreeter && !claimer.greeter">
              {{
                $msTranslate({
                  key: 'JoinOrganization.instructions.start.secondWithPreferredGreeter',
                  data: {
                    greeter: claimer.preferredGreeter.label,
                    greeterEmail: claimer.preferredGreeter.email,
                    organizationName: organizationName,
                  },
                })
              }}
            </ms-informative-text>
            <ms-informative-text v-if="claimer.greeter">
              {{
                $msTranslate({
                  key: 'JoinOrganization.instructions.start.secondWithGreeterReady',
                  data: {
                    greeter: claimer.greeter.label,
                    greeterEmail: claimer.greeter.email,
                    organizationName: organizationName,
                  },
                })
              }}
            </ms-informative-text>
            <ms-report-text :theme="MsReportTheme.Info">
              {{ $msTranslate('UsersPage.greet.subtitles.waitForGreeterVersionInfo') }}
            </ms-report-text>
          </div>
        </div>

        <!-- part 2 (host code)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.GetHostSasCode"
          class="step"
        >
          <ms-report-text :theme="MsReportTheme.Info">
            {{ $msTranslate('SasCodeChoice.securityInfo') }}
          </ms-report-text>
          <sas-code-choice
            :disabled="querying"
            :choices="claimer.SASCodeChoices"
            @select="selectHostSas($event)"
          />
        </div>

        <!-- part 3 (guest code)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.ProvideGuestCode"
          class="step guest-code"
        >
          <sas-code-provide :code="claimer.guestSASCode" />
        </div>

        <!-- part 4 (user info)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.GetUserInfo"
          class="step"
          id="get-user-info"
        >
          <user-information
            ref="userInfoPage"
            :email-enabled="false"
            :name-enabled="!waitingForHost"
            @field-update="fieldsUpdated = true"
            @on-enter-keyup="nextStep()"
          />
        </div>
        <!-- part 5 (get password)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.Authentication"
          class="step"
          id="get-password"
        >
          <choose-authentication ref="authChoice" />
        </div>
        <!-- part 6 (finish the process)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.Finish"
          class="step"
        >
          <ms-informative-text>
            {{ $msTranslate('JoinOrganization.instructions.finish.first') }}
          </ms-informative-text>
          <ms-informative-text>
            {{ $msTranslate('JoinOrganization.instructions.finish.second') }}
          </ms-informative-text>
        </div>
      </div>
      <!-- the buttons must be only enabled if all fields are filled in -->
      <ion-footer class="modal-footer">
        <div class="modal-footer-buttons">
          <ion-button
            fill="solid"
            size="default"
            id="next-button"
            v-show="nextButtonIsVisible"
            @click="nextStep()"
            :disabled="!canGoForward"
          >
            <span>
              {{ $msTranslate(getNextButtonText()) }}
            </span>
          </ion-button>
          <div
            v-show="waitingForHost || querying"
            class="spinner-container"
          >
            <ms-spinner :title="getSpinnerTitle()" />
          </div>
        </div>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { getDefaultDeviceName } from '@/common/device';
import ChooseAuthentication from '@/components/devices/ChooseAuthentication.vue';
import SmallDisplayModalHeader from '@/components/header/SmallDisplayModalHeader.vue';
import SmallDisplayStepModalHeader from '@/components/header/SmallDisplayStepModalHeader.vue';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import UserInformation from '@/components/users/UserInformation.vue';
import {
  AccessStrategy,
  CancelledGreetingAttemptReason,
  ClaimInProgressErrorTag,
  ClaimerRetrieveInfoErrorTag,
  DeviceSaveStrategy,
  OrganizationID,
  ParsedParsecAddrTag,
  UserClaim,
  parseParsecAddr,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { IonButton, IonFooter, IonHeader, IonIcon, IonPage, IonText, IonTitle, modalController } from '@ionic/vue';
import { close, personAdd } from 'ionicons/icons';
import {
  Answer,
  MsInformativeText,
  MsModalResult,
  MsReportText,
  MsReportTheme,
  MsSpinner,
  MsWizardStepper,
  Translatable,
  askQuestion,
  asyncComputed,
  useWindowSize,
} from 'megashark-lib';
import { Ref, computed, onMounted, ref, useTemplateRef } from 'vue';

enum UserJoinOrganizationStep {
  WaitForHost = 0,
  GetHostSasCode = 1,
  ProvideGuestCode = 2,
  GetUserInfo = 3,
  Authentication = 4,
  Finish = 5,
}

const { isLargeDisplay } = useWindowSize();
const pageStep = ref(UserJoinOrganizationStep.WaitForHost);
const userInfoPageRef = useTemplateRef<InstanceType<typeof UserInformation>>('userInfoPage');
const authChoiceRef = useTemplateRef<InstanceType<typeof ChooseAuthentication>>('authChoice');
const fieldsUpdated = ref(false);
const cancelled = ref(false);
const organizationName: Ref<OrganizationID> = ref('');
const querying = ref(false);

const claimer = ref(new UserClaim());

const props = defineProps<{
  invitationLink: string;
  informationManager: InformationManager;
}>();

const waitingForHost = ref(true);

const steps = computed(() => [
  {
    title: 'JoinOrganization.titles.waitForHost',
    subtitle: 'JoinOrganization.subtitles.waitForHost',
  },
  {
    title: 'JoinOrganization.titles.getHostCode',
    subtitle: 'JoinOrganization.subtitles.getHostCode',
  },
  {
    title: 'JoinOrganization.titles.provideGuestCode',
    subtitle: 'JoinOrganization.subtitles.provideGuestCode',
  },
  {
    title: 'JoinOrganization.titles.getUserInfo',
    subtitle: 'JoinOrganization.subtitles.getUserInfo',
  },
  {
    title: 'JoinOrganization.titles.getAuthentication',
    subtitle: 'JoinOrganization.subtitles.getAuthentication',
  },
  {
    title: { key: 'JoinOrganization.titles.finish', data: { org: organizationName.value } },
    subtitle: 'JoinOrganization.subtitles.finish',
  },
]);

async function showErrorAndRestart(message: Translatable): Promise<void> {
  props.informationManager.present(
    new Information({
      message: message,
      level: InformationLevel.Error,
    }),
    PresentationMode.Toast,
  );
  await restartProcess();
}

async function selectHostSas(selectedCode: string | null): Promise<void> {
  if (!selectedCode) {
    await showErrorAndRestart('JoinOrganization.errors.noneCodeSelected');
    return;
  }
  try {
    querying.value = true;
    if (selectedCode === claimer.value.correctSASCode) {
      const result = await claimer.value.signifyTrust();
      if (result.ok) {
        await nextStep();
      } else {
        if (result.error.tag === ClaimInProgressErrorTag.GreetingAttemptCancelled) {
          switch (result.error.reason) {
            case CancelledGreetingAttemptReason.ManuallyCancelled:
              await showErrorAndRestart('JoinOrganization.errors.greeter.manuallyCancelled');
              break;
            default:
              await showErrorAndRestart('JoinOrganization.errors.greeter.default');
              break;
          }
        } else {
          await showErrorAndRestart({ key: 'JoinOrganization.errors.unexpected', data: { reason: result.error.tag } });
        }
      }
    } else {
      await claimer.value.denyTrust();
      await showErrorAndRestart('JoinOrganization.errors.invalidCodeSelected');
    }
  } finally {
    querying.value = false;
  }
}
function getNextButtonText(): Translatable | undefined {
  switch (pageStep.value) {
    case UserJoinOrganizationStep.WaitForHost:
      return { key: 'JoinOrganization.continueWith', data: { greeter: claimer.value.greeter?.label } };
    case UserJoinOrganizationStep.GetUserInfo:
      return { key: 'JoinOrganization.validateUserInfo' };
    case UserJoinOrganizationStep.Authentication:
      return { key: 'JoinOrganization.createDevice' };
    case UserJoinOrganizationStep.Finish:
      return { key: 'JoinOrganization.logIn' };
    default:
      return undefined;
  }
}

function getSpinnerTitle(): Translatable | undefined {
  switch (pageStep.value) {
    case UserJoinOrganizationStep.WaitForHost:
      return 'JoinOrganization.waitingForHost';
    default:
      return !querying.value ? 'JoinOrganization.waitingForHost' : undefined;
  }
}

const nextButtonIsVisible = computed(() => {
  return (
    (pageStep.value === UserJoinOrganizationStep.WaitForHost && !waitingForHost.value) ||
    (pageStep.value === UserJoinOrganizationStep.GetUserInfo && !waitingForHost.value) ||
    pageStep.value === UserJoinOrganizationStep.Authentication ||
    pageStep.value === UserJoinOrganizationStep.Finish
  );
});

const canGoForward = asyncComputed(async () => {
  if (fieldsUpdated.value) {
    fieldsUpdated.value = false;
  }
  if (pageStep.value === UserJoinOrganizationStep.GetUserInfo && !(await userInfoPageRef.value?.areFieldsCorrect())) {
    return false;
  } else if (pageStep.value === UserJoinOrganizationStep.Authentication) {
    return await authChoiceRef.value?.areFieldsCorrect();
  }
  return true;
});

async function cancelModal(): Promise<void> {
  const answer = await askQuestion('JoinOrganization.cancelConfirm', 'JoinOrganization.cancelConfirmSubtitle', {
    keepMainModalHiddenOnYes: true,
    yesText: 'JoinOrganization.cancelYes',
    noText: 'JoinOrganization.cancelNo',
    yesIsDangerous: true,
    backdropDismiss: false,
  });

  if (answer === Answer.Yes) {
    cancelled.value = true;
    await claimer.value.abort();
    modalController.dismiss(null, MsModalResult.Cancel);
  }
}

async function nextStep(): Promise<void> {
  if (!canGoForward.value) {
    return;
  }
  if (pageStep.value === UserJoinOrganizationStep.Authentication) {
    const strategy = authChoiceRef.value?.getSaveStrategy();
    if (!strategy) return;
    const result = await claimer.value.finalize(strategy);
    if (!result.ok) {
      // Error here is quite bad because the user has been created in the organization
      // but we fail to save the device. Don't really know what to do here.
      // So we just keep the dialog as is, they can click the button again, hoping it will work.
      props.informationManager.present(
        new Information({
          message: 'JoinOrganization.errors.saveDeviceFailed',
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      return;
    }
  } else if (pageStep.value === UserJoinOrganizationStep.GetUserInfo) {
    waitingForHost.value = true;
    const deviceName = getDefaultDeviceName();
    const userInfo = userInfoPageRef.value;
    if (!userInfo) return;
    const result = await claimer.value.doClaim(deviceName, userInfo.fullName, userInfo.email);
    if (!result.ok) {
      await showErrorAndRestart('JoinOrganization.errors.sendUserInfoFailed');
      return;
    }
    waitingForHost.value = false;
  } else if (pageStep.value === UserJoinOrganizationStep.Finish) {
    if (!claimer.value.device) {
      return;
    }
    const notification = new Information({
      message: 'JoinOrganization.successMessage',
      level: InformationLevel.Success,
    });
    props.informationManager.present(notification, PresentationMode.Toast | PresentationMode.Console);
    const saveStrategy: DeviceSaveStrategy | undefined = authChoiceRef.value?.getSaveStrategy();
    if (!saveStrategy) return;
    const accessStrategy = await AccessStrategy.fromSaveStrategy(claimer.value.device, saveStrategy);
    await modalController.dismiss({ device: claimer.value.device, access: accessStrategy }, MsModalResult.Confirm);
    return;
  }

  pageStep.value += 1;

  if (pageStep.value === UserJoinOrganizationStep.ProvideGuestCode) {
    waitingForHost.value = true;
    const result = await claimer.value.waitHostTrust();
    if (result.ok) {
      waitingForHost.value = false;
      pageStep.value += 1;
      userInfoPageRef.value?.setFocus();
    } else {
      let message: Translatable = '';
      switch (result.error.tag) {
        case ClaimInProgressErrorTag.PeerReset:
          message = 'JoinOrganization.errors.peerResetCode';
          break;
        case ClaimInProgressErrorTag.Cancelled:
          message = 'JoinOrganization.errors.cancelled';
          break;
        case ClaimInProgressErrorTag.GreetingAttemptCancelled:
          switch (result.error.reason) {
            case CancelledGreetingAttemptReason.InvalidSasCode:
              message = 'JoinOrganization.errors.greeter.invalidSasCode';
              break;
            case CancelledGreetingAttemptReason.ManuallyCancelled:
              message = 'JoinOrganization.errors.greeter.manuallyCancelled';
              break;
            case CancelledGreetingAttemptReason.AutomaticallyCancelled:
              message = 'JoinOrganization.errors.greeter.automaticallyCancelled';
              break;
            default:
              message = 'JoinOrganization.errors.greeter.default';
              break;
          }
          break;
        default:
          message = { key: 'JoinOrganization.errors.unexpected', data: { reason: result.error.tag } };
          break;
      }
      await showErrorAndRestart(message);
    }
  }
}

async function startProcess(): Promise<void> {
  pageStep.value = UserJoinOrganizationStep.WaitForHost;
  waitingForHost.value = true;
  const retrieveResult = await claimer.value.retrieveInfo(props.invitationLink);

  if (!retrieveResult.ok) {
    await claimer.value.abort();
    await modalController.dismiss(null, MsModalResult.Cancel);
    let message: Translatable = '';
    switch (retrieveResult.error.tag) {
      case ClaimerRetrieveInfoErrorTag.AlreadyUsedOrDeleted:
        message = 'JoinOrganization.errors.tokenAlreadyUsed';
        break;
      case ClaimerRetrieveInfoErrorTag.NotFound:
        message = 'JoinOrganization.errors.invitationNotFound';
        break;
      case ClaimerRetrieveInfoErrorTag.Offline:
        message = 'JoinOrganization.errors.offline';
        break;
      default:
        message = { key: 'JoinOrganization.errors.unexpected', data: { reason: retrieveResult.error.tag } };
    }

    await props.informationManager.present(
      new Information({
        message: message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    return;
  }

  if (userInfoPageRef.value) {
    userInfoPageRef.value.email = retrieveResult.value.claimerEmail;
  }
  const waitResult = await claimer.value.initialWaitAllAdministrators();
  if (!waitResult.ok && !cancelled.value) {
    await claimer.value.abort();
    await modalController.dismiss(null, MsModalResult.Cancel);
    let message: Translatable = '';
    switch (waitResult.error.tag) {
      case ClaimInProgressErrorTag.ActiveUsersLimitReached:
        message = 'JoinOrganization.errors.usersLimitReached';
        break;
      case ClaimInProgressErrorTag.PeerReset:
        message = 'JoinOrganization.errors.peerReset';
        break;
      case ClaimInProgressErrorTag.GreetingAttemptCancelled:
        message = 'JoinOrganization.errors.greeter.default';
        break;
      case ClaimInProgressErrorTag.AlreadyUsedOrDeleted:
        message = 'JoinOrganization.errors.greeter.deleted';
        break;
      default:
        message = { key: 'JoinOrganization.errors.unexpected', data: { reason: waitResult.error.tag } };
        break;
    }

    await props.informationManager.present(
      new Information({
        message: message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );

    return;
  }

  waitingForHost.value = false;
}

async function restartProcess(): Promise<void> {
  if (pageStep.value !== UserJoinOrganizationStep.WaitForHost) {
    await claimer.value.abort();
  }
  await startProcess();
}

onMounted(async () => {
  const result = await parseParsecAddr(props.invitationLink);
  if (result.ok && result.value.tag === ParsedParsecAddrTag.InvitationUser) {
    organizationName.value = result.value.organizationId;
  }
  await startProcess();
});
</script>

<style lang="scss" scoped>
.modal-stepper {
  overflow: auto;
}

.organization-name {
  display: flex;
  flex-direction: column;

  &-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
}

.guest-code {
  margin: auto;
}
</style>
