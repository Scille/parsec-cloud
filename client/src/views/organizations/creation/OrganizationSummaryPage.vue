<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-page class="summary-page page-modal-container">
    <create-organization-modal-header
      @close-clicked="$emit('closeRequested')"
      title="CreateOrganization.title.overview"
      subtitle="CreateOrganization.subtitle.overview"
      :small-display-stepper="true"
    />

    <!-- error -->
    <ms-report-text
      class="form-error"
      v-show="error"
      :theme="MsReportTheme.Error"
    >
      {{ $msTranslate(error) }}
    </ms-report-text>

    <ion-list class="summary-list">
      <!-- organization name -->
      <ion-item class="summary-item-container ion-no-padding">
        <div class="summary-item">
          <ion-text class="summary-item__label">
            {{ $msTranslate('CreateOrganization.overview.organization') }}
          </ion-text>
          <ion-text class="summary-item__text">
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

      <!-- use authority key -->
      <ion-item
        class="summary-item-container ion-no-padding"
        v-if="useSequesterKey"
      >
        <div class="summary-item">
          <ion-text class="summary-item__label">
            {{ $msTranslate('CreateOrganization.overview.sequester') }}
          </ion-text>
          <ion-text class="summary-item__text">
            {{ $msTranslate('CreateOrganization.sequester.authorityKey') }}
          </ion-text>
          <ion-button
            fill="clear"
            class="summary-item__button"
            @click="$emit('updateOrganizationNameClicked')"
          >
            {{ $msTranslate('CreateOrganization.button.modify') }}
          </ion-button>
        </div>
      </ion-item>

      <hr
        class="summary-item-divider"
        v-if="useSequesterKey"
      />

      <!-- full name -->
      <ion-item class="summary-item-container ion-no-padding">
        <div class="summary-item">
          <ion-text class="summary-item__label">
            {{ $msTranslate('CreateOrganization.overview.fullname') }}
          </ion-text>
          <ion-text class="summary-item__text">
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
          <ion-text class="summary-item__label">
            {{ $msTranslate('CreateOrganization.overview.email') }}
          </ion-text>
          <ion-text class="summary-item__text">
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
          <ion-text class="summary-item__label">
            {{ $msTranslate('CreateOrganization.overview.server') }}
          </ion-text>
          <ion-text class="summary-item__text">
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
          <ion-text class="summary-item__label">
            {{ $msTranslate('CreateOrganization.overview.authentication') }}
          </ion-text>
          <ion-text
            class="summary-item__text"
            v-if="protectionStrategy === DevicePrimaryProtectionStrategyTag.Keyring"
          >
            {{ $msTranslate('CreateOrganization.keyringChoice') }}
          </ion-text>
          <ion-text
            class="summary-item__text"
            v-if="protectionStrategy === DevicePrimaryProtectionStrategyTag.Password"
          >
            {{ $msTranslate('CreateOrganization.passwordChoice') }}
          </ion-text>
          <ion-text
            class="summary-item__text"
            v-if="protectionStrategy === DevicePrimaryProtectionStrategyTag.AccountVault"
          >
            {{ $msTranslate('CreateOrganization.accountChoice') }}
          </ion-text>
          <ion-text
            class="summary-item__text"
            v-if="protectionStrategy === DevicePrimaryProtectionStrategyTag.PKI"
          >
            {{ $msTranslate('CreateOrganization.smartcardChoice') }}
          </ion-text>
          <ion-text
            class="summary-item__text"
            v-if="protectionStrategy === DevicePrimaryProtectionStrategyTag.OpenBao"
          >
            {{ $msTranslate('CreateOrganization.openBaoChoice') }}
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
      <ion-text class="item-radio__text ion-text-wrap">
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

    <ion-footer class="summary-page-footer">
      <div class="modal-footer-buttons">
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
      </div>
    </ion-footer>
  </ion-page>
</template>

