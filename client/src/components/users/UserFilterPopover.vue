<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- eslint-disable vue/no-mutating-props -->
  <ion-content class="filter-container">
    <ion-list
      class="filter-list"
      id="user-filter-list"
    >
      <ion-item-group class="list-group filter-list-group">
        <ion-text
          class="body-sm list-group-title"
          id="filter-title-status"
        >
          {{ $t('UsersPage.filter.status') }}
        </ion-text>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-active"
        >
          <user-status-tag :revoked="false" />
          <ion-checkbox
            aria-label=""
            class="checkbox"
            v-model="users.filters.statusActive"
            @ion-change="users.unselectHiddenUsers()"
          />
        </ion-item>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-revoked"
        >
          <user-status-tag :revoked="true" />
          <ion-checkbox
            aria-label=""
            class="checkbox"
            v-model="users.filters.statusRevoked"
            @ion-change="users.unselectHiddenUsers()"
          />
        </ion-item>
      </ion-item-group>
      <ion-item-group class="list-group">
        <ion-text
          class="body-sm list-group-title"
          id="filter-title-role"
        >
          {{ $t('UsersPage.filter.role') }}
        </ion-text>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-admin"
        >
          <span class="body">{{ $t('UsersPage.filter.admin') }}</span>
          <ion-checkbox
            aria-label=""
            class="checkbox"
            v-model="users.filters.profileAdmin"
            @ion-change="users.unselectHiddenUsers()"
          />
        </ion-item>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-standard"
        >
          <span class="body">{{ $t('UsersPage.filter.standard') }}</span>
          <ion-checkbox
            aria-label=""
            class="checkbox"
            v-model="users.filters.profileStandard"
            @ion-change="users.unselectHiddenUsers()"
          />
        </ion-item>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-outsider"
        >
          <span class="body">{{ $t('UsersPage.filter.outsider') }}</span>
          <ion-checkbox
            aria-label=""
            class="checkbox"
            v-model="users.filters.profileOutsider"
            @ion-change="users.unselectHiddenUsers()"
          />
        </ion-item>
      </ion-item-group>
    </ion-list>
  </ion-content>
  <!-- eslint-disable vue/no-mutating-props -->
</template>

<script setup lang="ts">
import { UserCollection } from '@/components/users/types';
import UserStatusTag from '@/components/users/UserStatusTag.vue';
import { IonCheckbox, IonContent, IonItem, IonItemGroup, IonList, IonText } from '@ionic/vue';

defineProps<{
  users: UserCollection;
}>();
</script>

<style lang="scss" scoped>
.checkbox {
  margin-left: auto;
}
</style>
