<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page
    class="modal"
    :class="theme"
  >
    <div
      @keyup.enter="$emit('onEnterKeyup')"
      tabindex="0"
      ref="modal"
    >
      <ion-buttons
        slot="end"
        class="closeBtn-container"
        v-if="closeButton"
      >
        <ion-button
          v-show="closeButton.visible"
          slot="icon-only"
          @click="closeButton && closeButton.onClick ? closeButton.onClick() : cancel()"
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
          <div
            class="ms-modal-header__title-container"
            v-if="title"
          >
            <ion-icon
              v-if="theme || titleIcon"
              :icon="getTitleIcon()"
              size="large"
              class="ms-modal-header__title-icon"
            />
            <ion-title class="ms-modal-header__title title-h2">
              {{ title }}
            </ion-title>
          </div>
          <template v-if="subtitle">
            <ms-report-text
              v-if="theme"
              :theme="theme"
            >
              {{ subtitle }}
            </ms-report-text>
            <ion-text
              class="ms-modal-header__text body"
              v-else
            >
              {{ subtitle }}
            </ion-text>
          </template>
        </ion-header>
        <div class="ms-modal-content inner-content">
          <slot />
        </div>
        <ion-footer
          class="ms-modal-footer"
          v-if="cancelButton || confirmButton"
        >
          <ion-toolbar class="ms-modal-toolbar">
            <ion-buttons
              slot="primary"
              class="ms-modal-footer-buttons"
            >
              <ion-button
                v-if="cancelButton"
                fill="clear"
                size="default"
                id="cancel-button"
                @click="cancelButton && cancelButton.onClick ? cancelButton.onClick() : cancel()"
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
                @click="confirmButton && confirmButton.onClick ? confirmButton.onClick() : confirm()"
                :disabled="confirmButton.disabled"
              >
                {{ confirmButton.label }}
              </ion-button>
            </ion-buttons>
          </ion-toolbar>
        </ion-footer>
      </div>
    </div>
  </ion-page>
</template>

<script lang="ts">
export interface MsModalConfig {
  title?: string;
  titleIcon?: string;
  theme?: MsReportTheme;
  subtitle?: string;
  closeButton?: {
    visible: boolean;
    onClick?: () => Promise<boolean>;
  };
  cancelButton?: {
    disabled: boolean;
    label: string;
    onClick?: () => Promise<boolean>;
  };
  confirmButton?: {
    disabled: boolean;
    label: string;
    onClick?: () => Promise<boolean>;
  };
}
</script>

<script setup lang="ts">
import { IonText, IonPage, IonHeader, IonTitle, IonToolbar, IonButtons, IonButton, modalController, IonFooter, IonIcon } from '@ionic/vue';
import { close, informationCircle, checkmarkCircle, warning, closeCircle } from 'ionicons/icons';
import { onMounted, ref, Ref } from 'vue';
import { MsReportTheme, MsModalResult } from '@/components/core/ms-types';
import MsReportText from '@/components/core/ms-text/MsReportText.vue';

const modal: Ref<HTMLDivElement | null> = ref(null);
const props = defineProps<MsModalConfig>();
defineEmits<{
  (e: 'onEnterKeyup'): void;
}>();

onMounted(() => {
  // we should use onMounted lonely, but for weird reasons onMounted is triggered before the focus is working
  setTimeout(() => {
    modal.value?.focus();
  }, 100);
});

async function cancel(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Cancel);
}

async function confirm(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Confirm);
}

function getTitleIcon(): string {
  if (props.theme) {
    switch (props.theme) {
      case MsReportTheme.Info:
        return informationCircle;
      case MsReportTheme.Success:
        return checkmarkCircle;
      case MsReportTheme.Warning:
        return warning;
      case MsReportTheme.Error:
        return closeCircle;
    }
  }
  return props.titleIcon ?? '';
}
</script>

<style lang="scss" scoped>
.ms-modal {
  padding: 2.5rem;
  justify-content: start;
}

.ms-modal-header {
  display: flex;
  flex-direction: column;
  gap: 1rem;

  &__title {
    padding: 0;
    color: var(--parsec-color-light-primary-800);
    display: flex;
    align-items: center;
    max-width: 22rem;

    .toolbar-title {
      text-overflow: clip !important;
    }

    &-container {
      display: flex;
      align-items: center;
    }

    &-icon {
      color: var(--parsec-color-light-primary-600);
      margin-right: 4px;
    }
  }

  &__text {
    color: var(--parsec-color-light-secondary-grey);
  }
}

.ms-modal-content {
  --background: transparent;
  overflow: auto;

  > :first-child {
    margin-top: 2rem;
  }
}

.ms-modal-footer {
  > :first-child:not([hidden]) {
    margin-top: 2.5rem;
  }

  &::before {
    background: transparent;
  }

  .ms-modal-toolbar {
    --padding: 0;
    --min-height: 0;
    --margin-inline: 0;
  }

  &-buttons {
    display: flex;
    justify-content: end;
    gap: 1rem;
    margin: 0;
  }
}

.ms-info {
  --ms-modal-title-text-color: var(--parsec-color-light-primary-600);
  --ms-modal-next-button-background-color: var(--parsec-color-light-primary-500);
  --ms-modal-next-button-background-hover-color: var(--parsec-color-light-primary-700);
}

.ms-success {
  --ms-modal-title-text-color: var(--parsec-color-light-success-500);
  --ms-modal-next-button-background-color: var(--parsec-color-light-success-500);
  --ms-modal-next-button-background-hover-color: var(--parsec-color-light-success-700);
}

.ms-warning {
  --ms-modal-title-text-color: var(--parsec-color-light-warning-500);
  --ms-modal-next-button-background-color: var(--parsec-color-light-warning-500);
  --ms-modal-next-button-background-hover-color: var(--parsec-color-light-warning-700);
}

.ms-error {
  --ms-modal-title-text-color: var(--parsec-color-light-danger-500);
  --ms-modal-next-button-background-color: var(--parsec-color-light-danger-500);
  --ms-modal-next-button-background-hover-color: var(--parsec-color-light-danger-700);
}

.ms-info,
.ms-success,
.ms-warning,
.ms-error {
  .ms-modal-header {
    &__title {
      color: var(--ms-modal-title-text-color);

      &-icon {
        color: var(--ms-modal-title-text-color);
      }
    }
  }

  .ms-modal-footer {
    margin-top: 0;

    &-buttons #next-button {
      --background: var(--ms-modal-next-button-background-color);
      --background-hover: var(--ms-modal-next-button-background-hover-color);
    }
  }
}
</style>
