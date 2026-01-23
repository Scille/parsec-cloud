<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- eslint-disable vue/no-mutating-props -->
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
            v-if="!filters.owner || !filters.manager || !filters.contributor || !filters.reader"
            @click="
              filters.owner = true;
              filters.manager = true;
              filters.contributor = true;
              filters.reader = true;
            "
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
            v-model="filters.owner"
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
            v-model="filters.manager"
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
            v-model="filters.contributor"
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
            v-model="filters.reader"
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
  <!-- eslint-disable vue/no-mutating-props -->
</template>

<script setup lang="ts">
import { WorkspacesPageFilters } from '@/components/workspaces/utils';
import { IonButton, IonContent, IonItem, IonItemGroup, IonList, IonText } from '@ionic/vue';
import { MsCheckbox } from 'megashark-lib';

defineProps<{
  filters: WorkspacesPageFilters;
}>();
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
