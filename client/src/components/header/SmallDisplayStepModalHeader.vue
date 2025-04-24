<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    v-if="!hideCloseButton"
    slot="icon-only"
    class="closeBtn"
    :class="{
      'closeBtn-stepper': step,
    }"
    @click="$emit('closeClicked')"
  >
    <ion-icon
      :icon="close"
      class="closeBtn__icon"
    />
  </ion-button>
  <ion-header
    class="modal-header"
    :class="{
      'modal-header-stepper': step,
    }"
  >
    <div
      class="modal-header-stepper-label"
      v-if="step"
    >
      <ion-icon
        :icon="step.icon"
        class="modal-header-stepper-label__icon"
      />
      <ion-text class="modal-header-stepper-label__text button-medium">
        {{ $msTranslate(step.title) }}
      </ion-text>
    </div>
    <div class="modal-header-title">
      <ion-text
        class="modal-header-title__step body"
        v-if="step?.current !== undefined"
      >
        {{
          $msTranslate({
            key: 'HeaderPage.modalHeader.step',
            data: {
              current: step.current,
              total: step.total,
            },
          })
        }}
      </ion-text>
      <ion-text class="modal-header-title__text title-h2">
        {{ $msTranslate(title) }}
      </ion-text>
    </div>
    <ion-text
      class="modal-header__subtitle body-lg"
      v-if="subtitle"
    >
      {{ $msTranslate(subtitle) }}
    </ion-text>
  </ion-header>
</template>

<script setup lang="ts">
import { IonButton, IonHeader, IonIcon, IonText } from '@ionic/vue';
import { close } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';

defineProps<{
  title: Translatable;
  subtitle?: Translatable;
  step?: {
    icon: string;
    title: Translatable;
    current: number | undefined;
    total: number;
  };
  hideCloseButton?: boolean;
}>();

defineEmits<{
  (e: 'closeClicked'): void;
}>();
</script>

<style scoped lang="scss">
.modal-header {
  display: flex;
  flex-direction: column;
  text-wrap: wrap;
  margin-bottom: 1.5rem;
  gap: 1rem;

  @include ms.responsive-breakpoint('sm') {
    padding: 2rem 3rem 1.5rem 2rem;
    border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
  }

  &-title {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;

    &__step {
      color: var(--parsec-color-light-secondary-grey);
    }

    &__text {
      color: var(--parsec-color-light-primary-800);
    }
  }

  &__subtitle {
    color: var(--parsec-color-light-secondary-soft-text);

    @include ms.responsive-breakpoint('sm') {
      display: none;
    }
  }
}

.closeBtn-stepper {
  position: absolute;
  top: 3rem;
  right: 1rem;
  padding: 0.25rem;
}

// only visible with stepper
.modal-header-stepper {
  padding: 0 0 1.5rem;

  &-label {
    background: var(--parsec-color-light-primary-50);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem;
    gap: 0.5rem;

    &__icon {
      color: var(--parsec-color-light-primary-600);
      border-radius: var(--parsec-radius-6);
      font-size: 1rem;
    }

    &__text {
      color: var(--parsec-color-light-primary-600);
    }
  }

  .modal-header-title {
    padding: 0.5rem 2rem 0;

    &:has(.modal-header-title__step) {
      padding: 0 2rem;
    }
  }
}
</style>
