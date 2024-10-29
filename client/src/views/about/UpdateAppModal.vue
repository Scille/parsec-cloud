<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="UPDATE APP"
    :close-button="{ visible: true }"
    class="update-app"
  >
    <ion-text>
      {{ $msTranslate('NEW UPDATE AVAILABLE') }}
    </ion-text>
    <ion-text>
      {{ currentVersion ?? APP_VERSION }}
    </ion-text>
    <ion-text>
      {{ targetVersion }}
    </ion-text>
    <ion-button @click="openChangelog">
      {{ $msTranslate('SEE CHANGELOG') }}
    </ion-button>
    <div>
      {{ $msTranslate('HomePage.topbar.updateConfirmQuestion') }}
    </div>
    <ion-button @click="dismiss(MsModalResult.Confirm)">
      {{ $msTranslate('HomePage.topbar.updateYes') }}
    </ion-button>
    <ion-button @click="dismiss(MsModalResult.Cancel)">
      {{ $msTranslate('HomePage.topbar.updateNo') }}
    </ion-button>
  </ms-modal>
</template>

<script setup lang="ts">
import { MsModal, MsModalResult } from 'megashark-lib';
import { modalController, IonText, IonButton } from '@ionic/vue';
import { APP_VERSION, Env } from '@/services/environment';

const props = defineProps<{
  currentVersion?: string;
  targetVersion: string;
}>();

async function openChangelog(): Promise<void> {
  await Env.Links.openChangelogLink(props.targetVersion);
}

async function dismiss(role: MsModalResult): Promise<void> {
  await modalController.dismiss(undefined, role);
}
</script>

<style lang="scss" scoped>
.update-app {
  background-color: rgb(255, 0, 212);
}
</style>
