<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page
    class="modal-stepper"
    :class="UserJoinOrganizationStep[pageStep]"
  >
    <ms-wizard-stepper
      v-if="pageStep > UserJoinOrganizationStep.WaitForHost && isLargeDisplay"
      :current-index="pageStep - 2"
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
        <ion-title
          v-if="getStep(pageStep).title !== ''"
          class="modal-header__title title-h2"
        >
          {{ $msTranslate(getStep(pageStep).title) }}
        </ion-title>
        <ion-text
          v-if="getStep(pageStep).subtitle !== ''"
          class="modal-header__text body"
        >
          {{ $msTranslate(getStep(pageStep).subtitle) }}
        </ion-text>
      </ion-header>
      <small-display-step-modal-header
        v-else
        @close-clicked="cancelModal()"
        :title="getStep(pageStep).title"
        :subtitle="getStep(pageStep).subtitle"
        :step="{
          icon: personAdd,
          title: 'HomePage.noExistingOrganization.joinOrganization',
          current: getStep(pageStep).currentStep,
          total: 4,
        }"
      />
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
          </div>
        </div>

        <!-- part 2 (host code)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.GetHostSasCode"
          class="step"
        >
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
        <ion-buttons
          slot="primary"
          class="modal-footer-buttons"
        >
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
        </ion-buttons>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { IonButton, IonButtons, IonFooter, IonHeader, IonIcon, IonPage, IonText, IonTitle, modalController } from '@ionic/vue';
import SmallDisplayStepModalHeader from '@/components/header/SmallDisplayStepModalHeader.vue';
import { getDefaultDeviceName } from '@/common/device';
import ChooseAuthentication from '@/components/devices/ChooseAuthentication.vue';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import UserInformation from '@/components/users/UserInformation.vue';
import {
  AccessStrategy,
  CancelledGreetingAttemptReason,
  ClaimInProgressErrorTag,
  ClaimerRetrieveInfoErrorTag,
  DeviceSaveStrategy,
  DeviceSaveStrategyPassword,
  DeviceSaveStrategyTag,
  UserClaim,
  parseParsecAddr,
  ParsedParsecAddrTag,
  OrganizationID,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import {
  Answer,
  MsModalResult,
  askQuestion,
  MsInformativeText,
  MsSpinner,
  MsWizardStepper,
  Translatable,
  asyncComputed,
  useWindowSize,
} from 'megashark-lib';
import { close, personAdd } from 'ionicons/icons';
import { computed, onMounted, ref, Ref } from 'vue';

enum UserJoinOrganizationStep {
  WaitForHost = 1,
  GetHostSasCode = 2,
  ProvideGuestCode = 3,
  GetUserInfo = 4,
  Authentication = 5,
  Finish = 6,
}

const { isLargeDisplay } = useWindowSize();
const pageStep = ref(UserJoinOrganizationStep.WaitForHost);
const userInfoPage = ref();
const authChoice = ref();
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

interface StepInfo {
  title: Translatable;
  subtitle: Translatable;
  currentStep?: number;
}

function getStep(step: UserJoinOrganizationStep): StepInfo {
  switch (step) {
    case UserJoinOrganizationStep.WaitForHost: {
      return {
        title: 'JoinOrganization.titles.waitForHost',
        subtitle: 'JoinOrganization.subtitles.waitForHost',
      };
    }
    case UserJoinOrganizationStep.GetHostSasCode: {
      return {
        title: 'JoinOrganization.titles.getHostCode',
        subtitle: 'JoinOrganization.subtitles.getHostCode',
        currentStep: 1,
      };
    }
    case UserJoinOrganizationStep.ProvideGuestCode: {
      return {
        title: 'JoinOrganization.titles.provideGuestCode',
        subtitle: 'JoinOrganization.subtitles.provideGuestCode',
        currentStep: 2,
      };
    }
    case UserJoinOrganizationStep.GetUserInfo: {
      return {
        title: 'JoinOrganization.titles.getUserInfo',
        subtitle: 'JoinOrganization.subtitles.getUserInfo',
        currentStep: 3,
      };
    }
    case UserJoinOrganizationStep.Authentication: {
      return {
        title: 'JoinOrganization.titles.getAuthentication',
        subtitle: 'JoinOrganization.subtitles.getAuthentication',
        currentStep: 4,
      };
    }
    case UserJoinOrganizationStep.Finish: {
      return {
        title: { key: 'JoinOrganization.titles.finish', data: { org: organizationName.value } },
        subtitle: 'JoinOrganization.subtitles.finish',
      };
    }
  }
}

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
  if (pageStep.value === UserJoinOrganizationStep.GetUserInfo && !(await userInfoPage.value.areFieldsCorrect())) {
    return false;
  } else if (pageStep.value === UserJoinOrganizationStep.Authentication) {
    return await authChoice.value.areFieldsCorrect();
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
    const strategy = authChoice.value.getSaveStrategy();
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
    const result = await claimer.value.doClaim(deviceName, userInfoPage.value.fullName, userInfoPage.value.email);
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
    const saveStrategy: DeviceSaveStrategy = authChoice.value.getSaveStrategy();
    const accessStrategy =
      saveStrategy.tag === DeviceSaveStrategyTag.Keyring
        ? AccessStrategy.useKeyring(claimer.value.device)
        : AccessStrategy.usePassword(claimer.value.device, (saveStrategy as DeviceSaveStrategyPassword).password);
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
      userInfoPage.value.setFocus();
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

  if (userInfoPage.value) {
    userInfoPage.value.email = retrieveResult.value.claimerEmail;
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
