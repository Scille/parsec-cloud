<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="summary-page">
    <create-organization-modal-header
      @close-clicked="$emit('closeRequested')"
      title="CreateOrganization.title.overview"
      subtitle="CreateOrganization.subtitle.overview"
    />

    <ion-list class="summary-list">
      <!-- organization name -->
      <ion-item class="ion-no-padding">
        <div class="summary-item">
          <ion-label class="summary-item__label subtitles-sm">
            {{ $msTranslate('CreateOrganization.overview.organization') }}
          </ion-label>
          <ion-text class="summary-item__text body">
            {{ organizationName }}
          </ion-text>
          <ion-button
            v-show="canEditOrganizationName"
            fill="clear"
            class="summary-item__button"
            @click="$emit('updateOrganizationNameClicked')"
          >
            {{ $msTranslate('CreateOrganization.button.modify') }}
          </ion-button>
        </div>
      </ion-item>

      <!-- full name -->
      <ion-item class="ion-no-padding">
        <div class="summary-item">
          <ion-label class="summary-item__label subtitles-sm">
            {{ $msTranslate('CreateOrganization.overview.fullname') }}
          </ion-label>
          <ion-text class="summary-item__text body">
            {{ name }}
          </ion-text>
          <ion-button
            fill="clear"
            class="summary-item__button"
            @click="$emit('updateNameClicked')"
            v-show="canEditName"
          >
            {{ $msTranslate('CreateOrganization.button.modify') }}
          </ion-button>
        </div>
      </ion-item>

      <!-- Email -->
      <ion-item class="ion-no-padding">
        <div class="summary-item">
          <ion-label class="summary-item__label subtitles-sm">
            {{ $msTranslate('CreateOrganization.overview.email') }}
          </ion-label>
          <ion-text class="summary-item__text body">
            {{ email }}
          </ion-text>
          <ion-button
            fill="clear"
            class="summary-item__button"
            @click="$emit('updateEmailClicked')"
            v-show="canEditEmail"
          >
            {{ $msTranslate('CreateOrganization.button.modify') }}
          </ion-button>
        </div>
      </ion-item>

      <!-- serverMode -->
      <ion-item class="ion-no-padding">
        <div class="summary-item">
          <ion-label class="summary-item__label subtitles-sm">
            {{ $msTranslate('CreateOrganization.overview.server') }}
          </ion-label>
          <ion-text class="summary-item__text body">
            {{ serverType === ServerType.Saas ? $msTranslate('CreateOrganization.saas') : $msTranslate('CreateOrganization.customServer') }}
          </ion-text>
        </div>
      </ion-item>

      <!-- authentication mode -->
      <ion-item class="ion-no-padding">
        <div class="summary-item">
          <ion-label class="summary-item__label subtitles-sm">
            {{ $msTranslate('CreateOrganization.overview.authentication') }}
          </ion-label>
          <ion-text class="summary-item__text body">
            {{
              saveStrategy === DeviceSaveStrategyTag.Keyring
                ? $msTranslate('CreateOrganization.keyringChoice')
                : $msTranslate('CreateOrganization.passwordChoice')
            }}
          </ion-text>
          <ion-button
            fill="clear"
            class="summary-item__button"
            v-show="canEditSaveStrategy"
            @click="$emit('updateSaveStrategyClicked')"
          >
            {{ $msTranslate('CreateOrganization.button.modify') }}
          </ion-button>
        </div>
      </ion-item>
    </ion-list>
    <p v-show="error">
      {{ $msTranslate(error) }}
    </p>

    <ion-footer class="summary-page-footer">
      <ion-buttons
        slot="primary"
        class="modal-footer-buttons"
      >
        <ion-button
          fill="clear"
          size="default"
          id="previous-button"
          @click="$emit('goBackRequested')"
        >
          {{ $msTranslate('CreateOrganization.button.previous') }}
          <ion-icon
            slot="start"
            :icon="chevronBack"
            size="small"
          />
        </ion-button>

        <ion-button
          fill="solid"
          size="default"
          @click="$emit('createClicked')"
        >
          <span>
            {{ $msTranslate('CreateOrganization.button.create') }}
          </span>
          <ion-icon
            slot="start"
            :icon="chevronForward"
            size="small"
          />
        </ion-button>
      </ion-buttons>
    </ion-footer>
  </ion-page>
</template>

<script setup lang="ts">
import { DeviceSaveStrategyTag, OrganizationID } from '@/parsec';
import { ServerType } from '@/services/parsecServers';
import { chevronForward, chevronBack } from 'ionicons/icons';
import { IonPage, IonItem, IonButton, IonText, IonLabel, IonButtons, IonIcon, IonFooter, IonList } from '@ionic/vue';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';
import { Translatable } from 'megashark-lib';

defineProps<{
  serverType: ServerType;
  organizationName: OrganizationID;
  email: string;
  name: string;
  saveStrategy: DeviceSaveStrategyTag;
  canEditOrganizationName?: boolean;
  canEditEmail?: boolean;
  canEditName?: boolean;
  canEditSaveStrategy?: boolean;
  error?: Translatable;
}>();

defineEmits<{
  (e: 'createClicked'): void;
  (e: 'updateOrganizationNameClicked'): void;
  (e: 'updateNameClicked'): void;
  (e: 'updateEmailClicked'): void;
  (e: 'updateSaveStrategyClicked'): void;
  (e: 'closeRequested'): void;
  (e: 'goBackRequested'): void;
}>();
</script>

<style scoped lang="scss">
.summary-page {
  padding: 2.5rem;
  display: flex;
  height: auto;
  width: 100%;

  &-footer {
    display: flex;
    justify-content: space-between;
    margin-top: 2.5rem;
  }
}

.summary-list {
  padding: 0;
  display: flex;
  flex-direction: column;
}

.summary-item {
  padding: 0.75rem 0;
  display: flex;
  align-items: center;
  flex: 1;
  position: relative;
  gap: 1rem;
  width: -webkit-fill-available;

  &::after {
    content: '';
    position: absolute;
    width: 100%;
    height: 1px;
    bottom: 0;
    left: 7.5rem;
    background: var(--parsec-color-light-secondary-disabled);
    z-index: 2;
  }

  &__label {
    min-width: 8rem;
    color: var(--parsec-color-light-secondary-grey);
  }

  &__text {
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--parsec-color-light-secondary-text);
  }
  &__button {
    margin-left: auto;

    &::part(native) {
      padding: 0.5rem;
    }
  }
}
</style>