<script setup lang="ts">
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';
import { DevicePrimaryProtectionStrategyTag, OrganizationID } from '@/parsec';
import { ServerType } from '@/services/parsecServers';
import { IonButton, IonFooter, IonIcon, IonItem, IonList, IonPage, IonText } from '@ionic/vue';
import { chevronBack, chevronForward } from 'ionicons/icons';
import { MsReportText, MsReportTheme, Translatable } from 'megashark-lib';

defineProps<{
  serverType: ServerType;
  organizationName: OrganizationID;
  email: string;
  name: string;
  protectionStrategy: DevicePrimaryProtectionStrategyTag;
  canEditOrganizationName?: boolean;
  canEditServerAddress?: boolean;
  canEditEmail?: boolean;
  canEditName?: boolean;
  canEditSaveStrategy?: boolean;
  useSequesterKey?: boolean;
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
.summary-list {
  padding: ms.spacing('padding-none');
  display: flex;
  flex-direction: column;
  border-radius: ms.radius('lg');
  border: ms.border('thin') solid ms.color('border-base-default');

  @include ms.responsive-breakpoint('sm') {
    margin: 0 2rem;
    border: none;
    border-radius: ms.radius('none');
    gap: ms.spacing('gap-2xl');
  }

  @include ms.responsive-breakpoint('xs') {
    margin: 0.5rem 1.5rem;
  }
}

.summary-item-container {
  --inner-padding-end: #{ms.spacing('padding-none')};
  --background: #{ms.color('surface-brand-default')};
  flex-shrink: 0;
}

.summary-item {
  display: flex;
  align-items: stretch;
  width: 100%;
  position: relative;
  gap: ms.spacing('gap-3xl');
  padding-right: ms.spacing('padding-3xl');

  @include ms.responsive-breakpoint('sm') {
    flex-direction: column;
    align-items: stretch;
    gap: ms.spacing('gap-none');
    padding-right: ms.spacing('padding-none');
    border-radius: ms.radius('lg');
    overflow: hidden;
    border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
  }

  &-divider {
    width: 100%;
    height: 1px;
    background: ms.color('border-base-default');
    z-index: 2;
    margin: 0;

    @include ms.responsive-breakpoint('sm') {
      display: none;
    }
  }

  &__label {
    @include ms.font('body-md-medium');
    max-width: 9rem;
    width: 100%;
    background: ms.color('surface-neutral-default-subtle');
    color: ms.color('text-base-description');
    align-items: center;
    padding: ms.spacing('padding-none') ms.spacing('padding-lg');
    display: flex;

    @include ms.responsive-breakpoint('sm') {
      max-width: 100%;
      padding: ms.spacing('padding-xs') ms.spacing('padding-2xl');
    }
  }

  &__text {
    @include ms.font('body-lg-regular');
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    color: ms.color('text-base-body');
    background: ms.color('surface-base-default');
    padding: ms.spacing('padding-3xl') ms.spacing('padding-none');

    @include ms.responsive-breakpoint('sm') {
      padding: ms.spacing('padding-2xl');
      background: transparent;
    }
  }

  &__button {
    // multiple lines for cross-browser compatibility
    height: 100%;
    color: ms.color('text-base-body');
    align-self: center;
    margin-left: auto;

    @include ms.responsive-breakpoint('sm') {
      position: absolute;
      right: 0.5rem;
      top: 2rem;
      height: fit-content;
      font-size: 0.875rem;
      color: ms.color('text-brand-default');
    }
  }

  &.authentication,
  &.server {
    .summary-item__text {
      background: ms.color('surface-base-default');
      color: ms.color('text-base-placeholder');
      font-style: italic;
    }
  }
}

.tos {
  .item-radio__text {
    @include ms.font('body-sm-regular');
  }

  padding: ms.spacing('padding-4xl') ms.spacing('padding-none') ms.spacing('padding-none') ms.spacing('padding-2xl');
  color: ms.color('text-base-label');

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-lg') ms.spacing('padding-5xl');
    height: fit-content;
  }
}
.form-error {
  margin: 0 1rem 1rem 1rem;
  height: auto;
}
</style>
