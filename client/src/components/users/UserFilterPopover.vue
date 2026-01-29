<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- eslint-disable vue/no-mutating-props -->
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
            v-if="!users.filters.statusActive || !users.filters.statusRevoked || !users.filters.statusFrozen"
            @click="
              users.filters.statusActive = true;
              users.filters.statusRevoked = true;
              users.filters.statusFrozen = true;
            "
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
            v-model="users.filters.statusActive"
            @change="users.unselectHiddenUsers()"
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
            v-model="users.filters.statusRevoked"
            @change="users.unselectHiddenUsers()"
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
            v-model="users.filters.statusFrozen"
            @change="users.unselectHiddenUsers()"
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
            v-if="!users.filters.profileAdmin || !users.filters.profileStandard || !users.filters.profileOutsider"
            @click="
              users.filters.profileAdmin = true;
              users.filters.profileStandard = true;
              users.filters.profileOutsider = true;
            "
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
            v-model="users.filters.profileAdmin"
            @change="users.unselectHiddenUsers()"
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
            v-model="users.filters.profileStandard"
            @change="users.unselectHiddenUsers()"
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
            v-model="users.filters.profileOutsider"
            @change="users.unselectHiddenUsers()"
          >
            <ion-text class="filter-text">
              {{ $msTranslate('UsersPage.filter.outsider') }}
            </ion-text>
          </ms-checkbox>
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-content>
  <!-- eslint-disable vue/no-mutating-props -->
</template>

<script setup lang="ts">
import UserStatusTag from '@/components/users/UserStatusTag.vue';
import { UserCollection } from '@/components/users/types';
import { IonButton, IonContent, IonItem, IonItemGroup, IonList, IonText } from '@ionic/vue';
import { MsCheckbox } from 'megashark-lib';

defineProps<{
  users: UserCollection;
}>();
</script>

<style lang="scss" scoped>
.status-tag > * {
  cursor: pointer;
}
</style>
