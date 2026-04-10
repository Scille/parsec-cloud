<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="profile-info-card">
    <ms-image
      :image="PersonCircle"
      class="profile-info-card__avatar"
    />
    <div class="profile-info-card-content">
      <div class="profile-info-card-item">
        <ion-text class="item-text title-h3">
          {{ label }}
        </ion-text>
        <user-profile-tag :profile="currentProfile" />
      </div>

      <div class="profile-info-card-item">
        <ion-text class="item-text body-lg">
          {{ email }}
        </ion-text>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import PersonCircle from '@/assets/images/person-circle.svg?raw';
import UserProfileTag from '@/components/users/UserProfileTag.vue';
import { UserProfile } from '@/parsec';
import { IonText } from '@ionic/vue';
import { MsImage } from 'megashark-lib';

defineProps<{
  label: string;
  email: string;
  currentProfile: UserProfile;
}>();
</script>

<style lang="scss" scoped>
.profile-info-card {
  display: flex;
  align-items: center;
  gap: 1rem;

  &__avatar {
    height: 3rem;
    color: var(--parsec-color-light-secondary-text);
  }

  &-content {
    display: flex;
    flex-direction: column;
  }

  &-item {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    justify-content: space-between;
    color: var(--parsec-color-light-secondary-text);
  }
}
</style>
