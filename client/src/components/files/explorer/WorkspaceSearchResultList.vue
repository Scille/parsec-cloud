<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="search-results">
    <div
      v-if="props.searchFolder"
      class="search-results__folder"
    >
      <ion-text class="body-sm">
        {{ $msTranslate({ key: 'FoldersPage.search.searchingIn', data: { folder: props.searchFolder } }) }}
      </ion-text>
    </div>
    <div
      v-if="props.isSearching && props.results.length === 0"
      class="search-results__loading"
    >
      <ms-spinner class="search-spinner" />
      <ion-text class="body">{{ $msTranslate('FoldersPage.search.searching') }}</ion-text>
    </div>
    <div
      v-else-if="!props.isSearching && props.results.length === 0"
      class="search-results__empty"
    >
      <ion-text class="body">{{ $msTranslate('FoldersPage.search.noResults') }}</ion-text>
    </div>
    <div
      v-else
      class="search-results__list"
    >
      <div
        v-for="(match, index) in props.results"
        :key="index"
        class="search-result-item"
        @click="$emit('resultClick', match)"
      >
        <ms-image
          :image="isFile(match) ? getFileIcon(getFilename(match.path)) : Folder"
          class="search-result-item__icon"
        />
        <div class="search-result-item__path">
          <span
            v-for="(part, partIndex) in getHighlightedParts(match)"
            :key="partIndex"
            :class="{ 'path-match': part.isMatch }"
          >
            {{ part.text }}
          </span>
        </div>
      </div>
      <div
        v-if="props.isSearching"
        class="search-results__loading-more"
      >
        <ms-spinner class="search-spinner" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getFileIcon } from '@/common/file';
import { WorkspaceSearchMatch } from '@/parsec/types';
import { EntryStatTag } from '@/plugins/libparsec';
import { IonText } from '@ionic/vue';
import { Folder, MsImage, MsSpinner } from 'megashark-lib';

const props = defineProps<{
  results: WorkspaceSearchMatch[];
  isSearching: boolean;
  searchFolder?: string;
}>();

defineEmits<{
  (e: 'resultClick', match: WorkspaceSearchMatch): void;
}>();

interface PathPart {
  text: string;
  isMatch: boolean;
}

function isFile(match: WorkspaceSearchMatch): boolean {
  return match.stat.tag === EntryStatTag.File;
}

function getFilename(path: string): string {
  const parts = path.split('/');
  return parts[parts.length - 1] || path;
}

function getHighlightedParts(match: WorkspaceSearchMatch): PathPart[] {
  const path = match.path;
  const matchSet = new Set(match.matchPositions.map((p) => Number(p)));
  const parts: PathPart[] = [];
  let currentText = '';
  let currentIsMatch = false;

  for (let i = 0; i < path.length; i++) {
    const isMatch = matchSet.has(i);
    if (i === 0) {
      currentIsMatch = isMatch;
      currentText = path[i];
    } else if (isMatch === currentIsMatch) {
      currentText += path[i];
    } else {
      parts.push({ text: currentText, isMatch: currentIsMatch });
      currentText = path[i];
      currentIsMatch = isMatch;
    }
  }
  if (currentText) {
    parts.push({ text: currentText, isMatch: currentIsMatch });
  }
  return parts;
}
</script>

<style scoped lang="scss">
.search-results {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
  overflow-y: auto;

  &__folder {
    padding: 0.5rem 1rem;
    color: var(--parsec-color-light-secondary-grey);
    border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);
  }

  &__loading,
  &__empty {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    padding: 2rem 1rem;
    color: var(--parsec-color-light-secondary-grey);
  }

  &__loading-more {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.75rem;
  }

  &__list {
    display: flex;
    flex-direction: column;
  }
}

.search-result-item {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 1rem;
  cursor: pointer;
  border-bottom: 1px solid var(--parsec-color-light-secondary-disabled);

  &:hover {
    background: var(--parsec-color-light-secondary-background);
  }

  &__icon {
    width: 1.5rem;
    height: 1.5rem;
    flex-shrink: 0;
  }

  &__path {
    font-size: 0.875rem;
    color: var(--parsec-color-light-secondary-text);
    word-break: break-all;
  }
}

.path-match {
  color: var(--parsec-color-light-primary-600);
  font-weight: 600;
}

.search-spinner {
  height: 1.25rem;
}
</style>
