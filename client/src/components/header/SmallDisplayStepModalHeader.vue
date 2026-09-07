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
      <ion-text class="modal-header-stepper__text">
        {{ $msTranslate(title) }}
      </ion-text>
    </div>
    <div class="modal-header-content">
      <ion-text class="modal-header__step">
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
      <ion-text class="modal-header__title">
        {{ $msTranslate(steps[currentStep].title) }}
      </ion-text>
    </div>
    <ion-text
      class="modal-header__subtitle"
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
  gap: ms.spacing('gap-3xl');
  padding: 0 0 ms.spacing('padding-4xl');

  @include ms.responsive-breakpoint('sm') {
    border-bottom: ms.border('thin') solid ms.color('border-base-default');
  }

  &-stepper {
    background: ms.color('surface-brand-default-subtle');
    display: flex;
    align-items: center;
    justify-content: center;
    padding: ms.spacing('padding-lg');
    gap: ms.spacing('gap-lg');

    &__icon {
      color: ms.color('icon-brand-default-hover');
      border-radius: ms.radius('md');
      font-size: 1rem;
    }

    &__text {
      @include ms.font('label-md-medium');
      color: ms.color('text-brand-default-hover');
    }
  }

  &-content {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-md');
    padding: ms.spacing('padding-lg') ms.spacing('padding-5xl') 0;

    .modal-header__step {
      @include ms.font('body-md-regular');
      color: ms.color('text-neutral-default');
    }

    .modal-header__title {
      @include ms.font('heading-h4');
      color: ms.color('text-brand-default');
    }
  }

  &__subtitle {
    @include ms.font('body-lg-regular');
    color: ms.color('text-base-label');

    @include ms.responsive-breakpoint('sm') {
      display: none;
    }
  }
}

.closeBtn-stepper {
  position: absolute;
  top: 2.75rem;
  right: 1rem;
  padding: ms.spacing('padding-sm');
}
</style>
