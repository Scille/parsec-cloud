<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="file-search-results">
    <div class="results-header">
      <ion-text class="results-header__title title-h2">
        {{ $msTranslate('FoldersPage.search.title') }}
        <span class="results-header__count subtitles-normal">
          ({{
            $msTranslate({
              key: 'FoldersPage.search.resultsCount',
              data: { count: filteredResults.length },
              count: filteredResults.length,
            })
          }})
        </span>
      </ion-text>
      <ms-report-text
        v-if="isLargeDisplay"
        :theme="MsReportTheme.Info"
        class="results-header__info"
      >
        <ion-text>{{ $msTranslate('FoldersPage.search.currentWorkspaceOnly') }}</ion-text>
      </ms-report-text>
    </div>

    <div class="results-filters">
      <document-filter v-model="documentFilters" />
      <ion-button
        @click="titlesOnly = !titlesOnly"
        class="only-titles-toggle filter-button button-medium"
        :class="{ active: titlesOnly }"
      >
        <ion-icon
          :icon="text"
          slot="start"
          class="button-icon-left"
        />
        {{ $msTranslate('FoldersPage.search.onlyMatchTitles') }}
        <ion-icon
          v-if="titlesOnly"
          :icon="checkmark"
          slot="end"
          class="active-icon"
        />
      </ion-button>
      <ms-spinner v-show="active" />
    </div>

    <div
      class="results-list"
      v-show="hasResults"
      ref="results"
    >
      <ion-list-header
        class="folder-list-header"
        lines="full"
        v-if="isLargeDisplay"
      >
        <ion-text class="folder-list-header__label cell-title ion-text-nowrap header-label-name header-label-name-results">
          <span class="header-label-name__text">{{ $msTranslate('FoldersPage.listDisplayTitles.name') }}</span>
        </ion-text>
        <ion-text class="folder-list-header__label cell-title ion-text-nowrap header-label-last-update">
          <span class="header-label-last-update__text">{{ $msTranslate('FoldersPage.listDisplayTitles.lastUpdate') }}</span>
        </ion-text>
        <ion-text class="folder-list-header__label cell-title ion-text-nowrap header-label-size">
          <span class="header-label-size__text">{{ $msTranslate('FoldersPage.listDisplayTitles.size') }}</span>
        </ion-text>
        <ion-text class="folder-list-header__label cell-title ion-text-nowrap header-label-space" />
      </ion-list-header>
      <file-search-result-item
        v-for="result in filteredResults"
        :key="result.stats.id"
        :search-item="result"
        :workspace-name="workspaceName"
        @click="$emit('itemClick', $event)"
        @menu-click="(event, entry, onFinished) => $emit('menuItemClick', event, entry, onFinished)"
      />
    </div>
    <div
      v-show="!hasResults"
      class="results-empty"
    >
      <ms-image
        :image="NoSearchResults"
        class="results-empty__image"
      />
      <ion-text class="results-empty__title subtitles-lg">
        {{ $msTranslate('FoldersPage.search.noResults.title') }}
      </ion-text>
      <ion-text class="results-empty__subtitle body-lg">
        {{ $msTranslate('FoldersPage.search.noResults.subtitle') }}
      </ion-text>
    </div>
  </div>
</template>

<script setup lang="ts">
import NoSearchResults from '@/assets/images/no-element-file.svg?raw';
import { detectFileContentType, FileContentType } from '@/common/fileTypes';
import DocumentFilter from '@/components/files/explorer/DocumentFilter.vue';
import FileSearchResultItem from '@/components/files/explorer/FileSearchResultItem.vue';
import { DocumentFilters, DocumentFiltersIncludeAll, DocumentFiltersIncludeNone } from '@/components/files/types';
import { EntryName, SearchResult } from '@/parsec';
import { IonButton, IonIcon, IonListHeader, IonText } from '@ionic/vue';
import { checkmark, text } from 'ionicons/icons';
import Mark from 'mark.js';
import { MsImage, MsReportText, MsReportTheme, MsSpinner, useWindowSize } from 'megashark-lib';
import { computed, nextTick, onMounted, ref, useTemplateRef, watch } from 'vue';

const props = defineProps<{
  pattern: string;
  searchResults: Array<SearchResult>;
  active: boolean;
  workspaceName: EntryName;
}>();

defineEmits<{
  (e: 'itemClick', entry: SearchResult): void;
  (e: 'menuItemClick', event: Event, entry: SearchResult, onFinished: () => void): void;
}>();

const { isLargeDisplay } = useWindowSize();
const resultsRef = useTemplateRef<HTMLDivElement>('results');

