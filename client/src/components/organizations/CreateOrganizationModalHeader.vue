<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
      <ion-text class="small-display-stepper__text">
        {{ $msTranslate('HomePage.noExistingOrganization.createOrganization') }}
      </ion-text>
    </div>
    <div class="modal-header-title">
      <ion-icon
        v-if="icon"
        :icon="icon"
        class="modal-header-title__icon"
      />
      <ion-text class="modal-header-title__text">
        {{ $msTranslate(title) }}
      </ion-text>
    </div>
    <ion-text
      class="modal-header__text"
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
  margin-bottom: 1rem;
  gap: ms.spacing('gap-xl');

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-4xl');
    border-bottom: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  }

  &-title {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-3xl');

    &__icon {
      color: ms.color('icon-brand-default');
      background: ms.color('surface-base-default');
      border-radius: ms.radius('md');
      font-size: 1.5rem;
      padding: ms.spacing('padding-sm');
    }

    &__text {
      @include ms.font('heading-h4');
      color: ms.color('text-brand-default');

      @include ms.responsive-breakpoint('sm') {
        margin-right: 2rem;
      }
    }
  }

  &__text {
    @include ms.font('body-lg-regular');
    color: ms.color('text-neutral-default');

    @include ms.responsive-breakpoint('sm') {
      display: none;
    }
  }
}

.closeBtn--small-display {
  position: absolute;
  top: 2.75rem;
  right: 1rem;
  padding: ms.spacing('padding-sm');
}

.modal-header--small-display {
  padding: ms.spacing('padding-none') ms.spacing('padding-none') ms.spacing('padding-lg');

  .small-display-stepper {
    background: ms.color('surface-brand-default-subtle-hover');
    display: flex;
    align-items: center;
    justify-content: center;
    padding: ms.spacing('padding-lg');
    gap: ms.spacing('gap-lg');

    &__icon {
      color: ms.color('icon-brand-default');
      border-radius: ms.radius('md');
      font-size: 1rem;
    }

    &__text {
      @include ms.font('label-md-medium');
      color: ms.color('text-brand-default');
    }
  }

  .modal-header-title {
    padding: ms.spacing('padding-lg') ms.spacing('padding-5xl') ms.spacing('padding-3xl');
  }
}
</style>
