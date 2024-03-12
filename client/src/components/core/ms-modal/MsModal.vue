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
                :color="themeToButtonColor(cancelButton.theme)"
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
                :color="themeToButtonColor(confirmButton.theme)"
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

<script setup lang="ts">
import { MsModalConfig, MsModalResult } from '@/components/core/ms-modal/types';
import { MsReportText } from '@/components/core/ms-text';
import { MsReportTheme } from '@/components/core/ms-types';
import { IonButton, IonButtons, IonFooter, IonHeader, IonIcon, IonPage, IonText, IonTitle, IonToolbar, modalController } from '@ionic/vue';
import { close } from 'ionicons/icons';
import { Ref, onMounted, ref } from 'vue';

const modal: Ref<HTMLDivElement | null> = ref(null);
defineProps<MsModalConfig>();
defineEmits<{
  (e: 'onEnterKeyup'): void;
}>();

onMounted(() => {
  // we should use onMounted lonely, but for weird reasons onMounted is triggered before the focus is working
  setTimeout(() => {
    modal.value?.focus();
  }, 100);
});

function themeToButtonColor(_theme?: MsReportTheme): string | undefined {
  // Buttons do not correctly handle colors right now.
  // Once they do, remove the early return and uncomment the switch/case.

  return undefined;

  // switch (theme) {
  //   case MsReportTheme.Success:
  //     return 'success';
  //   case MsReportTheme.Warning:
  //     return 'warning';
  //   case MsReportTheme.Error:
  //     return 'error';
  //   case MsReportTheme.Info:
  //   default:
  //     return undefined;
  // }
}

async function cancel(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Cancel);
}

async function confirm(): Promise<boolean> {
  return modalController.dismiss(null, MsModalResult.Confirm);
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
  overflow: visible;

  > :first-child {
    padding-top: 2rem;
  }
}

.ms-modal-footer {
  position: relative;
  z-index: 1;

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
