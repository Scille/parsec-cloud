<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->
<template>
  <div class="workspace-switch-modal-container">
    <ion-radio-group
      v-model="selectedRoute"
      class="workspace-switch-content"
    >
      <ion-radio
        class="switch-item"
        :value="Routes.Workspaces"
        @click.prevent
        :class="{ 'radio-checked': selectedRoute === Routes.Workspaces }"
      >
        <ion-icon
          :icon="business"
          class="switch-item__icon"
        />
        <ion-text class="subtitles-normal">{{ $msTranslate('HeaderPage.titles.workspaces') }}</ion-text>

        <ion-icon
          v-if="selectedRoute === Routes.Workspaces"
          :icon="checkmarkCircle"
          class="switch-item__check-icon"
        />
      </ion-radio>

      <ion-radio
        class="switch-item"
        :value="Routes.Archived"
        @click.prevent
        :class="{ 'radio-checked': selectedRoute === Routes.Archived }"
      >
        <ion-icon
          :icon="archive"
          class="switch-item__icon"
        />
        <ion-text class="subtitles-normal">{{ $msTranslate('HeaderPage.titles.archived') }}</ion-text>

        <ion-icon
          v-if="selectedRoute === Routes.Archived"
          :icon="checkmarkCircle"
          class="switch-item__check-icon"
        />
      </ion-radio>

      <ion-radio
        class="switch-item"
        :value="Routes.Trash"
        @click.prevent
        :class="{ 'radio-checked': selectedRoute === Routes.Trash }"
      >
        <ion-icon
          :icon="trash"
          class="switch-item__icon"
        />
        <ion-text class="subtitles-normal">{{ $msTranslate('HeaderPage.titles.trashed') }}</ion-text>

        <ion-icon
          v-if="selectedRoute === Routes.Trash"
          :icon="checkmarkCircle"
          class="switch-item__check-icon"
        />
      </ion-radio>
    </ion-radio-group>

    <ion-button
      class="workspace-switch-button button-medium button-default"
      size="default"
      fill="solid"
      @click="validate"
    >
      {{ $msTranslate('HeaderPage.workspaceSwitchModal.actions.confirm') }}
    </ion-button>
  </div>
</template>

<script setup lang="ts">
import { Routes } from '@/router';
import { IonButton, IonIcon, IonRadio, IonRadioGroup, IonText, modalController } from '@ionic/vue';
import { archive, business, checkmarkCircle, trash } from 'ionicons/icons';
import { MsModalResult } from 'megashark-lib';
import { ref } from 'vue';

const props = defineProps<{
  currentRoute: Routes.Workspaces | Routes.Archived | Routes.Trash | undefined;
}>();

const selectedRoute = ref<Routes.Workspaces | Routes.Archived | Routes.Trash | undefined>(props.currentRoute);

async function validate(): Promise<void> {
  await modalController.dismiss(selectedRoute.value, MsModalResult.Confirm);
}
</script>

<style scoped lang="scss">
.workspace-switch-modal-container {
  display: flex;
  flex-direction: column;
  gap: 2rem;
  align-items: center;
  justify-content: center;
  padding: 2rem 1rem;
}

.workspace-switch-content {
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: 1rem;

  .switch-item {
    gap: 0.5rem;
    font-size: var(--parsec-font-size-md);
    color: var(--parsec-color-light-secondary-soft-text);
    padding: 1rem;
    border: 1px solid var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-12);
    width: 100%;

    &::part(label) {
      padding: 0;
      display: flex;
      align-items: center;
      gap: 0.5rem;
      margin: 0;
      width: 100%;
    }

    &::part(container) {
      display: none;
    }

    &__icon {
      font-size: 1.25rem;
      color: var(--parsec-color-light-secondary-soft-text);
    }

    &__check-icon {
      font-size: 1.25rem;
      margin-left: auto;
      color: var(--parsec-color-light-primary-600);
    }

    &.radio-checked {
      border-color: var(--parsec-color-light-primary-600);
      background: var(--parsec-color-light-primary-50);
      color: var(--parsec-color-light-primary-600);

      .switch-item__icon {
        color: var(--parsec-color-light-primary-600);
      }
    }
  }
}

.workspace-switch-button {
  width: 100%;
}
</style>
