<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-label class="label-profile profile">
    <ion-chip
      class="button-small tag"
      :class="getClassForProfile(profile)"
    >
      {{ $t(getProfileLabel(profile)) }}
    </ion-chip>
  </ion-label>
</template>

<script setup lang="ts">
import { UserProfile } from '@/parsec';
import { IonChip, IonLabel } from '@ionic/vue';

defineProps<{
  profile: UserProfile;
}>();

function getProfileLabel(profile: UserProfile): string {
  if (profile === UserProfile.Admin) {
    return 'UsersPage.profile.admin.label';
  } else if (profile === UserProfile.Standard) {
    return 'UsersPage.profile.standard.label';
  } else if (profile === UserProfile.Outsider) {
    return 'UsersPage.profile.outsider.label';
  }
  return '';
}

function getClassForProfile(profile: UserProfile): string {
  switch (profile) {
    case UserProfile.Admin:
      return 'admin';
    case UserProfile.Standard:
      return 'standard';
    case UserProfile.Outsider:
      return 'outsider';
    default:
      return '';
  }
}
</script>

<style scoped lang="scss"></style>
