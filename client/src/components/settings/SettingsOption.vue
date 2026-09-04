<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="settings-option">
    <div class="settings-option__content">
      <ion-text class="title">
        {{ $msTranslate(title) }}
      </ion-text>
      <ion-text class="description">
        {{ $msTranslate(description) }}
      </ion-text>
    </div>
    <slot />
  </div>
</template>

<script setup lang="ts">
import { IonText } from '@ionic/vue';
import { Translatable } from 'megashark-lib';

defineProps<{
  title: Translatable;
  description?: Translatable;
}>();
</script>

<style scoped lang="scss">
.settings-option {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: ms.spacing('padding-2xl') ms.spacing('padding-none');
  gap: ms.spacing('gap-4xl');

  &__content {
    display: flex;
    flex-direction: column;
    flex-grow: 1;
    gap: ms.spacing('gap-sm');

    @include ms.responsive-breakpoint('xs') {
      width: 100%;
    }

    .title {
      @include ms.font('label-lg-medium');
      color: ms.color('text-base-label');
    }

    .description {
      @include ms.font('body-sm-regular');
      color: ms.color('text-base-description');
    }
  }
}
</style>
