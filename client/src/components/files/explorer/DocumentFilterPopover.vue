<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-content class="filter-container">
    <ion-list class="filter-list">
      <ion-item-group class="list-group filter-list-group">
        <ion-item class="list-group-item ion-no-padding">
          <ms-checkbox
            class="filter-checkbox"
            label-position="left"
            v-model="updatedFilters.Folders"
          >
            <div class="filter-checkbox-content">
              <ms-image
                :image="Folder"
                class="filter-checkbox__icon"
              />
              <ion-text class="filter-checkbox__label">
                {{ $msTranslate('FoldersPage.search.filters.folders') }}
              </ion-text>
            </div>
          </ms-checkbox>
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ms-checkbox
            class="filter-checkbox"
            label-position="left"
            v-model="updatedFilters.Documents"
          >
            <div class="filter-checkbox-content">
              <ms-image
                :image="File.Word"
                class="filter-checkbox__icon"
              />
              <ion-text class="filter-checkbox__label">
                {{ $msTranslate('FoldersPage.search.filters.documents') }}
              </ion-text>
            </div>
          </ms-checkbox>
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ms-checkbox
            label-position="left"
            class="filter-checkbox"
            v-model="updatedFilters.Spreadsheets"
          >
            <div class="filter-checkbox-content">
              <ms-image
                :image="File.Excel"
                class="filter-checkbox__icon"
              />
              <ion-text class="filter-checkbox__label">
                {{ $msTranslate('FoldersPage.search.filters.spreadsheets') }}
              </ion-text>
            </div>
          </ms-checkbox>
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ms-checkbox
            label-position="left"
            class="filter-checkbox"
            v-model="updatedFilters.Presentations"
          >
            <div class="filter-checkbox-content">
              <ms-image
                :image="File.Powerpoint"
                class="filter-checkbox__icon"
              />
              <ion-text class="filter-checkbox__label">
                {{ $msTranslate('FoldersPage.search.filters.presentations') }}
              </ion-text>
            </div>
          </ms-checkbox>
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ms-checkbox
            label-position="left"
            class="filter-checkbox"
            v-model="updatedFilters.PdfDocuments"
          >
            <div class="filter-checkbox-content">
              <ms-image
                :image="File.Pdf"
                class="filter-checkbox__icon"
              />
              <ion-text class="filter-checkbox__label">
                {{ $msTranslate('FoldersPage.search.filters.pdfDocuments') }}
              </ion-text>
            </div>
          </ms-checkbox>
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ms-checkbox
            label-position="left"
            class="filter-checkbox"
            v-model="updatedFilters.Audios"
          >
            <div class="filter-checkbox-content">
              <ms-image
                :image="File.Music"
                class="filter-checkbox__icon"
              />
              <ion-text class="filter-checkbox__label">
                {{ $msTranslate('FoldersPage.search.filters.audios') }}
              </ion-text>
            </div>
          </ms-checkbox>
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ms-checkbox
            label-position="left"
            class="filter-checkbox"
            v-model="updatedFilters.Videos"
          >
            <div class="filter-checkbox-content">
              <ms-image
                :image="File.Video"
                class="filter-checkbox__icon"
              />
              <ion-text class="filter-checkbox__label">
                {{ $msTranslate('FoldersPage.search.filters.videos') }}
              </ion-text>
            </div>
          </ms-checkbox>
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ms-checkbox
            label-position="left"
            class="filter-checkbox"
            v-model="updatedFilters.Images"
          >
            <div class="filter-checkbox-content">
              <ms-image
                :image="File.Image"
                class="filter-checkbox__icon"
              />
              <ion-text class="filter-checkbox__label">
                {{ $msTranslate('FoldersPage.search.filters.images') }}
              </ion-text>
            </div>
          </ms-checkbox>
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ms-checkbox
            label-position="left"
            class="filter-checkbox"
            v-model="updatedFilters.Texts"
          >
            <div class="filter-checkbox-content">
              <ms-image
                :image="File.Text"
                class="filter-checkbox__icon"
              />
              <ion-text class="filter-checkbox__label">
                {{ $msTranslate('FoldersPage.search.filters.texts') }}
              </ion-text>
            </div>
          </ms-checkbox>
        </ion-item>
      </ion-item-group>
      <div class="filter-buttons">
        <ion-button
          @click="applyFilters"
          class="filter-apply-button"
        >
          {{ $msTranslate('FoldersPage.search.applyFilters') }}
        </ion-button>
      </div>
    </ion-list>
  </ion-content>
</template>

<script setup lang="ts">
import { DocumentFilters } from '@/components/files/types';
import { IonButton, IonContent, IonItem, IonItemGroup, IonList, IonText, popoverController } from '@ionic/vue';
import { File, Folder, MsCheckbox, MsImage, MsModalResult } from 'megashark-lib';
import { ref, toRaw } from 'vue';

const props = defineProps<{
  filters: DocumentFilters;
}>();

const updatedFilters = ref<DocumentFilters>(structuredClone(toRaw(props.filters)));

async function applyFilters(): Promise<void> {
  await popoverController.dismiss({ filters: updatedFilters.value }, MsModalResult.Confirm);
}
</script>

<style lang="scss" scoped>
.filter-list {
  gap: ms.spacing('gap-lg') !important;
}

.filter-checkbox {
  display: flex;
  align-items: center;
  gap: ms.spacing('gap-lg');

  &-content {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-lg');
  }

  &__icon {
    max-width: 1.5rem;
    max-height: 1.5rem;
  }

  &__label {
    @include ms.font('label-md-medium');
    width: 100%;
    color: ms.color('text-neutral-default');
  }
}

.filter-buttons {
  display: flex;
  justify-content: flex-end;
  border-top: ms.border('thin') solid ms.color('border-neutral-default-subtle-hover');
  padding: ms.spacing('padding-lg') ms.spacing('padding-lg') ms.spacing('padding-none');

  .filter-apply-button {
    @include ms.font('label-md-medium');
    align-self: flex-end;
    border-radius: ms.radius('lg');
    color: ms.color('text-on-color-label');
  }
}
</style>
