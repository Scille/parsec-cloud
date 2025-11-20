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
          <ion-text
            class="filter-text button-medium"
            @click="filters.owner = !filters.owner"
          >
            {{ $msTranslate('workspaceRoles.owner.label') }}
          </ion-text>
          <ms-checkbox
            v-model="filters.owner"
            class="filter-checkbox"
          />
        </ion-item>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-standard"
        >
          <ion-text
            class="filter-text button-medium"
            @click="filters.manager = !filters.manager"
          >
            {{ $msTranslate('workspaceRoles.manager.label') }}
          </ion-text>
          <ms-checkbox
            v-model="filters.manager"
            class="filter-checkbox"
          />
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ion-text
            class="filter-text button-medium"
            @click="filters.contributor = !filters.contributor"
          >
            {{ $msTranslate('workspaceRoles.contributor.label') }}
          </ion-text>
          <ms-checkbox
            v-model="filters.contributor"
            class="filter-checkbox"
          />
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ion-text
            class="filter-text button-medium"
            @click="filters.reader = !filters.reader"
          >
            {{ $msTranslate('workspaceRoles.reader.label') }}
          </ion-text>
          <ms-checkbox
            v-model="filters.reader"
            class="filter-checkbox"
          />
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
