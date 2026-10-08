<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="file-search-results">
    <div class="results-header">
      <ion-text class="results-header__title">
        {{ $msTranslate('FoldersPage.search.title') }}
        <span class="results-header__count">
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
        v-if="isLargeDisplay && !multipleWorkspaces"
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
        class="only-titles-toggle filter-button"
        :class="{ active: titlesOnly }"
        fill="clear"
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

    <ion-list
      class="list-container files-list-container results-list-container"
      v-show="hasResults"
      id="search-page-file-list"
    >
      <ion-list-header
        class="list-header files-list-header"
        lines="full"
        v-if="isLargeDisplay"
      >
        <ion-text class="list-header-label cell-title ion-text-nowrap header-label-name">
          <span class="label-text">{{ $msTranslate('FoldersPage.listDisplayTitles.name') }}</span>
        </ion-text>
        <ion-text class="list-header-label cell-title ion-text-nowrap header-label-last-update">
          <span class="label-text">{{ $msTranslate('FoldersPage.listDisplayTitles.lastUpdate') }}</span>
        </ion-text>
        <ion-text class="list-header-label cell-title ion-text-nowrap header-label-size">
          <span class="label-text">{{ $msTranslate('FoldersPage.listDisplayTitles.size') }}</span>
        </ion-text>
        <ion-text class="list-header-label list-item-end cell-title ion-text-nowrap header-label-space" />
      </ion-list-header>
      <file-search-result-item
        v-for="result in filteredResults"
        :key="result.stats.id"
        :search-item="result"
        @click="$emit('itemClick', $event)"
        @menu-click="(event, entry, onFinished) => $emit('menuItemClick', event, entry, onFinished)"
      />
    </ion-list>
    <div
      v-show="!hasResults"
      class="results-empty"
    >
      <ms-image
        :image="NoSearchResults"
        class="results-empty__image"
      />
      <ion-text class="results-empty__title">
        {{ $msTranslate('FoldersPage.search.noResults.title') }}
      </ion-text>
      <ion-text class="results-empty__subtitle">
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
import { SearchResult } from '@/parsec';
import { IonButton, IonIcon, IonList, IonListHeader, IonText } from '@ionic/vue';
import { checkmark, text } from 'ionicons/icons';
import { MsImage, MsReportText, MsReportTheme, MsSpinner, useWindowSize } from 'megashark-lib';
import { computed, ref } from 'vue';

const props = defineProps<{
  pattern: string;
  searchResults: Array<SearchResult>;
  active: boolean;
  multipleWorkspaces: boolean;
}>();

defineEmits<{
  (e: 'itemClick', entry: SearchResult): void;
  (e: 'menuItemClick', event: Event, entry: SearchResult, onFinished: () => void): void;
}>();

const { isLargeDisplay } = useWindowSize();

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
</script>

<style scoped lang="scss">
.file-search-results {
  padding: ms.spacing('padding-3xl') ms.spacing('padding-3xl') ms.spacing('padding-none') ms.spacing('padding-3xl');
  border-radius: ms.radius('2xl');
  background: ms.color('surface-base-default');
  border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  box-shadow: ms.shadow('input');
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-3xl');
  overflow: hidden;
  position: relative;

  @include ms.responsive-breakpoint('sm') {
    border: none;
    padding: ms.spacing('padding-none');
  }

  &::after {
    content: '';
    position: absolute;
    left: 0;
    right: 0;
    bottom: 0;
    height: 1.75rem;
    background: linear-gradient(180deg, transparent, ms.color('surface-base-default'));
    z-index: 10;
    transition: opacity 0.2s;

    @include ms.responsive-breakpoint('sm') {
      display: none;
    }
  }
}

#search-page-file-list {
  padding: ms.spacing('padding-none');
}

.results-header {
  display: flex;
  gap: ms.spacing('gap-lg');
  position: relative;

  @include ms.responsive-breakpoint('lg') {
    flex-direction: column;
  }

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-3xl') ms.spacing('padding-3xl') ms.spacing('padding-none');
  }

  &__title {
    @include ms.font('heading-h3');
    color: ms.color('text-base-body');
  }

  &__count {
    @include ms.font('label-lg-medium');
    color: ms.color('text-base-description');
  }

  &__info {
    width: fit-content;
    position: absolute;
    right: 0;
    padding: ms.spacing('padding-lg') ms.spacing('padding-2xl') !important;

    @include ms.responsive-breakpoint('lg') {
      position: relative;
      width: 100%;
    }
  }
}

.results-filters {
  display: flex;
  gap: ms.spacing('gap-3xl');

  @include ms.responsive-breakpoint('sm') {
    padding-inline: ms.spacing('padding-3xl');
  }

  .only-titles-toggle {
    @include ms.font('label-md-medium');
    color: ms.color('text-neutral-default');

    .button-icon-left {
      font-size: 1rem;
      margin-right: ms.spacing('gap-sm');
    }

    &.active {
      color: ms.color('text-brand-default');

      .active-icon {
        color: ms.color('icon-brand-default');
        font-size: 1rem;
        margin-left: ms.spacing('gap-sm');
      }

      &:hover {
        color: ms.color('text-brand-default-hover');

        .active-icon {
          color: ms.color('icon-brand-default-hover');
        }
      }
    }
  }
}

.results-list-container {
  flex-grow: 1;
  overflow: auto;

  .files-list-header {
    background: ms.color('surface-base-default');
    backdrop-filter: none;
  }
}

.results-empty {
  display: flex;
  justify-content: center;
  flex-direction: column;
  align-items: center;
  text-align: center;
  color: ms.color('text-base-body');
  height: 100%;
  gap: ms.spacing('gap-lg');

  &__title {
    @include ms.font('body-lg-medium');
    color: ms.color('text-base-body');
    margin-top: 1rem;
  }

  &__subtitle {
    @include ms.font('body-lg-regular');
    color: ms.color('text-base-description');
    max-width: 25rem;
  }
}
</style>
