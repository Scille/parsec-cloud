<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <div>
    {{ name }}
  </div>
  <div>
    {{ email }}
  </div>
  <div class="user-profile">
    <tag-profile
      :profile="profile"
    />
  </div>
</template>

<script setup lang="ts">
import { Profile } from '@/common/mocks';
import TagProfile from '@/components/users/TagProfile.vue';

defineProps<{
  name: string
  email: string
  profile: Profile
}>();
</script>

<style scoped lang="scss">
</style>
