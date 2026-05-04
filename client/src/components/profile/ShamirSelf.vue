<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    SHAMIR SELF
    <div v-if="shamirInfo && !error">
      <div v-if="shamirInfo.tag === SelfShamirRecoveryInfoTag.NeverSetup || shamirInfo.tag === SelfShamirRecoveryInfoTag.Deleted">
        <shamir-setup @shamir-setup="refreshShamirStatus" />
      </div>
      <div v-else>
        <shamir-self-display
          :shamir-info="shamirInfo"
          @shamir-deleted="refreshShamirStatus"
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
import { MsReportText, MsReportTheme } from 'megashark-lib';
import { onMounted, ref } from 'vue';

const shamirInfo = ref<SelfShamirRecoveryInfo | undefined>(undefined);
const error = ref('');

onMounted(async () => {
  await refreshShamirStatus();
});

async function refreshShamirStatus(): Promise<void> {
  const result = await getSelfShamirRecovery();

  if (!result.ok) {
    error.value = 'ERROR';
  } else {
    shamirInfo.value = result.value;
  }
}
</script>

<style scoped lang="scss"></style>
