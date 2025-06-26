<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div>
    <span v-show="soloOwnerWorkspaces.length === 0">✓</span>
    {{
      $msTranslate({
        key: 'blablabla',
        count: soloOwnerWorkspaces.length,
        data: {
          workspace: soloOwnerWorkspaces.length === 1 ? soloOwnerWorkspaces[1].currentName : '',
          workspacesCount: soloOwnerWorkspaces.length,
        },
      })
    }}
  </div>
  <div><span v-show="hasMultipleDevices">✓</span> {{ 'HAS MULTIPLE DEVICES' }}</div>
  <div><span v-show="hasRecoveryDevice">✓</span> {{ 'CREATE A RECOVERY FILE' }}</div>
</template>

<script setup lang="ts">
import { getWorkspaceSharing, listOwnDevices, listWorkspaces, WorkspaceInfo, WorkspaceRole } from '@/parsec';
import { onMounted, ref } from 'vue';

const hasRecoveryDevice = ref(false);
const hasMultipleDevices = ref(false);
const soloOwnerWorkspaces = ref<Array<WorkspaceInfo>>([]);

onMounted(async () => {
  const devicesResult = await listOwnDevices();
  const nonRecoveryDevices = !devicesResult.ok ? [] : devicesResult.value.filter((d) => !d.isRecovery && !d.isCurrent);
  hasRecoveryDevice.value = devicesResult.ok && devicesResult.value.length !== nonRecoveryDevices.length;
  hasMultipleDevices.value = nonRecoveryDevices.length > 0;

  const workspacesResult = await listWorkspaces();
  for (const workspace of workspacesResult.ok ? workspacesResult.value : []) {
    if (workspace.currentSelfRole === WorkspaceRole.Owner) {
      const sharingResult = await getWorkspaceSharing(workspace.id, false, false);
      if (sharingResult.ok && !sharingResult.value.some(([_user, role]) => role === WorkspaceRole.Owner)) {
        soloOwnerWorkspaces.value.push(workspace);
      }
    }
  }
});
</script>

<style scoped lang="scss"></style>
