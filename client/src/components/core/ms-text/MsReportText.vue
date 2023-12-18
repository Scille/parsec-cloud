<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="container-textinfo"
    :class="props.theme ?? MsReportTheme.Info"
  >
    <ion-icon
      :icon="getIcon()"
      size="default"
      class="container-textinfo__icon"
    />
    <ion-text class="subtitles-normal container-textinfo__text">
      <slot />
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { MsReportTheme } from '@/components/core/ms-types';
import { IonIcon, IonText } from '@ionic/vue';
import { checkmarkCircle, closeCircle, informationCircle, warning } from 'ionicons/icons';

const props = defineProps<{
  theme?: MsReportTheme;
}>();

function getIcon(): string {
  switch (props.theme) {
    case MsReportTheme.Info:
      return informationCircle;
    case MsReportTheme.Success:
      return checkmarkCircle;
    case MsReportTheme.Warning:
      return warning;
    case MsReportTheme.Error:
      return closeCircle;
  }
  return informationCircle;
}
</script>

<style scoped lang="scss">
.ms-info {
  --ms-alert-text-background-color: var(--parsec-color-light-primary-100);
  --ms-alert-text-icon-color: var(--parsec-color-light-primary-500);
}

.ms-success {
  --ms-alert-text-background-color: var(--parsec-color-light-success-100);
  --ms-alert-text-icon-color: var(--parsec-color-light-success-500);
}

.ms-warning {
  --ms-alert-text-background-color: var(--parsec-color-light-warning-100);
  --ms-alert-text-icon-color: var(--parsec-color-light-warning-500);
}

.ms-error {
  --ms-alert-text-background-color: var(--parsec-color-light-danger-100);
  --ms-alert-text-icon-color: var(--parsec-color-light-danger-500);
}

.container-textinfo {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 1rem;
  background-color: var(--ms-alert-text-background-color);

  &__icon {
    color: var(--ms-alert-text-icon-color);
    font-size: 1.25rem;
    min-width: 1.25rem;
  }

  &__text {
    color: var(--parsec-color-light-secondary-grey);
  }
  &.ms-info,
  &.ms-success,
  &.ms-warning,
  &.ms-error {
    border-radius: 4px;
    padding: 1em;
  }
}
</style>
