<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <div class="modal-content inner-content">
      <server-type-choice
        v-if="serverType === undefined"
        @server-chosen="onServerChosen"
        @close-requested="onCloseRequested"
      />
      <create-organization-saas
        v-if="serverType === ServerType.Saas"
        :bootstrap-link="bootstrapLink"
      />
      <create-organization-custom-server
        v-if="serverType === ServerType.Custom"
        :bootstrap-link="bootstrapLink"
      />
      <create-organization-trial
        v-if="serverType === ServerType.Trial"
        :bootstrap-link="bootstrapLink"
      />
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { parseParsecAddr, ParsedParsecAddrTag } from '@/parsec';
import { InformationManager } from '@/services/informationManager';
import { onMounted, ref } from 'vue';
import ServerTypeChoice from '@/views/organizations/ServerTypeChoice.vue';
import { ServerType, getServerTypeFromHost } from '@/services/parsecServers';
import { IonPage, modalController } from '@ionic/vue';
import CreateOrganizationCustomServer from '@/views/organizations/CreateOrganizationCustomServer.vue';
import CreateOrganizationSaas from '@/views/organizations/CreateOrganizationSaas.vue';
import CreateOrganizationTrial from '@/views/organizations/CreateOrganizationTrial.vue';

const props = defineProps<{
  informationManager: InformationManager;
  bootstrapLink?: string;
}>();

const serverType = ref<ServerType | undefined>(undefined);

onMounted(async () => {
  if (props.bootstrapLink) {
    const result = await parseParsecAddr(props.bootstrapLink);
    if (result.ok && result.value.tag === ParsedParsecAddrTag.OrganizationBootstrap) {
      // TODO: port is not optional in ParsecParsecAddr, check the default value
      serverType.value = getServerTypeFromHost(result.value.hostname, result.value.port);
    }
  }
});

async function onServerChosen(chosenServerType: ServerType): Promise<void> {
  serverType.value = chosenServerType;
}

async function onCloseRequested(): Promise<void> {
  console.log('CLOSE');
  modalController.dismiss();
}
</script>

<style lang="scss" scoped>
.modal-content {
  min-width: 32em;
  min-height: 32em;
}
</style>
