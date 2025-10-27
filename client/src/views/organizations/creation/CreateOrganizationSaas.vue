<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <bms-login
      v-show="step === Steps.BmsLogin"
      @login-success="onLoginSuccess"
      @close-requested="$emit('closeRequested', false)"
      @go-back-requested="$emit('backRequested')"
      @forgotten-password-clicked="step = Steps.BmsForgotPassword"
      :show-back-button="bootstrapLink === undefined"
    />
    <bms-forgot-password
      v-if="step === Steps.BmsForgotPassword"
      @cancel="step = Steps.BmsLogin"
      @login-requested="step = Steps.BmsLogin"
      @close-requested="$emit('closeRequested', false)"
    />
    <organization-name-page
      :class="step === Steps.OrganizationName ? 'active' : ''"
      v-show="step === Steps.OrganizationName"
      :disable-organization-name-field="bootstrapLink !== undefined"
      :organization-name="organizationName ?? ''"
      @organization-name-chosen="onOrganizationNameChosen"
      :error="currentError"
      @go-back-requested="$emit('backRequested')"
      :hide-previous="bootstrapLink !== undefined"
      @close-requested="$emit('closeRequested', false)"
    />
    <organization-authentication-page
      :class="step === Steps.Authentication ? 'active' : ''"
      v-show="step === Steps.Authentication"
      @authentication-chosen="onAuthenticationChosen"
      @close-requested="$emit('closeRequested', false)"
      @go-back-requested="onGoBackRequested"
      :hide-back-button="bootstrapLink !== undefined"
    />
    <organization-summary-page
      :class="step === Steps.Summary ? 'active' : ''"
      v-show="step === Steps.Summary"
      v-if="personalInformation && saveStrategy && organizationName"
      :error="currentError"
      :email="personalInformation.email"
      :name="`${personalInformation.firstName} ${personalInformation.lastName}`"
      :save-strategy="saveStrategy.tag"
      :organization-name="organizationName"
      :server-type="ServerType.Saas"
      :can-edit-email="false"
      :can-edit-name="false"
      :can-edit-organization-name="bootstrapLink === undefined"
      :can-edit-server-address="false"
      :can-edit-save-strategy="saveStrategy.tag !== DeviceSaveStrategyTag.AccountVault"
      :use-sequester-key="sequesterKey !== undefined"
      @create-clicked="onCreateClicked"
      @update-save-strategy-clicked="onUpdateSaveStrategyClicked"
      @update-organization-name-clicked="onUpdateOrganizationNameClicked"
      @close-requested="$emit('closeRequested', false)"
      @go-back-requested="onGoBackRequested"
      :show-tos="true"
    />
    <organization-creation-page
      :class="step === Steps.Creation ? 'active' : ''"
      v-show="step === Steps.Creation"
    />
    <organization-created-page
      :class="step === Steps.Created ? 'active' : ''"
      v-show="step === Steps.Created"
      v-if="organizationName"
      @go-clicked="onGoClicked"
      :organization-name="organizationName"
    />
  </ion-page>
</template>

<script setup lang="ts">
import { getDefaultDeviceName } from '@/common/device';
import {
  AvailableDevice,
  bootstrapOrganization,
  BootstrapOrganizationErrorTag,
  DeviceSaveStrategy,
  DeviceSaveStrategyTag,
  isWeb,
  OrganizationID,
  ParsecAccount,
  ParsedParsecAddrTag,
  parseParsecAddr,
  SaveStrategy,
} from '@/parsec';
import { wait } from '@/parsec/internals';
import {
  AuthenticationToken,
  BillingSystem,
  BmsAccessInstance,
  CONNECTION_ERROR_STATUS,
  DataType,
  PersonalInformationResultData,
} from '@/services/bms';
import { Env } from '@/services/environment';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { ServerType } from '@/services/parsecServers';
import BmsLogin from '@/views/client-area/BmsLogin.vue';
import BmsForgotPassword from '@/views/client-area/forgot-password/BmsForgotPassword.vue';
import OrganizationAuthenticationPage from '@/views/organizations/creation/OrganizationAuthenticationPage.vue';
import OrganizationCreatedPage from '@/views/organizations/creation/OrganizationCreatedPage.vue';
import OrganizationCreationPage from '@/views/organizations/creation/OrganizationCreationPage.vue';
import OrganizationNamePage from '@/views/organizations/creation/OrganizationNamePage.vue';
import OrganizationSummaryPage from '@/views/organizations/creation/OrganizationSummaryPage.vue';
import { IonPage } from '@ionic/vue';
import { Translatable } from 'megashark-lib';
import { isProxy, onMounted, ref, toRaw } from 'vue';

