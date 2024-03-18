<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content :fullscreen="true">
      <template v-if="orgInfo">
        <div class="org-info-container">
          <!-- ------------- Configuration list ------------- -->
          <div class="org-config">
            <ion-title class="org-info-title title-h3">
              {{ $t('OrganizationPage.infoPage.configuration.title') }}
            </ion-title>

            <div class="org-config-list">
              <!-- Outsider profile -->
              <div class="org-config-list-item">
                <ion-label class="org-info-item-title body">
                  {{ $t('OrganizationPage.infoPage.configuration.outsidersAllowed') }}
                </ion-label>
                <div
                  class="org-config-list-item__value body-sm"
                  :class="orgInfo.outsidersAllowed ? 'success' : 'warning'"
                >
                  {{
                    orgInfo.outsidersAllowed
                      ? $t('OrganizationPage.infoPage.configuration.allowed')
                      : $t('OrganizationPage.infoPage.configuration.forbidden')
                  }}
                </div>
              </div>
              <!-- User limit -->
              <div class="org-config-list-item">
                <ion-text class="org-info-item-title body">
                  {{ $t('OrganizationPage.infoPage.configuration.userLimit') }}
                </ion-text>
                <div
                  class="org-config-list-item__value body-sm"
                  :class="orgInfo.hasUserLimit ? 'warning' : 'success'"
                >
                  {{ orgInfo.hasUserLimit ? orgInfo.userLimit : $t('OrganizationPage.infoPage.configuration.unlimited') }}
                </div>
              </div>
              <!-- Backend addr -->
              <div class="org-config-list-item server-address">
                <ion-text class="org-info-item-title server-address__title body">
                  {{ $t('OrganizationPage.infoPage.configuration.serverAddr') }}
                </ion-text>
                <div class="server-address-value">
                  <ion-text class="server-address-value__text body">
                    {{ orgInfo.organizationAddr }}
                  </ion-text>
                  <ion-button
                    fill="clear"
                    size="small"
                    id="copy-link-btn"
                    @click="copyAddress(orgInfo.organizationAddr)"
                    v-if="!addressCopiedToClipboard"
                  >
                    <ion-icon
                      class="icon-copy"
                      :icon="copy"
                    />
                  </ion-button>
                  <ion-text
                    v-if="addressCopiedToClipboard"
                    class="server-address-value__copied body copied"
                  >
                    {{ $t('OrganizationPage.infoPage.configuration.copyPath') }}
                  </ion-text>
                </div>
              </div>
            </div>
          </div>

          <!-- ------------- Storage list ------------- -->
          <div
            class="org-storage"
            v-show="false"
          >
            <ion-title class="org-info-title title-h3">
              {{ $t('OrganizationPage.infoPage.size.title') }}
            </ion-title>
            <div class="org-storage-list">
              <div class="org-storage-list-item">
                <ion-label class="org-info-item-title body">
                  {{ $t('OrganizationPage.infoPage.size.total') }}
                </ion-label>
                <ion-text
                  class="org-storage-list-item__value title-h5"
                  slot="end"
                  v-show="orgInfo.size.data + orgInfo.size.metadata > 0"
                >
                  {{ formatFileSize(orgInfo.size.data + orgInfo.size.metadata) }}
                </ion-text>
                <div
                  v-show="orgInfo.size.data + orgInfo.size.metadata === 0"
                  class="warning body-sm"
                >
                  {{ $t('OrganizationPage.infoPage.size.unavailable') }}
                </div>
              </div>
              <!-- Meta data -->
              <div class="org-storage-list-item">
                <ion-label class="org-info-item-title body">
                  {{ $t('OrganizationPage.infoPage.size.metadata') }}
                </ion-label>
                <ion-text
                  class="org-storage-list-item__value title-h5"
                  v-show="orgInfo.size.metadata > 0"
                >
                  {{ formatFileSize(orgInfo.size.metadata) }}
                </ion-text>
                <div
                  v-show="orgInfo.size.metadata === 0"
                  class="warning body-sm"
                >
                  {{ $t('OrganizationPage.infoPage.size.unavailable') }}
                </div>
              </div>
            </div>
          </div>

          <!-- ------------- User list ------------- -->
          <div class="org-user">
            <ion-title class="org-info-title title-h3">
              {{ $t('OrganizationPage.infoPage.users.title') }}
            </ion-title>

            <div class="org-user-list">
              <!-- Active users -->
              <div class="org-user-list-item user-active">
                <div class="user-active-header">
                  <ion-title class="user-active-header__title title-h5">
                    {{ $t('OrganizationPage.infoPage.users.activeUsers') }}
                  </ion-title>
                  <span class="title-h4">{{ orgInfo.users.active }}</span>
                  <!-- if we decide to show a modal with the active users list -->
                  <ion-button
                    fill="clear"
                    size="small"
                    class="show-users-button"
                    v-show="false"
                  >
                    {{ $t('OrganizationPage.infoPage.users.seeUsers') }}
                  </ion-button>
                </div>
                <div class="user-active-list">
                  <!-- Admin -->
                  <div class="user-active-list-item">
                    <tag-profile :profile="UserProfile.Admin" />
                    <ion-text class="user-active-list-item__value title-h4">
                      {{ orgInfo.users.admins }}
                    </ion-text>
                  </div>
                  <!-- Standard -->
                  <div class="user-active-list-item">
                    <tag-profile :profile="UserProfile.Standard" />
                    <ion-text class="user-active-list-item__value title-h4">
                      {{ orgInfo.users.standards }}
                    </ion-text>
                  </div>
                  <!-- Outsiders if allowed -->
                  <div
                    v-if="orgInfo.outsidersAllowed"
                    class="user-active-list-item"
                  >
                    <tag-profile :profile="UserProfile.Outsider" />
                    <ion-text class="user-active-list-item__value title-h4">
                      {{ orgInfo.users.outsiders }}
                    </ion-text>
                  </div>
                </div>
              </div>

              <!-- Revoked -->
              <div class="org-user-list-item user-revoked">
                <div class="user-revoked-header">
                  <ion-title class="user-revoked-header__title title-h5">
                    {{ $t('OrganizationPage.infoPage.users.revokedUsers') }}
                  </ion-title>
                  <span class="title-h4">{{ orgInfo.users.revoked }}</span>
                  <!-- if we decide to show a modal with the revoked users list -->
                  <ion-button
                    fill="clear"
                    size="small"
                    class="show-users-button"
                    v-show="false"
                  >
                    {{ $t('OrganizationPage.infoPage.users.seeUsers') }}
                  </ion-button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </template>
      <template v-else>
        <div class="information-not-found">
          <ion-icon
            :icon="warning"
            size="large"
          />
          <ion-text class="body">
            {{ $t('OrganizationPage.infoPage.getInfoFailed') }}
          </ion-text>
        </div>
      </template>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { writeTextToClipboard } from '@/common/clipboard';
