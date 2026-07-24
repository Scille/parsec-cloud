<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="checklist-security-container"
    id="trigger-checklist-button"
  >
    <div
      class="checklist-security"
      v-if="securityWarnings"
    >
      <ion-text class="checklist-security__title button-large">
        {{ $msTranslate('SideMenu.checklist.title') }}
      </ion-text>
      <ion-icon
        v-if="securityWarnings.hasMultipleDevices === true"
        class="checklist-security__icon"
        :icon="checkmarkCircle"
      />
      <ion-icon
        v-if="securityWarnings.hasRecoveryDevice === true"
        class="checklist-security__icon"
        :icon="checkmarkCircle"
      />
      <ion-icon
        v-if="
          userProfile !== UserProfile.Outsider && securityWarnings.soloOwnerWorkspaces.length === 0 && securityWarnings.needsSecondOwner
        "
        class="checklist-security__icon"
        :icon="checkmarkCircle"
      />
      <ion-icon
        class="checklist-security__icon to-do"
        v-for="index in securityWarningsCount"
        :key="index"
        :icon="ellipseOutline"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { SecurityWarnings } from '@/components/misc';
import { UserProfile } from '@/parsec';
import { IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle, ellipseOutline } from 'ionicons/icons';

defineProps<{
  userProfile: UserProfile;
  securityWarnings: SecurityWarnings | undefined;
  securityWarningsCount: number;
}>();
</script>

<style lang="scss">
#trigger-checklist-button {
  background: var(--parsec-color-light-secondary-background);
  cursor: pointer;

  .checklist-security {
    padding: 0.5rem 0.825rem;
    border-radius: var(--parsec-radius-8);
    display: flex;
    gap: 0.25rem;
    align-items: center;
    background: var(--parsec-color-light-gradient-background);
    box-shadow: var(--parsec-shadow-soft);
    display: flex;
    align-items: center;
    gap: 0.5rem;

    &__title {
      color: var(--parsec-color-light-secondary-white);
      margin-inline-end: 0.5rem;
      width: 100%;
    }

    &__icon {
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
      gap: 0.15rem;
      font-size: 1.5rem;
      color: var(--parsec-color-light-secondary-white);
      opacity: 0.8;

      &.to-do {
        color: var(--parsec-color-light-secondary-white);
      }
    }
  }
}
</style>
