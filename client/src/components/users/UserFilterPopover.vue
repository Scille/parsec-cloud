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
          {{ $msTranslate('UsersPage.filter.status') }}
        </ion-text>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-active"
        >
          <user-status-tag
            :revoked="false"
            class="status-tag"
            @click="
              users.filters.statusActive = !users.filters.statusActive;
              users.unselectHiddenUsers();
            "
          />
          <ms-checkbox
            class="filter-checkbox"
            v-model="users.filters.statusActive"
            @change="users.unselectHiddenUsers()"
          />
        </ion-item>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-revoked"
        >
          <user-status-tag
            :revoked="true"
            class="status-tag"
            @click="
              users.filters.statusRevoked = !users.filters.statusRevoked;
              users.unselectHiddenUsers();
            "
          />
          <ms-checkbox
            class="filter-checkbox"
            v-model="users.filters.statusRevoked"
            @change="users.unselectHiddenUsers()"
          />
        </ion-item>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-frozen"
        >
          <user-status-tag
            :revoked="false"
            :frozen="true"
            class="status-tag"
            @click="
              users.filters.statusFrozen = !users.filters.statusFrozen;
              users.unselectHiddenUsers();
            "
          />
          <ms-checkbox
            class="filter-checkbox"
            v-model="users.filters.statusFrozen"
            @change="users.unselectHiddenUsers()"
          />
        </ion-item>
      </ion-item-group>
      <ion-item-group class="list-group">
        <ion-text
          class="body-sm list-group-title"
          id="filter-title-role"
        >
          {{ $msTranslate('UsersPage.filter.profile') }}
        </ion-text>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-admin"
        >
          <ion-text
            class="filter-text body"
            @click="
              users.filters.profileAdmin = !users.filters.profileAdmin;
              users.unselectHiddenUsers();
            "
          >
            {{ $msTranslate('UsersPage.filter.admin') }}
          </ion-text>
          <ms-checkbox
            class="filter-checkbox"
            v-model="users.filters.profileAdmin"
            @change="users.unselectHiddenUsers()"
          />
        </ion-item>
        <ion-item
          class="list-group-item ion-no-padding"
          id="filter-check-standard"
        >
          <ion-text
            class="filter-text body"
            @click="
              users.filters.profileStandard = !users.filters.profileStandard;
              users.unselectHiddenUsers();
            "
          >
            {{ $msTranslate('UsersPage.filter.standard') }}
          </ion-text>
          <ms-checkbox
            class="filter-checkbox"
            v-model="users.filters.profileStandard"
            @ion-change="users.unselectHiddenUsers()"
          />
        </ion-item>
        <ion-item class="list-group-item ion-no-padding">
          <ion-text
            class="filter-text body"
            @click="
              users.filters.profileOutsider = !users.filters.profileOutsider;
              users.unselectHiddenUsers();
            "
          >
            {{ $msTranslate('UsersPage.filter.outsider') }}
          </ion-text>
          <ms-checkbox
            class="filter-checkbox"
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
import UserStatusTag from '@/components/users/UserStatusTag.vue';
import { UserCollection } from '@/components/users/types';
import { IonContent, IonItem, IonItemGroup, IonList, IonText } from '@ionic/vue';
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
