<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    v-if="!hideCloseButton"
    slot="icon-only"
    class="closeBtn closeBtn-stepper"
    @click="$emit('closeClicked')"
  >
    <ion-icon
      :icon="close"
      class="closeBtn__icon"
    />
  </ion-button>
  <ion-header class="modal-header">
    <div class="modal-header-stepper">
      <ion-icon
        :icon="icon"
        class="modal-header-stepper__icon"
      />
      <ion-text class="modal-header-stepper__text button-medium">
        {{ $msTranslate(title) }}
      </ion-text>
    </div>
    <div class="modal-header-content">
      <ion-text class="modal-header__step body">
        {{
          $msTranslate({
            key: 'HeaderPage.modalHeader.step',
            data: {
              current: currentStep + 1,
              total: steps.length,
            },
          })
        }}
      </ion-text>
      <ion-text class="modal-header__title title-h3">
        {{ $msTranslate(steps[currentStep].title) }}
      </ion-text>
    </div>
    <ion-text
      class="modal-header__subtitle body-lg"
      v-if="steps[currentStep]?.subtitle"
    >
      {{ $msTranslate(steps[currentStep]?.subtitle) }}
    </ion-text>
  </ion-header>
</template>

<script setup lang="ts">
import { IonButton, IonHeader, IonIcon, IonText } from '@ionic/vue';
import { close } from 'ionicons/icons';
import { Translatable } from 'megashark-lib';

defineProps<{
  title: Translatable;
  icon: string;
  steps: Array<{
    title: Translatable;
    subtitle?: Translatable;
  }>;
  currentStep: number;
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
  padding: 0 0 1.5rem;

  @include ms.responsive-breakpoint('sm') {
    border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
  }

  &-stepper {
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

  &-content {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    padding: 0.5rem 2rem 0;

    .modal-header__step {
      color: var(--parsec-color-light-secondary-grey);
    }

    .modal-header__title {
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
  top: 2.75rem;
  right: 1rem;
  padding: 0.25rem;
}
</style>
