<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div id="shamir-self">
    <div v-if="shamirInfo && !error">
      <div v-if="shamirInfo.tag === SelfShamirRecoveryInfoTag.NeverSetup || shamirInfo.tag === SelfShamirRecoveryInfoTag.Deleted">
        <shamir-setup
          @shamir-setup="refreshShamirStatus"
          :information-manager="props.informationManager"
        />
      </div>
      <div v-else>
        <shamir-self-display
          :shamir-info="shamirInfo"
          @shamir-deleted="refreshShamirStatus"
          @close-modal="emits('closeModal')"
        />
      </div>
    </div>
    <div v-if="error">
      <ms-report-text :theme="MsReportTheme.Error">
        {{ $msTranslate(error) }}
      </ms-report-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import ShamirSelfDisplay from '@/components/profile/ShamirSelfDisplay.vue';
import ShamirSetup from '@/components/profile/ShamirSetup.vue';
import { getSelfShamirRecovery, SelfShamirRecoveryInfo, SelfShamirRecoveryInfoTag } from '@/parsec';
import { InformationManager } from '@/services/informationManager';
import { MsReportText, MsReportTheme } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const shamirInfo = ref<SelfShamirRecoveryInfo | undefined>(undefined);
const error = ref('');

const emits = defineEmits<{
  (e: 'shamirStatus', shamirInfo: SelfShamirRecoveryInfo): void;
  (e: 'closeModal'): void;
}>();

const props = defineProps<{
  informationManager: InformationManager;
}>();

onMounted(async () => {
  await refreshShamirStatus();
});

async function refreshShamirStatus(): Promise<void> {
  const result = await getSelfShamirRecovery();

  if (!result.ok) {
    error.value = 'OrganizationRecovery.shamir.errors.generic';
  } else {
    emits('shamirStatus', result.value);
    shamirInfo.value = result.value;
  }
}
</script>

<style scoped lang="scss"></style>
