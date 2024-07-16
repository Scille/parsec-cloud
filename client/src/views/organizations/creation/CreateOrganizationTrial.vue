<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <organization-user-information-page
      v-show="step === Steps.PersonalInformation"
      :class="step === Steps.PersonalInformation ? 'active' : ''"
      @user-information-filled="onUserInformationFilled"
      @close-requested="$emit('closeRequested')"
      :hide-previous="true"
    />
    <organization-authentication-page
      v-show="step === Steps.Authentication"
      :class="step === Steps.Authentication ? 'active' : ''"
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
import { IonPage } from '@ionic/vue';
import { onMounted, ref } from 'vue';
import OrganizationUserInformationPage from '@/views/organizations/creation/OrganizationUserInformationPage.vue';
import OrganizationAuthenticationPage from '@/views/organizations/creation/OrganizationAuthenticationPage.vue';
import OrganizationCreationPage from '@/views/organizations/creation/OrganizationCreationPage.vue';
import OrganizationCreatedPage from '@/views/organizations/creation/OrganizationCreatedPage.vue';
import {
  AvailableDevice,
  BootstrapOrganizationErrorTag,
  createOrganization as parsecCreateOrganization,
  bootstrapOrganization as parsecBootstrapOrganization,
  DeviceSaveStrategy,
  Result,
  BootstrapOrganizationError,
  OrganizationID,
  parseParsecAddr,
  ParsedParsecAddrTag,
} from '@/parsec';
import { adjectives, animals, colors, uniqueNamesGenerator } from 'unique-names-generator';
import { getServerAddress, ServerType } from '@/services/parsecServers';
import { getDefaultDeviceName } from '@/common/device';
import { Translatable, I18n } from 'megashark-lib';
import { wait } from '@/parsec/internals';

enum Steps {
  PersonalInformation,
  Authentication,
  Creation,
  Created,
}

const props = defineProps<{
  bootstrapLink?: string;
}>();

const emits = defineEmits<{
  (e: 'closeRequested'): void;
  (e: 'organizationCreated', organizationName: OrganizationID, device: AvailableDevice, saveStrategy: DeviceSaveStrategy): void;
}>();

const step = ref<Steps>(Steps.PersonalInformation);
const bootstrapLink = ref<string | undefined>(props.bootstrapLink);
const email = ref<string | undefined>(undefined);
const name = ref<string | undefined>(undefined);
const saveStrategy = ref<DeviceSaveStrategy | undefined>(undefined);
const availableDevice = ref<AvailableDevice | undefined>(undefined);
const currentError = ref<Translatable | undefined>(undefined);
const organizationName = ref<OrganizationID | undefined>(undefined);

onMounted(async () => {
  if (bootstrapLink.value) {
    const result = await parseParsecAddr(bootstrapLink.value);
    if (result.ok && result.value.tag === ParsedParsecAddrTag.OrganizationBootstrap) {
      organizationName.value = result.value.organizationId;
    }
  }
});

async function onUserInformationFilled(chosenName: string, chosenEmail: string): Promise<void> {
  email.value = chosenEmail;
  name.value = chosenName;
  step.value = Steps.Authentication;
}

function generateOrganizationName(): string {
  return `${uniqueNamesGenerator({ dictionaries: [colors, adjectives, animals], separator: '_' })}`.slice(0, 32);
}

async function createOrganization(): Promise<Result<AvailableDevice, BootstrapOrganizationError>> {
  if (!name.value || !email.value || !saveStrategy.value) {
    return { ok: false, error: { tag: BootstrapOrganizationErrorTag.Internal, error: 'Missing data' } };
  }

  let retry = 0;

  while (retry < 10) {
    const orgName = generateOrganizationName();
    const result = await parsecCreateOrganization(
      getServerAddress(ServerType.Trial) as string,
      orgName,
      name.value,
      email.value,
      getDefaultDeviceName(),
      saveStrategy.value,
    );
    if (result.ok) {
      organizationName.value = orgName;
      return result;
    } else {
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
  if (!name.value || !email.value || !saveStrategy.value || !bootstrapLink.value) {
    return { ok: false, error: { tag: BootstrapOrganizationErrorTag.Internal, error: 'Missing data' } };
  }
  const result = await parsecBootstrapOrganization(
    bootstrapLink.value,
    name.value,
    email.value,
    getDefaultDeviceName(),
    saveStrategy.value,
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

  if (bootstrapLink.value) {
    result = await bootstrapOrganization();
  } else {
    result = await createOrganization();
  }

  const endTime = new Date().valueOf();
  // If we're too fast, a weird blinking will occur. Add some artificial time.
  if (endTime - startTime < 2000) {
    await wait(endTime - startTime);
  }

  if (result.ok) {
    availableDevice.value = result.value;
    currentError.value = undefined;
    step.value = Steps.Created;
  } else {
    switch (result.error.tag) {
      case BootstrapOrganizationErrorTag.Offline:
        currentError.value = 'CreateOrganization.errors.offline';
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
    // TODO: DISPLAY SOMETHING TO THE USER
    console.error(I18n.translate(currentError.value));
  }
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
    step.value = Steps.PersonalInformation;
  } else {
    console.log(`Cannot go back from ${step.value}: should not happen`);
  }
}
</script>

<style scoped lang="scss"></style>
