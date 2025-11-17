<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="organization-request"
    :class="{
      pending: request.info.tag === PKIInfoItemTag.Submitted,
      rejected: request.info.tag === PKIInfoItemTag.Rejected,
      accepted: request.info.tag === PKIInfoItemTag.Accepted,
      cancelled: request.info.tag === PKIInfoItemTag.Cancelled,
    }"
  >
    <ion-card-content class="organization-request-content">
      <div class="organization-request-info">
        <ion-text class="organization-request-organization title-h4">{{ addr?.organizationId ?? '' }}</ion-text>
        <ion-text class="organization-request-username body">{{ request.enrollment.payload.humanHandle.label }}</ion-text>
      </div>
      <div class="organization-request-status-container">
        <ion-text
          class="organization-request-status button-small"
          :class="{
            'status-pending': request.info.tag === PKIInfoItemTag.Submitted,
            'status-rejected': request.info.tag === PKIInfoItemTag.Rejected,
            'status-accepted': request.info.tag === PKIInfoItemTag.Accepted,
            'status-cancelled': request.info.tag === PKIInfoItemTag.Cancelled,
          }"
          ref="statusText"
        >
          <span v-if="statusText">{{ $msTranslate(statusText) }}</span>
        </ion-text>
        <ion-icon
          v-if="request.info.tag !== PKIInfoItemTag.Accepted"
          @click="$emit('deleteRequest', request)"
          :icon="closeCircle"
          class="organization-request-icon"
        />
        <ion-button
          v-if="request.info.tag === PKIInfoItemTag.Accepted"
          fill="clear"
          class="organization-request-button"
          @click="$emit('joinOrganization', request)"
        >
          {{ $msTranslate('HomePage.organizationRequest.joinButton') }}
          <ion-icon
            :icon="arrowForward"
            class="button-icon"
          />
        </ion-button>
      </div>
    </ion-card-content>
  </div>
</template>

<script lang="ts" setup>
import { ParsedParsecAddrPkiEnrollment, ParsedParsecAddrTag, parseParsecAddr, PKIInfoItemTag, PkiLocalRequest } from '@/parsec';
import { IonButton, IonCardContent, IonIcon, IonText } from '@ionic/vue';
import { arrowForward, closeCircle } from 'ionicons/icons';
import { attachMouseOverTooltip } from 'megashark-lib';
import { computed, onMounted, ref, useTemplateRef, watch } from 'vue';

const statusTextRef = useTemplateRef<InstanceType<typeof IonText>>('statusText');
const addr = ref<ParsedParsecAddrPkiEnrollment | undefined>(undefined);

const props = defineProps<{
  request: PkiLocalRequest;
}>();

onMounted(async () => {
  if (statusTextRef.value && props.request.info.tag === PKIInfoItemTag.Submitted) {
    attachMouseOverTooltip(statusTextRef.value.$el, 'HomePage.organizationRequest.pending.tooltip');
  } else if (statusTextRef.value && props.request.info.tag === PKIInfoItemTag.Rejected) {
    attachMouseOverTooltip(statusTextRef.value.$el, 'HomePage.organizationRequest.rejected.tooltip');
  }
  const addrResult = await parseParsecAddr(props.request.enrollment.addr);
  if (addrResult.ok && addrResult.value.tag === ParsedParsecAddrTag.PkiEnrollment) {
    addr.value = addrResult.value;
  }
});

watch(
  () => props.request.info.tag,
  (newStatus) => {
    if (statusTextRef.value && newStatus === PKIInfoItemTag.Submitted) {
      attachMouseOverTooltip(statusTextRef.value.$el, 'HomePage.organizationRequest.pending.tooltip');
    } else if (statusTextRef.value && newStatus === PKIInfoItemTag.Rejected) {
      attachMouseOverTooltip(statusTextRef.value.$el, 'HomePage.organizationRequest.rejected.tooltip');
    }
  },
);

defineEmits<{
  (e: 'joinOrganization', request: PkiLocalRequest): void;
  (e: 'deleteRequest', request: PkiLocalRequest): void;
}>();

const statusText = computed(() => {
  switch (props.request.info.tag) {
    case PKIInfoItemTag.Submitted:
      return 'HomePage.organizationRequest.status.pending';
    case PKIInfoItemTag.Rejected:
      return 'HomePage.organizationRequest.status.rejected';
    case PKIInfoItemTag.Cancelled:
      return 'HomePage.organizationRequest.status.cancelled';
    default:
      return undefined;
  }
});
</script>

<style lang="scss" scoped>
.organization-request {
  border-radius: var(--parsec-radius-12);
  box-shadow: none;
  margin: 0;
  padding: 0;
  background: var(--parsec-color-light-secondary-white);
  border: 1px dashed var(--parsec-color-light-secondary-medium);
  display: flex;
  overflow: hidden;
  min-height: 4.5rem;

  &-content {
    align-items: center;
    display: flex;
    justify-content: space-between;
    padding: 0.75rem 1rem;
    flex-shrink: 0;
    width: 100%;
  }

  &-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    width: 100%;

    .organization-request-organization {
      color: var(--parsec-color-light-secondary-text);
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
    }

    .organization-request-username {
      color: var(--parsec-color-light-secondary-grey);
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
      line-height: normal;
    }
  }

  &-status-container {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    flex-shrink: 0;

    .organization-request-status {
      align-items: center;
      border-radius: var(--parsec-radius-8);
      display: flex;
      justify-content: center;
      padding: 0.125rem 0.5rem;
      flex-shrink: 0;

      &.status-pending {
        background-color: var(--parsec-color-light-secondary-grey);
        color: var(--parsec-color-light-secondary-white);
      }

      &.status-rejected,
      &.status-cancelled {
        background-color: var(--parsec-color-light-danger-500);
        color: var(--parsec-color-light-secondary-white);
      }
    }

    .organization-request-icon {
      margin-left: auto;
      padding: 0.25rem;
      border-radius: var(--parsec-radius-8);
      font-size: 1.25rem;
      flex-shrink: 0;
      color: var(--parsec-color-light-secondary-soft-text);
      cursor: pointer;

      &:hover {
        background: var(--parsec-color-light-secondary-premiere);
        color: var(--parsec-color-light-secondary-text);
      }
    }
  }

  &-button {
    background: var(--parsec-color-light-secondary-text);
    border-radius: var(--parsec-radius-8);
    --color: var(--parsec-color-light-secondary-white);
    box-shadow: none;
    transition: box-shadow 0.2s ease-in-out;

    &::part(native) {
      padding: 0.625rem 1rem;
      --background-hover: var(--parsec-color-light-secondary-text);
    }

    .button-icon {
      font-size: 1rem;
      margin-left: 0.5rem;
    }

    &:hover {
      color: var(--parsec-color-light-secondary-white);
      background: var(--parsec-color-light-secondary-inversed-contrast);
      box-shadow: var(--parsec-shadow-soft);
    }
  }

  &.pending {
    border-color: var(--parsec-color-light-secondary-light);

    .organization-request-icon {
      color: var(--parsec-color-light-secondary-light);

      &:hover {
        color: var(--parsec-color-light-secondary-text);
      }
    }
  }

  &.rejected {
    border: 1px solid var(--parsec-color-light-danger-500);
    background: var(--parsec-color-light-secondary-premiere);
  }

  &.accepted {
    border: 1px solid var(--parsec-color-light-secondary-medium);
    background: var(--parsec-color-light-secondary-white);
  }
}
</style>
