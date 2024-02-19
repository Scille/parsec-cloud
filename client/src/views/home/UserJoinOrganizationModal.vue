<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal-stepper">
    <ms-wizard-stepper
      v-if="pageStep > UserJoinOrganizationStep.WaitForHost"
      :current-index="pageStep - 2"
      :titles="[
        $t('JoinOrganization.stepTitles.GetHostCode'),
        $t('JoinOrganization.stepTitles.ProvideGuestCode'),
        $t('JoinOrganization.stepTitles.ContactDetails'),
        $t('JoinOrganization.stepTitles.Password'),
        $t('JoinOrganization.stepTitles.Validation'),
      ]"
    />
    <ion-buttons
      slot="end"
      class="closeBtn-container"
    >
      <ion-button
        slot="icon-only"
        @click="cancelModal()"
        class="closeBtn"
      >
        <ion-icon
          :icon="close"
          size="large"
          class="closeBtn__icon"
        />
      </ion-button>
    </ion-buttons>
    <!-- v-if pageStep === WaitForHost so add class wizardTrue -->
    <div
      class="modal"
      :class="{
        wizardTrue: pageStep > 1,
      }"
    >
      <ion-header class="modal-header">
        <ion-title
          v-if="titles.get(pageStep)?.title !== ''"
          class="modal-header__title title-h2"
        >
          {{ titles.get(pageStep)?.title }}
        </ion-title>
        <ion-text
          v-if="titles.get(pageStep)?.subtitle !== ''"
          class="modal-header__text body"
        >
          {{ titles.get(pageStep)?.subtitle }}
        </ion-text>
      </ion-header>
      <!-- modal content: create component for each part-->
      <div class="modal-content inner-content">
        <!-- part 1 (wait for host)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.WaitForHost"
          class="step orga-name"
        >
          <div class="orga-name-content">
            <ms-informative-text>
              {{ $t('JoinOrganization.instructions.start.first') }}
            </ms-informative-text>
            <ms-informative-text v-if="!claimer.greeter">
              {{ $t('JoinOrganization.instructions.start.second') }}
            </ms-informative-text>
            <ms-informative-text v-if="claimer.greeter">
              {{
                $t('JoinOrganization.instructions.start.greeter', {
                  greeter: claimer.greeter.label,
                  greeterEmail: claimer.greeter.email,
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
            :device-enabled="!waitingForHost"
            :name-enabled="!waitingForHost"
            @field-update="fieldsUpdated = true"
            @on-enter-keyup="nextStep()"
          />
        </div>
        <!-- part 5 (get password)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.GetPassword"
          class="step"
          id="get-password"
        >
          <ms-choose-password-input
            ref="passwordPage"
            @on-enter-keyup="nextStep()"
          />
        </div>
        <!-- part 6 (finish the process)-->
        <div
          v-show="pageStep === UserJoinOrganizationStep.Finish"
          class="step"
        >
          <ms-informative-text>
            {{ $t('JoinOrganization.instructions.finish.first') }}
          </ms-informative-text>
          <ms-informative-text>
            {{ $t('JoinOrganization.instructions.finish.second') }}
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
              {{ getNextButtonText() }}
            </span>
          </ion-button>
          <div
            v-show="waitingForHost"
            class="spinner-container"
          >
            <ion-label class="label-waiting">
              {{ $t('JoinOrganization.waitingForHost') }}
            </ion-label>
            <ion-spinner
              name="crescent"
              color="primary"
            />
          </div>
        </ion-buttons>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonButton,
  IonButtons,
  IonFooter,
  IonHeader,
  IonIcon,
  IonLabel,
  IonPage,
  IonSpinner,
  IonText,
  IonTitle,
  modalController,
} from '@ionic/vue';

import { asyncComputed } from '@/common/asyncComputed';
import { getDefaultDeviceName } from '@/common/device';
import { Answer, MsChoosePasswordInput, MsInformativeText, MsModalResult, MsWizardStepper, askQuestion } from '@/components/core';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import UserInformation from '@/components/users/UserInformation.vue';
import { UserClaim } from '@/parsec';
import { Information, InformationKey, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { translate } from '@/services/translation';
import { close } from 'ionicons/icons';
import { computed, inject, onMounted, ref } from 'vue';

enum UserJoinOrganizationStep {
  WaitForHost = 1,
  GetHostSasCode = 2,
  ProvideGuestCode = 3,
  GetUserInfo = 4,
  GetPassword = 5,
  Finish = 6,
}

const informationManager: InformationManager = inject(InformationKey)!;
const pageStep = ref(UserJoinOrganizationStep.WaitForHost);
const userInfoPage = ref();
const passwordPage = ref();
const fieldsUpdated = ref(false);

const claimer = ref(new UserClaim());

const props = defineProps<{
  invitationLink: string;
}>();

const waitingForHost = ref(true);

interface Title {
  title: string;
  subtitle: string;
}

const titles = new Map<UserJoinOrganizationStep, Title>([
  [
    UserJoinOrganizationStep.WaitForHost,
    {
      title: translate('JoinOrganization.titles.waitForHost'),
      subtitle: translate('JoinOrganization.subtitles.waitForHost'),
    },
  ],
  [
    UserJoinOrganizationStep.GetHostSasCode,
    {
      title: translate('JoinOrganization.titles.getHostCode'),
      subtitle: translate('JoinOrganization.subtitles.getHostCode'),
    },
  ],
  [
    UserJoinOrganizationStep.ProvideGuestCode,
    {
      title: translate('JoinOrganization.titles.provideGuestCode'),
      subtitle: translate('JoinOrganization.subtitles.provideGuestCode'),
    },
  ],
  [
    UserJoinOrganizationStep.GetUserInfo,
    {
      title: translate('JoinOrganization.titles.getUserInfo'),
      subtitle: translate('JoinOrganization.subtitles.getUserInfo'),
    },
  ],
  [
    UserJoinOrganizationStep.GetPassword,
    {
      title: translate('JoinOrganization.titles.getPassword'),
      subtitle: translate('JoinOrganization.subtitles.getPassword'),
    },
  ],
  [
    UserJoinOrganizationStep.Finish,
    {
      title: translate('JoinOrganization.titles.finish', { org: '' }),
      subtitle: translate('JoinOrganization.subtitles.finish'),
    },
  ],
]);

async function showErrorAndRestart(message: string): Promise<void> {
  informationManager.present(
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
    await showErrorAndRestart(translate('JoinOrganization.errors.noneCodeSelected'));
  } else {
    if (selectedCode === claimer.value.correctSASCode) {
      const result = await claimer.value.signifyTrust();
      if (result.ok) {
        await nextStep();
      } else {
        await showErrorAndRestart(translate('JoinOrganization.errors.unexpected', { reason: result.error.tag }));
      }
    } else {
      await showErrorAndRestart(translate('JoinOrganization.errors.invalidCodeSelected'));
    }
  }
}

function getNextButtonText(): string {
  if (pageStep.value === UserJoinOrganizationStep.GetUserInfo) {
    return translate('JoinOrganization.validateUserInfo');
  } else if (pageStep.value === UserJoinOrganizationStep.GetPassword) {
    return translate('JoinOrganization.createDevice');
  } else if (pageStep.value === UserJoinOrganizationStep.Finish) {
    return translate('JoinOrganization.logIn');
  } else if (pageStep.value === UserJoinOrganizationStep.WaitForHost) {
    return translate('JoinOrganization.understand');
  }

  return '';
}

const nextButtonIsVisible = computed(() => {
  return (
    (pageStep.value === UserJoinOrganizationStep.WaitForHost && !waitingForHost.value) ||
    (pageStep.value === UserJoinOrganizationStep.GetUserInfo && !waitingForHost.value) ||
    pageStep.value === UserJoinOrganizationStep.GetPassword ||
    pageStep.value === UserJoinOrganizationStep.Finish
  );
});

const canGoForward = asyncComputed(async () => {
  if (fieldsUpdated.value) {
    fieldsUpdated.value = false;
  }
  if (pageStep.value === UserJoinOrganizationStep.GetUserInfo && !(await userInfoPage.value.areFieldsCorrect())) {
    return false;
  } else if (pageStep.value === UserJoinOrganizationStep.GetPassword && !(await passwordPage.value.areFieldsCorrect())) {
    return false;
  }
  return true;
});

async function cancelModal(): Promise<boolean> {
  const answer = await askQuestion(translate('JoinOrganization.cancelConfirm'), translate('JoinOrganization.cancelConfirmSubtitle'), {
    keepMainModalHiddenOnYes: true,
    yesText: translate('JoinOrganization.cancelYes'),
    noText: translate('JoinOrganization.cancelNo'),
    yesIsDangerous: true,
  });

  if (answer === Answer.Yes) {
    await claimer.value.abort();
    return modalController.dismiss(null, MsModalResult.Cancel);
  }
  return false;
}

async function nextStep(): Promise<void> {
  if (!canGoForward.value) {
    return;
  }
  if (pageStep.value === UserJoinOrganizationStep.GetPassword) {
    const result = await claimer.value.finalize(passwordPage.value.password);
    if (!result.ok) {
      // Error here is quite bad because the user has been created in the organization
      // but we fail to save the device. Don't really know what to do here.
      // So we just keep the dialog as is, they can click the button again, hoping it will work.
      informationManager.present(
        new Information({
          message: translate('JoinOrganization.errors.saveDeviceFailed.message'),
          level: InformationLevel.Error,
        }),
        PresentationMode.Toast,
      );
      return;
    }
  } else if (pageStep.value === UserJoinOrganizationStep.GetUserInfo) {
    waitingForHost.value = true;
    const deviceName = await getDefaultDeviceName();
    const result = await claimer.value.doClaim(deviceName, userInfoPage.value.fullName, userInfoPage.value.email);
    if (!result.ok) {
      await showErrorAndRestart(translate('JoinOrganization.errors.sendUserInfoFailed'));
      return;
    }
    waitingForHost.value = false;
    passwordPage.value.setFocus();
  } else if (pageStep.value === UserJoinOrganizationStep.Finish) {
    const notification = new Information({
      message: translate('JoinOrganization.successMessage.message'),
      level: InformationLevel.Success,
    });
    informationManager.present(notification, PresentationMode.Toast | PresentationMode.Console);
    await modalController.dismiss({ device: claimer.value.device, password: passwordPage.value.password }, MsModalResult.Confirm);
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
      await showErrorAndRestart(translate('JoinOrganization.errors.unexpected', { reason: result.error.tag }));
    }
  }
}

async function startProcess(): Promise<void> {
  pageStep.value = UserJoinOrganizationStep.WaitForHost;
  waitingForHost.value = true;
  const retrieveResult = await claimer.value.retrieveInfo(props.invitationLink);

  if (!retrieveResult.ok) {
    await informationManager.present(
      new Information({
        message: translate('JoinOrganization.errors.startFailed.message'),
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
    await cancelModal();
    return;
  }
  if (userInfoPage.value) {
    userInfoPage.value.email = retrieveResult.value.claimerEmail;
  }
  const waitResult = await claimer.value.initialWaitHost();
  if (!waitResult.ok) {
    await informationManager.present(
      new Information({
        message: translate('JoinOrganization.errors.startFailed.message'),
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
    await cancelModal();
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
  await startProcess();
});
</script>

<style lang="scss" scoped>
.orga-name {
  display: flex;
  flex-direction: column;

  &-content {
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
}

.guest-code {
  margin: 4.7rem auto;
}
</style>
