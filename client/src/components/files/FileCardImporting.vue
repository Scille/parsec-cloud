<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item class="file-card-item ion-no-padding">
    <div class="card-content">
      <vue3-lottie
        class="card-content__spinner"
        :animation-data="SpinnerJSON"
        :height="24"
        :width="24"
        :loop="true"
      />
      <ion-avatar class="card-content-icons">
        <ion-icon
          class="icon-item"
          :icon="document"
        />
      </ion-avatar>

      <ion-title class="card-content__title body">
        {{ data.file.name }}
      </ion-title>

      <ion-text class="card-content-last-update caption-caption">
        <span>{{ $t('FoldersPage.File.importing') }}</span>
      </ion-text>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import SpinnerJSON from '@/assets/spinner.json';
import { ImportData } from '@/services/importManager';
import { IonAvatar, IonIcon, IonItem, IonText, IonTitle } from '@ionic/vue';
import { document } from 'ionicons/icons';
import { Vue3Lottie } from 'vue3-lottie';

defineProps<{
  data: ImportData;
  progress: number;
}>();
</script>

<style lang="scss" scoped>
.file-card-item {
  position: relative;
  cursor: pointer;
  text-align: center;
  --background: var(--parsec-color-light-secondary-background);
  background: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-medium);
  user-select: none;
  border-radius: var(--parsec-radius-12);
  width: 10.5rem;

  &::part(native) {
    --inner-padding-end: 0px;
  }

  &:hover {
    --background: var(--parsec-color-light-primary-30);
    --background-hover: var(--parsec-color-light-primary-30);
    --background-hover-opacity: 1;
  }

  &.selected {
    --background: var(--parsec-color-light-primary-100);
    --background-hover: var(--parsec-color-light-primary-100);
    border: 1px solid var(--parsec-color-light-primary-100);
  }
}

.card-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 1.5rem 0.5rem;
  width: 100%;
  margin: auto;

  &__spinner {
    position: absolute;
    top: 0.75rem;
    left: 0.75rem;
  }

  &-icons {
    position: relative;
    color: var(--parsec-color-light-primary-600);
    height: fit-content;
    width: fit-content;
    margin: 0 auto 0.875rem;

    .icon-item {
      font-size: 3rem;
    }
  }

  &__title {
    color: var(--parsec-color-light-primary-900);
    text-align: center;
    padding: 0 0 0.25rem;
    text-overflow: ellipsis;
    white-space: nowrap;
    width: inherit;

    ion-text {
      width: 100%;
      overflow: hidden;
    }
  }
}

.card-content-last-update {
  color: var(--parsec-color-light-secondary-grey);
  text-align: center;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}

/* No idea how to change the color of the ion-item */
.card-content__title::part(native),
.card-content-last-update::part(native) {
  background-color: var(--parsec-color-light-secondary-background);
}
</style>
