<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="$t('WorkspaceSharing.title')"
      :close-button-enabled="true"
      :cancel-button="{
        label: $t('WorkspaceSharing.cancel'),
        disabled: false,
        onClick: cancel
      }"
    >
      <!-- content -->
      <div>
        <ms-input
          v-model="search"
          :label="$t('WorkspaceSharing.searchLabel')"
          :placeholder="$t('WorkspaceSharing.searchPlaceholder')"
        />
        <ion-list class="user-list">
          <workspace-user-role
            :disabled="true"
            :user="$t('WorkspaceSharing.currentUserLabel')"
            :role="workspaceInfo ? workspaceInfo.role : null"
          />
          <workspace-user-role
            v-for="entry in userRoles"
            :key="entry[0]"
            :disabled="entry[0] === 'Me'"
            :user="entry[0]"
            :role="entry[1]"
            @role-update="updateUserRole"
          />
        </ion-list>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonPage,
  IonList,
  modalController,
} from '@ionic/vue';
import { ref, watch, onUnmounted, onMounted, Ref } from 'vue';
import { MsModalResult } from '@/components/core/ms-types';
import { WorkspaceID, WorkspaceRole, getWorkspaceInfo, UserID, WorkspaceInfo } from '@/parsec';
import WorkspaceUserRole from '@/components/workspaces/WorkspaceUserRole.vue';
import MsModal from '@/components/core/ms-modal/MsModal.vue';
import MsInput from '@/components/core/ms-input/MsInput.vue';

const search = ref('');

const props = defineProps<{
  workspaceId: WorkspaceID
}>();

const workspaceInfo: Ref<WorkspaceInfo | null> = ref(null);
const userRoles: Ref<Array<[UserID, WorkspaceRole | null]>> = ref([]);

// Would prefere to use a computed instead of a watch but
// Vue doesn't handle async in computed.
const unwatchSearch = watch(search, async() => {
  const result = await getWorkspaceInfo(props.workspaceId);
  const roles: Array<[UserID, WorkspaceRole | null]> = [];

  if (result.ok) {
    const lowerCaseSearch = search.value.toLocaleLowerCase();
    workspaceInfo.value = result.value;
    for (const entry of result.value.sharingInfo) {
      if (entry[0].toLocaleLowerCase().includes(lowerCaseSearch)) {
        roles.push([entry[0], entry[1]]);
      }
    }
  } else {
    console.log(`Could not get workspace info for ${props.workspaceId}`, result.error);
  }
  userRoles.value = roles;
});

onMounted(async () => {
  const result = await getWorkspaceInfo(props.workspaceId);

  if (result.ok) {
    workspaceInfo.value = result.value;
    userRoles.value = result.value.sharingInfo;
  } else {
    console.log(`Could not get workspace info for ${props.workspaceId}`, result.error);
  }
});

onUnmounted(() => {
  unwatchSearch();
});

async function updateUserRole(user: string, role: WorkspaceRole | null): Promise<void> {
  console.log(`Update user ${user} role to ${role}`);
}

function cancel(): Promise<boolean> {
  return modalController.dismiss(userRoles.value, MsModalResult.Cancel);
}
</script>

<!-- eslint-disable-next-line vue-scoped-css/enforce-style-type -->
<style lang="scss">
.ms-modal {
  display: flex;
  flex-direction: column;
  height: -webkit-fill-available;

  .inner-content {
    max-height: none;
    height: 100%;
  }
}
</style>

<style scoped lang="scss">
.modal {
  padding: 2.5rem;
}

.user-list {
  padding: .5rem;
}
</style>
