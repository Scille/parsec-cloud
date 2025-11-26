<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <organization-user-information-page
      v-show="step === Steps.PersonalInformation"
      :class="step === Steps.PersonalInformation ? 'active' : ''"
      @user-information-filled="onUserInformationFilled"
      @go-back-requested="$emit('backRequested')"
      @close-requested="$emit('closeRequested')"
      :hide-previous="bootstrapLink !== undefined"
      :require-tos="true"
    />
    <organization-authentication-page
      v-show="step === Steps.Authentication"
      :class="step === Steps.Authentication ? 'active' : ''"
      :is-last-step="true"
      :server-config="serverConfig"
      @authentication-chosen="onAuthenticationChosen"
      @go-back-requested="onGoBackRequested"
      @close-requested="$emit('closeRequested')"
    />
    <organization-creation-page
      v-show="step === Steps.Creation"
      :class="step === Steps.Creation ? 'active' : ''"
    />

    <organization-created-page
      v-if="organizationName"
      v-show="step === Steps.Created"
      :class="step === Steps.Created ? 'active' : ''"
      @go-clicked="onGoClicked"
      :organization-name="organizationName"
    />
  </ion-page>
</template>

<script setup lang="ts">
import { getDefaultDeviceName } from '@/common/device';
import { generateTrialOrganizationName } from '@/common/organization';
import {
  AvailableDevice,
  BootstrapOrganizationError,
  BootstrapOrganizationErrorTag,
  DeviceSaveStrategy,
  forgeServerAddr,
  getServerConfig,
  OrganizationID,
  bootstrapOrganization as parsecBootstrapOrganization,
  createOrganization as parsecCreateOrganization,
  ParsedParsecAddrTag,
  parseParsecAddr,
  Result,
  ServerConfig,
} from '@/parsec';
import { wait } from '@/parsec/internals';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { getTrialServerAddress } from '@/services/parsecServers';
import OrganizationAuthenticationPage from '@/views/organizations/creation/OrganizationAuthenticationPage.vue';
import OrganizationCreatedPage from '@/views/organizations/creation/OrganizationCreatedPage.vue';
import OrganizationCreationPage from '@/views/organizations/creation/OrganizationCreationPage.vue';
import OrganizationUserInformationPage from '@/views/organizations/creation/OrganizationUserInformationPage.vue';
import { IonPage, modalController } from '@ionic/vue';
import { I18n, MsModalResult, Translatable } from 'megashark-lib';
import { isProxy, onMounted, ref, toRaw } from 'vue';

enum Steps {
  PersonalInformation,
  Authentication,
  Creation,
  Created,
}

const props = defineProps<{
  bootstrapLink?: string;
  informationManager: InformationManager;
}>();

const emits = defineEmits<{
  (e: 'closeRequested'): void;
  (e: 'backRequested'): void;
  (e: 'organizationCreated', organizationName: OrganizationID, device: AvailableDevice, saveStrategy: DeviceSaveStrategy): void;
}>();

const step = ref<Steps>(Steps.PersonalInformation);
const email = ref<string | undefined>(undefined);
const name = ref<string | undefined>(undefined);
const saveStrategy = ref<DeviceSaveStrategy | undefined>(undefined);
const availableDevice = ref<AvailableDevice | undefined>(undefined);
const currentError = ref<Translatable | undefined>(undefined);
const organizationName = ref<OrganizationID | undefined>(undefined);
const serverConfig = ref<ServerConfig | undefined>(undefined);

onMounted(async () => {
  let server = getTrialServerAddress();
  if (props.bootstrapLink) {
    const result = await parseParsecAddr(props.bootstrapLink);
    if (result.ok && result.value.tag === ParsedParsecAddrTag.OrganizationBootstrap) {
      organizationName.value = result.value.organizationId;
      server = await forgeServerAddr(result.value);
    }
  }
  const configResult = await getServerConfig(server);
  if (configResult.ok) {
    serverConfig.value = configResult.value;
  }
});

async function onUserInformationFilled(chosenName: string, chosenEmail: string): Promise<void> {
  email.value = chosenEmail;
  name.value = chosenName;
  step.value = Steps.Authentication;
}

