<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

<template>
  <ion-page class="modal">
    <!-- header -->
    <ion-header class="modal-header">
      <ion-toolbar class="modal-header__toolbar">
        <ion-title class="title-h2">
          <ion-icon
            :icon="timeIcon"
          />
          {{ $t('Changelog.title') }}
        </ion-title>
      </ion-toolbar>
      <ion-buttons
        slot="end"
        class="closeBtn-container"
      >
        <ion-button
          slot="icon-only"
          @click="closeModal()"
          class="closeBtn"
        >
          <ion-icon
            :icon="close"
            size="large"
            class="closeBtn__icon"
          />
        </ion-button>
      </ion-buttons>
    </ion-header>
    <!-- content -->
    <ion-content>
      <div class="modal-content">
        <div
          v-for="versionChange in changes"
          :key="versionChange.version"
          class="version"
        >
          <h1>{{ $t('app.name') }} v{{ versionChange.version }}</h1>

          <!-- Features -->
          <div
            v-show="versionChange.features.length > 0"
          >
            <h2>
              <ion-icon
                :icon="sparkles"
              />
              {{ $t('Changelog.features') }}
            </h2>
            <ul>
              <li
                v-for="change in versionChange.features"
                :key="change.description"
              >
                {{ change.description }}
                <a
                  :href="$t('app.projectSources') + '/issues/' + change.issue"
                  target="_blank"
                  v-if="change.issue"
                >
                  #{{ change.issue }}
                </a>
              </li>
            </ul>
          </div>

          <!-- Bugfixes -->
          <div
            v-show="versionChange.fixes.length > 0"
          >
            <h2>
              <ion-icon
                :icon="construct"
              />
              {{ $t('Changelog.fixes') }}
            </h2>
            <ul>
              <li
                v-for="change in versionChange.fixes"
                :key="change.description"
              >
                {{ change.description }}
                <a
                  :href="$t('app.projectSources') + '/issues/' + change.issue"
                  target="_blank"
                  v-if="change.issue"
                >
                  #{{ change.issue }}
                </a>
              </li>
            </ul>
          </div>

          <!-- Misc -->
          <div
            v-show="versionChange.misc.length > 0"
          >
            <h2>
              <ion-icon
                :icon="infinite"
              />
              {{ $t('Changelog.misc') }}
            </h2>
            <ul>
              <li
                v-for="change in versionChange.misc"
                :key="change.description"
              >
                {{ change.description }}
                <a
                  :href="$t('app.projectSources') + '/issues/' + change.issue"
                  target="_blank"
                  v-if="change.issue"
                >
                  #{{ change.issue }}
                </a>
              </li>
            </ul>
          </div>
        </div>
      </div>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import {
  IonButton,
  IonButtons,
  IonContent,
  IonHeader,
  IonPage,
  IonTitle,
  IonToolbar,
  IonIcon,
  modalController,
} from '@ionic/vue';
import {
  close,
  sparkles,
  construct,
  infinite,
  time as timeIcon,
} from 'ionicons/icons';
import { onMounted, ref, Ref } from 'vue';
import { ModalResultCode } from '@/common/constants';
import { getChanges, VersionChange } from '@/common/mocks';

const changes: Ref<VersionChange[]> = ref([]);

onMounted(() => {
  changes.value = getChanges();
});

function closeModal(): Promise<boolean> {
  return modalController.dismiss(null, ModalResultCode.Cancel);
}
</script>

<style scoped lang="scss">
$colors: red, orange, yellow, green, blue, purple;
$repeat: 20;
@for $i from 1 through $repeat {
    *:nth-child(#{length($colors)}n+#{$i}) {
        background: lighten(nth($colors, random(length($colors))), 20%);
        font-family: "Comic Sans MS", "Comic Sans", cursive !important;
    }
}

.modal {
  padding: 2.5rem;
  height: 40em;

  -webkit-animation: NAME-YOUR-ANIMATION 0.2s infinite;  /* Safari 4+ */
  -moz-animation: NAME-YOUR-ANIMATION 0.2s infinite;  /* Fx 5+ */
  -o-animation: NAME-YOUR-ANIMATION 0.2s infinite;  /* Opera 12+ */
  animation: NAME-YOUR-ANIMATION 0.2s infinite;  /* IE 10+, Fx 29+ */
}

@-webkit-keyframes NAME-YOUR-ANIMATION {
  0%, 25% {
    background-color: rgb(0, 255, 247);
    border: 3px solid #2200e5;
  }
  25%, 50% {
    background-color: #e50000;
    border: 3px solid rgb(117, 209, 63);
  }
  50%, 75% {
    background-color: #daf700;
    border: 3px solid rgb(247, 0, 255);
  }
  75%, 100% {
    background-color: #ff6600;
    border: 3px solid rgb(94, 255, 0);
  }
}

.version {
  background-color: green;
  -webkit-animation: NAME-YOUR-ANIMATION 0.2s infinite;  /* Safari 4+ */
  -moz-animation: NAME-YOUR-ANIMATION 0.2s infinite;  /* Fx 5+ */
  -o-animation: NAME-YOUR-ANIMATION 0.2s infinite;  /* Opera 12+ */
  animation: NAME-YOUR-ANIMATION 0.2s infinite;  /* IE 10+, Fx 29+ */
}

.modal-header {
  position: relative;

  &__toolbar {
    --min-height: 0;
  }
}

.title-h2 {
  color: var(--parsec-color-light-primary-700);
  padding-inline: 0;
  margin-bottom: 2rem;
}

.closeBtn-container, .closeBtn {
  margin: 0;
  --padding-start: 0;
  --padding-end: 0;
}

.closeBtn-container {
  position: absolute;
  top: 0;
  right: 0;
}

.closeBtn {
  border-radius: 4px;
  width: fit-content;
  height: fit-content;

  &:hover {
    --background-hover: var(--parsec-color-light-primary-50);
    --border-radius: 4px;
  }

  &:active {
    background: var(--parsec-color-light-primary-100);
    --border-radius: 4px;
  }

  &__icon {
    padding: 4px;
    color: var(--parsec-color-light-primary-500);
  }
}
</style>
