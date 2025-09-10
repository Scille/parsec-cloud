<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    v-if="!hideCloseButton"
    slot="icon-only"
    class="closeBtn"
    :class="{
      'closeBtn--small-display': smallDisplayStepper && isSmallDisplay,
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
      'modal-header--small-display': smallDisplayStepper && isSmallDisplay,
    }"
  >
    <div
      class="small-display-stepper"
      v-if="smallDisplayStepper && isSmallDisplay"
    >
      <ion-icon
        :icon="shapes"
        class="small-display-stepper__icon"
      />
      <ion-text class="small-display-stepper__text button-medium">
        {{ $msTranslate('HomePage.noExistingOrganization.createOrganization') }}
      </ion-text>
    </div>
    <div class="modal-header-title">
      <ion-icon
        v-if="icon"
        :icon="icon"
        class="modal-header-title__icon"
      />
      <ion-text class="modal-header-title__text title-h3">
        {{ $msTranslate(title) }}
      </ion-text>
    </div>
    <ion-text
      class="modal-header__text body-lg"
      v-if="subtitle"
    >
      {{ $msTranslate(subtitle) }}
    </ion-text>
  </ion-header>
</template>

<script setup lang="ts">
import { IonButton, IonHeader, IonIcon, IonText } from '@ionic/vue';
import { close, shapes } from 'ionicons/icons';
import { Translatable, useWindowSize } from 'megashark-lib';

const { isSmallDisplay } = useWindowSize();

defineProps<{
  title: Translatable;
  subtitle?: Translatable;
  icon?: string;
  hideCloseButton?: boolean;
  smallDisplayStepper?: boolean;
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
    padding: 1.5rem;
    border-bottom: 1px solid var(--parsec-color-light-secondary-medium);
  }

  &-title {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    &__icon {
      color: var(--parsec-color-light-primary-600);
      background: var(--parsec-color-light-secondary-white);
      border-radius: var(--parsec-radius-6);
      font-size: 1.5rem;
      padding: 0.25rem;
    }

    &__text {
      color: var(--parsec-color-light-primary-800);

      @include ms.responsive-breakpoint('sm') {
        margin-right: 2rem;
      }
    }
  }

  &__text {
    color: var(--parsec-color-light-secondary-soft-text);

    @include ms.responsive-breakpoint('sm') {
      display: none;
    }
  }
}

.closeBtn--small-display {
  position: absolute;
  top: 2.75rem;
  right: 1rem;
  padding: 0.25rem;
}

.modal-header--small-display {
  padding: 0 0 0.5rem;

  .small-display-stepper {
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
    padding: 0.5rem 2rem 1rem;
  }
}
</style>
