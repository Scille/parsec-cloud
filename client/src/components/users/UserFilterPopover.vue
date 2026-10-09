<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-content class="filter-container">
    <ion-list
      class="filter-list"
      id="user-filter-list"
    >
      <ion-item-group class="list-group filter-list-group">
        <div class="list-group-header">
          <ion-text class="body-sm list-group-title">
            {{ $msTranslate('UsersPage.filter.status') }}
          </ion-text>
          <ion-button
            v-if="!currentFilters.statusActive || !currentFilters.statusRevoked || !currentFilters.statusFrozen"
            @click="updateFilters({ statusActive: true, statusRevoked: true, statusFrozen: true })"
            class="reset-filters-button"
            fill="clear"
          >
            {{ $msTranslate('UsersPage.filter.reset') }}
          </ion-button>
        </div>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-active"
        >
          <ms-checkbox
            class="filter-checkbox"
            label-position="left"
            :model-value="currentFilters.statusActive"
            @update:model-value="updateFilters({ statusActive: $event })"
          >
            <user-status-tag
              :revoked="false"
              class="status-tag"
            />
          </ms-checkbox>
        </ion-item>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-revoked"
        >
          <ms-checkbox
            class="filter-checkbox"
            label-position="left"
            :model-value="currentFilters.statusRevoked"
            @update:model-value="updateFilters({ statusRevoked: $event })"
          >
            <user-status-tag
              :revoked="true"
              class="status-tag"
            />
          </ms-checkbox>
        </ion-item>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-frozen"
        >
          <ms-checkbox
            label-position="left"
            class="filter-checkbox"
            :model-value="currentFilters.statusFrozen"
            @update:model-value="updateFilters({ statusFrozen: $event })"
          >
            <user-status-tag
              :revoked="false"
              :frozen="true"
              class="status-tag"
            />
          </ms-checkbox>
        </ion-item>
      </ion-item-group>
      <ion-item-group class="list-group">
        <div class="list-group-header">
          <ion-text class="body-sm list-group-title">
            {{ $msTranslate('UsersPage.filter.profile') }}
          </ion-text>
          <ion-button
            v-if="!currentFilters.profileAdmin || !currentFilters.profileStandard || !currentFilters.profileOutsider"
            @click="updateFilters({ profileAdmin: true, profileStandard: true, profileOutsider: true })"
            class="reset-filters-button"
            fill="clear"
          >
            {{ $msTranslate('UsersPage.filter.reset') }}
          </ion-button>
        </div>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-admin"
        >
          <ms-checkbox
            class="filter-checkbox"
            label-position="left"
            :model-value="currentFilters.profileAdmin"
            @update:model-value="updateFilters({ profileAdmin: $event })"
          >
            <ion-text class="filter-text">
              {{ $msTranslate('UsersPage.filter.admin') }}
            </ion-text>
          </ms-checkbox>
        </ion-item>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-standard"
        >
          <ms-checkbox
            class="filter-checkbox"
            label-position="left"
            :model-value="currentFilters.profileStandard"
            @update:model-value="updateFilters({ profileStandard: $event })"
          >
            <ion-text class="filter-text">
              {{ $msTranslate('UsersPage.filter.standard') }}
            </ion-text>
          </ms-checkbox>
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ms-checkbox
            label-position="left"
            class="filter-checkbox"
            :model-value="currentFilters.profileOutsider"
            @update:model-value="updateFilters({ profileOutsider: $event })"
          >
            <ion-text class="filter-text">
              {{ $msTranslate('UsersPage.filter.outsider') }}
            </ion-text>
          </ms-checkbox>
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-content>
</template>

<script setup lang="ts">
import UserStatusTag from '@/components/users/UserStatusTag.vue';
import { UserFilterLabels } from '@/components/users/types';
import { IonButton, IonContent, IonItem, IonItemGroup, IonList, IonText } from '@ionic/vue';
import { MsCheckbox } from 'megashark-lib';
import { ref } from 'vue';

const props = defineProps<{
  filters: UserFilterLabels;
}>();

const emits = defineEmits<{
  (event: 'update:filters', filters: UserFilterLabels): void;
}>();

// Props passed through `popoverController` are not reactive, keep a local copy
const currentFilters = ref<UserFilterLabels>({ ...props.filters });

function updateFilters(changes: UserFilterLabels): void {
  currentFilters.value = { ...currentFilters.value, ...changes };
  emits('update:filters', currentFilters.value);
}
</script>

<style lang="scss" scoped>
.status-tag > * {
  cursor: pointer;
}
</style>
