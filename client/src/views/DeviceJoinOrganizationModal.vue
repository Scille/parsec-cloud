<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonPage,
} from '@ionic/vue';
import { ref } from 'vue';

enum DeviceJoinOrganizationStep {
  WaitForHost = 1,
  GetHostSasCode = 2,
  ProvideGuestCode = 3,
  GetUserInfo = 4,
  GetPassword = 5,
  Finish = 6,
}

const props = defineProps<{
  invitationLink: string
}>();

interface ClaimDeviceLink {
  backendAddr: string,
  token: string,
  org: string
}

// Will be done in Rust
function parseInvitationLink(link: string): ClaimDeviceLink {
  return {
    backendAddr: link,
    token: 'aaa',
    org: 'My Org',
  };
}

const claimDeviceLink = parseInvitationLink(props.invitationLink);
const waitingForHost = ref(true);
</script>

<style scoped lang="scss">
</style>
