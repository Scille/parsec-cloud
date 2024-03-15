<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal-stepper">
    <div class="modal">
      <ion-buttons
        slot="end"
        class="closeBtn-container"
      >
        <ion-button
          slot="icon-only"
          @click="cancelModal()"
          class="closeBtn"
          v-show="canClose()"
        >
          <ion-icon
            :icon="close"
            size="large"
            class="closeBtn__icon"
          />
        </ion-button>
      </ion-buttons>
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
        <!-- part 1 (org name)-->
        <div
          v-show="pageStep === CreateOrganizationStep.OrgNameStep"
          class="step org-name"
        >
          <ms-input
            :label="$t('CreateOrganization.organizationName')"
            :placeholder="$t('CreateOrganization.organizationNamePlaceholder')"
            name="organization"
            id="org-name-input"
            v-model="orgName"
            ref="organizationNameInputRef"
            @on-enter-keyup="nextStep()"
            :validator="organizationNameValidator"
          />

          <ion-text class="subtitles-sm org-name-criteria">
            {{ $t('CreateOrganization.organizationNameCriteria') }}
          </ion-text>
        </div>

        <!-- part 2 (user info)-->
        <div
          v-show="pageStep === CreateOrganizationStep.UserInfoStep"
          class="step user-info"
        >
          <user-information
            ref="userInfo"
            @field-update="fieldsUpdated = true"
            @on-enter-keyup="nextStep()"
          />
        </div>

        <!-- part 3 (server)-->
        <div
          class="step org-server"
          v-show="pageStep === CreateOrganizationStep.ServerStep"
        >
          <choose-server ref="serverChoice" />
        </div>

        <!-- part 4 (password)-->
        <div
          class="step org-password"
          v-show="pageStep === CreateOrganizationStep.AuthenticationStep"
        >
          <choose-authentication ref="authChoice" />
        </div>

        <!-- part 5 (summary) -->
        <div
          class="step org-summary"
          v-show="pageStep === CreateOrganizationStep.SummaryStep"
        >
          <summary-step
            v-if="orgInfo"
            ref="summaryInfo"
            :organization="orgInfo.orgName"
            :fullname="orgInfo.userName"
            :email="orgInfo.email"
            :server-mode="orgInfo.serverMode"
            :server-addr="orgInfo.serverAddr"
            :authentication="authChoice.authentication"
            @update-request="onUpdateRequested"
          />
        </div>
        <!-- part 6 (loading)-->
        <div
          class="step org-loading"
          v-show="pageStep === CreateOrganizationStep.SpinnerStep"
        >
          <ms-spinner :title="$t('CreateOrganization.loading')" />
        </div>

        <!-- part 7 (loading) -->
        <div
          class="step orga-created"
          v-show="pageStep === CreateOrganizationStep.FinishStep"
        >
          <ms-informative-text>
            {{ $t('CreateOrganization.organizationCreated') }}
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
            fill="clear"
            size="default"
            id="previous-button"
            @click="previousStep()"
            v-show="canGoBackward()"
          >
            {{ $t('CreateOrganization.button.previous') }}
            <ion-icon
              slot="start"
              :icon="chevronBack"
              size="small"
            />
          </ion-button>
          <ion-button
            fill="solid"
            size="default"
            id="next-button"
            v-show="shouldShowNextStep()"
            @click="nextStep()"
            :disabled="!canGoForward"
          >
            <span>
              {{ getNextButtonText() }}
            </span>
            <ion-icon
              v-show="pageStep !== CreateOrganizationStep.SummaryStep"
              slot="start"
              :icon="chevronForward"
              size="small"
            />
            <ion-icon
              v-show="pageStep === CreateOrganizationStep.SummaryStep"
              slot="start"
              :icon="checkmarkDone"
              size="small"
            />
          </ion-button>
        </ion-buttons>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { asyncComputed } from '@/common/asyncComputed';
import { getDefaultDeviceName } from '@/common/device';
import { IValidator, Validity, organizationValidator } from '@/common/validators';
import { Answer, MsInformativeText, MsInput, MsModalResult, MsSpinner, askQuestion } from '@/components/core';
import ChooseAuthentication from '@/components/devices/ChooseAuthentication.vue';
import ChooseServer, { ServerMode } from '@/components/organizations/ChooseServer.vue';
import UserInformation from '@/components/users/UserInformation.vue';
import {
  AccessStrategy,
  AvailableDevice,
  BootstrapOrganizationErrorTag,
  DeviceSaveStrategy,
  DeviceSaveStrategyPassword,
  DeviceSaveStrategyTag,
  createOrganization as parsecCreateOrganization,
} from '@/parsec';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { formatDate, translate } from '@/services/translation';
import SummaryStep, { OrgInfo } from '@/views/home/SummaryStep.vue';
import { IonButton, IonButtons, IonFooter, IonHeader, IonIcon, IonPage, IonText, IonTitle, modalController } from '@ionic/vue';
import { checkmarkDone, chevronBack, chevronForward, close } from 'ionicons/icons';
import { Ref, inject, onMounted, ref } from 'vue';

