<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="LongPathsSupportModal.title"
    :close-button="{ visible: false }"
    :confirm-button="{
      label: 'LongPathsSupportModal.closeButton',
      onClick: dismiss,
      disabled: false,
    }"
  >
    <div class="long-paths-modal">
      <ion-text>
        {{ $msTranslate('LongPathsSupportModal.labelInfo') }}
      </ion-text>
      <ion-text>
        <i18n-t
          keypath="LongPathsSupportModal.labelFix"
          scope="global"
        >
          <template #link>
            <a
              :href="$msTranslate('LongPathsSupportModal.fixUrl')"
              target="_blank"
            >
              {{ $msTranslate('LongPathsSupportModal.linkLabel') }}
            </a>
          </template>
        </i18n-t>
      </ion-text>
    </div>
    <ms-checkbox v-model="skipLongPathsSupportWarning">
      <ion-text>
        {{ $msTranslate('LongPathsSupportModal.noReminder') }}
      </ion-text>
    </ms-checkbox>
  </ms-modal>
</template>

<script setup lang="ts">
import { MsModal, MsCheckbox, MsModalResult } from 'megashark-lib';
import { IonText, modalController } from '@ionic/vue';
import { ref } from 'vue';

const skipLongPathsSupportWarning = ref(false);

async function dismiss(): Promise<boolean> {
  return await modalController.dismiss({ skipLongPathsSupportWarning: skipLongPathsSupportWarning.value }, MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped>
.long-paths-modal {
  display: flex;
  flex-direction: column;
  gap: 1.5em;
}
</style>