const filteredResults = computed(() => {
  const activeFilters = Object.values(documentFilters.value).some((selected) => selected)
    ? documentFilters.value
    : DocumentFiltersIncludeAll();

  return props.searchResults.filter((result) => {
    if (titlesOnly.value && !result.titleMatch) {
      return false;
    }
    if (!activeFilters.Folders && !result.stats.isFile()) {
      return false;
    }
    const docType = detectFileContentType(result.stats.name);
    switch (docType.type) {
      case FileContentType.Document:
        return activeFilters.Documents;
      case FileContentType.Spreadsheet:
        return activeFilters.Spreadsheets;
      case FileContentType.Presentation:
        return activeFilters.Presentations;
      case FileContentType.PdfDocument:
        return activeFilters.PdfDocuments;
      case FileContentType.Audio:
        return activeFilters.Audios;
      case FileContentType.Image:
        return activeFilters.Images;
      case FileContentType.Text:
        return activeFilters.Texts;
      case FileContentType.Video:
        return activeFilters.Videos;
      default:
        break;
    }
    return true;
  });
});

const hasResults = computed(() => {
  return filteredResults.value.length > 0;
});

const titlesOnly = ref(false);

const documentFilters = ref<DocumentFilters>(DocumentFiltersIncludeNone());

const marker = ref<Mark | undefined>(undefined);

async function mark(): Promise<void> {
  await nextTick();
  if (!resultsRef.value) {
    return;
  }
  marker.value = new Mark(resultsRef.value.querySelectorAll('.can-highlight'));
  marker.value.unmark({
    done: () => {
      marker.value?.mark(props.pattern, { element: 'span', className: 'search-highlight' });
    },
  });
}

watch(
  () => [filteredResults, props.pattern],
  () => {
    mark();
  },
  { deep: true },
);

onMounted(async () => {
  mark();
});
</script>

<style scoped lang="scss">
:deep(.search-highlight) {
  color: var(--parsec-color-light-primary-500);
  font-weight: bold;
}

.file-search-results {
  padding: 1rem 1rem 0 1rem;
  border-radius: var(--parsec-radius-12);
  background: var(--parsec-color-light-secondary-white);
  border: 1px solid var(--parsec-color-light-secondary-premiere);
  box-shadow: var(--parsec-shadow-input);
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  overflow: hidden;
  position: relative;

  @include ms.responsive-breakpoint('sm') {
    border: none;
    padding: 0;
  }

  &::after {
    content: '';
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    height: 1.75rem;
    background: linear-gradient(180deg, transparent, var(--parsec-color-light-secondary-white));
    z-index: 10;
    transition: opacity 0.2s;

    @include ms.responsive-breakpoint('sm') {
      display: none;
    }
  }
}

.results-header {
  display: flex;
  gap: 0.5rem;
  position: relative;

  @include ms.responsive-breakpoint('lg') {
    flex-direction: column;
  }

  @include ms.responsive-breakpoint('sm') {
    padding: 1rem 1rem 0;
  }

  &__title {
    color: var(--parsec-color-light-secondary-text);
  }

  &__count {
    color: var(--parsec-color-light-secondary-grey);
  }

  &__info {
    width: fit-content;
    position: absolute;
    right: 0;
    padding: 0.5rem 0.75rem !important;

    @include ms.responsive-breakpoint('lg') {
      position: relative;
      width: 100%;
    }
  }
}

.results-filters {
  display: flex;
  gap: 1rem;

  @include ms.responsive-breakpoint('sm') {
    padding-inline: 1rem;
  }

  .only-titles-toggle {
    --background-hover: transparent;
    padding: 0.5rem 0.75rem;
    border-radius: var(--parsec-radius-8);
    background: var(--parsec-color-light-secondary-background);
    border: 1px solid var(--parsec-color-light-secondary-premiere);
    color: var(--parsec-color-light-secondary-soft-text);
    box-shadow: var(--parsec-shadow-input);

    &::part(native) {
      padding: 0 !important;
      background: none;
    }

    .button-icon-left {
      font-size: 1rem;
      margin-right: 0.25rem;
    }

    &:hover {
      background-color: var(--parsec-color-light-secondary-medium) !important;
    }

    &.active {
      background-color: var(--parsec-color-light-primary-50);
      border-color: var(--parsec-color-light-primary-200);
      color: var(--parsec-color-light-primary-500);

      .active-icon {
        color: var(--parsec-color-light-primary-500);
        font-size: 1rem;
        margin-left: 0.25rem;
      }

      &:hover {
        background-color: var(--parsec-color-light-primary-100) !important;
        color: var(--parsec-color-light-primary-600);

        .active-icon {
          color: var(--parsec-color-light-primary-600);
        }
      }
    }
  }
}

.results-list {
  flex-grow: 1;
  overflow: auto;

  .folder-list-header {
    background: var(--parsec-color-light-secondary-white);
    backdrop-filter: none;
  }
}

.results-empty {
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;
  text-align: center;
  color: var(--parsec-color-light-secondary-text);
  height: 100%;
  gap: 0.5rem;

  &__title {
    color: var(--parsec-color-light-secondary-text);
    margin-top: 1rem;
  }

  &__subtitle {
    color: var(--parsec-color-light-secondary-hard-grey);
    max-width: 25rem;
  }
}
</style>
