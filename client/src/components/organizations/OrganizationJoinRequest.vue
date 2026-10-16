<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <div
    class="organization-request"
    :class="{
      pending: request.info.tag === PendingAsyncEnrollmentInfoTag.Submitted,
      rejected: request.info.tag === PendingAsyncEnrollmentInfoTag.Rejected,
      accepted: request.info.tag === PendingAsyncEnrollmentInfoTag.Accepted,
      cancelled: request.info.tag === PendingAsyncEnrollmentInfoTag.Cancelled,
    }"
  >
    <ion-card-content class="organization-request-content">
      <div class="organization-request-info">
        <ion-text class="organization-request-organization">{{ request.organizationId }}</ion-text>
        <ion-text class="organization-request-username">{{ request.enrollment.requestedHumanHandle.label }}</ion-text>
      </div>
      <div class="organization-request-status-container">
        <ion-text
          class="organization-request-status"
          :class="{
            'status-pending': request.info.tag === PendingAsyncEnrollmentInfoTag.Submitted,
            'status-rejected': request.info.tag === PendingAsyncEnrollmentInfoTag.Rejected,
            'status-accepted': request.info.tag === PendingAsyncEnrollmentInfoTag.Accepted,
            'status-cancelled': request.info.tag === PendingAsyncEnrollmentInfoTag.Cancelled,
          }"
          ref="statusTextEl"
        >
          <span v-if="statusText">{{ $msTranslate(statusText) }}</span>
        </ion-text>
        <ion-icon
          v-if="request.info.tag !== PendingAsyncEnrollmentInfoTag.Accepted"
          @click="$emit('deleteRequest', request)"
          :icon="trash"
          class="organization-request-icon"
          ref="trashIconEl"
        />
        <ion-button
          v-if="request.info.tag === PendingAsyncEnrollmentInfoTag.Accepted"
          fill="clear"
          class="organization-request-button"
          @click="$emit('joinOrganization', request)"
        >
          {{ $msTranslate('HomePage.organizationRequest.joinButton') }}
        </ion-button>
      </div>
    </ion-card-content>
  </div>
</template>

<script lang="ts" setup>
import { AsyncEnrollmentRequest, PendingAsyncEnrollmentInfoTag } from '@/parsec';
import { IonButton, IonCardContent, IonIcon, IonText } from '@ionic/vue';
import { trash } from 'ionicons/icons';
import { attachMouseOverTooltip } from 'megashark-lib';
import { computed, onMounted, useTemplateRef, watch } from 'vue';

const statusTextRef = useTemplateRef<InstanceType<typeof IonText>>('statusTextEl');
const trashIconRef = useTemplateRef<InstanceType<typeof IonIcon>>('trashIconEl');

const props = defineProps<{
  request: AsyncEnrollmentRequest;
}>();

onMounted(async () => {
  attachTooltip(statusTextRef.value?.$el, props.request.info.tag);
  attachMouseOverTooltip(trashIconRef.value?.$el, 'HomePage.organizationRequest.rejected.yes');
});

watch(
  () => props.request.info.tag,
  (newStatus) => {
    attachTooltip(statusTextRef.value?.$el, newStatus);
  },
);

function attachTooltip(el: HTMLElement | undefined, status: PendingAsyncEnrollmentInfoTag): void {
  if (!el) {
    return;
  }
  if (status === PendingAsyncEnrollmentInfoTag.Submitted) {
    attachMouseOverTooltip(el, 'HomePage.organizationRequest.pending.tooltip');
  } else if (status === PendingAsyncEnrollmentInfoTag.Rejected) {
    attachMouseOverTooltip(el, 'HomePage.organizationRequest.rejected.tooltip');
  } else if (status === PendingAsyncEnrollmentInfoTag.Cancelled) {
    attachMouseOverTooltip(el, 'HomePage.organizationRequest.cancelled.tooltip');
  } else if (status === PendingAsyncEnrollmentInfoTag.Accepted) {
    attachMouseOverTooltip(el, 'HomePage.organizationRequest.accepted.tooltip');
  }
}

defineEmits<{
  (e: 'joinOrganization', request: AsyncEnrollmentRequest): void;
  (e: 'deleteRequest', request: AsyncEnrollmentRequest): void;
}>();

