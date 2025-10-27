<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- eslint-disable vue/no-mutating-props -->
  <ion-content class="filter-container">
    <ion-list
      class="filter-list"
      id="user-filter-list"
    >
      <ion-item-group class="list-group">
        <ion-text
          class="body-sm list-group-title"
          id="filter-title-role"
        >
          {{ $msTranslate('WorkspacesPage.filter.roles') }}
        </ion-text>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-admin"
        >
          <ion-text
            class="filter-text body"
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
            class="filter-text body"
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
            class="filter-text body"
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
            class="filter-text body"
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
import { IonContent, IonItem, IonItemGroup, IonList, IonText } from '@ionic/vue';
import { MsCheckbox } from 'megashark-lib';

defineProps<{
  filters: WorkspacesPageFilters;
}>();
</script>

<style lang="scss" scoped>
.status-tag > * {
  cursor: pointer;
}
</style>
