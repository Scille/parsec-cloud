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
      <div class="element-type">
        <div class="element-type__icon">
          <!-- This icon is only a default placeholder, replace/add notification specific icons -->
          <ms-image
            :image="LogoIconGradient"
            class="file-icon"
          />
        </div>
      </div>
      <div class="element-details">
        <ion-text class="element-details__title body">
          {{ notification.title }}
        </ion-text>
        <ion-label class="element-details__message body-sm">
          <span class="default-state">{{ notification.message }}</span>
          <span class="hover-state">
            {{ $t('notificationCenter.browse') }}
          </span>
        </ion-label>
      </div>

      <ion-label class="element-details__time body-sm default-state">
        <span class="default-state">{{ formatTimeSince(notification.time, '', 'short') }}</span>
      </ion-label>
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

  .hover-state {
    display: none;
  }

  &:hover {
    background: var(--parsec-color-light-primary-50);
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
  // @SharkDesigner: this is commented because of the ion-list border bottom
  // padding: 1rem;

  .file-icon {
    width: 2rem;
    height: 2rem;
    // @SharkDesigner: this is a temporary fix to manage the padding: 1rem that should be on .element
    margin: 1rem 0 1rem 1rem;
  }

  &-details {
    display: flex;
    flex-direction: column;
    position: relative;
    margin-left: 0.875rem;

    &__title {
      color: var(--parsec-color-light-primary-800);
    }

    &__message,
    &__time {
      color: var(--parsec-color-light-secondary-grey);
      text-align: right;
    }
  }

  .element-details__time,
  .arrow-icon {
    margin-left: auto;
    // @SharkDesigner: this is a temporary fix to manage the padding: 1rem that should be on .element
    margin-right: 1rem;
  }
}

.unread {
  background: var(--parsec-color-light-secondary-medium);
}
</style>
