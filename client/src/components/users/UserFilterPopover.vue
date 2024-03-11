<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- eslint-disable vue/no-mutating-props -->
  <ion-list
    class="filter-container"
    id="user-filter-list"
  >
    <div id="filter-title-status">
      {{ $t('UsersPage.filter.status') }}
    </div>
    <div id="filter-check-active">
      <user-status-tag :revoked="false" />
      <ion-checkbox
        aria-label=""
        class="checkbox"
        v-model="users.filters.statusActive"
        @ion-change="users.unselectHiddenUsers()"
      />
    </div>
    <div id="filter-check-revoked">
      <user-status-tag :revoked="true" />
      <ion-checkbox
        aria-label=""
        class="checkbox"
        v-model="users.filters.statusRevoked"
        @ion-change="users.unselectHiddenUsers()"
      />
    </div>
    <div id="filter-title-role">
      {{ $t('UsersPage.filter.role') }}
    </div>
    <div id="filter-check-admin">
      {{ $t('UsersPage.filter.admin') }}
      <ion-checkbox
        aria-label=""
        class="checkbox"
        v-model="users.filters.profileAdmin"
        @ion-change="users.unselectHiddenUsers()"
      />
    </div>
    <div id="filter-check-standard">
      {{ $t('UsersPage.filter.standard') }}
      <ion-checkbox
        aria-label=""
        class="checkbox"
        v-model="users.filters.profileStandard"
        @ion-change="users.unselectHiddenUsers()"
      />
    </div>
    <div id="filter-check-outsider">
      {{ $t('UsersPage.filter.outsider') }}
      <ion-checkbox
        aria-label=""
        class="checkbox"
        v-model="users.filters.profileOutsider"
        @ion-change="users.unselectHiddenUsers()"
      />
    </div>
  </ion-list>
  <!-- eslint-disable vue/no-mutating-props -->
</template>

<script setup lang="ts">
import { UserCollection } from '@/components/users/types';
import UserStatusTag from '@/components/users/UserStatusTag.vue';
import { IonCheckbox, IonList } from '@ionic/vue';

defineProps<{
  users: UserCollection;
}>();
</script>

<style lang="scss" scoped></style>