enum CreateOrganizationStep {
  OrgNameStep = 1,
  UserInfoStep = 2,
  ServerStep = 3,
  AuthenticationStep = 4,
  SummaryStep = 5,
  SpinnerStep = 6,
  FinishStep = 7,
}

const DEFAULT_SAAS_ADDR = 'parsec3://saas-demo-v3-fireraptor.parsec.cloud/ ';

const informationManager: InformationManager = inject(InformationManagerKey)!;
const pageStep = ref(CreateOrganizationStep.OrgNameStep);
const orgName = ref('');
const userInfo = ref();
const serverChoice = ref();
const authChoice = ref();
const summaryInfo = ref();
const organizationNameInputRef = ref();

const device: Ref<AvailableDevice | null> = ref(null);
const orgInfo: Ref<null | OrgInfoValues> = ref(null);

const fieldsUpdated = ref(false);

interface Title {
  title: string;
  subtitle?: string;
}

interface OrgInfoValues {
  orgName: string;
  userName: string;
  email: string;
  serverMode: ServerMode;
  serverAddr: string;
}

const titles = new Map<CreateOrganizationStep, Title>([
  [
    CreateOrganizationStep.OrgNameStep,
    {
      title: translate('CreateOrganization.title.create'),
      subtitle: translate('CreateOrganization.subtitle.nameYourOrg'),
    },
  ],
  [
    CreateOrganizationStep.UserInfoStep,
    {
      title: translate('CreateOrganization.title.personalDetails'),
      subtitle: translate('CreateOrganization.subtitle.personalDetails'),
    },
  ],
  [
    CreateOrganizationStep.ServerStep,
    {
      title: translate('CreateOrganization.title.server'),
      subtitle: translate('CreateOrganization.subtitle.server'),
    },
  ],
  [
    CreateOrganizationStep.AuthenticationStep,
    {
      title: translate('CreateOrganization.title.authentication'),
      subtitle: translate('CreateOrganization.subtitle.authentication'),
    },
  ],
  [
    CreateOrganizationStep.SummaryStep,
    {
      title: translate('CreateOrganization.title.overview'),
      subtitle: translate('CreateOrganization.subtitle.overview'),
    },
  ],
  [
    CreateOrganizationStep.FinishStep,
    {
      title: translate('CreateOrganization.title.done'),
    },
  ],
]); // eslint-disable-next-line @typescript-eslint/no-non-null-assertion

onMounted(async () => {
  await organizationNameInputRef.value.setFocus();
});

const orgAlreadyExists = ref(false);
const organizationNameValidator: IValidator = async function (value: string) {
  const result = await organizationValidator(value);

  if (result.validity === Validity.Valid && orgAlreadyExists.value) {
    const ret = { validity: Validity.Invalid, reason: translate('CreateOrganization.errors.alreadyExists') };
    orgAlreadyExists.value = false;
    return ret;
  }
  return result;
};

function getNextButtonText(): string {
  if (pageStep.value === CreateOrganizationStep.SummaryStep) {
    return translate('CreateOrganization.button.create');
  } else if (pageStep.value === CreateOrganizationStep.FinishStep) {
    return translate('CreateOrganization.button.done');
  } else {
    return translate('CreateOrganization.button.next');
  }
}

function canGoBackward(): boolean {
  return ![CreateOrganizationStep.OrgNameStep, CreateOrganizationStep.SpinnerStep, CreateOrganizationStep.FinishStep].includes(
    pageStep.value,
  );
}

function canClose(): boolean {
  return ![CreateOrganizationStep.SpinnerStep, CreateOrganizationStep.FinishStep].includes(pageStep.value);
}

const canGoForward = asyncComputed(async () => {
  // No reason other than making vue watch the variable so that this function is called
  if (fieldsUpdated.value) {
    fieldsUpdated.value = false;
  }

  switch (pageStep.value) {
    case CreateOrganizationStep.FinishStep:
    case CreateOrganizationStep.SpinnerStep:
      return true;
    case CreateOrganizationStep.OrgNameStep:
      return (await organizationValidator(orgName.value)).validity === Validity.Valid;
    case CreateOrganizationStep.AuthenticationStep:
      return await authChoice.value.areFieldsCorrect();
    case CreateOrganizationStep.ServerStep:
      return await serverChoice.value.areFieldsCorrect();
    case CreateOrganizationStep.UserInfoStep:
      return await userInfo.value.areFieldsCorrect();
    default:
      return true;
  }
});

function shouldShowNextStep(): boolean {
  return pageStep.value !== CreateOrganizationStep.SpinnerStep;
}

