<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <ms-modal
      :title="$t('Changelog.title')"
      :close-button-enabled="true"
    >
      <div>
        <div
          v-for="versionChange in changes"
          :key="versionChange.version"
          class="container"
        >
          <div class="version">
            <ion-text class="version-title title-h3">
              {{ $t('app.name') }} v{{ versionChange.version }}
            </ion-text>
            <ion-text class="version-date body">
              {{ versionChange.date }}
            </ion-text>
          </div>

          <!-- Features -->
          <div
            v-show="versionChange.features.length > 0"
            class="categorie"
          >
            <ion-text class="categorie-title body-lg">
              <ion-icon
                :icon="sparkles"
              />
              {{ $t('Changelog.features') }}
            </ion-text>
            <ion-list class="list">
              <ion-item
                v-for="change in versionChange.features"
                :key="change.description"
                class="list-item"
              >
                <ion-text class="list-item__text">
                  {{ change.description }}
                </ion-text>
                <a
                  :href="$t('app.projectSources') + '/issues/' + change.issue"
                  target="_blank"
                  class="list-item__link body"
                  v-if="change.issue"
                >
                  #{{ change.issue }}
                </a>
              </ion-item>
            </ion-list>
          </div>

          <!-- Bugfixes -->
          <div
            v-show="versionChange.fixes.length > 0"
            class="categorie"
          >
            <ion-text class="categorie-title body-lg">
              <ion-icon
                :icon="construct"
              />
              {{ $t('Changelog.fixes') }}
            </ion-text>
            <ion-list class="list">
              <ion-item
                v-for="change in versionChange.fixes"
                :key="change.description"
                class="list-item"
              >
                <ion-text class="list-item__text">
                  {{ change.description }}
                </ion-text>
                <a
                  :href="$t('app.projectSources') + '/issues/' + change.issue"
                  target="_blank"
                  class="list-item__link body"
                  v-if="change.issue"
                >
                  #{{ change.issue }}
                </a>
              </ion-item>
            </ion-list>
          </div>

          <!-- Misc -->
          <div
            v-show="versionChange.misc.length > 0"
            class="categorie"
          >
            <ion-text class="categorie-title body-lg">
              <ion-icon
                :icon="infinite"
              />
              {{ $t('Changelog.misc') }}
            </ion-text>
            <ion-list class="list">
              <ion-item
                v-for="change in versionChange.misc"
                :key="change.description"
                class="list-item"
              >
                <ion-text class="list-item__text">
                  {{ change.description }}
                </ion-text>
                <a
                  :href="$t('app.projectSources') + '/issues/' + change.issue"
                  target="_blank"
                  class="list-item__link body"
                  v-if="change.issue"
                >
                  #{{ change.issue }}
                </a>
              </ion-item>
            </ion-list>
          </div>
        </div>
      </div>
    </ms-modal>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonPage,
  IonIcon,
  IonList,
  IonItem,
  IonText,
} from '@ionic/vue';
import {
  sparkles,
  construct,
  infinite,
} from 'ionicons/icons';
import { onMounted, ref, Ref } from 'vue';
import { getChanges, VersionChange } from '@/common/mocks';
import MsModal from '@/components/core/ms-modal/MsModal.vue';

const changes: Ref<VersionChange[]> = ref([]);

onMounted(() => {
  changes.value = getChanges();
});
</script>

<style scoped lang="scss">
.modal {
  padding: 2.5rem;
  height: 40em;
}

.version {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: var(--parsec-color-light-primary-50);
  border-radius: var(--parsec-radius-4);
  padding: 0.75rem 1rem 0.75rem 0.5rem;
  margin-bottom: 2rem;
  position: sticky;
  top: 0;
  z-index: 10;

  .version-title {
    color: var(--parsec-color-light-primary-700);
    margin-bottom: 0;
    font-size: 1.1rem;
  }

  .version-date {
    color: var(--parsec-color-light-secondary-text);
  }
}

.categorie {
  margin-bottom: 2.5rem;
}

.categorie-title {
  display: flex;
  align-items: center;
  gap: .5rem;
  padding: 0.5rem 0.5rem 0.5rem 0.5rem;
  margin-bottom: 0.5rem;
  color: var(--parsec-color-light-primary-700);
  background: var(--parsec-color-light-secondary-inversed-contrast);
  position: sticky;
  top: 3rem;
  z-index: 9;

  &::after {
    content: '';
    position: absolute;
    bottom: 0;
    width: 100%;
    height: 1px;
    background: var(--parsec-color-light-secondary-disabled);
  }
}

.list {
  display: flex;
  width: 100%;
  flex-direction: column;
  justify-content: space-between;

  .list-item {
    display: flex;
    --inner-padding-end: 0;
    --padding-start: 0;
    background: transparent;
    position: relative;
    color: var(--parsec-color-light-secondary-text);
    --min-height: 40px;

    &:nth-child(odd) {
      --background: var(--parsec-color-light-secondary-background);
      background: var(--parsec-color-light-secondary-background);
      border-radius: var(--parsec-radius-4);
    }

    &::before {
      content: '';
      display: block;
      position: relative;
      width: 6px;
      height: 6px;
      background: var(--parsec-color-light-secondary-text);
      margin: 0 10px 0 1rem;
      border-radius: var(--parsec-radius-circle);
      z-index: 3;
    }

    &__text {
      padding: .4rem 0;
    }

    &__link {
      margin-left: auto;
      margin-right: 1rem;
      color: var(--parsec-color-light-primary-600);
      text-decoration: none;

      &:hover {
        text-decoration: underline;
      }
    }
  }
}
</style>
