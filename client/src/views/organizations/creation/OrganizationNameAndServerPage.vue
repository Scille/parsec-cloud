<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-page class="organization-name-and-server-page">
    <create-organization-modal-header
      @close-clicked="$emit('closeRequested')"
      title="CreateOrganization.title.nameAndServer"
    />

    <div>
      <ms-input
        :label="'CreateOrganization.organizationName'"
        :placeholder="'CreateOrganization.organizationNamePlaceholder'"
        id="org-name-input"
        v-model="organizationName"
        :disabled="disableOrganizationNameField"
        ref="organizationNameInputRef"
        :validator="organizationValidator"
      />

      <ion-text class="subtitles-sm org-name-criteria">
        {{ $msTranslate('CreateOrganization.organizationNameCriteria') }}
      </ion-text>

      <ms-input
        :label="'CreateOrganization.serverDescription'"
        id="server-addr-input"
        v-model="serverAddr"
        :disabled="disableServerAddrField"
        ref="serverAddrInputRef"
        :validator="parsecAddrValidator"
      />
      <p v-show="error">
        {{ $msTranslate(error) }}
      </p>

      <ion-footer class="organization-name-and-server-page-footer">
        <ion-buttons
          slot="primary"
          class="modal-footer-buttons"
        >
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
            @click="$emit('organizationNameAndServerChosen', organizationName, serverAddr)"
            :disabled="!valid"
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
        </ion-buttons>
      </ion-footer>
    </div>
  </ion-page>
</template>

<script setup lang="ts">
import { OrganizationID } from '@/parsec';
import { IonPage, IonButton, IonText, IonButtons, IonFooter, IonIcon } from '@ionic/vue';
import { computed, onMounted, ref } from 'vue';
import { chevronForward, chevronBack } from 'ionicons/icons';
import { organizationValidator, parsecAddrValidator } from '@/common/validators';
import { Translatable, Validity, MsInput } from 'megashark-lib';
import CreateOrganizationModalHeader from '@/components/organizations/CreateOrganizationModalHeader.vue';

const props = defineProps<{
  organizationName?: OrganizationID;
  serverAddr?: string;
  disableServerAddrField?: boolean;
  disableOrganizationNameField?: boolean;
  error?: Translatable;
  hidePrevious?: boolean;
}>();

defineEmits<{
  (e: 'organizationNameAndServerChosen', name: OrganizationID, serverAddr: string): void;
  (e: 'closeRequested'): void;
  (e: 'goBackRequested'): void;
}>();

const organizationNameInputRef = ref();
const serverAddrInputRef = ref();
const organizationName = ref<OrganizationID>(props.organizationName ?? '');
const serverAddr = ref<string>(props.serverAddr ?? '');
const valid = computed(() => {
  return (
    organizationNameInputRef.value &&
    organizationNameInputRef.value.validity === Validity.Valid &&
    serverAddrInputRef.value &&
    serverAddrInputRef.value.validity === Validity.Valid
  );
});

onMounted(async () => {
  if (organizationNameInputRef.value && organizationName.value) {
    await organizationNameInputRef.value.validate(organizationName.value);
  }
  if (serverAddrInputRef.value && serverAddr.value) {
    await serverAddrInputRef.value.validate(serverAddr.value);
  }
});
</script>

<style scoped lang="scss">
.organization-name-and-server-page {
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
</style>