import { formatFileSize } from '@/common/file';
import TagProfile from '@/components/users/TagProfile.vue';
import { OrganizationInfo, UserProfile, getOrganizationInfo } from '@/parsec';
import { Information, InformationLevel, InformationManager, InformationManagerKey, PresentationMode } from '@/services/informationManager';
import { translate } from '@/services/translation';
import { IonButton, IonContent, IonIcon, IonLabel, IonPage, IonText, IonTitle } from '@ionic/vue';
import { copy, warning } from 'ionicons/icons';
import { Ref, inject, onMounted, ref } from 'vue';

const orgInfo: Ref<OrganizationInfo | null> = ref(null);
const informationManager: InformationManager = inject(InformationManagerKey)!;
const addressCopiedToClipboard = ref(false);

async function copyAddress(address: string): Promise<void> {
  const result = await writeTextToClipboard(address);
  if (result) {
    addressCopiedToClipboard.value = true;
    setTimeout(() => {
      addressCopiedToClipboard.value = false;
    }, 5000);
  } else {
    informationManager.present(
      new Information({
        message: translate('OrganizationPage.infoPage.configuration.copyFailed'),
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
}

onMounted(async () => {
  const result = await getOrganizationInfo();

  if (!result.ok) {
    return;
  }
  orgInfo.value = result.value;
});
</script>

<style scoped lang="scss">
// global styles
ion-label,
ion-title {
  margin: 0;
  padding: 0;
}

.org-info-container {
  margin: 2em;
  width: 80%;
  display: flex;
  flex-wrap: wrap;
  justify-content: start;
  gap: 4rem;

  .org-config,
  .org-storage,
  .org-user {
    display: flex;
    flex-direction: column;
    align-self: baseline;
    flex-wrap: wrap;
    gap: 1.5rem;
    position: relative;
    min-width: 24em;

    .org-info-title {
      color: var(--parsec-color-light-primary-800);
      padding: 0;
    }

    .org-info-item-title {
      color: var(--parsec-color-light-secondary-grey);
    }

    .success {
      color: var(--parsec-color-light-success-700);
      padding: 0.125rem 0.365rem;
      border-radius: var(--parsec-radius-32);
      background: var(--parsec-color-light-success-100);
      border: 1px solid var(--parsec-color-light-success-500);
    }

    .warning {
      color: var(--parsec-color-light-warning-700);
      padding: 0.125rem 0.365rem;
      border-radius: var(--parsec-radius-32);
      background: var(--parsec-color-light-warning-100);
      border: 1px solid var(--parsec-color-light-warning-500);
    }

    // &::after {
    //   content: '';
    //   position: absolute;
    //   width: 100%;
    //   height: 1px;
    //   bottom: -2rem;
    //   background: var(--parsec-color-light-secondary-disabled);
    // }
  }
}

// ----- Configuration
.org-config {
  &-list {
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 1rem;

    &-item {
      justify-content: space-between;
      display: flex;
      align-items: center;
    }
  }

  .server-address {
    max-width: 24em;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    align-items: start;

    &-value {
      color: var(--parsec-color-light-secondary-text);
      background-color: var(--parsec-color-light-secondary-background);
      border-radius: var(--parsec-radius-6);
      padding: 0.5rem 1rem;
      display: flex;
      align-items: center;
      justify-content: space-between;
      gap: 0.5rem;
      overflow: hidden;
      width: 100%;

      &__text {
        padding: 0.25rem 0;
        text-overflow: ellipsis;
        white-space: nowrap;
        overflow: hidden;
      }

      &__copied {
        color: var(--parsec-color-light-success-700);
      }
    }

    #copy-link-btn {
      color: var(--parsec-color-light-secondary-text);
      margin: 0;

      &::part(native) {
        padding: 0.5rem;
        border-radius: var(--parsec-radius-6);
      }

      &:hover {
        color: var(--parsec-color-light-primary-600);
      }
    }
  }
}

// ----- Storage
.org-storage {
  &-list {
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 1rem;

    &-item {
      justify-content: space-between;
      display: flex;
      align-items: center;

      &__value {
        color: var(--parsec-color-light-secondary-text);
      }
    }
  }
}

// ----- Users
.org-user {
  &-list {
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 1rem;

    &-item {
      justify-content: space-between;
      display: flex;
      border: 1px solid var(--parsec-color-light-secondary-medium);
      border-radius: var(--parsec-radius-6);
    }
  }

  .user-active,
  .user-revoked {
    display: flex;
    flex-direction: column;

    &-header {
      display: flex;
      align-items: center;
      padding: 1rem;
      gap: 0.5rem;

      &__title {
        color: var(--parsec-color-light-primary-800);
        flex: 0;
      }

      // style for the button if we decide to show a modal with the users list
      // .show-users-button {
      //   margin: 0;
      //   margin-left: auto;
      //   position: relative;
      //   width: fit-content;
      //   color: var(--parsec-color-light-secondary-grey);

      //   &::part(native) {
      //     --background-hover: none;
      //     padding: 0 0 0 2px;
      //   }

      //   &::after {
      //     content: '';
      //     position: absolute;
      //     bottom: -4px;
      //     left: 1px;
      //     width: 100%;
      //     height: 1px;
      //     background: var(--parsec-color-light-secondary-grey);
      //   }

      //   &:hover {
      //     color: var(--parsec-color-light-primary-500);

      //     &::after {
      //       background: var(--parsec-color-light-primary-500);
      //     }
      //   }
      // }
    }
  }

  .user-active {
    span {
      color: var(--parsec-color-light-primary-600);
    }
    &-list {
      display: flex;
      gap: 1rem;
      justify-content: space-evenly;
      background: var(--parsec-color-light-secondary-background);
      padding: 1.5rem 0 1rem;

      &-item {
        display: flex;
        align-items: center;
        flex-direction: column;
        gap: 0.25rem;
      }
    }
  }

  .user-revoked {
    span {
      color: var(--parsec-color-light-danger-500);
    }
  }
}

// ----- Information not found
.information-not-found {
  margin: 2rem;
  max-width: fit-content;
  background: var(--parsec-color-light-danger-100);
  color: var(--parsec-color-light-danger-700);
  padding: 1rem;
  display: flex;
  gap: 0.75rem;
  border-left: 0.25rem solid var(--parsec-color-light-danger-500);

  ion-text {
    padding: 0.25rem 0;
  }
}
</style>
