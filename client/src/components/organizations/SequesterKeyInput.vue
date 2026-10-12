<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="sequester-container">
    <ion-toggle
      @ion-change="onAddKeyToggled"
      class="sequester-toggle"
    >
      <ion-text class="sequester-toggle__title">
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
        <ion-text class="upload-key-update__name">{{ fileName }}</ion-text>
        <ion-button
          size="small"
          class="upload-key-update__button"
          @click="onUploadKeyClicked"
        >
          {{ $msTranslate('CreateOrganization.sequester.update') }}
        </ion-button>
      </div>
    </div>
    <div class="sequester-info">
      <ion-text class="sequester-info__text">
        {{ $msTranslate('CreateOrganization.sequester.info') }}
      </ion-text>
      <ion-text
        button
        class="sequester-info__button"
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
  margin-top: 1.5rem;
  padding-left: ms.spacing('padding-2xl');
  border-left: ms.border('thick') solid ms.color('border-base-default');
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-2xl');

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-3xl');
    margin-top: 1rem;
  }
}

.sequester-toggle__title {
  @include ms.font('body-md-medium');
  color: ms.color('text-base-body');
  font-size: 0.9375rem;
}

.sequester-info {
  color: ms.color('text-base-description');
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-2xl');

  @include ms.responsive-breakpoint('sm') {
    flex-direction: column;
    align-items: start;
  }

  &__text {
    @include ms.font('body-md-regular');
  }

  &__button {
    @include ms.font('label-md-medium');
    height: fit-content;
    color: ms.color('text-brand-default');
    flex-shrink: 0;
    cursor: pointer;
    position: relative;

    &::after {
      content: '';
      position: absolute;
      bottom: -2px;
      left: 0;
      height: 1px;
      background: ms.color('surface-brand-default');
      transition: width 0.2s;
      width: 0;
    }

    &:hover {
      color: ms.color('text-brand-default-hover');

      &::after {
        color: ms.color('text-brand-default-hover');
        width: 100%;
      }
    }
  }
}

.upload-key {
  &__button {
    display: flex;
    align-items: center;
    color: ms.color('text-base-body');
    border-radius: ms.radius('lg');
    overflow: auto;

    &::part(native) {
      border: ms.border('thin') dashed ms.color('border-base-default');
      // --background: #{ms.color('surface-base-default')};
      --background: #{ms.color('surface-neutral-default-subtle')};
      --background-hover: #{ms.color('surface-neutral-default-subtle-hover')};
      --box-shadow: var(--parsec-shadow-2);
    }

    &-icon {
      margin-right: 0.5rem;
    }
  }

  &-update {
    background: ms.color('surface-neutral-default-subtle-pressed');
    color: ms.color('text-base-body');
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
    padding: ms.spacing('padding-sm') ms.spacing('padding-2xl');
    border-radius: ms.radius('lg');

    &__icon {
      color: ms.color('icon-neutral-default');
      font-size: 1rem;
    }

    &__name {
      @include ms.font('label-md-medium');
    }

    &__button {
      margin-left: auto;
      color: ms.color('text-base-body');

      &::part(native) {
        --background: none;
        --background-hover: #{ms.color('surface-neutral-default-subtle-hover')};
        --box-shadow: var(--parsec-shadow-2);
        padding-inline: ms.spacing('padding-none');
      }

      &:hover {
        color: ms.color('text-base-heading');
      }
    }
  }
}
</style>
