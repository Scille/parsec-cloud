<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
        <ion-text class="switch-item__label">{{ $msTranslate('HeaderPage.titles.workspaces') }}</ion-text>

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
        <ion-text class="switch-item__label">{{ $msTranslate('HeaderPage.titles.archived') }}</ion-text>

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
        <ion-text class="switch-item__label">{{ $msTranslate('HeaderPage.titles.trashed') }}</ion-text>

        <ion-icon
          v-if="selectedRoute === Routes.Trash"
          :icon="checkmarkCircle"
          class="switch-item__check-icon"
        />
      </ion-radio>
    </ion-radio-group>

    <ion-button
      class="workspace-switch-button"
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
  gap: ms.spacing('gap-5xl');
  align-items: center;
  justify-content: center;
  padding: ms.spacing('padding-5xl') ms.spacing('padding-3xl');
}

.workspace-switch-content {
  display: flex;
  flex-direction: column;
  width: 100%;
  gap: ms.spacing('gap-3xl');

  .switch-item {
    gap: ms.spacing('gap-lg');
    color: ms.color('text-neutral-default');
    padding: ms.spacing('padding-3xl');
    border: ms.border('thin') solid ms.color('border-base-default');
    border-radius: ms.radius('2xl');
    width: 100%;

    &::part(label) {
      padding: ms.spacing('padding-none');
      display: flex;
      align-items: center;
      gap: ms.spacing('gap-lg');
      margin: 0;
      width: 100%;
    }

    &::part(container) {
      display: none;
    }

    &__label {
      @include ms.font('body-lg-medium');
    }

    &__icon {
      font-size: 1.25rem;
      color: ms.color('icon-neutral-default');
    }

    &__check-icon {
      font-size: 1.25rem;
      margin-left: auto;
      color: ms.color('icon-brand-default');
    }

    &.radio-checked {
      border-color: ms.color('border-brand-default');
      background: ms.color('surface-brand-default-subtle');
      color: ms.color('text-brand-default');

      .switch-item__icon {
        color: ms.color('icon-brand-default');
      }
    }
  }
}

.workspace-switch-button {
  @include ms.font('label-md-medium');
  width: 100%;
}
</style>
