<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="container-textinfo"
    :class="props.theme ?? MsTheme.Basic"
  >
    <ion-icon
      :icon="getIcon()"
      size="default"
      class="container-textinfo__icon"
    />
    <ion-text
      class="subtitles-normal container-textinfo__text"
    >
      <slot />
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { IonIcon, IonText } from '@ionic/vue';
import {
  caretForwardCircle,
  informationCircle,
  checkmarkCircle,
  warning,
  closeCircle,
} from 'ionicons/icons';
import { MsTheme } from '@/components/core/ms-types';

const props = defineProps<{
  icon?: string,
  theme?: MsTheme,
}>();

function getIcon(): string {
  switch (props.theme) {
    case MsTheme.Basic:
      return caretForwardCircle;
    case MsTheme.Info:
      return informationCircle;
    case MsTheme.Success:
      return checkmarkCircle;
    case MsTheme.Warning:
      return warning;
    case MsTheme.Error:
      return closeCircle;
  }
  return props.icon ?? caretForwardCircle;
}
</script>

<style scoped lang="scss">
.ms-basic {
  --ms-informative-text-background-color: none;
  --ms-informative-text-icon-color: var(--parsec-color-light-primary-600);
}

.ms-info {
  --ms-informative-text-background-color: var(--parsec-color-light-primary-100);
  --ms-informative-text-icon-color: var(--parsec-color-light-primary-500);
}

.ms-success {
  --ms-informative-text-background-color: var(--parsec-color-light-success-100);
  --ms-informative-text-icon-color: var(--parsec-color-light-success-500);
}

.ms-warning {
  --ms-informative-text-background-color: var(--parsec-color-light-warning-100);
  --ms-informative-text-icon-color: var(--parsec-color-light-warning-500);
}

.ms-error {
  --ms-informative-text-background-color: var(--parsec-color-light-danger-100);
  --ms-informative-text-icon-color: var(--parsec-color-light-danger-500);
}

.container-textinfo {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 1rem;
  background-color: var(--ms-informative-text-background-color);

  &__icon {
    color: var(--ms-informative-text-icon-color);
    font-size: 1.25rem;
    min-width: 1.25rem;
  }

  &__text {
    color: var(--parsec-color-light-secondary-grey);
  }
  &.ms-info, &.ms-success, &.ms-warning, &.ms-error {
    border-radius: 4px;
    padding: 1em;
  }
}
</style>
