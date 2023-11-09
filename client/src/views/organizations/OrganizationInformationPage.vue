<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <div
        class="org-info-container"
        v-if="orgInfo !== null && clientInfo !== null"
      >
        <div class="title">
          <h1>
            {{
              $t('OrganizationPage.infoPage.title', {
                organizationName: clientInfo.organizationId,
              })
            }}
          </h1>
        </div>

        <!-- Configuration list -->
        <ion-list :inset="true">
          <ion-list-header lines="full">
            <ion-title class="organization-info-title">
              {{ $t('OrganizationPage.infoPage.configuration.title') }}
            </ion-title>
          </ion-list-header>

          <!-- Outsider profile -->
          <ion-item class="organization-info">
            <ion-label class="organization-info-key">
              {{ $t('OrganizationPage.infoPage.configuration.outsidersAllowed') }}
            </ion-label>
            <ion-chip
              class="organization-info-value"
              :outline="true"
              :color="orgInfo.outsidersAllowed ? 'success' : 'warning'"
              slot="end"
            >
              {{
                orgInfo.outsidersAllowed
                  ? $t('OrganizationPage.infoPage.configuration.allowed')
                  : $t('OrganizationPage.infoPage.configuration.forbidden')
              }}
            </ion-chip>
          </ion-item>
          <!-- User limit -->
          <ion-item class="organization-info">
            <ion-label class="organization-info-key">
              {{ $t('OrganizationPage.infoPage.configuration.userLimit') }}
            </ion-label>
            <ion-chip
              class="organization-info-value"
              :outline="true"
              :color="orgInfo.userLimit === -1 ? 'success' : 'warning'"
              slot="end"
            >
              {{ orgInfo.userLimit === -1 ? $t('OrganizationPage.infoPage.configuration.unlimited') : orgInfo.userLimit }}
            </ion-chip>
          </ion-item>
          <!-- Backend addr -->
          <ion-item class="organization-info">
            <ion-label class="organization-info-key">
              {{ $t('OrganizationPage.infoPage.configuration.serverAddr') }}
            </ion-label>
            <ion-label
              class="organization-info-value"
              slot="end"
            >
              {{ clientInfo.organizationAddr }}
            </ion-label>
          </ion-item>
        </ion-list>

        <!-- Data size list -->
        <ion-list :inset="true">
          <ion-list-header lines="full">
            <ion-title class="organization-info-title">
              {{ $t('OrganizationPage.infoPage.size.title') }}
            </ion-title>
          </ion-list-header>
          <!-- Total -->
          <ion-item class="organization-info">
            <ion-label class="organization-info-key">
              {{ $t('OrganizationPage.infoPage.size.total') }}
            </ion-label>
            <ion-label
              class="organization-info-value"
              slot="end"
            >
              {{ fileSize(orgInfo.size.total) }}
            </ion-label>
          </ion-item>
          <!-- Meta data -->
          <ion-item class="organization-info">
            <ion-label class="organization-info-key">
              {{ $t('OrganizationPage.infoPage.size.metadata') }}
            </ion-label>
            <ion-label
              class="organization-info-value"
              slot="end"
            >
              {{ fileSize(orgInfo.size.metadata) }}
            </ion-label>
          </ion-item>
        </ion-list>

        <!-- User list -->
        <ion-list
          class="user-list"
          :inset="true"
        >
          <ion-list-header lines="full">
            <ion-title class="organization-info-title">
              {{ $t('OrganizationPage.infoPage.users.title') }}
            </ion-title>
          </ion-list-header>
          <!-- Active users -->
          <ion-item class="organization-info">
            <ion-chip color="success">
              {{ $t('OrganizationPage.infoPage.users.activeUsers') }}
            </ion-chip>
            <ion-label
              class="organization-info-value"
              slot="end"
            >
              {{ orgInfo.users.active }}
            </ion-label>
          </ion-item>
          <!-- Admins -->
          <ion-item class="organization-info">
            <ion-chip color="primary">
              {{ $t('OrganizationPage.infoPage.users.adminUsers') }}
            </ion-chip>
            <ion-label
              class="organization-info-value"
              slot="end"
            >
              {{ orgInfo.users.admins }}
            </ion-label>
          </ion-item>

          <!-- Standard -->
          <ion-item class="organization-info">
            <ion-chip color="secondary">
              {{ $t('OrganizationPage.infoPage.users.standardUsers') }}
            </ion-chip>
            <ion-label
              class="organization-info-value"
              slot="end"
            >
              {{ orgInfo.users.standards }}
            </ion-label>
          </ion-item>

          <!-- Outsiders if allowed -->
          <ion-item
            class="organization-info"
            v-if="orgInfo.outsidersAllowed"
          >
            <ion-chip color="tertiary">
              {{ $t('OrganizationPage.infoPage.users.outsiderUsers') }}
            </ion-chip>
            <ion-label
              class="organization-info-value"
              slot="end"
            >
              {{ orgInfo.users.outsiders }}
            </ion-label>
          </ion-item>

          <!-- Revoked -->
          <ion-item class="organization-info">
            <ion-chip color="danger">
              {{ $t('OrganizationPage.infoPage.users.revokedUsers') }}
            </ion-chip>
            <ion-label
              class="organization-info-value"
              slot="end"
            >
              {{ orgInfo.users.revoked }}
            </ion-label>
          </ion-item>
        </ion-list>
      </div>
      <div
        class="org-info-container"
        v-else
      >
        {{ $t('OrganizationPage.infoPage.getInfoFailed') }}
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { IonPage, IonContent, IonLabel, IonChip, IonItem, IonList, IonListHeader, IonTitle } from '@ionic/vue';
import { onMounted, ref, Ref, inject } from 'vue';
import { getClientInfo, getOrganizationInfo, OrganizationInfo, ClientInfo } from '@/parsec';
import { Formatters, FormattersKey } from '@/common/injectionKeys';

const orgInfo: Ref<OrganizationInfo | null> = ref(null);
const clientInfo: Ref<ClientInfo | null> = ref(null);
// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
const { fileSize } = inject(FormattersKey)! as Formatters;

onMounted(async () => {
  const orgResult = await getOrganizationInfo();
  const clientResult = await getClientInfo();

  if (!orgResult.ok || !clientResult.ok) {
    return;
  }
  orgInfo.value = orgResult.value;
  clientInfo.value = clientResult.value;
});
</script>

<style scoped lang="scss">
.organization-info-title {
  color: var(--parsec-color-light-primary-700);
  padding: 0;
}

.organization-info {
  display: flex;
  flex-direction: column;
}
.organization-info-key {
  color: var(--parsec-color-light-secondary-grey);
}

.organization-info-value {
  font-weight: 500;
  &:not(ion-chip) {
    color: var(--parsec-color-light-primary-800);
  }
}
.org-info-container {
  margin: 2em;
  background-color: white;
  width: 40em;
}

.title {
  display: flex;
  h1 {
    color: var(--parsec-color-light-primary-700);
  }
}
</style>
