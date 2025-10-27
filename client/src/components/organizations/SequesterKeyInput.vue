<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="sequester-container">
    <ion-toggle
      @ion-change="onAddKeyToggled"
      class="sequester-toggle"
    >
      <ion-text class="sequester-toggle__title subtitles-sm">
        {{ $msTranslate('CreateOrganization.sequester.title') }}
      </ion-text>
    </ion-toggle>

    <input
      type="file"
      ref="input"
      hidden
      @change="onInputChange"
    />

    <div
      class="upload-key"
      v-if="isAddKeyToggled"
    >
      <ion-button
        class="upload-key__button"
        @click="onUploadKeyClicked"
        v-if="!key"
        :disabled="!isAddKeyToggled"
      >
        <ion-icon
          class="upload-key__button-icon"
          :icon="documentOutline"
        />
        {{ $msTranslate('CreateOrganization.sequester.addAuthorityKey') }}
      </ion-button>
      <div
        v-if="key"
        class="upload-key-update"
      >
        <ion-icon
          class="upload-key-update__icon"
          :icon="document"
        />
        <ion-text class="button-medium">{{ fileName }}</ion-text>
        <ion-button
          class="upload-key-update__button"
          @click="onUploadKeyClicked"
        >
          {{ $msTranslate('CreateOrganization.sequester.update') }}
        </ion-button>
      </div>
    </div>
    <div class="sequester-info">
      <ion-text class="sequester-info__text body">
        {{ $msTranslate('CreateOrganization.sequester.info') }}
      </ion-text>
      <ion-text
        button
        class="sequester-info__button button-medium"
        fill="clear"
        @click="Env.Links.openSequesterDocumentationLink"
      >
        {{ $msTranslate('CreateOrganization.sequester.more') }}
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Env } from '@/services/environment';
import { IonButton, IonIcon, IonText, IonToggle, ToggleCustomEvent } from '@ionic/vue';
import { document, documentOutline } from 'ionicons/icons';
import { ref, useTemplateRef } from 'vue';

const isAddKeyToggled = ref(false);
const key = ref<string | undefined>(undefined);
const fileName = ref('');
const inputRef = useTemplateRef<HTMLInputElement>('input');

defineExpose({
  isAddKeyToggled,
  getKey,
});

function getKey(): string | undefined {
  if (isAddKeyToggled.value) {
    return key.value;
  }
  return undefined;
}

async function onAddKeyToggled(event: ToggleCustomEvent): Promise<void> {
  isAddKeyToggled.value = event.detail.checked;
}

async function onInputChange(): Promise<void> {
  if (!inputRef.value) {
    return;
  }
  if (!inputRef.value.files || !inputRef.value.files.length || !inputRef.value.files.item(0)) {
    return;
  }
  const file = inputRef.value.files.item(0) as File;
  fileName.value = file.name;
  key.value = await file.text();
}

async function onUploadKeyClicked(): Promise<void> {
  inputRef.value?.click();
}
</script>

<style scoped lang="scss">
.sequester-container {
  background: var(--parsec-color-light-secondary-background);
  padding: 0.75rem;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  border-radius: var(--parsec-radius-8);

  @include ms.responsive-breakpoint('sm') {
    padding: 1rem;
    margin-top: 1rem;
  }
}

.sequester-toggle__title {
  color: var(--parsec-color-light-secondary-text);
  font-size: 0.9375rem;
}

.sequester-info {
  color: var(--parsec-color-light-secondary-hard-grey);
  display: flex;
  align-items: center;
  gap: 0.75rem;

  @include ms.responsive-breakpoint('sm') {
    flex-direction: column;
    align-items: start;
  }

  &__button {
    height: fit-content;
    color: var(--parsec-color-light-primary-500);
    flex-shrink: 0;
    cursor: pointer;
    position: relative;

    &::after {
      content: '';
      position: absolute;
      bottom: -2px;
      left: 0;
      height: 1px;
      background: var(--parsec-color-light-primary-500);
      transition: width 0.2s;
      width: 0;
    }

    &:hover {
      color: var(--parsec-color-light-primary-600);

      &::after {
        color: var(--parsec-color-light-primary-600);
        width: 100%;
      }
    }
  }
}

.upload-key {
  &__button {
    display: flex;
    align-items: center;
    color: var(--parsec-color-light-secondary-text);
    border: 1px dashed var(--parsec-color-light-secondary-light);
    border-radius: var(--parsec-radius-8);
    overflow: auto;

    &::part(native) {
      --background: var(--parsec-color-light-secondary-white);
      --background-hover: var(--parsec-color-light-secondary-medium);
      --box-shadow: var(--parsec-shadow-2);
    }

    &-icon {
      margin-right: 0.5rem;
    }
  }

  &-update {
    background: var(--parsec-color-light-secondary-medium);
    color: var(--parsec-color-light-secondary-text);
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.25rem 1rem 0.25rem 0.5rem;
    border-radius: var(--parsec-radius-8);

    &__icon {
      color: var(--parsec-color-light-secondary-text);
      font-size: 1rem;
    }

    &__button {
      margin-left: auto;
      color: var(--parsec-color-light-secondary-text);

      &::part(native) {
        --background: none;
        --background-hover: var(--parsec-color-light-secondary-medium);
        --box-shadow: var(--parsec-shadow-2);
        padding-inline: 0;
      }

      &:hover {
        color: var(--parsec-color-light-secondary-contrast);
      }
    }
  }
}
</style>
