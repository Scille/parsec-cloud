<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="content"
  >
    <user-avatar-name
      class="user"
      :user-avatar="user.humanHandle.label"
      :user-name="user.humanHandle.label"
    />

    <ms-select
      class="select"
      :options="options"
      :disabled="disabled"
      :default-option="role || NOT_SHARED_KEY"
      @change="$emit('roleUpdate', user, getRoleFromString($event.option.key))"
    />
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import { WorkspaceRole } from '@/parsec';
import MsSelect from '@/components/core/ms-select/MsSelect.vue';
import { Ref, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { MsSelectOption } from '@/components/core/ms-select/MsSelectOption';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { UserTuple } from '@/parsec';
import { translateWorkspaceRole } from '@/common/translations';

const { t } = useI18n();

defineProps<{
  user: UserTuple
  role: WorkspaceRole | null
  disabled?: boolean
}>();

defineEmits<{
  (e: 'roleUpdate', user: UserTuple, newRole: WorkspaceRole | null): void
}>();

const NOT_SHARED_KEY = 'not_shared';

const options: Ref<MsSelectOption[]> = ref([
  { key:  WorkspaceRole.Reader, label: translateWorkspaceRole(t, WorkspaceRole.Reader) },
  { key: WorkspaceRole.Contributor, label: translateWorkspaceRole(t, WorkspaceRole.Contributor) },
  { key: WorkspaceRole.Manager, label: translateWorkspaceRole(t, WorkspaceRole.Manager) },
  { key: WorkspaceRole.Owner, label: translateWorkspaceRole(t, WorkspaceRole.Owner) },
  { key: NOT_SHARED_KEY, label: translateWorkspaceRole(t, null) },
]);

function getRoleFromString(role: string): WorkspaceRole | null {
  if (role === NOT_SHARED_KEY) {
    return null;
  }
  return role as WorkspaceRole;
}
</script>

<style scoped lang="scss">
.content {
  height: 4em;
  padding: .5rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  position: relative;

  &::after {
    content: '';
    position: absolute;
    display: block;
    width: 90%;
    border: 1px solid var(--parsec-color-light-secondary-disabled);
    bottom: 0;
    right: 0;
  }
}
</style>
