<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <div
      v-show="securityWarnings.isWorkspaceOwner"
      :class="{ clickable: securityWarnings.soloOwnerWorkspaces.length > 0 }"
      @click="securityWarnings.soloOwnerWorkspaces.length > 0 && onClick(RecommendationAction.AddWorkspaceOwner)"
    >
      <span v-show="securityWarnings.soloOwnerWorkspaces.length === 0">✓</span>
      {{
        $msTranslate({
          key: 'blablabla',
          count: securityWarnings.soloOwnerWorkspaces.length,
          data: {
            workspace: securityWarnings.soloOwnerWorkspaces.length === 1 ? securityWarnings.soloOwnerWorkspaces[0].currentName : '',
            workspacesCount: securityWarnings.soloOwnerWorkspaces.length,
          },
        })
      }}
      <span v-show="securityWarnings.soloOwnerWorkspaces.length > 0">></span>
    </div>
    <div
      :class="{ clickable: !securityWarnings.hasMultipleDevices }"
      @click="!securityWarnings.hasMultipleDevices && onClick(RecommendationAction.AddDevice)"
    >
      <span v-show="securityWarnings.hasMultipleDevices">✓</span>
      {{ 'HAS MULTIPLE DEVICES' }}
      <span v-show="!securityWarnings.hasMultipleDevices">></span>
    </div>
    <div
      :class="{ clickable: !securityWarnings.hasRecoveryDevice }"
      @click="!securityWarnings.hasRecoveryDevice && onClick(RecommendationAction.CreateRecoveryFiles)"
    >
      <span v-show="securityWarnings.hasRecoveryDevice">✓</span>
      {{ 'CREATE A RECOVERY FILE' }}
      <span v-show="!securityWarnings.hasRecoveryDevice">></span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { popoverController } from '@ionic/vue';
import { MsModalResult } from 'megashark-lib';
import { RecommendationAction, SecurityWarnings } from '@/components/misc/securityRecommendations';

defineProps<{
  securityWarnings: SecurityWarnings;
}>();

async function onClick(action: RecommendationAction): Promise<void> {
  await popoverController.dismiss({ action: action }, MsModalResult.Confirm);
}
</script>

<style scoped lang="scss">
.clickable {
  cursor: pointer;
}
</style>
