<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="container">
    <ms-avatar
      class="avatar person-avatar body-sm"
      v-for="person in people.slice(0, maxDisplay)"
      :key="person"
      :data="person"
      :type="AvatarType.Initials"
    />

    <ms-avatar
      class="avatar extra-avatar"
      :data="''"
      :display="`+${people.length - maxDisplay}`"
      :type="AvatarType.Initials"
      :background-color="'#FFFFFF'"
      :text-color="''"
    />
  </div>
</template>

<script setup lang="ts">
import { defineProps } from 'vue';
import MsAvatar, { AvatarType } from '@/components/core/ms-avatar/MsAvatar.vue';

defineProps<{
  people: string[];
  maxDisplay: number;
}>();
</script>

<style lang="scss" scoped>
.container {
  display: flex;
  flex-wrap: nowrap;
  background: var(--parsec-color-light-secondary-inversed-contrast);
  border-radius: 32px;
  height: 1rem;

  .avatar {
    display: flex;
    align-items: center;
    justify-content: center;
    text-transform: uppercase;
    margin-left: -0.6em;
    border: 2px solid var(--parsec-color-light-secondary-inversed-contrast);

    &:first-of-type {
      margin-left: 0;
    }
  }
}

.person-avatar {
  background-color: var(--parsec-color-tags-indigo100);
  color: var(--parsec-color-tags-indigo500);
}

.extra-avatar {
  color: var(--parsec-color-light-secondary-grey);
}
</style>
