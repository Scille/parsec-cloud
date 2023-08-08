<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-label class="label-role profile">
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
import { Profile } from '@/common/mocks';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

defineProps<{
  profile: Profile
}>();

function translateProfile(profile: Profile): string {
  if (profile === Profile.Admin) {
    return t('UsersPage.role.admin');
  } else if (profile === Profile.Standard) {
    return t('UsersPage.role.standard');
  } else if (profile === Profile.Outsider) {
    return t('UsersPage.role.external');
  }
  return '';
}
</script>

<style scoped lang="scss">
</style>
