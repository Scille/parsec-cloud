<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-list class="choose-server-page">
    <ion-item
      class="recap-item"
    >
      <organization-recap-item
        :title="t('CreateOrganization.overview.organization')"
        :text="organization"
      />
    </ion-item>
  </ion-list>
</template>

<script setup lang="ts">
import {
  IonList,
  IonItem,
} from '@ionic/vue';
import OrganizationRecapItem from '@/components/CreateOrganization/OrganizationRecapItem.vue';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();

defineProps<{
  organization: string,
}>();
</script>

<style scoped lang="scss">

</style>
