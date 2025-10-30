<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page
    class="modal-stepper"
    :class="GreetUserStep[pageStep]"
  >
    <ms-wizard-stepper
      v-show="pageStep > GreetUserStep.WaitForGuest && pageStep < GreetUserStep.Summary && isLargeDisplay"
      :current-index="getStepperIndex()"
      :titles="['UsersPage.greet.steps.hostCode', 'UsersPage.greet.steps.guestCode', 'UsersPage.greet.steps.contactDetails']"
    />
    <ion-button
      slot="icon-only"
      @click="cancelModal()"
      class="closeBtn"
      v-if="pageStep !== GreetUserStep.Summary && isLargeDisplay"
    >
      <ion-icon
        :icon="close"
        class="closeBtn__icon"
      />
    </ion-button>
    <div class="modal">
      <ion-header
        class="modal-header"
        v-if="isLargeDisplay"
      >
        <ion-title class="modal-header__title title-h3">
          {{ $msTranslate(steps[pageStep]?.title) }}
        </ion-title>
        <ion-text
          class="modal-header__text body"
          v-if="steps[pageStep]?.subtitle"
        >
          {{ $msTranslate(steps[pageStep]?.subtitle) }}
        </ion-text>
      </ion-header>
      <template v-else>
        <small-display-modal-header
          v-if="pageStep === GreetUserStep.WaitForGuest || pageStep === GreetUserStep.Summary"
          @close-clicked="cancelModal()"
          :title="steps[pageStep]?.title"
        />
        <small-display-step-modal-header
          v-else
          @close-clicked="cancelModal()"
          :title="'UsersPage.greet.titles.waitForGuest'"
          :icon="personAdd"
          :steps="steps.slice(1, steps.length - 1)"
          :current-step="pageStep - 1"
        />
      </template>
      <div class="modal-content inner-content">
        <!-- waiting step -->
        <div
          v-show="pageStep === GreetUserStep.WaitForGuest"
          class="step"
        >
          <ms-informative-text>
            {{ $msTranslate('UsersPage.greet.subtitles.waitForGuest1') }}
          </ms-informative-text>
          <ms-informative-text>
            {{ $msTranslate('UsersPage.greet.subtitles.waitForGuest2') }}
          </ms-informative-text>
          <ms-report-text :theme="MsReportTheme.Info">
            {{ $msTranslate('UsersPage.greet.subtitles.waitForGuestVersionInfo') }}
          </ms-report-text>
        </div>

        <!-- give code step -->
        <div
          v-show="pageStep === GreetUserStep.ProvideHostSasCode"
          class="step host-code"
        >
          <sas-code-provide :code="greeter.hostSASCode" />
        </div>

        <!-- choose code step -->
        <div
          v-show="pageStep === GreetUserStep.GetGuestSasCode"
          class="step"
        >
          <ms-report-text :theme="MsReportTheme.Info">
            {{ $msTranslate('SasCodeChoice.securityInfo') }}
          </ms-report-text>
          <sas-code-choice
            :disabled="querying"
            :choices="greeter.SASCodeChoices"
            @select="selectGuestSas"
          />
        </div>

        <!-- Waiting guest info step -->
        <div
          v-show="pageStep === GreetUserStep.WaitForGuestInfo"
          class="step"
        >
          <ms-informative-text>
            {{ $msTranslate('UsersPage.greet.subtitles.getUserInfo') }}
          </ms-informative-text>
        </div>

        <!-- Check guest info step -->
        <div
          v-show="pageStep === GreetUserStep.CheckGuestInfo"
          class="step user-info-page"
        >
          <user-information
            class="user-details"
            :default-email="''"
            :default-name="''"
            :email-enabled="false"
            ref="guestInfoPage"
            @on-enter-keyup="nextStep()"
            @field-update="updateCanGoForward"
          />
          <ms-dropdown
            class="dropdown"
            :title="'UsersPage.greet.profileDropdownTitle'"
            :label="'UsersPage.greet.profileDropdownPlaceholder'"
            :options="profileOptions"
            @change="setUserProfile"
          />
        </div>

        <!-- Final Step -->
        <div
          v-show="pageStep === GreetUserStep.Summary"
          v-if="guestInfoPageRef"
          class="final-step"
        >
          <user-avatar-name
            class="avatar"
            :user-name="guestInfoPageRef.fullName"
            :user-avatar="guestInfoPageRef.fullName"
          />
          <div class="user-info">
            <div class="user-info__email">
              <ion-text class="info-label body">{{ $msTranslate('UsersPage.success.email') }}</ion-text>
              <ion-text class="info-cell cell">{{ guestInfoPageRef.email }}</ion-text>
            </div>
            <div class="user-info__role">
              <ion-text class="info-label body">{{ $msTranslate('UsersPage.success.profile') }}</ion-text>
              <tag-profile :profile="profile ? profile : UserProfile.Outsider" />
            </div>
          </div>
        </div>
      </div>
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
            v-show="waitingForGuest || querying"
            class="spinner-container"
          >
            <ion-text
              class="subtitles-normal"
              v-show="!querying"
            >
              {{ $msTranslate('UsersPage.greet.waiting') }}
            </ion-text>
            <ms-spinner class="spinner" />
          </div>
        </div>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { IonButton, IonFooter, IonHeader, IonIcon, IonPage, IonText, IonTitle, modalController } from '@ionic/vue';
