<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <!-- This component should be encapsulated in a parent page within a <ion-menu> -->
  <!-- It can be shown with <ion-menu-button> -->
  <!-- Check the documentation for more information: https://ionicframework.com/docs/api/menu -->
  <div class="menu-secondary-collapse-buttons">
    <ms-image
      :image="LogoIconGradient"
      class="menu-secondary-collapse-logo"
    />
    <ion-menu-toggle>
      <ion-icon
        class="menu-secondary-collapse__close"
        :icon="close"
      />
    </ion-menu-toggle>
    <!-- about button -->
    <ion-menu-toggle>
      <ion-button
        id="trigger-version-button"
        class="menu-secondary-collapse-buttons-item button-large"
        @click="openAboutModal"
      >
        <ion-icon
          :icon="informationCircle"
          class="menu-secondary-collapse-buttons-item__icon"
        />
        <span class="menu-secondary-collapse-buttons-item__text">{{ $msTranslate('MenuPage.about') }}</span>
      </ion-button>
    </ion-menu-toggle>
    <!-- doc button -->
    <ion-menu-toggle>
      <ion-button
        class="menu-secondary-collapse-buttons-item button-large"
        @click="Env.Links.openDocumentationLink"
      >
        <ion-icon
          :icon="library"
          class="menu-secondary-collapse-buttons-item__icon"
        />
        <span class="menu-secondary-collapse-buttons-item__text">{{ $msTranslate('MenuPage.documentation') }}</span>
        <ion-icon :icon="open" />
      </ion-button>
    </ion-menu-toggle>
    <!-- contact button -->
    <ion-menu-toggle>
      <ion-button
        class="menu-secondary-collapse-buttons-item button-large"
        @click="Env.Links.openContactLink"
      >
        <ion-icon
          :icon="chatbubble"
          class="menu-secondary-collapse-buttons-item__icon"
        />
        <span class="menu-secondary-collapse-buttons-item__text">{{ $msTranslate('MenuPage.contact') }}</span>
        <ion-icon :icon="open" />
      </ion-button>
    </ion-menu-toggle>
    <!-- settings button -->
    <ion-menu-toggle>
      <ion-button
        id="trigger-settings-button"
        class="menu-secondary-collapse-buttons-item button-large"
        @click="$emit('settingsClick')"
      >
        <ion-icon
          :icon="cog"
          class="menu-secondary-collapse-buttons-item__icon"
        />
        <span class="menu-secondary-collapse-buttons-item__text">{{ $msTranslate('MenuPage.settings') }}</span>
      </ion-button>
    </ion-menu-toggle>

    <!-- report bug button -->
    <ion-menu-toggle>
      <ion-button
        id="report-bug-button"
        class="menu-secondary-collapse-buttons-item button-large"
        @click="$emit('reportBugClick')"
      >
        <ion-icon
          :icon="bug"
          class="menu-secondary-collapse-buttons-item__icon"
        />
        <span class="menu-secondary-collapse-buttons-item__text">{{ $msTranslate('MenuPage.reportBug') }}</span>
      </ion-button>
    </ion-menu-toggle>

    <!-- customer area button -->
    <ion-menu-toggle>
      <ion-button
        id="trigger-customer-area-button"
        class="menu-secondary-collapse-buttons-item button-large"
        v-show="!Env.isStripeDisabled()"
        @click="$emit('customerAreaClick')"
        v-if="showCustomerAreaButton"
      >
        {{ $msTranslate('HomePage.topbar.customerArea') }}
      </ion-button>
    </ion-menu-toggle>
  </div>
</template>

<script setup lang="ts">
import { Env } from '@/services/environment';
import { openAboutModal } from '@/views/about';
import { IonButton, IonIcon, IonMenuToggle } from '@ionic/vue';
import { bug, chatbubble, close, cog, informationCircle, library, open } from 'ionicons/icons';
import { LogoIconGradient, MsImage } from 'megashark-lib';

defineEmits<{
  (e: 'settingsClick'): void;
  (e: 'customerAreaClick'): void;
  (e: 'reportBugClick'): void;
}>();

defineProps<{
  showCustomerAreaButton: boolean;
}>();
</script>

<style lang="scss" scoped></style>
