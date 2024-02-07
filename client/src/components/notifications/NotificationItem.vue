<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="element-container"
    :class="{
      unread: notification.read === false,
      info: notification.information.level === InformationLevel.Info,
      success: notification.information.level === InformationLevel.Success,
      warning: notification.information.level === InformationLevel.Warning,
      error: notification.information.level === InformationLevel.Error,
    }"
    @mouseover="$emit('mouseOver', notification)"
  >
    <ion-item
      class="element ion-no-padding"
      @click="onClick"
    >
      <slot />
    </ion-item>
  </div>
</template>

<script setup lang="ts">
import { InformationLevel } from '@/services/informationManager';
import { Notification } from '@/services/notificationManager';
import { IonItem } from '@ionic/vue';

const props = defineProps<{
  notification: Notification;
}>();

const emits = defineEmits<{
  (event: 'click', notification: Notification): void;
  (event: 'mouseOver', notification: Notification): void;
}>();

async function onClick(): Promise<void> {
  emits('click', props.notification);
}
</script>

<style scoped lang="scss">
.element-container {
  cursor: pointer;
  background: var(--parsec-color-light-secondary-white);
  transition: background 0.2s ease-in-out;
  position: relative;

  // This will allow to change background and color of the icon
  --background-icon-info: var(--parsec-color-light-primary-50);
  --background-icon-success: var(--parsec-color-light-success-100);
  --background-icon-danger: var(--parsec-color-light-danger-100);
  --background-icon-warning: var(--parsec-color-light-warning-100);
  --color-icon-info: var(--parsec-color-light-primary-700);
  --color-icon-success: var(--parsec-color-light-success-700);
  --color-icon-danger: var(--parsec-color-light-danger-700);
  --color-icon-warning: var(--parsec-color-light-warning-700);

  &:not(:last-child) {
    border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
  }

  .hover-state {
    display: none;
  }

  &:hover {
    background: var(--parsec-color-light-secondary-disabled);
    .default-state {
      display: none;
    }
    .hover-state {
      display: block;
    }
  }

  .arrow-icon {
    color: var(--parsec-color-light-primary-600);
  }
}

.element {
  --background: none;
  --inner-padding-end: 0;
  --border-width: 0;
  padding: 1rem 1rem 1rem 1.75rem;

  &-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
    padding: 0.5rem;
    border-radius: var(--parsec-radius-12);

    .file-icon {
      width: 1.5rem;
      height: 1.5rem;
    }
  }

  &-details {
    display: flex;
    flex-direction: column;
    position: relative;
    margin-left: 0.875rem;

    &__message {
      color: var(--parsec-color-light-secondary-text);
    }

    &__time {
      color: var(--parsec-color-light-secondary-grey);
    }
  }

  .arrow-icon {
    margin-left: auto;
  }
}

.unread {
  background: var(--parsec-color-light-secondary-medium);

  &::before {
    content: '';
    position: absolute;
    top: 50%;
    left: 0.5rem;
    transform: translateY(-50%);
    width: 0.5rem;
    height: 0.5rem;
    display: flex;
    background: var(--parsec-color-light-gradient);
    border-radius: var(--parsec-radius-circle);
  }
}

.info {
  .element-icon {
    background: var(--background-icon-info);
    color: var(--color-icon-info);
  }
}

.success {
  .element-icon {
    background: var(--background-icon-success);
    color: var(--color-icon-success);
  }
}

.warning {
  .element-icon {
    background: var(--background-icon-warning);
    color: var(--color-icon-warning);
  }
}

.error {
  .element-icon {
    background: var(--background-icon-danger);
    color: var(--color-icon-danger);
  }
}
</style>
