<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      title="globalErrors.incompatibleEnvironment"
      :theme="MsReportTheme.Error"
      :close-button="{ visible: false }"
    >
      <div class="modal-container">
        {{ $msTranslate(message) }}
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage } from '@ionic/vue';
import { MsModal, MsReportTheme, Translatable } from 'megashark-lib';

defineProps<{
  message: Translatable;
}>();
</script>