const statusText = computed(() => {
  switch (props.request.info.tag) {
    case PendingAsyncEnrollmentInfoTag.Submitted:
      return 'HomePage.organizationRequest.status.pending';
    case PendingAsyncEnrollmentInfoTag.Rejected:
      return 'HomePage.organizationRequest.status.rejected';
    case PendingAsyncEnrollmentInfoTag.Cancelled:
      return 'HomePage.organizationRequest.status.cancelled';
    default:
      return undefined;
  }
});
</script>

<style lang="scss" scoped>
.organization-request {
  border-radius: ms.radius('2xl');
  box-shadow: none;
  margin: 0;
  padding: ms.spacing('padding-none');
  background: ms.color('surface-neutral-default-subtle');
  border: ms.border('thin') dashed ms.color('border-neutral-default-subtle');
  display: flex;
  overflow: hidden;
  min-height: 4.5rem;

  &-content {
    align-items: center;
    display: flex;
    justify-content: space-between;
    padding: ms.spacing('padding-2xl') ms.spacing('padding-3xl');
    flex-shrink: 0;
    width: 100%;
  }

  &-info {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-sm');
    width: 100%;
    overflow: hidden;

    .organization-request-organization {
      @include ms.font('heading-h5');
      color: ms.color('text-base-body');
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
    }

    .organization-request-username {
      @include ms.font('body-md-regular');
      color: ms.color('text-base-description');
      text-overflow: ellipsis;
      white-space: nowrap;
      overflow: hidden;
      line-height: normal;
    }
  }

  &-status-container {
    display: flex;
    align-items: center;
    gap: ms.spacing('gap-4xl');
    flex-shrink: 0;

    .organization-request-status {
      @include ms.font('label-sm-medium');
      align-items: center;
      border-radius: ms.radius('lg');
      display: flex;
      justify-content: center;
      padding: ms.spacing('padding-xs') ms.spacing('padding-lg');
      flex-shrink: 0;

      &.status-cancelled {
        background-color: ms.color('surface-neutral-default-subtle');
        color: ms.color('text-on-color-heading');
      }

      &.status-rejected {
        background-color: ms.color('surface-error-default-subtle');
        color: ms.color('text-error-on-color');
      }

      &.status-pending {
        background-color: ms.color('surface-warning-default-subtle');
        color: ms.color('text-warning-default');
      }
    }

    .organization-request-icon {
      margin-left: auto;
      padding: ms.spacing('padding-sm');
      border-radius: ms.radius('lg');
      font-size: 1.25rem;
      flex-shrink: 0;
      color: ms.color('icon-neutral-default');
      cursor: pointer;

      &:hover {
        background: ms.color('surface-base-page-secondary');
        color: ms.color('icon-neutral-default-hover');
      }
    }
  }

  &-button {
    background: ms.color('surface-neutral-default');
    border-radius: ms.radius('lg');
    --color: #{ms.color('text-on-color-heading')};
    box-shadow: none;
    transition: box-shadow 0.2s ease-in-out;

    &::part(native) {
      padding: ms.spacing('padding-xl') ms.spacing('padding-3xl');
      --background-hover: #{ms.color('surface-neutral-default')};
    }

    .button-icon {
      font-size: 1rem;
      margin-left: 0.5rem;
    }

    &:hover {
      color: ms.color('text-on-color-heading');
      background: ms.color('surface-neutral-default-hover');
      box-shadow: var(--parsec-shadow-soft);
    }
  }

  &.pending {
    border-color: ms.color('border-base-default');

    .organization-request-icon {
      &:hover {
        color: ms.color('icon-neutral-default-hover');
      }
    }
  }

  &.rejected {
    border: ms.border('thin') solid ms.color('border-error-default');
    background: ms.color('surface-base-page-secondary');
  }

  &.accepted {
    border: ms.border('thin') solid ms.color('border-neutral-default-subtle');
    background: ms.color('surface-base-default');
    transition: all 0.15s ease-in-out;

    &:hover {
      box-shadow: var(--parsec-shadow-soft);
      background: ms.color('surface-base-default-secondary');
    }
  }
}
</style>
