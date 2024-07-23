<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div id="page">
        <client-area-sidebar v-show="true" />
        <div class="main-content">
          <client-area-header v-show="true" />
          <div
            class="organization"
            v-for="organization in organizations"
            :key="organization.bmsId"
          >
            {{ organization }}
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonContent } from '@ionic/vue';
import ClientAreaHeader from '@/views/client-area/ClientAreaHeader.vue';
import ClientAreaSidebar from '@/views/client-area/ClientAreaSidebar.vue';
import { BmsAccessInstance, BmsOrganization, DataType } from '@/services/bms';
import { onMounted, ref } from 'vue';

const organizations = ref<Array<BmsOrganization>>([]);

onMounted(async () => {
  const response = await BmsAccessInstance.get().listOrganizations();
  if (!response.isError && response.data && response.data.type === DataType.ListOrganizations) {
    organizations.value = response.data.organizations;
    console.log(organizations.value);
  }
});
</script>

<style scoped lang="scss">
#page {
  position: relative;
  display: flex;
  height: 100%;
}

.main-content {
  background-color: blue;
  width: -webkit-fill-available;
  height: 100%;
  color: white;
  font-size: 16px;
  display: flex;
  flex-direction: column;
}

.organization {
  height: -webkit-fill-available;
  width: -webkit-fill-available;
  background-color: cornsilk;
  color: black;
}
</style>
