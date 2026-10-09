<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-content class="filter-container">
    <ion-list
      class="filter-list"
      id="workspace-filter-list"
    >
      <ion-item-group class="list-group">
        <div class="list-group-header">
          <ion-text
            class="body-sm list-group-title"
            id="filter-title-role"
          >
            {{ $msTranslate('WorkspacesPage.filter.roles') }}
          </ion-text>
          <ion-button
            v-if="!currentFilters.owner || !currentFilters.manager || !currentFilters.contributor || !currentFilters.reader"
            @click="updateFilters({ owner: true, manager: true, contributor: true, reader: true })"
            class="reset-filters-button"
            fill="clear"
          >
            {{ $msTranslate('WorkspacesPage.filter.reset') }}
          </ion-button>
        </div>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-admin"
        >
          <ms-checkbox
            :model-value="currentFilters.owner"
            @update:model-value="updateFilters({ owner: $event })"
            class="filter-checkbox"
            label-position="left"
          >
            <ion-text>
              {{ $msTranslate('workspaceRoles.owner.label') }}
            </ion-text>
          </ms-checkbox>
        </ion-item>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-standard"
        >
          <ms-checkbox
            :model-value="currentFilters.manager"
            @update:model-value="updateFilters({ manager: $event })"
            class="filter-checkbox"
            label-position="left"
          >
            <ion-text>
              {{ $msTranslate('workspaceRoles.manager.label') }}
            </ion-text>
          </ms-checkbox>
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ms-checkbox
            :model-value="currentFilters.contributor"
            @update:model-value="updateFilters({ contributor: $event })"
            class="filter-checkbox"
            label-position="left"
          >
            <ion-text>
              {{ $msTranslate('workspaceRoles.contributor.label') }}
            </ion-text>
          </ms-checkbox>
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ms-checkbox
            :model-value="currentFilters.reader"
            @update:model-value="updateFilters({ reader: $event })"
            class="filter-checkbox"
            label-position="left"
          >
            <ion-text>
              {{ $msTranslate('workspaceRoles.reader.label') }}
            </ion-text>
          </ms-checkbox>
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-content>
</template>

<script setup lang="ts">
import { WorkspacesPageFilters } from '@/components/workspaces/types';
import { IonButton, IonContent, IonItem, IonItemGroup, IonList, IonText } from '@ionic/vue';
import { MsCheckbox } from 'megashark-lib';
import { ref } from 'vue';

const props = defineProps<{
  filters: WorkspacesPageFilters;
}>();

const emits = defineEmits<{
  (event: 'update:filters', filters: WorkspacesPageFilters): void;
}>();

// Props passed through `popoverController` are not reactive, keep a local copy
const currentFilters = ref<WorkspacesPageFilters>({ ...props.filters });

function updateFilters(changes: Partial<WorkspacesPageFilters>): void {
  currentFilters.value = { ...currentFilters.value, ...changes };
  emits('update:filters', currentFilters.value);
}
</script>

<style lang="scss" scoped>
.status-tag > * {
  cursor: pointer;
}

.filter-text {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}
</style>