import SmallDisplayStepModalHeader from '@/components/header/SmallDisplayStepModalHeader.vue';
import SmallDisplayModalHeader from '@/components/header/SmallDisplayModalHeader.vue';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import TagProfile from '@/components/users/TagProfile.vue';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import UserInformation from '@/components/users/UserInformation.vue';
import { CancelledGreetingAttemptReason, GreetInProgressErrorTag, UserGreet, UserInvitation, UserProfile } from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import {
  Answer,
  MsModalResult,
  MsOptions,
  askQuestion,
  MsDropdown,
  MsInformativeText,
  MsSpinner,
  MsWizardStepper,
  Translatable,
  MsDropdownChangeEvent,
  useWindowSize,
  MsReportText,
  MsReportTheme,
} from 'megashark-lib';
import { close, personAdd } from 'ionicons/icons';
import { Ref, computed, onMounted, ref, useTemplateRef } from 'vue';

enum GreetUserStep {
  WaitForGuest = 0,
  ProvideHostSasCode = 1,
  GetGuestSasCode = 2,
  WaitForGuestInfo = 3,
  CheckGuestInfo = 4,
  Summary = 5,
}

const { isLargeDisplay } = useWindowSize();
const pageStep = ref(GreetUserStep.WaitForGuest);
const props = defineProps<{
  invitation: UserInvitation;
  informationManager: InformationManager;
}>();

const profile: Ref<UserProfile | null> = ref(null);
const guestInfoPageRef = useTemplateRef<InstanceType<typeof UserInformation>>('guestInfoPage');
const canGoForward = ref(false);
const waitingForGuest = ref(true);
const greeter = ref(new UserGreet());
const cancelled = ref(false);
const querying = ref(false);

const profileOptions: MsOptions = new MsOptions([
  {
    key: UserProfile.Admin,
    label: 'UsersPage.profile.admin.label',
    description: 'UsersPage.profile.admin.description',
  },
  {
    key: UserProfile.Standard,
    label: 'UsersPage.profile.standard.label',
    description: 'UsersPage.profile.standard.description',
  },
  {
    key: UserProfile.Outsider,
    label: 'UsersPage.profile.outsider.label',
    description: 'UsersPage.profile.outsider.description',
  },
]);

const steps = ref([
  {
    title: 'UsersPage.greet.titles.waitForGuest',
  },
  {
    title: 'UsersPage.greet.titles.provideHostCode',
    subtitle: 'UsersPage.greet.subtitles.provideHostCode',
  },
  { title: 'UsersPage.greet.titles.getGuestCode', subtitle: 'UsersPage.greet.subtitles.getGuestCode' },
  {
    title: 'UsersPage.greet.titles.contactDetails',
  },
  {
    title: 'UsersPage.greet.titles.contactDetails',
    subtitle: 'UsersPage.greet.subtitles.checkUserInfo',
  },
  {
    title: 'UsersPage.greet.titles.summary',
  },
]);

async function setUserProfile(event: MsDropdownChangeEvent): Promise<void> {
  profile.value = event.option.key;
  await updateCanGoForward();
}

async function updateCanGoForward(): Promise<void> {
  if (pageStep.value === GreetUserStep.WaitForGuest && waitingForGuest.value === true) {
    canGoForward.value = false;
  } else if (pageStep.value === GreetUserStep.CheckGuestInfo) {
    canGoForward.value = profile.value !== null && ((await guestInfoPageRef.value?.areFieldsCorrect()) ?? false);
  } else {
    canGoForward.value = true;
  }
}

