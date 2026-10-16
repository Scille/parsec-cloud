<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

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
        label="CreateOrganization.organizationName"
        placeholder="CreateOrganization.organizationNamePlaceholder"
        name="organization"
        id="org-name-input"
        v-model="organizationName"
        :disabled="disableOrganizationNameField"
        ref="organizationNameInput"
        @on-enter-keyup="!isNextDisabled && $emit('organizationNameChosen', organizationName, sequesterKeyInputRef?.getKey())"
        :validator="organizationValidator"
      />

      <ion-text class="org-name-criteria">
        {{ $msTranslate('CreateOrganization.organizationNameCriteria') }}
      </ion-text>

      <sequester-key-input
        ref="sequesterKeyInput"
        v-if="advancedSettings"
      />

      <!-- error -->
      <ion-text
        class="form-error login-button-error"
        v-show="error"
      >
        <ion-icon
          class="form-error-icon"
          :icon="warning"
        />{{ $msTranslate(error) }}
      </ion-text>
    </div>

    <ion-footer class="organization-name-page-footer">
      <ion-text
        button
        class="advanced-settings"
        @click="advancedSettings = !advancedSettings"
      >
        <ion-icon :icon="advancedSettings ? remove : add" />
        {{ $msTranslate('CreateOrganization.button.advancedSettings') }}
      </ion-text>
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
          @click="$emit('organizationNameChosen', organizationName, sequesterKeyInputRef?.getKey())"
          :disabled="isNextDisabled"
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
import { organizationValidator } from '@/common/validators';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';
import SequesterKeyInput from '@/components/organizations/SequesterKeyInput.vue';
import { OrganizationID } from '@/parsec';
import { IonButton, IonFooter, IonIcon, IonPage, IonText } from '@ionic/vue';
import { add, chevronBack, chevronForward, remove, warning } from 'ionicons/icons';
import { MsInput, Translatable, Validity } from 'megashark-lib';
import { computed, onMounted, onUnmounted, ref, useTemplateRef, watch } from 'vue';

const props = defineProps<{
  organizationName?: OrganizationID;
  error?: Translatable;
  disableOrganizationNameField?: boolean;
  hidePrevious?: boolean;
}>();

defineEmits<{
  (e: 'organizationNameChosen', name: OrganizationID, sequesterKey: string | undefined): void;
  (e: 'goBackRequested'): void;
  (e: 'closeRequested'): void;
}>();

const organizationNameInputRef = useTemplateRef<InstanceType<typeof MsInput>>('organizationNameInput');
const sequesterKeyInputRef = useTemplateRef<InstanceType<typeof SequesterKeyInput>>('sequesterKeyInput');
const organizationName = ref<OrganizationID>(props.organizationName ?? '');
const advancedSettings = ref<boolean>(false);

const cancelWatch = watch(
  () => props.organizationName,
  (newName?: OrganizationID) => {
    if (newName) {
      organizationName.value = newName;
    }
    organizationNameInputRef.value?.validate(organizationName.value);
  },
);

const isNextDisabled = computed(() => {
  if (organizationNameInputRef.value?.validity !== Validity.Valid) {
    return true;
  }
  if (sequesterKeyInputRef.value?.isAddKeyToggled && !sequesterKeyInputRef.value?.getKey()) {
    return true;
  }
  return false;
});

onMounted(async () => {
  if (organizationNameInputRef.value && organizationName.value) {
    await organizationNameInputRef.value.validate(organizationName.value);
  }
});

onUnmounted(() => {
  cancelWatch();
});
</script>

<style scoped lang="scss">
.organization-name-page-content {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-lg');
  width: 100%;

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-lg') ms.spacing('padding-5xl');
  }

  @include ms.responsive-breakpoint('sm') {
    padding: ms.spacing('padding-lg') ms.spacing('padding-4xl');
  }
}

.login-button-error {
  @include ms.font('body-md-regular');
}

.org-name-criteria {
  @include ms.font('body-md-regular');
  color: ms.color('text-base-description');
}

.advanced-settings {
  @include ms.font('label-md-medium');
  display: flex;
  gap: ms.spacing('gap-lg');
  margin-right: auto;
  color: ms.color('text-base-description');
  cursor: pointer;

  @include ms.responsive-breakpoint('sm') {
    margin: auto;
  }
}
</style>
