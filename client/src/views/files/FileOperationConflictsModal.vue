<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
          <ion-text class="files-concerned__title">
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
                <ion-text class="conflict-item__label">{{ file.name }}</ion-text>
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
                <ion-text class="conflict-item__label">
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
            class="modal-footer-buttons__item color-secondary"
            fill="clear"
          >
            {{ $msTranslate('FoldersPage.ConflictsFile.buttons.ignore') }}
          </ion-button>
          <ion-button
            @click="resolve(DuplicatePolicy.Replace)"
            class="modal-footer-buttons__item color-secondary"
          >
            {{ $msTranslate('FoldersPage.ConflictsFile.buttons.replace') }}
          </ion-button>
          <ion-button
            @click="resolve(DuplicatePolicy.AddCounter)"
            class="modal-footer-buttons__item color-secondary"
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
  return [...props.files].slice(0, MAX_DISPLAYED_FILES);
});

async function resolve(action: DuplicatePolicy): Promise<void> {
  await modalController.dismiss(action, MsModalResult.Confirm);
}
</script>

<style scoped lang="scss">
.conflict-modal-page {
  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-none') ms.spacing('padding-none') ms.spacing('padding-5xl');
  }
}

.modal {
  display: flex;
  flex-direction: column;

  &-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-lg');
    overflow-y: auto;

    @include ms.responsive-breakpoint('sm') {
      margin-top: 1rem;
    }
  }
}

.files-concerned {
  display: flex;
  gap: ms.spacing('gap-lg');
  flex-direction: column;
  max-height: 13rem;

  @include ms.responsive-breakpoint('sm') {
    max-height: 20rem;
  }

  &__title {
    @include ms.font('label-sm-medium');
    color: ms.color('text-base-description');
  }

  &-content {
    overflow-y: auto;
    overflow-x: hidden;
    background: ms.color('surface-base-default-secondary');
    border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
    border-radius: ms.radius('lg');

    .conflict-item {
      padding: ms.spacing('padding-lg');
      border-bottom: ms.border('thin') solid ms.color('border-neutral-default-subtle');

      &:last-child {
        border-bottom: none;
      }

      &__label {
        @include ms.font('label-md-medium');
        color: ms.color('text-base-body');
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
      }

      .element-type {
        display: flex;
        align-items: center;
        gap: ms.spacing('gap-lg');
        overflow: hidden;

        .file-icon {
          width: 1.5rem;
          height: 1.5rem;
          flex-shrink: 0;
        }
      }

      &.more-files {
        .conflict-item__label {
          color: ms.color('text-base-description');
          font-style: italic;
        }
      }
    }
  }
}

.modal-footer-buttons {
  display: flex;
  gap: ms.spacing('gap-3xl');
  justify-content: flex-end;
  width: 100%;
  margin-top: 1.5rem;
}
</style>