function getNextButtonText(): string | undefined {
  if (pageStep.value === GreetUserStep.WaitForGuest) {
    return 'UsersPage.greet.actions.start';
  } else if (pageStep.value === GreetUserStep.Summary) {
    return 'UsersPage.greet.actions.finish';
  } else if (pageStep.value === GreetUserStep.CheckGuestInfo) {
    return 'UsersPage.greet.actions.approve';
  }
  return undefined;
}

async function selectGuestSas(code: string | null): Promise<void> {
  if (!code) {
    await showErrorAndRestart('UsersPage.greet.errors.noneCodeSelected');
    return;
  }
  try {
    querying.value = true;
    if (code === greeter.value.correctSASCode) {
      const result = await greeter.value.signifyTrust();
      if (result.ok) {
        await nextStep();
      } else {
        if (result.error.tag === GreetInProgressErrorTag.GreetingAttemptCancelled) {
          switch (result.error.reason) {
            case CancelledGreetingAttemptReason.ManuallyCancelled:
              await showErrorAndRestart('UsersPage.greet.errors.claimer.manuallyCancelled');
              break;
            default:
              await showErrorAndRestart('UsersPage.greet.errors.claimer.default');
              break;
          }
        } else {
          await showErrorAndRestart({ key: 'UsersPage.greet.errors.unexpected', data: { reason: result.error.tag } });
        }
      }
    } else {
      await greeter.value.denyTrust();
      await showErrorAndRestart('UsersPage.greet.errors.invalidCodeSelected');
    }
  } finally {
    querying.value = false;
  }
}

async function restartProcess(): Promise<void> {
  await greeter.value.abort();
  await startProcess();
}

