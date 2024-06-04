<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="content">
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
    </div>

    <ms-dropdown
      ref="dropdownRef"
      class="dropdown"
      :options="options"
      :disabled="disabled"
      :default-option-key="role"
      :appearance="MsAppearance.Clear"
      @change="onRoleChanged(user, $event.option, $event.oldOption)"
    />
  </div>
</template>

<script setup lang="ts">
import { MsAppearance, MsDropdown, MsOption, MsOptions } from 'megashark-lib';
import UserAvatarName from '@/components/users/UserAvatarName.vue';
import { canChangeRole } from '@/components/workspaces/utils';
import { UserProfile, UserTuple, WorkspaceRole } from '@/parsec';
import { getWorkspaceRoleTranslationKey } from '@/services/translation';
import { computed, ref } from 'vue';

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

const dropdownRef = ref();

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
    dropdownRef.value.setCurrentKey(oldRoleOption ? oldRoleOption.key : null);
  });
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

.content-user {
  display: flex;
  align-items: center;
  gap: 0.5rem;

  &__you {
    color: var(--parsec-color-light-primary-600);
  }
}
</style>
