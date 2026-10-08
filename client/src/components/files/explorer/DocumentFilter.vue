<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-button
    fill="clear"
    size="default"
    @click="openPopover($event)"
    id="select-filter-popover-button"
    class="filter-button"
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
  @include ms.font('label-md-medium');
  color: ms.color('text-neutral-default');

  &[class^='button-icon'] {
    color: ms.color('text-neutral-default');
  }

  .button-icon-left {
    margin-right: ms.spacing('gap-sm');
  }

  .button-icon-right {
    margin-left: ms.spacing('gap-lg');
    font-size: 1rem;
  }
}

.has-filters {
  color: ms.color('text-brand-default');

  [class^='button-icon'] {
    color: ms.color('icon-brand-default') !important;
  }

  &:hover {
    color: ms.color('text-brand-default-hover');

    [class^='button-icon'] {
      color: ms.color('icon-brand-default-hover') !important;
    }
  }
}
</style>
