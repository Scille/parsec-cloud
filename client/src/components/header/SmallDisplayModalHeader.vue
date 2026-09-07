<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    v-if="!hideCloseButton"
    slot="icon-only"
    class="closeBtn"
    @click="$emit('closeClicked')"
  >
    <ion-icon
      :icon="close"
      class="closeBtn__icon"
    />
  </ion-button>
  <ion-header class="modal-header">
    <ion-text class="modal-header__title">
      {{ $msTranslate(title) }}
    </ion-text>
    <ion-text
      class="modal-header__subtitle"
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

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-4xl');
    border-bottom: ms.border('thin') solid ms.color('border-base-default');
  }

  &__title {
    @include ms.font('heading-h4');
    color: ms.color('text-brand-default');
    margin-bottom: 0 !important;
  }

  &__subtitle {
    @include ms.font('body-lg-regular');
    color: ms.color('text-base-label');
  }
}
</style>
