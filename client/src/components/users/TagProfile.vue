<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-label class="label-profile profile">
    <ion-chip
      class="caption-caption tag"
      :class="profile"
    >
      {{ translateProfile(profile) }}
    </ion-chip>
  </ion-label>
</template>

<script setup lang="ts">
import { IonChip, IonLabel } from '@ionic/vue';
import { defineProps } from 'vue';
import { useI18n } from 'vue-i18n';
import { UserProfile } from '@/parsec';

const { t } = useI18n();

defineProps<{
  profile: UserProfile
}>();

function translateProfile(profile: UserProfile): string {
  if (profile === UserProfile.Admin) {
    return t('UsersPage.profile.admin');
  } else if (profile === UserProfile.Standard) {
    return t('UsersPage.profile.standard');
  } else if (profile === UserProfile.Outsider) {
    return t('UsersPage.profile.outsider');
  }
  return '';
}
</script>

<style scoped lang="scss">
</style>
