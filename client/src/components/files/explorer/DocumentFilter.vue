<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    fill="clear"
    @click="openPopover($event)"
    id="select-filter-popover-button"
    class="filter-button button-medium"
    :class="{ 'has-filters': hasFilters }"
  >
    <ion-icon
      :icon="documentText"
      class="button-icon-left"
    />
    {{ hasFilters ? displayedFiltersLabel : $msTranslate('FoldersPage.search.filterButtonDefault') }}
    <ion-icon
      :icon="caretDown"
      class="button-icon-right"
    />
  </ion-button>
</template>

<script setup lang="ts">
import DocumentFilterPopover from '@/components/files/explorer/DocumentFilterPopover.vue';
import { DocumentFilters } from '@/components/files/types';
import { IonButton, IonIcon, popoverController } from '@ionic/vue';
import { caretDown, documentText } from 'ionicons/icons';
import { I18n, MsModalResult, Translatable } from 'megashark-lib';
import { computed } from 'vue';

const props = defineProps<{
  modelValue: DocumentFilters;
}>();

const emits = defineEmits<{
  (e: 'update:modelValue', value: DocumentFilters): void;
}>();

const filterLabels: Record<keyof DocumentFilters, Translatable> = {
  Folders: 'FoldersPage.search.filters.folders',
  Documents: 'FoldersPage.search.filters.documents',
  Spreadsheets: 'FoldersPage.search.filters.spreadsheets',
  Presentations: 'FoldersPage.search.filters.presentations',
  PdfDocuments: 'FoldersPage.search.filters.pdfDocuments',
  Texts: 'FoldersPage.search.filters.texts',
  Images: 'FoldersPage.search.filters.images',
  Videos: 'FoldersPage.search.filters.videos',
  Audios: 'FoldersPage.search.filters.audios',
};

const selectedFilterLabels = computed(() => {
  return (Object.entries(props.modelValue) as [keyof DocumentFilters, boolean][])
    .filter(([, activeFilter]) => activeFilter)
    .map(([activeFilter]) => I18n.translate(filterLabels[activeFilter]));
});

const hasFilters = computed(() => selectedFilterLabels.value.length > 0);

const displayedFiltersLabel = computed(() => {
  const [firstLabel, secondLabel] = selectedFilterLabels.value;

  if (!secondLabel) {
    return firstLabel;
  }
  const remainingCount = selectedFilterLabels.value.length - 2;

  return remainingCount > 0 ? `${firstLabel}, ${secondLabel} +${remainingCount}` : `${firstLabel}, ${secondLabel}`;
});

async function openPopover(event: Event): Promise<void> {
  const popover = await popoverController.create({
    component: DocumentFilterPopover,
    cssClass: 'document-filter-popover',
    componentProps: {
      filters: props.modelValue,
    },
    event: event,
    alignment: 'start',
    showBackdrop: false,
  });
  await popover.present();
  const { data, role } = await popover.onDidDismiss();
  await popover.dismiss();
  if (role === MsModalResult.Confirm && data) {
    emits('update:modelValue', data.filters);
  }
}
</script>

<style lang="scss" scoped>
.filter-button {
  padding: 0.625rem;
  border-radius: var(--parsec-radius-8);
  background: var(--parsec-color-light-secondary-background);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  color: var(--parsec-color-light-secondary-soft-text);
  box-shadow: var(--parsec-shadow-input);
  cursor: pointer;

  &::part(native) {
    padding: 0 !important;
    --background-hover: transparent;
  }

  &[class^='button-icon'] {
    color: var(--parsec-color-light-secondary-soft-text);
  }

  .button-icon-left {
    margin-right: 0.25rem;
  }

  .button-icon-right {
    margin-left: 0.5rem;
    font-size: 1rem;
  }
}

.has-filters {
  border-color: var(--parsec-color-light-primary-200);
  background-color: var(--parsec-color-light-primary-50);
  color: var(--parsec-color-light-primary-500);

  [class^='button-icon'] {
    color: var(--parsec-color-light-primary-500) !important;
  }

  &:hover {
    background-color: var(--parsec-color-light-primary-100) !important;
    color: var(--parsec-color-light-primary-600);

    [class^='button-icon'] {
      color: var(--parsec-color-light-primary-600) !important;
    }
  }
}
</style>
