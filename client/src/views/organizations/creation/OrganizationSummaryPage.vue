<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="summary-page page-modal-container">
    <create-organization-modal-header
      @close-clicked="$emit('closeRequested')"
      title="CreateOrganization.title.overview"
      subtitle="CreateOrganization.subtitle.overview"
    />

    <ion-list class="summary-list">
      <!-- organization name -->
      <ion-item class="summary-item-container ion-no-padding">
        <div class="summary-item">
          <ion-text class="summary-item__label subtitles-sm">
            {{ $msTranslate('CreateOrganization.overview.organization') }}
          </ion-text>
          <ion-text class="summary-item__text body-lg">
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

      <hr class="summary-item-divider" />

      <!-- full name -->
      <ion-item class="summary-item-container ion-no-padding">
        <div class="summary-item">
          <ion-text class="summary-item__label subtitles-sm">
            {{ $msTranslate('CreateOrganization.overview.fullname') }}
          </ion-text>
          <ion-text class="summary-item__text body-lg">
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

      <hr class="summary-item-divider" />

      <!-- Email -->
      <ion-item class="summary-item-container ion-no-padding">
        <div class="summary-item">
          <ion-text class="summary-item__label subtitles-sm">
            {{ $msTranslate('CreateOrganization.overview.email') }}
          </ion-text>
          <ion-text class="summary-item__text body-lg">
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

      <hr class="summary-item-divider" />

      <!-- serverMode -->
      <ion-item class="summary-item-container ion-no-padding">
        <div class="summary-item server">
          <ion-text class="summary-item__label subtitles-sm">
            {{ $msTranslate('CreateOrganization.overview.server') }}
          </ion-text>
          <ion-text class="summary-item__text body-lg">
            {{ serverType === ServerType.Saas ? $msTranslate('CreateOrganization.saas') : $msTranslate('CreateOrganization.customServer') }}
          </ion-text>
          <ion-button
            v-show="canEditServerAddress"
            fill="clear"
            class="summary-item__button"
            @click="$emit('updateServerAddressClicked')"
          >
            {{ $msTranslate('CreateOrganization.button.modify') }}
          </ion-button>
        </div>
      </ion-item>

      <hr class="summary-item-divider" />

      <!-- authentication mode -->
      <ion-item class="summary-item-container ion-no-padding">
        <div class="summary-item authentication">
          <ion-text class="summary-item__label subtitles-sm">
            {{ $msTranslate('CreateOrganization.overview.authentication') }}
          </ion-text>
          <ion-text class="summary-item__text body-lg">
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

    <div
      class="tos"
      v-show="showTos"
    >
      <ion-text class="body-sm item-radio__text ion-text-wrap">
        {{ $msTranslate('CreateOrganization.acceptTOS.label') }}
        <a
          class="link"
          target="_blank"
          @click="$event.stopPropagation()"
          :href="$msTranslate('CreateOrganization.acceptTOS.tosLink')"
        >
          {{ $msTranslate('CreateOrganization.acceptTOS.tos') }}
        </a>
        {{ $msTranslate('CreateOrganization.acceptTOS.and') }}
        <a
          class="link"
          target="_blank"
          @click="$event.stopPropagation()"
          :href="$msTranslate('CreateOrganization.acceptTOS.privacyPolicyLink')"
        >
          {{ $msTranslate('CreateOrganization.acceptTOS.privacyPolicy') }}
        </a>
      </ion-text>
    </div>

    <!-- error -->
    <ion-text
      class="form-error body login-button-error"
      v-show="error"
    >
      <ion-icon
        class="form-error-icon"
        :icon="warning"
      />{{ $msTranslate(error) }}
    </ion-text>

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
import { chevronForward, chevronBack, warning } from 'ionicons/icons';
import { IonPage, IonItem, IonButton, IonText, IonButtons, IonIcon, IonFooter, IonList } from '@ionic/vue';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';
import { Translatable } from 'megashark-lib';

defineProps<{
  serverType: ServerType;
  organizationName: OrganizationID;
  email: string;
  name: string;
  saveStrategy: DeviceSaveStrategyTag;
  canEditOrganizationName?: boolean;
  canEditServerAddress?: boolean;
  canEditEmail?: boolean;
  canEditName?: boolean;
  canEditSaveStrategy?: boolean;
  error?: Translatable;
  showTos?: boolean;
}>();

defineEmits<{
  (e: 'createClicked'): void;
  (e: 'updateOrganizationNameClicked'): void;
  (e: 'updateServerAddressClicked'): void;
  (e: 'updateNameClicked'): void;
  (e: 'updateEmailClicked'): void;
  (e: 'updateSaveStrategyClicked'): void;
  (e: 'closeRequested'): void;
  (e: 'goBackRequested'): void;
}>();
</script>

<style scoped lang="scss">
.summary-page {
  overflow: auto;
}

.summary-list {
  padding: 0;
  display: flex;
  flex-direction: column;
  border-radius: var(--parsec-radius-8);
  border: 1px solid var(--parsec-color-light-secondary-medium);

  @include ms.responsive-breakpoint('sm') {
    margin: 0.5rem 2rem;
    border: none;
    border-radius: 0;
    gap: 0.75rem;
  }

  @include ms.responsive-breakpoint('xs') {
    margin: 0.5rem 1.5rem;
  }
}

.summary-item-container {
  --inner-padding-end: 0px;
}

.summary-item {
  display: flex;
  align-items: stretch;
  width: 100%;
  position: relative;
  gap: 1rem;
  padding-right: 1rem;

  @include ms.responsive-breakpoint('sm') {
    flex-direction: column;
    align-items: stretch;
    gap: 0;
    padding-right: 0;
    border-radius: var(--parsec-radius-8);
    overflow: hidden;
    border: 1px solid var(--parsec-color-light-secondary-premiere);
  }

  &-divider {
    width: 100%;
    height: 1px;
    background: var(--parsec-color-light-secondary-medium);
    z-index: 2;
    margin: 0;

    @include ms.responsive-breakpoint('sm') {
      display: none;
    }
  }

  &__label {
    min-width: 8rem;
    background: var(--parsec-color-light-secondary-premiere);
    color: var(--parsec-color-light-secondary-grey);
    align-items: center;
    padding: 0 0.5rem;
    display: flex;

    @include ms.responsive-breakpoint('sm') {
      padding: 0.125rem 0.75rem;
    }
  }

  &__text {
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: var(--parsec-color-light-secondary-text);
    background: var(--parsec-color-light-secondary-white);
    padding: 1rem 0;

    @include ms.responsive-breakpoint('sm') {
      padding: 0.75rem;
      background: transparent;
    }
  }

  &__button {
    // multiple lines for cross-browser compatibility
    height: 100%;
    color: var(--parsec-color-light-secondary-text);
    align-self: center;
    margin-left: auto;

    @include ms.responsive-breakpoint('sm') {
      position: absolute;
      right: 0;
      top: 1.75rem;
      height: fit-content;
      font-size: 0.875rem;
      color: var(--parsec-color-light-primary-500);
    }
  }

  &.authentication,
  &.server {
    .summary-item__text {
      background: var(--parsec-color-light-secondary-white);
      color: var(--parsec-color-light-secondary-grey);
      font-style: italic;
    }
  }
}

.tos {
  padding: 1.5rem 0 0 0.75rem;
  color: var(--parsec-color-light-secondary-soft-text);

  @include ms.responsive-breakpoint('sm') {
    padding: 0.5rem 2rem;
  }
}
</style>
