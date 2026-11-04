<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="missing-ownership-label">
    <ion-icon
      :icon="warning"
      class="missing-ownership-label__icon"
    />
    <ion-text
      class="missing-ownership-label__text"
      v-if="showText"
    >
      {{ $msTranslate('WorkspacesPage.missingOwnership.label') }}
    </ion-text>
  </div>
</template>

<script setup lang="ts">
import { IonIcon, IonText } from '@ionic/vue';
import { warning } from 'ionicons/icons';

defineProps<{
  showText?: boolean;
}>();
</script>

<style scoped lang="scss">
.missing-ownership-label {
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-md');
  padding: ms.spacing('padding-md');
  border-radius: ms.radius('2xl');
  overflow: hidden;
  background: ms.color('surface-error-default-subtle');
  border: ms.border('thin') solid ms.color('border-error-default-subtle');
  transition: all 0.1s ease-in-out;
  flex-shrink: 0;

  &__icon {
    font-size: 1rem;
    color: ms.color('icon-error-default');
    flex-shrink: 0;
  }

  &__text {
    text-wrap: nowrap;
    text-align: left;
    @include ms.font('label-md-medium');
    color: ms.color('text-error-default');
    text-shadow: var(--parsec-shadow-light);
  }

  &:hover {
    background: ms.color('surface-error-default-subtle-hover');

    .missing-ownership-label__icon {
      color: ms.color('icon-error-default-hover');
    }

    .missing-ownership-label__text {
      color: ms.color('text-error-default-hover');
    }
  }
}
</style>
