<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ms-modal
    :title="modalConfig.title"
    :close-button="{ visible: false }"
    :cancel-button="modalConfig.cancelButton"
    :confirm-button="modalConfig.confirmButton"
  >
    <div class="editor-error">
      <ms-report-text
        v-if="modalConfig.message"
        :theme="modalConfig.theme"
      >
        {{ $msTranslate(modalConfig.message) }}
      </ms-report-text>
      <div
        v-if="status === EditorIssueStatus.LoadingTimeout"
        class="editor-error-advices"
      >
        <ion-text class="editor-error-advices__title">{{ $msTranslate('fileEditors.advices.title') }}</ion-text>
        <ion-list class="editor-error-advices-list ion-no-padding">
          <ion-item class="editor-error-advices-list__item ion-no-padding">
            <ion-icon
              class="item-icon"
              :icon="checkmarkCircle"
            />
            {{ $msTranslate('fileEditors.advices.advice1') }}
          </ion-item>
          <ion-item class="editor-error-advices-list__item ion-no-padding">
            <ion-icon
              class="item-icon"
              :icon="checkmarkCircle"
            />
            {{ $msTranslate('fileEditors.advices.advice2') }}
          </ion-item>
        </ion-list>
      </div>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import { isWeb } from '@/parsec';
import { EditorButtonAction, EditorErrorMessage, EditorErrorTitle, EditorIssueStatus } from '@/views/files/handler/editor/types';
import { IonIcon, IonItem, IonList, IonText, modalController } from '@ionic/vue';
import { checkmarkCircle } from 'ionicons/icons';
import { MsModal, MsModalResult, MsReportText, MsReportTheme, Translatable } from 'megashark-lib';
import { computed, Ref } from 'vue';

const props = defineProps<{
  status: EditorIssueStatus;
  fileLoaded?: Ref<boolean>;
}>();

interface ModalConfig {
  title: Translatable;
  message?: Translatable;
  theme: MsReportTheme;
  confirmButton: {
    label: Translatable;
    disabled: boolean;
    onClick: () => Promise<boolean>;
  };
  cancelButton?: {
    label: Translatable;
    disabled: boolean;
  };
}

const modalConfig = computed<ModalConfig>(() => {
  let title: Translatable;
  let message: Translatable | undefined;
  let theme: MsReportTheme;
  let confirmButtonLabel: Translatable;
  let cancelButton: { label: Translatable; disabled: boolean } | undefined;

  switch (props.status) {
    case EditorIssueStatus.EditionNotAvailable:
      title = EditorErrorTitle.EditionNotAvailable;
      message = EditorErrorMessage.EditableOnlyOnSystem;
      theme = MsReportTheme.Info;
      confirmButtonLabel = EditorButtonAction.BackToFiles;
      break;
    case EditorIssueStatus.LoadingTimeout:
      title = EditorErrorTitle.TooLongToOpen;
      message = isWeb() ? EditorErrorMessage.TooLongToOpenOnWeb : EditorErrorMessage.TooLongToOpenOnDesktop;
      theme = MsReportTheme.Warning;
      confirmButtonLabel = props.fileLoaded?.value ? EditorButtonAction.Close : EditorButtonAction.Wait;
      if (!props.fileLoaded?.value) {
        cancelButton = {
          label: EditorButtonAction.BackToFiles,
          disabled: false,
        };
      }
      break;
    case EditorIssueStatus.NetworkOffline:
      title = EditorErrorTitle.NetworkOffline;
      message = EditorErrorMessage.NetworkOffline;
      theme = MsReportTheme.Warning;
      confirmButtonLabel = EditorButtonAction.Close;
      break;
    case EditorIssueStatus.UnsupportedFileType:
      title = EditorErrorTitle.UnsupportedFileType;
      message = EditorErrorMessage.EditableOnlyOnSystem;
      theme = MsReportTheme.Info;
      confirmButtonLabel = EditorButtonAction.BackToFiles;
      break;
    default:
      title = EditorErrorTitle.GenericError;
      theme = MsReportTheme.Warning;
      confirmButtonLabel = EditorButtonAction.BackToFiles;
      break;
  }

  return {
    title,
    message,
    theme,
    confirmButton: {
      label: confirmButtonLabel,
      disabled: false,
      onClick: async (): Promise<boolean> => {
        return modalController.dismiss(true, MsModalResult.Confirm);
      },
    },
    cancelButton,
  };
});
</script>

<style scoped lang="scss">
.editor-error {
  margin-top: 0.5rem;
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');

  &-advices {
    border-top: ms.border('thin') solid ms.color('border-base-default');
    padding-top: ms.spacing('padding-4xl');
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-3xl');

    &__title {
      @include ms.font('heading-h5');
      color: ms.color('text-base-heading');
    }

    &-list {
      padding-left: ms.spacing('padding-lg');
      list-style-type: circle;
      background: none;

      &__item {
        margin-bottom: 0.5rem;
        display: flex;
        align-items: center;
        gap: ms.spacing('gap-lg');
        color: ms.color('text-base-body');
        --background: none;
        @include ms.font('body-md-regular');

        .item-icon {
          color: ms.color('icon-neutral-default');
          flex-shrink: 0;
          margin-right: 0.5rem;
          font-size: 1rem;
        }
      }
    }
  }
}
</style>
