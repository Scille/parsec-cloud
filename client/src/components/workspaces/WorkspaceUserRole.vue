<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div
    class="content"
  >
    <user-avatar-name
      class="user"
      :user-avatar="$props.user"
      :user-name="$props.user"
    />

    <ms-select
      class="select"
      :options="options"
      :default-option="$props.role || NOT_SHARED_KEY"
      @change="$emit('roleUpdate', $props.user, getRoleFromString($event.option.key))"
    />
  </div>
</template>

<script setup lang="ts">
import { defineProps, defineEmits } from 'vue';
import { WorkspaceRole } from '@/common/mocks';
import MsSelect from '@/components/core/ms-select/MsSelect.vue';
import { Ref, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { MsSelectOption } from '@/components/core/ms-select/MsSelectOption';
import UserAvatarName from '@/components/users/UserAvatarName.vue';

const { t } = useI18n();

defineProps<{
  user: string
  role: WorkspaceRole | null
}>();

defineEmits<{
  (e: 'roleUpdate', user: string, newRole: WorkspaceRole | null): void
}>();

const NOT_SHARED_KEY = 'not_shared';

const options: Ref<MsSelectOption[]> = ref([
  { key:  WorkspaceRole.Reader, label: t('WorkspaceSharing.roles.Reader') },
  { key: WorkspaceRole.Contributor, label: t('WorkspaceSharing.roles.Contributor') },
  { key: WorkspaceRole.Manager, label: t('WorkspaceSharing.roles.Manager') },
  { key: WorkspaceRole.Owner, label: t('WorkspaceSharing.roles.Owner') },
  { key: NOT_SHARED_KEY, label: t('WorkspaceSharing.roles.NotShared') },
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
