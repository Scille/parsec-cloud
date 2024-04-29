<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal-stepper">
    <ms-wizard-stepper
      v-if="pageStep > UserJoinOrganizationStep.WaitForHost"
      :current-index="pageStep - 2"
      :titles="[
        'JoinOrganization.stepTitles.GetHostCode',
        'JoinOrganization.stepTitles.ProvideGuestCode',
        'JoinOrganization.stepTitles.ContactDetails',
        'JoinOrganization.stepTitles.Authentication',
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
          {{ $msTranslate(titles.get(pageStep)?.title) }}
        </ion-title>
        <ion-text
          v-if="titles.get(pageStep)?.subtitle !== ''"
          class="modal-header__text body"
        >
          {{ $msTranslate(titles.get(pageStep)?.subtitle) }}
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
              {{ $msTranslate('JoinOrganization.instructions.start.first') }}
            </ms-informative-text>
            <ms-informative-text v-if="!claimer.greeter">
              {{ $msTranslate('JoinOrganization.instructions.start.second') }}
            </ms-informative-text>
            <ms-informative-text v-if="claimer.greeter">
              {{
                $msTranslate({
                  key: 'JoinOrganization.instructions.start.greeter',
                  data: {
                    greeter: claimer.greeter.label,
                    greeterEmail: claimer.greeter.email,
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
          v-show="pageStep === UserJoinOrganizationStep.GetAuthentication"
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
            v-show="waitingForHost"
            class="spinner-container"
          >
            <ms-spinner title="JoinOrganization.waitingForHost" />
          </div>
        </ion-buttons>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { IonButton, IonButtons, IonFooter, IonHeader, IonIcon, IonPage, IonText, IonTitle, modalController } from '@ionic/vue';

import { getDefaultDeviceName } from '@/common/device';
import { Answer, MsInformativeText, MsModalResult, askQuestion } from '@/components/core';
import ChooseAuthentication from '@/components/devices/ChooseAuthentication.vue';
import SasCodeChoice from '@/components/sas-code/SasCodeChoice.vue';
import SasCodeProvide from '@/components/sas-code/SasCodeProvide.vue';
import UserInformation from '@/components/users/UserInformation.vue';
import {
  AccessStrategy,
  ClaimInProgressErrorTag,
  ClaimerRetrieveInfoErrorTag,
  DeviceSaveStrategy,
  DeviceSaveStrategyPassword,
  DeviceSaveStrategyTag,
  UserClaim,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { MsSpinner, MsWizardStepper, Translatable, asyncComputed } from 'megashark-lib';
import { close } from 'ionicons/icons';
import { computed, onMounted, ref } from 'vue';

enum UserJoinOrganizationStep {
  WaitForHost = 1,
  GetHostSasCode = 2,
  ProvideGuestCode = 3,
  GetUserInfo = 4,
  GetAuthentication = 5,
  Finish = 6,
}

const pageStep = ref(UserJoinOrganizationStep.WaitForHost);
const userInfoPage = ref();
const authChoice = ref();
const fieldsUpdated = ref(false);
const cancelled = ref(false);

const claimer = ref(new UserClaim());

const props = defineProps<{
  invitationLink: string;
  informationManager: InformationManager;
}>();

const waitingForHost = ref(true);

interface Title {
  title: Translatable;
  subtitle: Translatable;
}

const titles = new Map<UserJoinOrganizationStep, Title>([
  [
    UserJoinOrganizationStep.WaitForHost,
    {
      title: 'JoinOrganization.titles.waitForHost',
      subtitle: 'JoinOrganization.subtitles.waitForHost',
    },
  ],
  [
    UserJoinOrganizationStep.GetHostSasCode,
    {
      title: 'JoinOrganization.titles.getHostCode',
      subtitle: 'JoinOrganization.subtitles.getHostCode',
    },
  ],
  [
    UserJoinOrganizationStep.ProvideGuestCode,
    {
      title: 'JoinOrganization.titles.provideGuestCode',
      subtitle: 'JoinOrganization.subtitles.provideGuestCode',
    },
  ],
  [
    UserJoinOrganizationStep.GetUserInfo,
    {
      title: 'JoinOrganization.titles.getUserInfo',
      subtitle: 'JoinOrganization.subtitles.getUserInfo',
    },
  ],
  [
    UserJoinOrganizationStep.GetAuthentication,
    {
      title: 'JoinOrganization.titles.getAuthentication',
      subtitle: 'JoinOrganization.subtitles.getAuthentication',
    },
  ],
  [
    UserJoinOrganizationStep.Finish,
    {
      title: { key: 'JoinOrganization.titles.finish', data: { org: '' } },
      subtitle: 'JoinOrganization.subtitles.finish',
    },
  ],
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
  } else {
    if (selectedCode === claimer.value.correctSASCode) {
      const result = await claimer.value.signifyTrust();
      if (result.ok) {
        await nextStep();
      } else {
        await showErrorAndRestart({ key: 'JoinOrganization.errors.unexpected', data: { reason: result.error.tag } });
      }
    } else {
      await showErrorAndRestart('JoinOrganization.errors.invalidCodeSelected');
    }
  }
}

function getNextButtonText(): string {
  if (pageStep.value === UserJoinOrganizationStep.GetUserInfo) {
    return 'JoinOrganization.validateUserInfo';
  } else if (pageStep.value === UserJoinOrganizationStep.GetAuthentication) {
    return 'JoinOrganization.createDevice';
  } else if (pageStep.value === UserJoinOrganizationStep.Finish) {
    return 'JoinOrganization.logIn';
  } else if (pageStep.value === UserJoinOrganizationStep.WaitForHost) {
    return 'JoinOrganization.understand';
  }

  return '';
}

const nextButtonIsVisible = computed(() => {
  return (
    (pageStep.value === UserJoinOrganizationStep.WaitForHost && !waitingForHost.value) ||
    (pageStep.value === UserJoinOrganizationStep.GetUserInfo && !waitingForHost.value) ||
    pageStep.value === UserJoinOrganizationStep.GetAuthentication ||
    pageStep.value === UserJoinOrganizationStep.Finish
  );
});

const canGoForward = asyncComputed(async () => {
  if (fieldsUpdated.value) {
    fieldsUpdated.value = false;
  }
  if (pageStep.value === UserJoinOrganizationStep.GetUserInfo && !(await userInfoPage.value.areFieldsCorrect())) {
    return false;
  } else if (pageStep.value === UserJoinOrganizationStep.GetAuthentication) {
    return await authChoice.value.areFieldsCorrect();
  }
  return true;
});

async function cancelModal(): Promise<boolean> {
  const answer = await askQuestion('JoinOrganization.cancelConfirm', 'JoinOrganization.cancelConfirmSubtitle', {
    keepMainModalHiddenOnYes: true,
    yesText: 'JoinOrganization.cancelYes',
    noText: 'JoinOrganization.cancelNo',
    yesIsDangerous: true,
  });

  if (answer === Answer.Yes) {
    cancelled.value = true;
    await claimer.value.abort();
    return modalController.dismiss(null, MsModalResult.Cancel);
  }
  return false;
}

async function nextStep(): Promise<void> {
  if (!canGoForward.value) {
    return;
  }
  if (pageStep.value === UserJoinOrganizationStep.GetAuthentication) {
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
    const deviceName = await getDefaultDeviceName();
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
      await showErrorAndRestart({ key: 'JoinOrganization.errors.unexpected', data: { reason: result.error.tag } });
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
      case ClaimerRetrieveInfoErrorTag.AlreadyUsed:
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
      PresentationMode.Modal,
    );
    return;
  }

  if (userInfoPage.value) {
    userInfoPage.value.email = retrieveResult.value.claimerEmail;
  }
  const waitResult = await claimer.value.initialWaitHost();
  if (!waitResult.ok && !cancelled.value) {
    await claimer.value.abort();
    await modalController.dismiss(null, MsModalResult.Cancel);
    let message: Translatable = '';
    switch (waitResult.error.tag) {
      case ClaimInProgressErrorTag.ActiveUsersLimitReached:
        message = 'JoinOrganization.errors.usersLimitReached';
        break;
      default:
        message = { key: 'JoinOrganization.errors.unexpected', data: { reason: waitResult.error.tag } };
    }

    await props.informationManager.present(
      new Information({
        message: message,
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
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
