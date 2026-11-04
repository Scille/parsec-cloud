<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <ion-page class="user-information-page page-modal-container">
    <create-organization-modal-header
      @close-clicked="$emit('closeRequested')"
      title="CreateOrganization.title.personalDetails"
      :small-display-stepper="true"
    />

    <user-information
      ref="userInformation"
      :default-email="email"
      :default-name="name"
      :email-enabled="email === undefined"
      :name-enabled="name === undefined"
      @field-update="onFieldUpdated"
      class="user-information-content"
    />

    <div
      class="tos-checkbox"
      v-if="requireTos"
    >
      <ms-checkbox
        @change="onFieldUpdated"
        v-model="tosAccepted"
        label-placement="end"
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
      </ms-checkbox>
    </div>

    <ion-footer class="user-information-page-footer">
      <div class="modal-footer-buttons">
        <ion-button
          fill="clear"
          size="default"
          id="previous-button"
          @click="$emit('goBackRequested')"
          v-show="!hidePrevious"
        >
          <ion-icon
            class="button-icon-left"
            :icon="chevronBack"
          />
          {{ $msTranslate('CreateOrganization.button.previous') }}
        </ion-button>

        <ion-button
          fill="solid"
          size="default"
          @click="onButtonClicked"
          :disabled="!valid"
        >
          <span>
            {{ $msTranslate('CreateOrganization.button.next') }}
          </span>
          <ion-icon
            class="button-icon-right"
            :icon="chevronForward"
          />
        </ion-button>
      </div>
    </ion-footer>
  </ion-page>
</template>

<script setup lang="ts">
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';
import UserInformation from '@/components/users/UserInformation.vue';
import { IonButton, IonFooter, IonIcon, IonPage, IonText } from '@ionic/vue';
import { chevronBack, chevronForward } from 'ionicons/icons';
import { MsCheckbox } from 'megashark-lib';
import { onMounted, ref, useTemplateRef } from 'vue';

const props = defineProps<{
  name?: string;
  email?: string;
  hidePrevious?: boolean;
  requireTos?: boolean;
}>();

const emits = defineEmits<{
  (e: 'userInformationFilled', name: string, email: string): void;
  (e: 'closeRequested'): void;
  (e: 'goBackRequested'): void;
}>();

defineExpose({
  setFocus,
});

const userInformationRef = useTemplateRef<InstanceType<typeof UserInformation>>('userInformation');
const valid = ref(false);
const tosAccepted = ref(false);

onMounted(async () => {
  if (userInformationRef.value) {
    await userInformationRef.value.setFocus();
  }
  await onFieldUpdated();
});

async function setFocus(): Promise<void> {
  if (userInformationRef.value) {
    await userInformationRef.value.setFocus();
  }
}

async function onButtonClicked(): Promise<void> {
  if (!userInformationRef.value) {
    console.log('Missing ref, should not happen');
    return;
  }
  emits('userInformationFilled', userInformationRef.value.fullName, userInformationRef.value.email);
}

async function onFieldUpdated(): Promise<void> {
  if (userInformationRef.value) {
    valid.value = await userInformationRef.value.areFieldsCorrect();
  } else {
    valid.value = false;
  }
  if (props.requireTos) {
    valid.value = valid.value && tosAccepted.value;
  }
}
</script>

<style scoped lang="scss">
.user-information-content,
.tos-checkbox {
  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-none') ms.spacing('padding-5xl');
  }

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-none') ms.spacing('padding-4xl');
  }
}

.tos-checkbox {
  padding-top: ms.spacing('padding-3xl');

  .item-radio__text {
    @include ms.font('body-sm-regular');
  }
  color: ms.color('text-base-label');

  a {
    color: ms.color('text-brand-default');

    &:hover {
      text-decoration: underline;
    }
  }
}
</style>