async function createOrganization(): Promise<Result<AvailableDevice, BootstrapOrganizationError>> {
  if (!name.value || !email.value || !saveStrategy.value) {
    return { ok: false, error: { tag: BootstrapOrganizationErrorTag.Internal, error: 'Missing data' } };
  }

  let retry = 0;

  while (retry < 2) {
    const orgName = generateTrialOrganizationName(email.value);
    const result = await parsecCreateOrganization(
      getTrialServerAddress(),
      orgName,
      name.value,
      email.value,
      getDefaultDeviceName(),
      (isProxy(saveStrategy.value) ? toRaw(saveStrategy.value) : saveStrategy.value) as DeviceSaveStrategy,
    );
    if (result.ok) {
      organizationName.value = orgName;
      return result;
    } else {
      // Very unlikely
      // Name already used, retry with something else
      if (result.error.tag === BootstrapOrganizationErrorTag.AlreadyUsedToken) {
        retry += 1;
      } else {
        return result;
      }
    }
  }
  return {
    ok: false,
    error: { tag: BootstrapOrganizationErrorTag.AlreadyUsedToken, error: 'Could not find an available organization name' },
  };
}

async function bootstrapOrganization(): Promise<Result<AvailableDevice, BootstrapOrganizationError>> {
  if (!name.value || !email.value || !saveStrategy.value || !props.bootstrapLink) {
    return { ok: false, error: { tag: BootstrapOrganizationErrorTag.Internal, error: 'Missing data' } };
  }
  const result = await parsecBootstrapOrganization(
    props.bootstrapLink,
    name.value,
    email.value,
    getDefaultDeviceName(),
    (isProxy(saveStrategy.value) ? toRaw(saveStrategy.value) : saveStrategy.value) as DeviceSaveStrategy,
  );
  return result;
}

async function onAuthenticationChosen(strategy: DeviceSaveStrategy): Promise<void> {
  if (!name.value || !email.value) {
    window.electronAPI.log('error', 'Missing data on org creation step, should not happen');
    return;
  }

  const startTime = new Date().valueOf();
  saveStrategy.value = strategy;
  step.value = Steps.Creation;
  let result;

  if (props.bootstrapLink) {
    result = await bootstrapOrganization();
  } else {
    result = await createOrganization();
  }

  const endTime = new Date().valueOf();
  // If we're too fast, a weird blinking will occur. Add some artificial time.
  if (endTime - startTime < 1500) {
    await wait(1500 - (endTime - startTime));
  }

  if (result.ok) {
    availableDevice.value = result.value;
    currentError.value = undefined;
    step.value = Steps.Created;
  } else {
    window.electronAPI.log('error', `Failed to create organization: ${JSON.stringify(result.error)}`);
    switch (result.error.tag) {
      case BootstrapOrganizationErrorTag.Offline:
        currentError.value = 'CreateOrganization.errors.offline';
        break;
      case BootstrapOrganizationErrorTag.Internal:
        if (result.error.error.includes('Unsupported API version')) {
          currentError.value = 'CreateOrganization.errors.incompatibleServer';
        } else {
          currentError.value = {
            key: 'CreateOrganization.errors.generic',
            data: {
              reason: result.error.tag,
            },
          };
        }
        break;
      case BootstrapOrganizationErrorTag.TimestampOutOfBallpark:
        currentError.value = {
          key: 'CreateOrganization.errors.timestampOutOfBallpark',
          data: {
            clientTime: I18n.translate(I18n.formatDate(result.error.clientTimestamp, 'long')),
            serverTime: I18n.translate(I18n.formatDate(result.error.serverTimestamp, 'long')),
          },
        };
        break;
      default:
        currentError.value = {
          key: 'CreateOrganization.errors.generic',
          data: {
            reason: result.error.tag,
          },
        };
        break;
    }
    await modalController.dismiss(null, MsModalResult.Cancel);
    await props.informationManager.present(
      new Information({
        message: I18n.translate(currentError.value),
        level: InformationLevel.Error,
      }),
      PresentationMode.Modal,
    );
  }
}

async function onGoClicked(): Promise<void> {
  if (!saveStrategy.value || !availableDevice.value || !organizationName.value) {
    window.electronAPI.log('error', 'OrganizationCreation: missing data at the end step, should not happen');
    return;
  }
  emits('organizationCreated', organizationName.value, availableDevice.value, saveStrategy.value as DeviceSaveStrategy);
}

async function onGoBackRequested(): Promise<void> {
  if (step.value === Steps.Authentication) {
    step.value = Steps.PersonalInformation;
  } else {
    console.log(`Cannot go back from ${step.value}: should not happen`);
  }
}
</script>

<style scoped lang="scss"></style>
