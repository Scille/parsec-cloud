<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="profile-info-card"
    :class="{ 'profile-info-card--center': centerContent }"
  >
    <ion-text class="profile-info-card__name title-h2">
      {{ label }}
    </ion-text>
    <ion-text class="profile-info-card__email body">
      {{ email }}
    </ion-text>
    <tag-profile :profile="currentProfile" />
  </div>
</template>

<script setup lang="ts">
import TagProfile from '@/components/users/TagProfile.vue';
import { UserProfile } from '@/parsec';
import { IonText } from '@ionic/vue';

defineProps<{
  label: string;
  email: string;
  currentProfile: UserProfile;
  centerContent?: boolean;
}>();
</script>

<style lang="scss" scoped>
.profile-info-card {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;

  &--center {
    align-items: center;
  }

  &__name {
    color: var(--parsec-color-light-secondary-text);
  }

  &__email {
    margin-bottom: 0.5rem;
    color: var(--parsec-color-light-secondary-soft-text);
  }
}
</style>
