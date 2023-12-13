<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page>
    <ion-content
      :fullscreen="true"
      color="secondary"
    >
      <ion-card class="login-popup">
        <ion-card-content class="organization-container">
          <ion-text class="title-h1">
            {{ $t('HomePage.organizationLogin.login') }}
          </ion-text>
          <!-- login -->
          <div id="login-container">
            <ion-card class="login-card">
              <ion-card-content class="login-card__content">
                <ion-grid>
                  <organization-card :device="device" />
                  <ms-password-input
                    :label="$t('HomePage.organizationLogin.passwordLabel')"
                    v-model="password"
                    @on-enter-keyup="onLoginClick()"
                    id="ms-password-input"
                  />
                  <ion-button
                    fill="clear"
                    @click="$emit('forgottenPasswordClick', device)"
                    id="forgotten-password-button"
                  >
                    {{ $t('HomePage.organizationLogin.forgottenPassword') }}
                  </ion-button>
                </ion-grid>
              </ion-card-content>
            </ion-card>
            <div class="login-button-container">
              <ion-button
                @click="onLoginClick()"
                size="large"
                :disabled="password.length == 0"
                class="login-button"
              >
                <ion-icon
                  slot="start"
                  :icon="logIn"
                />
                {{ $t('HomePage.organizationLogin.login') }}
              </ion-button>
            </div>
          </div>
          <!-- end of login -->
        </ion-card-content>
      </ion-card>
    </ion-content>
  </ion-page>
</template>

<script setup lang="ts">
import { IonButton, IonCard, IonCardContent, IonContent, IonPage, IonIcon, IonGrid, IonText } from '@ionic/vue';
import { logIn } from 'ionicons/icons';
import { AvailableDevice } from '@/parsec';
import { ref } from 'vue';
import OrganizationCard from '@/components/organizations/OrganizationCard.vue';
import { MsPasswordInput } from '@/components/core';

const props = defineProps<{
  device: AvailableDevice;
}>();

const emits = defineEmits<{
  (e: 'loginClick', device: AvailableDevice, password: string): void;
  (e: 'forgottenPasswordClick', device: AvailableDevice): void;
}>();

async function onLoginClick(): Promise<void> {
  emits('loginClick', props.device, password.value);
}

const password = ref('');
</script>

<style lang="scss" scoped>
.login-popup {
  box-shadow: none;
  display: flex;
  margin: auto 0;
  align-items: center;
  justify-content: center;
  flex-grow: 1;
  max-height: 80%;

  .organization-container {
    max-width: 62.5rem;
    padding: 0 3.5rem 3.5rem;
    flex-grow: 1;
  }

  .title-h1 {
    color: var(--parsec-color-light-primary-700);
  }

  #login-container {
    margin-top: 2.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .login-card {
    background: var(--parsec-color-light-secondary-background);
    border-radius: 8px;
    padding: 2em;
    box-shadow: none;
    margin: 0;

    &__content {
      padding: 0;

      #ms-password-input {
        margin: 1.5rem 0 1rem;
      }
    }

    .organization-card {
      margin-bottom: 2em;
      display: flex;

      &__body {
        padding: 0;
      }
    }
  }

  .login-button-container {
    text-align: right;

    .login-button {
      margin: 0;
    }
  }
}
.login-popup {
  box-shadow: none;
  display: flex;
  margin: auto 0;
  align-items: center;
  justify-content: center;
  flex-grow: 1;
  max-height: 80%;

  .organization-container {
    max-width: 62.5rem;
    padding: 0 3.5rem 3.5rem;
    flex-grow: 1;
  }

  .title-h1 {
    color: var(--parsec-color-light-primary-700);
  }

  #login-container {
    margin-top: 2.5rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .login-card {
    background: var(--parsec-color-light-secondary-background);
    border-radius: 8px;
    padding: 2em;
    box-shadow: none;
    margin: 0;

    &__content {
      padding: 0;

      #ms-password-input {
        margin: 1.5rem 0 1rem;
      }
    }

    .organization-card {
      margin-bottom: 2em;
      display: flex;

      &__body {
        padding: 0;
      }
    }
  }

  .login-button-container {
    text-align: right;

    .login-button {
      margin: 0;
    }
  }
}
</style>
