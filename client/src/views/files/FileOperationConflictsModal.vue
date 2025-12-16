<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ms-modal
    title="FoldersPage.ConflictsFile.title"
    subtitle="FoldersPage.ConflictsFile.description"
    :close-button="{ visible: true }"
    class="conflict-modal-page"
  >
    <!-- modal content -->
    <div class="modal">
      <div class="modal-content inner-content">
        <div class="files-concerned">
          <ion-text class="files-concerned__title button-small">
            {{ $msTranslate({ key: 'FoldersPage.ConflictsFile.fileConcerned', data: { count: files.length }, count: files.length }) }}
          </ion-text>
          <div class="files-concerned-content">
            <div
              v-for="file in filesList"
              :key="file.name"
              class="conflict-item"
            >
              <div class="element-type">
                <ms-image
                  :image="getFileIcon(file.name || '')"
                  class="file-icon"
                />
                <ion-text class="conflict-item__label button-medium">{{ file.name }}</ion-text>
              </div>
            </div>
            <div
              v-if="filesList.length < files.length"
              class="conflict-item more-files"
            >
              <div class="element-type">
                <ms-image
                  :image="MultiImport"
                  class="file-icon"
                />
                <ion-text class="conflict-item__label button-medium">
                  {{
                    $msTranslate({
                      key: 'FoldersPage.ConflictsFile.moreFiles',
                      data: { count: files.length - filesList.length },
                      count: files.length - filesList.length,
                    })
                  }}
                </ion-text>
              </div>
            </div>
          </div>
        </div>
      </div>

      <ion-footer class="modal-footer">
        <div class="modal-footer-buttons">
          <ion-button
            @click="resolve(DuplicatePolicy.Ignore)"
            class="modal-footer-buttons__item button-default outline-button"
            fill="clear"
          >
            {{ $msTranslate('FoldersPage.ConflictsFile.buttons.ignore') }}
          </ion-button>
          <ion-button
            @click="resolve(DuplicatePolicy.Replace)"
            class="modal-footer-buttons__item button-default fill-button"
          >
            {{ $msTranslate('FoldersPage.ConflictsFile.buttons.replace') }}
          </ion-button>
          <ion-button
            @click="resolve(DuplicatePolicy.AddCounter)"
            class="modal-footer-buttons__item button-default fill-button"
          >
            {{ $msTranslate('FoldersPage.ConflictsFile.buttons.keepAll') }}
          </ion-button>
        </div>
      </ion-footer>
    </div>
  </ms-modal>
</template>

<script setup lang="ts">
import MultiImport from '@/assets/images/multi-import.svg?raw';
import { getFileIcon } from '@/common/file';
import { EntryModel } from '@/components/files';
import { DuplicatePolicy } from '@/services/fileOperation';
import { IonButton, IonFooter, IonText, modalController } from '@ionic/vue';
import { MsImage, MsModal, MsModalResult } from 'megashark-lib';
import { computed } from 'vue';

const props = defineProps<{
  files: Array<EntryModel> | Array<File>;
}>();

const MAX_DISPLAYED_FILES = 10;

const filesList = computed(() => {
  return props.files.slice(0, MAX_DISPLAYED_FILES);
});

async function resolve(action: DuplicatePolicy): Promise<void> {
  await modalController.dismiss(action, MsModalResult.Confirm);
}
</script>

<style scoped lang="scss">
.conflict-modal-page {
  padding: 0.5rem;

  @include ms.responsive-breakpoint('sm') {
    padding: 0 0 2rem;
  }
}

.modal {
  display: flex;
  flex-direction: column;

  &-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    overflow-y: auto;

    @include ms.responsive-breakpoint('sm') {
      margin-top: 1rem;
    }
  }
}

.files-concerned {
  display: flex;
  gap: 0.5rem;
  flex-direction: column;
  max-height: 13rem;

  @include ms.responsive-breakpoint('sm') {
    max-height: 20rem;
  }

  &__title {
    color: var(--parsec-color-light-secondary-soft-text);
  }

  &-content {
    overflow-y: auto;
    overflow-x: hidden;
    background: var(--parsec-color-light-secondary-background);
    border: 1px solid var(--parsec-color-light-secondary-medium);
    border-radius: var(--parsec-radius-8);

    .conflict-item {
      padding: 0.5rem;
      border-bottom: 1px solid var(--parsec-color-light-secondary-medium);

      &:last-child {
        border-bottom: none;
      }

      &__label {
        color: var(--parsec-color-light-secondary-text);
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }

      .element-type {
        display: flex;
        align-items: center;
        gap: 0.5rem;
        overflow: hidden;

        .file-icon {
          width: 1.5rem;
          height: 1.5rem;
          flex-shrink: 0;
        }
      }

      &.more-files {
        .conflict-item__label {
          color: var(--parsec-color-light-secondary-soft-text);
          font-style: italic;
        }
      }
    }
  }
}

.modal-footer-buttons {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  width: 100%;
  margin-top: 1.5rem;

  &__item {
    width: fit-content;

    &.outline-button::part(native) {
      color: var(--parsec-color-light-secondary-text);
      --background-hover: var(--parsec-color-light-secondary-medium);
    }

    &.fill-button::part(native) {
      --background: var(--parsec-color-light-secondary-text);
      --background-hover: var(--parsec-color-light-secondary-contrast);
    }
  }
}
</style>