enum Steps {
  BmsLogin,
  BmsForgotPassword,
  OrganizationName,
  Authentication,
  Summary,
  Creation,
  Created,
}

const props = defineProps<{
  bootstrapLink?: string;
  informationManager: InformationManager;
}>();

const emits = defineEmits<{
  (e: 'closeRequested', force: boolean): void;
  (e: 'backRequested'): void;
  (e: 'organizationCreated', organizationName: OrganizationID, device: AvailableDevice, saveStrategy: DeviceSaveStrategy): void;
}>();

const step = ref<Steps>(Steps.BmsLogin);
const organizationName = ref<OrganizationID | undefined>(undefined);
const personalInformation = ref<PersonalInformationResultData | undefined>(undefined);
const currentError = ref<Translatable | undefined>(undefined);
const saveStrategy = ref<DeviceSaveStrategy | undefined>(undefined);
const availableDevice = ref<AvailableDevice | undefined>(undefined);
const sequesterKey = ref<string | undefined>(undefined);

onMounted(async () => {
  if (props.bootstrapLink) {
    const result = await parseParsecAddr(props.bootstrapLink);
    if (result.ok && result.value.tag === ParsedParsecAddrTag.OrganizationBootstrap) {
      organizationName.value = result.value.organizationId;
    }
  }
});

