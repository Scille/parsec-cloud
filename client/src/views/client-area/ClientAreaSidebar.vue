<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div class="sidebar-content">
    <ms-dropdown
      class="dropdown"
      :options="organizationsOptions"
      :default-option-key="organization.bmsId"
      :label="I18n.valueAsTranslatable(organization.name)"
      @change="onOrganizationSelected"
    />
    <ul>
      <li :class="{ 'current-page': currentPage === ClientAreaPages.Summary }">
        <ion-button @click="goToPageClicked(ClientAreaPages.Summary)">Summary</ion-button>
      </li>
      <li :class="{ 'current-page': currentPage === ClientAreaPages.Stats }">
        <ion-button @click="goToPageClicked(ClientAreaPages.Stats)">Stats</ion-button>
      </li>
      <li :class="{ 'current-page': currentPage === ClientAreaPages.Invoices }">
        <ion-button @click="goToPageClicked(ClientAreaPages.Invoices)">Invoices</ion-button>
      </li>
      <li :class="{ 'current-page': currentPage === ClientAreaPages.Payment }">
        <ion-button @click="goToPageClicked(ClientAreaPages.Payment)">Paiement</ion-button>
      </li>
      <li :class="{ 'current-page': currentPage === ClientAreaPages.Billing }">
        <ion-button @click="goToPageClicked(ClientAreaPages.Billing)">Billing</ion-button>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { BmsAccessInstance, BmsOrganization, DataType } from '@/services/bms';
import { IonButton } from '@ionic/vue';
import { ClientAreaPages } from '@/views/client-area/types';
import { MsDropdown, MsDropdownChangeEvent, MsOptions, I18n } from 'megashark-lib';
import { ref, onMounted, computed } from 'vue';

const props = defineProps<{
  organization: BmsOrganization;
  currentPage: ClientAreaPages;
}>();

const organizations = ref<Array<BmsOrganization>>([]);
const organizationsOptions = computed(() => {
  return new MsOptions(
    organizations.value.map((org) => {
      return {
        key: org.bmsId,
        label: I18n.valueAsTranslatable(org.name),
      };
    }),
  );
});

const emits = defineEmits<{
  (e: 'pageSelected', page: ClientAreaPages): void;
  (e: 'organizationSelected', organization: BmsOrganization): void;
}>();

onMounted(async () => {
  const result = await BmsAccessInstance.get().listOrganizations();
  if (!result.isError && result.data && result.data.type === DataType.ListOrganizations) {
    organizations.value = result.data.organizations;
  } else {
    organizations.value.push(props.organization);
  }
});

async function onOrganizationSelected(event: MsDropdownChangeEvent): Promise<void> {
  const org = organizations.value.find((o) => o.bmsId === event.option.key);

  if (org && org.bmsId !== props.organization.bmsId) {
    emits('organizationSelected', org);
  }
}

async function goToPageClicked(page: ClientAreaPages): Promise<void> {
  emits('pageSelected', page);
}
</script>

<style scoped lang="scss">
.sidebar-content {
  background-color: green;
  max-width: 16em;
  width: 100%;
  height: 100%;
}

.current-page {
  border: orange;
  color: orange;
  background-color: cyan;
}
</style>
