<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-label class="label-profile profile">
    <ion-chip
      class="button-small tag"
      :class="getClassForProfile(profile)"
    >
      {{ translateProfile(profile) }}
    </ion-chip>
  </ion-label>
</template>

<script setup lang="ts">
import { UserProfile } from '@/parsec';
import { translateProfile } from '@/services/translation';
import { IonChip, IonLabel } from '@ionic/vue';

defineProps<{
  profile: UserProfile;
}>();

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
