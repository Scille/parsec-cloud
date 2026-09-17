<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal-container">
    <div class="modal-content inner-content">
      <server-type-choice
        v-if="serverType === undefined"
        @server-chosen="onServerChosen"
        @close-requested="onCloseRequested"
      />
      <create-organization-saas
        v-if="serverType === ServerType.Saas"
        :bootstrap-link="bootstrapLink"
        :information-manager="informationManager"
        @close-requested="onCloseRequested"
        @organization-created="onOrganizationCreated"
        @back-requested="onBackToServerChoice"
      />
      <create-organization-custom-server
        v-if="serverType === ServerType.Custom"
        :bootstrap-link="bootstrapLink"
        @close-requested="onCloseRequested"
        @organization-created="onOrganizationCreated"
        @back-requested="onBackToServerChoice"
      />
      <create-organization-trial
        v-if="serverType === ServerType.Trial"
        :bootstrap-link="bootstrapLink"
        @close-requested="onCloseRequested"
        @organization-created="onOrganizationCreated"
        @back-requested="onBackToServerChoice"
        :information-manager="informationManager"
      />
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import {
  AvailableDevice,
  constructAccessStrategy,
  DeviceSaveStrategy,
  isWeb,
  OrganizationID,
  ParsedParsecAddrTag,
  parseParsecAddr,
} from '@/parsec';
import { Env } from '@/services/environment';
import { InformationManager } from '@/services/informationManager';
import {
  currentLocationIsSaasServer,
  currentLocationIsTrialServer,
  getServerTypeFromParsedParsecAddr,
  ServerType,
} from '@/services/parsecServers';
import CreateOrganizationCustomServer from '@/views/organizations/creation/CreateOrganizationCustomServer.vue';
import CreateOrganizationSaas from '@/views/organizations/creation/CreateOrganizationSaas.vue';
import CreateOrganizationTrial from '@/views/organizations/creation/CreateOrganizationTrial.vue';
import ServerTypeChoice from '@/views/organizations/creation/ServerTypeChoice.vue';
import { IonPage, modalController } from '@ionic/vue';
import { Answer, askQuestion, MsModalResult } from 'megashark-lib';
import { onMounted, ref, toRaw } from 'vue';

const props = defineProps<{
  informationManager: InformationManager;
  bootstrapLink?: string;
  defaultChoice?: ServerType;
}>();

const serverType = ref<ServerType | undefined>(props.defaultChoice);
const forced = ref(false);

onMounted(async () => {
  if (props.bootstrapLink) {
    const result = await parseParsecAddr(props.bootstrapLink);
    if (result.ok && result.value.tag === ParsedParsecAddrTag.OrganizationBootstrap) {
      serverType.value = getServerTypeFromParsedParsecAddr(result.value);
    }
  } else if ((window as any).TESTING === true || window.isDev()) {
    // On Playwright, everything's available
    serverType.value = undefined;
  } else if (currentLocationIsTrialServer()) {
    // Trial server, can't have custom or saas
    serverType.value = ServerType.Trial;
    forced.value = true;
  } else if (!currentLocationIsSaasServer() && !currentLocationIsTrialServer()) {
    // Custom server, can't have saas or trial
    serverType.value = ServerType.Custom;
    forced.value = true;
  } else if (currentLocationIsSaasServer()) {
    serverType.value = ServerType.Saas;
    forced.value = true;
  }
});

async function onServerChosen(chosenServerType: ServerType): Promise<void> {
  if (chosenServerType === ServerType.Trial && isWeb() && !currentLocationIsTrialServer() && !(window as any).TESTING === true) {
    await Env.Links.openUrl(`https://${Env.getTrialServers()[0]}/client/home?createOrg=trial`);
  } else if (chosenServerType === ServerType.Saas && isWeb() && !currentLocationIsSaasServer() && !(window as any).TESTING === true) {
    await Env.Links.openUrl(`https://${Env.getSaasServers()[0]}/client/home?createOrg=saas`);
  } else {
    serverType.value = chosenServerType;
  }
}

async function onCloseRequested(force = false): Promise<void> {
  let answer = Answer.Yes;
  // No point in having confirmation at this stage
  if (serverType.value !== undefined && !force) {
    answer = await askQuestion('CreateOrganization.cancelConfirm', 'CreateOrganization.cancelConfirmSubtitle', {
      keepMainModalHiddenOnYes: true,
      yesText: 'CreateOrganization.cancelYes',
      noText: 'CreateOrganization.cancelNo',
      yesIsDangerous: true,
      backdropDismiss: false,
    });
  }

  if (answer === Answer.Yes) {
    await modalController.dismiss(null, MsModalResult.Cancel);
  }
}

async function onOrganizationCreated(
  _organizationName: OrganizationID,
  device: AvailableDevice,
  saveStrategy: DeviceSaveStrategy,
): Promise<void> {
  const accessStrategy = constructAccessStrategy(device, toRaw(saveStrategy.primaryProtection));
  await modalController.dismiss({ device: device, access: accessStrategy }, MsModalResult.Confirm);
}

async function onBackToServerChoice(): Promise<void> {
  if (!forced.value) {
    serverType.value = undefined;
  } else {
    await onCloseRequested(true);
  }
}
</script>

<style lang="scss" scoped>
.modal-content {
  --height: 100%;
  height: 100%;

  @include ms.responsive-breakpoint('sm') {
    overflow: auto;
    min-width: 100%;
    --max-height: 90vh;
    max-height: 90vh;
  }
}
</style>
