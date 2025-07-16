<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="organization-name-page page-modal-container">
    <create-organization-modal-header
      @close-clicked="$emit('closeRequested')"
      title="CreateOrganization.title.create"
      subtitle="CreateOrganization.subtitle.nameYourOrg"
      :small-display-stepper="true"
    />
    <div class="organization-name-page-content">
      <ms-input
        :label="'CreateOrganization.organizationName'"
        :placeholder="'CreateOrganization.organizationNamePlaceholder'"
        name="organization"
        id="org-name-input"
        v-model="organizationName"
        :disabled="disableOrganizationNameField"
        ref="organizationNameInput"
        @on-enter-keyup="$emit('organizationNameChosen', organizationName)"
        :validator="organizationValidator"
      />

      <ion-text class="body org-name-criteria">
        {{ $msTranslate('CreateOrganization.organizationNameCriteria') }}
      </ion-text>

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
    </div>

    <ion-footer class="organization-name-page-footer">
      <div class="modal-footer-buttons">
        <ion-button
          fill="clear"
          size="default"
          id="previous-button"
          @click="$emit('goBackRequested')"
          v-show="!hidePrevious"
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
          @click="$emit('organizationNameChosen', organizationName)"
          :disabled="!organizationNameInputRef || organizationNameInputRef.validity !== Validity.Valid"
        >
          <span>
            {{ $msTranslate('CreateOrganization.button.next') }}
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
import { OrganizationID } from '@/parsec';
import { IonPage, IonButton, IonText, IonFooter, IonIcon } from '@ionic/vue';
import { onMounted, ref, useTemplateRef } from 'vue';
import { chevronBack, chevronForward } from 'ionicons/icons';
import { organizationValidator } from '@/common/validators';
import { Translatable, Validity, MsInput } from 'megashark-lib';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';
import { warning } from 'ionicons/icons';

const props = defineProps<{
  organizationName?: OrganizationID;
  error?: Translatable;
  disableOrganizationNameField?: boolean;
  hidePrevious?: boolean;
}>();

defineEmits<{
  (e: 'organizationNameChosen', name: OrganizationID): void;
  (e: 'goBackRequested'): void;
  (e: 'closeRequested'): void;
}>();

const organizationNameInputRef = useTemplateRef<InstanceType<typeof MsInput>>('organizationNameInput');
const organizationName = ref<OrganizationID>(props.organizationName ?? '');

onMounted(async () => {
  if (organizationNameInputRef.value && organizationName.value) {
    await organizationNameInputRef.value.validate(organizationName.value);
  }
});
</script>

<style scoped lang="scss">
.organization-name-page-content {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  width: 100%;

  @include ms.responsive-breakpoint('sm') {
    padding: 0.5rem 2rem;
  }

  @include ms.responsive-breakpoint('sm') {
    padding: 0.5rem 1.5rem;
  }
}

.org-name-criteria {
  color: var(--parsec-color-light-secondary-hard-grey);
}
</style>
