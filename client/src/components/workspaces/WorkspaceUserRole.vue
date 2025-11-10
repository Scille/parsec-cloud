<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="workspace-user-role">
    <div class="content-user">
      <user-avatar-name
        :user-avatar="user.humanHandle.label"
        :user-name="user.humanHandle.label"
      />
      <span
        v-if="isCurrentUser"
        class="body content-user__you"
      >
        {{ $msTranslate('UsersPage.currentUser') }}
      </span>

      <user-profile-tag
        v-if="user.profile === UserProfile.Outsider"
        :profile="UserProfile.Outsider"
      />
    </div>

    <ms-dropdown
      ref="dropdown"
      class="dropdown"
      :options="options"
      :disabled="disabled"
      :default-option-key="role"
      :appearance="MsAppearance.Outline"
      @change="onRoleChanged(user, $event.option, $event.oldOption)"
    />
  </div>
</template>

<script setup lang="ts">
import { UserAvatarName, UserProfileTag } from '@/components/users';
import { canChangeRole } from '@/components/workspaces/utils';
import { UserProfile, UserTuple, WorkspaceRole } from '@/parsec';
import { getWorkspaceRoleTranslationKey } from '@/services/translation';
import { MsAppearance, MsDropdown, MsOption, MsOptions } from 'megashark-lib';
import { computed, useTemplateRef } from 'vue';

const props = defineProps<{
  user: UserTuple;
  role: WorkspaceRole | null;
  disabled?: boolean;
  clientProfile: UserProfile;
  clientRole: WorkspaceRole | null;
  isCurrentUser?: boolean;
}>();

const emits = defineEmits<{
  (e: 'roleUpdate', user: UserTuple, oldRole: WorkspaceRole | null, newRole: WorkspaceRole | null, reject: () => void): void;
}>();

const dropdownRef = useTemplateRef<InstanceType<typeof MsDropdown>>('dropdown');

const options = computed((): MsOptions => {
  return new MsOptions(
    [WorkspaceRole.Owner, WorkspaceRole.Manager, WorkspaceRole.Contributor, WorkspaceRole.Reader, null].map(
      (role: WorkspaceRole | null) => {
        return {
          key: role,
          label: getWorkspaceRoleTranslationKey(role).label,
          description: getWorkspaceRoleTranslationKey(role).description,
          disabled: !canChangeRole(props.clientProfile, props.user.profile, props.clientRole, props.role, role).authorized,
          disabledReason: canChangeRole(props.clientProfile, props.user.profile, props.clientRole, props.role, role).reason,
        };
      },
    ),
  );
});

function onRoleChanged(user: UserTuple, newRoleOption: MsOption, oldRoleOption?: MsOption): void {
  if (oldRoleOption && newRoleOption.key === oldRoleOption.key) {
    return;
  }
  emits('roleUpdate', user, oldRoleOption ? oldRoleOption.key : null, newRoleOption.key, () => {
    dropdownRef.value?.setCurrentKey(oldRoleOption ? oldRoleOption.key : null);
  });
}
</script>

<style scoped lang="scss">
.workspace-user-role {
  padding: 0.5rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  position: relative;
  overflow: hidden;

  &::after {
    content: '';
    position: absolute;
    display: block;
    width: calc(100% - 3rem);
    border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
    bottom: 0;
    right: 0;
  }
}

.content-user {
  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
  display: flex;
  align-items: center;
  gap: 0.5rem;

  &__you {
    color: var(--parsec-color-light-secondary-grey);
  }
}
</style>
