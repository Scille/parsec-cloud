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
      <ion-text class="checklist-security__title">
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
import { computed } from 'vue';

const props = defineProps<{
  userProfile: UserProfile;
  securityWarnings: SecurityWarnings;
}>();

const securityWarningsCount = computed(() => {
  return (
    (props.securityWarnings.hasRecoveryDevice ? 0 : 1) +
    (props.securityWarnings.hasMultipleDevices ? 0 : 1) +
    (props.securityWarnings.soloOwnerWorkspaces.length === 0 ? 0 : 1)
  );
});
</script>

<style lang="scss">
#trigger-checklist-button {
  background: ms.color('surface-base-default-secondary');
  cursor: pointer;

  .checklist-security {
    padding: ms.spacing('padding-lg') 0.825rem;
    border-radius: ms.radius('lg');
    display: flex;
    gap: ms.spacing('gap-sm');
    align-items: center;
    background: linear-gradient(113deg, #{ms.color('surface-gradient-from')} -1.49%, #{ms.color('surface-gradient-to')} 100%);
    box-shadow: ms.shadow('light');
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');

    &__title {
      @include ms.font('label-lg-medium');
      color: ms.color('text-on-color-heading');
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
      color: ms.color('icon-neutral-on-color');
      opacity: ms.opacity('8');

      &.to-do {
        color: ms.color('icon-neutral-on-color');
      }
    }
  }
}
</style>
