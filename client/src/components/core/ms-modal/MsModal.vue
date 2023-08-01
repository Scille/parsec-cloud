<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-buttons
      slot="end"
      class="closeBtn-container"
      v-if="closeButtonEnabled"
    >
      <ion-button
        slot="icon-only"
        @click="cancel()"
        class="closeBtn"
      >
        <ion-icon
          :icon="close"
          size="large"
          class="closeBtn__icon"
        />
      </ion-button>
    </ion-buttons>
    <div class="ms-modal">
      <ion-header class="ms-modal-header">
        <ion-title class="ms-modal-header__title title-h2">
          {{ title }}
        </ion-title>
        <ion-text
          class="ms-modal-header__text body"
          v-if="subtitle"
        >
          {{ subtitle }}
        </ion-text>
      </ion-header>
      <div
        class="ms-modal-content inner-content"
      >
        <slot />
      </div>
      <ion-footer class="ms-modal-footer">
        <ion-toolbar>
          <ion-buttons
            slot="primary"
            class="ms-modal-footer-buttons"
          >
            <ion-button
              v-if="cancelButton"
              fill="clear"
              size="default"
              id="cancel-button"
              @click="cancelButton.onClick ? cancelButton.onClick() : cancel()"
              :disabled="cancelButton.disabled"
            >
              {{ cancelButton.label }}
            </ion-button>
            <ion-button
              v-if="confirmButton"
              fill="solid"
              size="default"
              id="next-button"
              type="submit"
              @click="confirmButton.onClick ? confirmButton.onClick() : confirm()"
              :disabled="confirmButton.disabled"
            >
              {{ confirmButton.label }}
            </ion-button>
          </ion-buttons>
        </ion-toolbar>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonText,
  IonPage,
  IonHeader,
  IonTitle,
  IonToolbar,
  IonButtons,
  IonButton,
  modalController,
  IonFooter,
  IonIcon,
} from '@ionic/vue';
import { close } from 'ionicons/icons';
import { ModalResultCode } from '@/common/constants';

// We are forced to re-define interface here since we are not in Vue 3.3 see: https://github.com/vuejs/core/issues/4294
export interface MsModalConfig {
  title: string,
  subtitle?: string,
  closeButtonEnabled?: boolean,
  cancelButton?: {
    disabled: boolean,
    label: string,
    // eslint-disable-next-line @typescript-eslint/ban-types
    onClick?: Function,
  },
  confirmButton?: {
    disabled: boolean,
    label: string,
    // eslint-disable-next-line @typescript-eslint/ban-types
    onClick?: Function,
  }
}

defineProps<MsModalConfig>();

function cancel(): Promise<boolean> {
  return modalController.dismiss(null, ModalResultCode.Cancel);
}
function confirm(): Promise<boolean> {
  return modalController.dismiss(null, ModalResultCode.Confirm);
}
</script>

<style lang="scss" scoped>
.ms-modal {
  padding: 3.5rem;
  justify-content: start;
}

.closeBtn-container {
    position: absolute;
    top: 2rem;
    right: 2rem;
  }

.closeBtn-container, .closeBtn {
  margin: 0;
  --padding-start: 0;
  --padding-end: 0;
}

.closeBtn {
  width: fit-content;
  height: fit-content;
  --border-radius: var(--parsec-radius-4);
  --background-hover: var(--parsec-color-light-primary-50);
  border-radius: var(--parsec-radius-4);

  &__icon {
    padding: 4px;
    color: var(--parsec-color-light-primary-500);

    &:hover {
      --background-hover: var(--parsec-color-light-primary-50);
    }
  }

  &:active {
    border-radius: var(--parsec-radius-4);
    background: var(--parsec-color-light-primary-100);
  }
}

.ms-modal-header {
  margin-bottom: 2rem;

  &__title {
    padding: 0;
    margin-bottom: 1.5rem;
    color: var(--parsec-color-light-primary-600);
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  &__text {
    color: var(--parsec-color-light-secondary-grey);
  }
}

.ms-modal-content {
  --background: transparent;
}

.ms-modal-footer {
  margin-top: 2.5rem;

  &::before {
    background: transparent;
  }

  &-buttons {
    display: flex;
    justify-content: end;
    gap: 1rem;
  }
}
</style>
