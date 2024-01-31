<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="element-container"
    :class="{
      unread: notification.read === false,
    }"
    @mouseover="$emit('mouseOver', notification)"
  >
    <ion-item
      class="element ion-no-padding"
      @click="onClick"
    >
      <div class="element-icon">
        <!-- This icon is only a default placeholder, replace/add notification specific icons -->
        <ms-image
          :image="LogoIconGradient"
          class="file-icon"
        />
      </div>
      <div class="element-details">
        <ion-label class="element-details__message body">
          <span>{{ notification.message }}</span>
        </ion-label>
        <ion-text class="element-details__time body-sm">
          <span class="default-state">{{ formatTimeSince(notification.time, '', 'short') }}</span>
          <span class="hover-state">{{ $t('notificationCenter.browse') }}</span>
        </ion-text>
      </div>
      <ion-icon
        class="arrow-icon hover-state"
        :icon="arrowForward"
      />
    </ion-item>
  </div>
</template>

<script setup lang="ts">
import { formatTimeSince } from '@/common/date';
import { LogoIconGradient, MsImage } from '@/components/core/ms-image';
import { Notification } from '@/services/notificationManager';
import { IonIcon, IonItem, IonLabel, IonText } from '@ionic/vue';
import { arrowForward } from 'ionicons/icons';

const props = defineProps<{
  notification: Notification;
}>();

defineExpose({
  props,
});

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

  // This will allow to change background and color of the icon
  --background-icon-info: var(--parsec-color-light-primary-50);
  --background-icon-success: var(--parsec-color-light-success-100);
  --background-icon-danger: var(--parsec-color-light-danger-100);
  --background-icon-warning: var(--parsec-color-light-warning-100);
  --color-icon-info: var(--parsec-color-light-primary-700);
  --color-icon-success: var(--parsec-color-light-success-700);
  --color-icon-danger: var(--parsec-color-light-danger-700);
  --color-icon-warning: var(--parsec-color-light-warning-700);

  &-icon {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.5rem;
    height: 2.5rem;
    padding: 0.5rem;
    background: var(--background-icon-info);
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
</style>
