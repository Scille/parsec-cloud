<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="profile-info-card">
    <ms-image
      :image="PersonCircle"
      class="profile-info-card__avatar"
    />
    <div class="profile-info-card-content">
      <div class="profile-info-card-item">
        <ion-text class="item-text item-text--name">
          {{ label }}
        </ion-text>
        <user-profile-tag :profile="currentProfile" />
      </div>

      <div class="profile-info-card-item">
        <ion-text class="item-text item-text--email">
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
  gap: ms.spacing('gap-3xl');

  &__avatar {
    height: 3rem;
    color: ms.color('text-base-body');
    flex-shrink: 0;
  }

  &-content {
    display: flex;
    flex-direction: column;
  }

  &-item {
    display: flex;
    gap: ms.spacing('gap-2xl');
    justify-content: space-between;

    .item-text--name {
      @include ms.font('heading-h4');
    }

    .item-text--email {
      @include ms.font('body-lg-regular');
      color: ms.color('text-base-description');
    }
  }
}
</style>
