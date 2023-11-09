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
      :default-option="role || NOT_SHARED_KEY"
      @change="$emit('roleUpdate', user, getRoleFromString($event.option.key))"
    />
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits, computed } from 'vue';
import { UserProfile, WorkspaceRole, canChangeRole } from '@/parsec';
import MsDropdown from '@/components/core/ms-dropdown/MsDropdown.vue';
import { MsDropdownOption } from '@/components/core/ms-types';
import { useI18n } from 'vue-i18n';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { UserTuple } from '@/parsec';
import { translateWorkspaceRole } from '@/common/translations';

const { t } = useI18n();

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

const options = computed((): MsDropdownOption[] => {
  return [
    {
      key: WorkspaceRole.Reader,
      label: translateWorkspaceRole(t, WorkspaceRole.Reader),
      disabled: !canChangeRole(props.clientProfile, props.user.profile, props.clientRole, props.role, WorkspaceRole.Reader),
    },
    {
      key: WorkspaceRole.Contributor,
      label: translateWorkspaceRole(t, WorkspaceRole.Contributor),
      disabled: !canChangeRole(props.clientProfile, props.user.profile, props.clientRole, props.role, WorkspaceRole.Contributor),
    },
    {
      key: WorkspaceRole.Manager,
      label: translateWorkspaceRole(t, WorkspaceRole.Manager),
      disabled: !canChangeRole(props.clientProfile, props.user.profile, props.clientRole, props.role, WorkspaceRole.Manager),
    },
    {
      key: WorkspaceRole.Owner,
      label: translateWorkspaceRole(t, WorkspaceRole.Owner),
      disabled: !canChangeRole(props.clientProfile, props.user.profile, props.clientRole, props.role, WorkspaceRole.Owner),
    },
    {
      key: NOT_SHARED_KEY,
      label: translateWorkspaceRole(t, null),
      disabled: !canChangeRole(props.clientProfile, props.user.profile, props.clientRole, props.role, null),
    },
  ];
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
  height: 4em;
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
    border: 1px solid var(--parsec-color-light-secondary-disabled);
    bottom: 0;
    right: 0;
  }
}
</style>
