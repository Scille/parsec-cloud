<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="content">
    <user-avatar-name
      class="user"
      :user-avatar="user.humanHandle.label"
      :user-name="user.humanHandle.label"
    />

    <ms-dropdown
      class="dropdown"
      :options="options"
      :disabled="disabled"
      :default-option-key="role || NOT_SHARED_KEY"
      :appearance="MsAppearance.Clear"
      @change="$emit('roleUpdate', user, getRoleFromString($event.option.key))"
    />
  </div>
</template>

<script setup lang="ts">
import { MsAppearance, MsDropdown, MsOptions } from '@/components/core';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { canChangeRole } from '@/components/workspaces/utils';
import { UserProfile, UserTuple, WorkspaceRole } from '@/parsec';
import { translateWorkspaceRole } from '@/services/translation';
import { computed } from 'vue';

const props = defineProps<{
  user: UserTuple;
  role: WorkspaceRole | null;
  disabled?: boolean;
  clientProfile: UserProfile;
  clientRole: WorkspaceRole | null;
}>();

defineEmits<{
  (e: 'roleUpdate', user: UserTuple, newRole: WorkspaceRole | null): void;
}>();

const NOT_SHARED_KEY = 'not_shared';

const options = computed((): MsOptions => {
  return new MsOptions(
    [WorkspaceRole.Owner, WorkspaceRole.Manager, WorkspaceRole.Contributor, WorkspaceRole.Reader, null].map(
      (role: WorkspaceRole | null) => {
        return {
          key: role === null ? NOT_SHARED_KEY : role,
          label: translateWorkspaceRole(role).label,
          description: translateWorkspaceRole(role).description,
          disabled: !canChangeRole(props.clientProfile, props.user.profile, props.clientRole, props.role, role).authorized,
          disabledReason: canChangeRole(props.clientProfile, props.user.profile, props.clientRole, props.role, role).reason,
        };
      },
    ),
  );
});

function getRoleFromString(role: string): WorkspaceRole | null {
  if (role === NOT_SHARED_KEY) {
    return null;
  }
  return role as WorkspaceRole;
}
</script>

<style scoped lang="scss">
.content {
  padding: 0.5rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  position: relative;

  &::after {
    content: '';
    position: absolute;
    display: block;
    width: 90%;
    border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
    bottom: 0;
    right: 0;
  }
}
</style>
