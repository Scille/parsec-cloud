<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="element-container">
    <ion-item class="element ion-no-padding">
      <div class="element-type">
        <div class="element-type__icon">
          <ms-image
            :image="ImportMultipleFiles"
            class="file-icon"
          />
        </div>
      </div>
      <div class="element-details">
        <ion-text class="element-details__name body">
          {{
            $msTranslate({
              key: isError ? 'FoldersPage.itemsErrorAmount' : 'FoldersPage.itemsDoneAmount',
              data: { amount: amount },
              count: amount,
            })
          }}
        </ion-text>
      </div>
    </ion-item>
  </div>
</template>

<script setup lang="ts">
import { IonItem, IonText } from '@ionic/vue';
import { ImportMultipleFiles, MsImage } from 'megashark-lib';

const props = defineProps<{
  amount: number;
  isError?: boolean;
}>();

defineExpose({
  props,
});

defineEmits<{
  (event: 'cancel'): void;
}>();
</script>

<style scoped lang="scss">
.element-container {
  border-top: 1px solid var(--parsec-color-light-secondary-premiere);
  border-bottom: 1px solid var(--parsec-color-light-secondary-premiere);
  transition: background 0.2s ease-in-out;
  position: relative;
  overflow: hidden;
  flex-shrink: 0;
}

.element {
  --background: none;
  padding: 0.75rem 1rem 1rem;
  --inner-padding-end: 0;

  .file-icon {
    width: 2rem;
    height: 2rem;
  }

  &-details {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    margin-left: 0.875rem;

    &__name {
      color: var(--parsec-color-light-primary-700);
      margin-right: auto;
    }
  }
}
</style>