async function onLoginSuccess(_token: AuthenticationToken, info: PersonalInformationResultData): Promise<void> {
  if (
    (info.billingSystem === BillingSystem.CustomOrder ||
      info.billingSystem === BillingSystem.ExperimentalCandidate ||
      info.billingSystem === BillingSystem.None) &&
    !props.bootstrapLink
  ) {
    emits('closeRequested', true);
    props.informationManager.present(
      new Information({
        message:
          info.billingSystem === BillingSystem.None
            ? 'CreateOrganization.errors.notClientAccount'
            : 'CreateOrganization.errors.customOrderNotAuthorized',
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
    return;
  }
  personalInformation.value = info;

  step.value = Steps.OrganizationName;
  if (props.bootstrapLink) {
    if (isWeb() && ParsecAccount.isLoggedIn() && ParsecAccount.addressMatchesAccountServer(props.bootstrapLink)) {
      const parseResult = await parseParsecAddr(props.bootstrapLink);
      if (parseResult.ok && parseResult.value.tag === ParsedParsecAddrTag.OrganizationBootstrap) {
        const result = await SaveStrategy.useAccountVault(parseResult.value.organizationId);
        if (result.ok) {
          // Skip the auth page
          await onAuthenticationChosen(result.value);
        }
      }
    }
  }
}

async function onOrganizationNameChosen(chosenOrganizationName: OrganizationID, seqKey: string | undefined): Promise<void> {
  if (!personalInformation.value) {
    window.electronAPI.log(
      'error',
      'OrganizationCreation: no authentication token after choosing the organization name, should not happen',
    );
    return;
  }
  organizationName.value = chosenOrganizationName;
  sequesterKey.value = seqKey;
  if (
    isWeb() &&
    ParsecAccount.isLoggedIn() &&
    Env.getSaasServers().some((addr) => ParsecAccount.addressMatchesAccountServer(`parsec3://${addr}`))
  ) {
    // Fake one, will get overwritten later
    saveStrategy.value = { tag: DeviceSaveStrategyTag.AccountVault, ciphertextKey: new Uint8Array(), ciphertextKeyId: '' };
    step.value = Steps.Summary;
  } else {
    step.value = Steps.Authentication;
  }
}

async function onAuthenticationChosen(chosenSaveStrategy: DeviceSaveStrategy): Promise<void> {
  if (!personalInformation.value) {
    window.electronAPI.log('error', 'OrganizationCreation: missing data on auth chosen, should not happen');
  }
  currentError.value = undefined;
  saveStrategy.value = chosenSaveStrategy;
  step.value = Steps.Summary;
}

async function onCreationError(startTime: number): Promise<void> {
  const endTime = new Date().valueOf();
  // If we're too fast, a weird blinking will occur. Add some artificial time.
  if (endTime - startTime < 1500) {
    await wait(1500 - (endTime - startTime));
  }
  step.value = Steps.Summary;
}

async function onCreateClicked(): Promise<void> {
  if (!organizationName.value || !personalInformation.value) {
    window.electronAPI.log('error', 'OrganizationCreation: missing data at the creation step, should not happen');
    return;
  }

  step.value = Steps.Creation;

  const startTime = new Date().valueOf();
  let bootstrapLink: string | undefined = props.bootstrapLink;
  if (!props.bootstrapLink) {
    const response = await BmsAccessInstance.get().createOrganization(organizationName.value);

    if (response.isError) {
      window.electronAPI.log('error', `Failed to create organization: ${JSON.stringify(response.errors)}`);
      // TODO: Change this error handling with the real backend response
      if (response.errors && response.errors.some((error) => error.code === 'parsec_bad_status')) {
        currentError.value = 'CreateOrganization.errors.alreadyExists';
      } else if (response.status === CONNECTION_ERROR_STATUS) {
        currentError.value = 'CreateOrganization.errors.offline';
      } else {
        window.electronAPI.log('error', `Failed to create Saas organization, unhandled error ${JSON.stringify(response.errors)}`);
        currentError.value = {
          key: 'CreateOrganization.errors.generic',
          data: { reason: 'Unknown' },
        };
      }
      await onCreationError(startTime);
      return;
    } else if (!response.data || response.data.type !== DataType.CreateOrganization) {
      // Should not happen
      console.log('Incorrect response data type');
      currentError.value = { key: 'CreateOrganization.errors.generic', data: { reason: 'IncorrectDataType' } };
      await onCreationError(startTime);
      return;
    } else if (!response.isError && !response.data.bootstrapLink) {
      window.electronAPI.log('error', 'Server did not provide a bootstrap link');
      currentError.value = { key: 'CreateOrganization.errors.generic', data: { reason: 'MissingBootstrapLink' } };
      return;
    } else {
      bootstrapLink = response.data.bootstrapLink;
      if (isWeb() && ParsecAccount.isLoggedIn() && ParsecAccount.addressMatchesAccountServer(bootstrapLink)) {
        // Don't like it, but we can't create the organization from the tests
        const result = await SaveStrategy.useAccountVault(window.isTesting() ? '__no_check__' : organizationName.value);
        if (result.ok) {
          saveStrategy.value = result.value;
        } else {
          window.electronAPI.log('error', `Failed to create a save strategy using account: ${result.error.tag} (${result.error.error})`);
          // No idea what to do here. Going back to auth choice, but it's not ideal
          step.value = Steps.Authentication;
          return;
        }
      }
    }
  }
  if (!bootstrapLink) {
    currentError.value = { key: 'CreateOrganization.errors.generic', data: { reason: 'NoBootstrapLink' } };
    await onCreationError(startTime);
    return;
  }
  if (!saveStrategy.value) {
    return;
  }
  const result = await bootstrapOrganization(
    bootstrapLink,
    `${personalInformation.value.firstName} ${personalInformation.value.lastName}`,
    personalInformation.value.email,
    getDefaultDeviceName(),
    isProxy(saveStrategy.value) ? toRaw(saveStrategy.value) : saveStrategy.value,
    sequesterKey.value,
  );

  if (!result.ok) {
    if (result.error.tag === BootstrapOrganizationErrorTag.AlreadyUsedToken) {
      currentError.value = 'CreateOrganization.errors.alreadyExists';
    } else if (result.error.tag === BootstrapOrganizationErrorTag.Offline) {
      currentError.value = 'CreateOrganization.errors.offline';
    } else if (result.error.tag === BootstrapOrganizationErrorTag.Internal && result.error.error.includes('Unsupported')) {
      currentError.value = 'CreateOrganization.errors.incompatibleServer';
    } else if (result.error.tag === BootstrapOrganizationErrorTag.InvalidSequesterAuthorityVerifyKey) {
      currentError.value = 'CreateOrganization.errors.invalidSequesterKey';
    } else {
      currentError.value = { key: 'CreateOrganization.errors.generic', data: { reason: result.error.tag } };
    }
    await onCreationError(startTime);
    console.log('Failed to create organization', result.error);
    return;
  }

  const endTime = new Date().valueOf();
  // If we're too fast, a weird blinking will occur. Add some artificial time.
  if (endTime - startTime < 1500) {
    await wait(1500 - (endTime - startTime));
  }
  availableDevice.value = result.value;

  step.value = Steps.Created;
}

async function onGoClicked(): Promise<void> {
  if (!saveStrategy.value || !availableDevice.value || !organizationName.value) {
    window.electronAPI.log('error', 'OrganizationCreation: missing data at the end step, should not happen');
    return;
  }
  emits('organizationCreated', organizationName.value, availableDevice.value, saveStrategy.value);
}

async function onGoBackRequested(): Promise<void> {
  if (step.value === Steps.Authentication) {
    step.value = Steps.OrganizationName;
  } else if (step.value === Steps.Summary) {
    step.value = Steps.Authentication;
  } else {
    console.log(`Cannot go back from ${step.value}, should not happen`);
  }
}

async function onUpdateOrganizationNameClicked(): Promise<void> {
  step.value = Steps.OrganizationName;
}

async function onUpdateSaveStrategyClicked(): Promise<void> {
  step.value = Steps.Authentication;
}
</script>

<style scoped lang="scss"></style>