async function cancelModal(): Promise<boolean> {
  // No need to ask any question if we're at the beginning
  if (pageStep.value === CreateOrganizationStep.OrgNameStep) {
    return await modalController.dismiss(null, MsModalResult.Cancel);
  }

  const answer = await askQuestion(translate('CreateOrganization.cancelConfirm'), translate('CreateOrganization.cancelConfirmSubtitle'), {
    keepMainModalHiddenOnYes: true,
    yesText: translate('CreateOrganization.cancelYes'),
    noText: translate('CreateOrganization.cancelNo'),
    yesIsDangerous: true,
  });

  if (answer === Answer.Yes) {
    return await modalController.dismiss(null, MsModalResult.Cancel);
  }
  return false;
}

async function nextStep(): Promise<void> {
  if (!canGoForward.value) {
    return;
  }
  if (pageStep.value === CreateOrganizationStep.FinishStep) {
    if (!device.value) {
      return;
    }
    const saveStrategy: DeviceSaveStrategy = authChoice.value.getSaveStrategy();
    const accessStrategy =
      saveStrategy.tag === DeviceSaveStrategyTag.Keyring
        ? AccessStrategy.useKeyring(device.value)
        : AccessStrategy.usePassword(device.value, (saveStrategy as DeviceSaveStrategyPassword).password);
    modalController.dismiss({ device: device.value, access: accessStrategy }, MsModalResult.Confirm);
    return;
  } else {
    pageStep.value += 1;
    if (pageStep.value === CreateOrganizationStep.UserInfoStep) {
      await userInfo.value.setFocus();
    }
  }
  if (pageStep.value === CreateOrganizationStep.SpinnerStep) {
    const addr = serverChoice.value.mode === serverChoice.value.ServerMode.SaaS ? DEFAULT_SAAS_ADDR : serverChoice.value.backendAddr;

    const deviceName = await getDefaultDeviceName();
    const strategy = authChoice.value.getSaveStrategy();

    const result = await parsecCreateOrganization(addr, orgName.value, userInfo.value.fullName, userInfo.value.email, deviceName, strategy);
    if (result.ok) {
      device.value = result.value;
      await nextStep();
    } else {
      let message = '';
      switch (result.error.tag) {
        case BootstrapOrganizationErrorTag.AlreadyUsedToken:
          pageStep.value = CreateOrganizationStep.OrgNameStep;
          orgAlreadyExists.value = true;
          await organizationNameInputRef.value.setFocus();
          await organizationNameInputRef.value.selectText();
          await organizationNameInputRef.value.validate(orgName.value);
          break;

        case BootstrapOrganizationErrorTag.Offline:
          message = translate('CreateOrganization.errors.offline');
          pageStep.value = CreateOrganizationStep.SummaryStep;
          break;

        case BootstrapOrganizationErrorTag.TimestampOutOfBallpark:
          message = translate('CreateOrganization.errors.timestampOutOfBallpark', {
            clientTime: formatDate(result.error.clientTimestamp, 'long'),
            serverTime: formatDate(result.error.serverTimestamp, 'long'),
          });
          pageStep.value = CreateOrganizationStep.SummaryStep;
          break;

        default:
          message = translate('CreateOrganization.errors.generic', {
            reason: result.error.tag,
          });
          pageStep.value = CreateOrganizationStep.SummaryStep;
          break;
      }
      if (message) {
        informationManager.present(
          new Information({
            message: message,
            level: InformationLevel.Error,
          }),
          PresentationMode.Toast,
        );
      }
    }
  }

  if (pageStep.value === CreateOrganizationStep.SummaryStep) {
    orgInfo.value = {
      orgName: orgName.value,
      userName: userInfo.value.fullName,
      email: userInfo.value.email,
      serverMode: serverChoice.value.mode,
      serverAddr: serverChoice.value.mode === serverChoice.value.ServerMode.SaaS ? DEFAULT_SAAS_ADDR : serverChoice.value.backendAddr,
    };
  }
}

function previousStep(): void {
  pageStep.value = pageStep.value - 1;
}

function onUpdateRequested(info: OrgInfo): void {
  if (info === OrgInfo.Organization) {
    pageStep.value = CreateOrganizationStep.OrgNameStep;
  } else if (info === OrgInfo.UserInfo) {
    pageStep.value = CreateOrganizationStep.UserInfoStep;
  } else if (info === OrgInfo.ServerMode) {
    pageStep.value = CreateOrganizationStep.ServerStep;
  } else if (info === OrgInfo.AuthenticationMode) {
    pageStep.value = CreateOrganizationStep.AuthenticationStep;
  }
}
</script>

<style lang="scss" scoped>
.org-name {
  display: flex;
  flex-direction: column;
}

.org-name-criteria {
  color: var(--parsec-color-light-secondary-grey);
}

.org-server {
  display: flex;
  flex-direction: column;
}

.org-password {
  display: flex;
  flex-direction: column;
}

.org-loading {
  display: flex;
  flex-direction: column;
  align-items: center;
  text-align: center;
  gap: 1rem;
}
</style>