async function startProcess(): Promise<void> {
  pageStep.value = GreetUserStep.WaitForGuest;
  waitingForGuest.value = true;
  const result = await greeter.value.startGreet(props.invitation.token);
  if (!result.ok) {
    props.informationManager.present(
      new Information({
        message: 'UsersPage.greet.errors.startFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
    await cancelModal();
    return;
  }
  const waitResult = await greeter.value.initialWaitGuest();
  if (!waitResult.ok) {
    let message: Translatable = '';
    let level: InformationLevel;
    switch (waitResult.error.tag) {
      case GreetInProgressErrorTag.Cancelled:
        message = 'UsersPage.greet.cancelled';
        level = InformationLevel.Info;
        break;
      default:
        message = 'UsersPage.greet.errors.startFailed';
        level = InformationLevel.Error;
        break;
    }
    props.informationManager.present(
      new Information({
        message: message,
        level: level,
      }),
      PresentationMode.Toast,
    );
    await cancelModal();
    return;
  }
  waitingForGuest.value = false;
  await updateCanGoForward();
}

function getStepperIndex(): number {
  if (pageStep.value <= GreetUserStep.ProvideHostSasCode) {
    return 0;
  } else if (pageStep.value === GreetUserStep.GetGuestSasCode) {
    return 1;
  } else {
    return 2;
  }
}

const nextButtonIsVisible = computed(() => {
  if (querying.value) {
    return false;
  } else if (pageStep.value === GreetUserStep.WaitForGuest && waitingForGuest.value) {
    return false;
  } else if (pageStep.value === GreetUserStep.ProvideHostSasCode) {
    return false;
  } else if (pageStep.value === GreetUserStep.GetGuestSasCode) {
    return false;
  } else if (pageStep.value === GreetUserStep.WaitForGuestInfo) {
    return false;
  }
  return true;
});

async function cancelModal(): Promise<void> {
  let answer = Answer.Yes;
  if (pageStep.value !== GreetUserStep.Summary && pageStep.value !== GreetUserStep.WaitForGuest) {
    answer = await askQuestion('UsersPage.greet.cancelConfirm', 'UsersPage.greet.cancelConfirmSubtitle', {
      yesIsDangerous: true,
      keepMainModalHiddenOnYes: true,
      yesText: 'UsersPage.greet.cancelYes',
      noText: 'UsersPage.greet.cancelNo',
      backdropDismiss: false,
    });
  }

  if (answer === Answer.Yes) {
    cancelled.value = true;
    await greeter.value.abort();
    await modalController.dismiss(null, MsModalResult.Cancel);
  }
}

async function showErrorAndRestart(message: Translatable): Promise<void> {
  if (cancelled.value) {
    return;
  }
  props.informationManager.present(
    new Information({
      message: message,
      level: InformationLevel.Error,
    }),
    PresentationMode.Toast,
  );
  await restartProcess();
}

async function nextStep(): Promise<void> {
  await updateCanGoForward();
  if (!canGoForward.value) {
    return;
  }
  if (pageStep.value === GreetUserStep.Summary) {
    props.informationManager.present(
      new Information({
        message: {
          key: 'UsersPage.greet.success',
          data: {
            user: guestInfoPageRef.value?.fullName,
          },
        },
        level: InformationLevel.Success,
      }),
      PresentationMode.Toast,
    );
    await modalController.dismiss({}, MsModalResult.Confirm);
    return;
  } else if (pageStep.value === GreetUserStep.CheckGuestInfo && profile.value) {
    const fullName = guestInfoPageRef.value?.fullName?.trim();
    const email = guestInfoPageRef.value?.email?.trim();

    if (!fullName || !email) {
      await showErrorAndRestart('UsersPage.greet.errors.missingUserInfo');
      return;
    }

    querying.value = true;
    const result = await greeter.value.createUser({ label: fullName, email: email }, profile.value);
    querying.value = false;
    if (!result.ok) {
      await showErrorAndRestart('UsersPage.greet.errors.createUserFailed');
      return;
    }
  }

  pageStep.value = pageStep.value + 1;

  await updateCanGoForward();

  if (pageStep.value === GreetUserStep.ProvideHostSasCode) {
    waitingForGuest.value = true;
    const result = await greeter.value.waitGuestTrust();
    waitingForGuest.value = false;
    if (result.ok) {
      await nextStep();
    } else {
      switch (result.error.tag) {
        case GreetInProgressErrorTag.PeerReset:
          await showErrorAndRestart('UsersPage.greet.errors.peerResetCode');
          break;
        case GreetInProgressErrorTag.GreetingAttemptCancelled:
          switch (result.error.reason) {
            case CancelledGreetingAttemptReason.InvalidSasCode:
              await showErrorAndRestart('UsersPage.greet.errors.claimer.invalidSasCode');
              break;
            case CancelledGreetingAttemptReason.ManuallyCancelled:
              await showErrorAndRestart('UsersPage.greet.errors.claimer.manuallyCancelled');
              break;
            case CancelledGreetingAttemptReason.AutomaticallyCancelled:
              await showErrorAndRestart('UsersPage.greet.errors.claimer.automaticallyCancelled');
              break;
            default:
              await showErrorAndRestart('UsersPage.greet.errors.claimer.default');
              break;
          }
          break;
        default:
          await showErrorAndRestart({ key: 'UsersPage.greet.errors.unexpected', data: { reason: result.error.tag } });
          break;
      }
    }
  } else if (pageStep.value === GreetUserStep.WaitForGuestInfo) {
    waitingForGuest.value = true;
    const result = await greeter.value.getClaimRequests();
    waitingForGuest.value = false;
    if (result.ok && guestInfoPageRef.value) {
      guestInfoPageRef.value.fullName = greeter.value.requestedHumanHandle?.label || '';
      guestInfoPageRef.value.email = greeter.value.requestedHumanHandle?.email || '';
      await nextStep();
    } else {
      await showErrorAndRestart('UsersPage.greet.errors.retrieveUserInfoFailed');
    }
  }
}

onMounted(async () => {
  await startProcess();
});
</script>

<style scoped lang="scss">
.final-step {
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  color: var(--parsec-color-light-secondary-text);
  gap: 1rem;

  @include ms.responsive-breakpoint('sm') {
    padding-inline: 1.5rem;
  }

  .avatar {
    padding-left: 0.725rem;
    border-left: 2px solid var(--parsec-color-light-secondary-disabled);
  }

  .user-info {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    background: var(--parsec-color-light-secondary-background);
    padding: 1rem;
    border-radius: var(--parsec-radius-6);

    & > [class^='user-info'] {
      display: flex;
      align-items: center;
      gap: 1rem;
    }

    .info-label {
      color: var(--parsec-color-light-secondary-grey);
      flex-shrink: 0;
    }

    .info-cell {
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
  }
}

.spinner {
  padding-bottom: 0.4rem;
}

.user-details {
  width: 100%;
}
</style>
