<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="user-activity">
    <div class="user-activity-header">
      <div class="user-activity-header__top">
        <div class="user-activity-header__text">
          <ion-text class="user-activity-header__title">
            {{ $msTranslate('UsersPage.activity.title') }}
          </ion-text>
        </div>
        <ion-button
          class="user-activity-header__close"
          fill="clear"
          slot="icon-only"
          @click="sidePopoverMenu.close()"
        >
          <ion-icon :icon="close" />
        </ion-button>
      </div>
      <ion-button
        class="user-activity-header__export button-small color-secondary"
        fill="outline"
        @click="exportUsers"
      >
        <ms-image
          class="user-activity-header__export-icon button-icon-left"
          :image="DownloadIcon"
        />
        {{ $msTranslate('UsersPage.activity.exportAll') }}
      </ion-button>
    </div>

    <div class="user-activity-body">
      <ms-spinner
        v-if="loading"
        class="user-activity-spinner"
      />
      <div
        v-else-if="error"
        class="user-activity-empty"
      >
        <ms-report-text :theme="MsReportTheme.Error">
          {{ $msTranslate('UsersPage.activity.loadError') }}
        </ms-report-text>
      </div>
      <!-- Manage empty state -->
      <div
        v-else-if="groupedEvents.size === 0"
        class="user-activity-empty"
      >
        <ion-text class="user-activity-empty__text">
          {{ $msTranslate('UsersPage.activity.noActivity') }}
        </ion-text>
      </div>
      <template v-else>
        <div
          v-for="[day, events] in groupedEvents"
          :key="day.toISO() || ''"
          class="activity-group"
        >
          <ion-text class="user-group__title">
            {{ $msTranslate(I18n.formatDate(day, 'short')) }}
          </ion-text>
          <div class="activity-group__events">
            <div
              v-for="(event, index) in events"
              :key="index"
              class="activity-event"
            >
              <div class="activity-event-timeline">
                <div
                  class="activity-event-timeline__icon"
                  :class="`activity-event-timeline__icon--${event.variant}`"
                >
                  <ms-image
                    v-if="event.image"
                    class="activity-event-timeline__image"
                    :image="event.image"
                  />
                  <ion-icon
                    v-else
                    :icon="event.icon"
                  />
                </div>
                <div class="activity-event-timeline__line" />
              </div>
              <div class="activity-event-content">
                <ms-rich-text
                  class="activity-event-content__text"
                  :text="event.text"
                />
                <ion-text class="activity-event-content__date">
                  {{ $msTranslate(I18n.formatDate(event.timestamp, 'full')) }}
                </ion-text>
              </div>
            </div>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getUserTimeline, UserTimelineEntry, UserTimelineEntryJoined, UserTimelineEntryRevoked, UserTimelineEntryTag } from '@/parsec';
import { exportAllUsers } from '@/services/export';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { useSidePopoverMenu } from '@/services/sidePopoverMenu';
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { close, personAdd, personRemove } from 'ionicons/icons';
import { DateTime } from 'luxon';
import {
  DownloadIcon,
  I18n,
  MsImage,
  MsReportText,
  MsReportTheme,
  MsRichText,
  MsSpinner,
  openSpinnerModal,
  Translatable,
} from 'megashark-lib';
import { computed, onMounted, ref } from 'vue';

type EventVariant = 'success' | 'success-subtle' | 'error' | 'error-subtle' | 'brand' | 'brand-subtle' | 'neutral';

type UserTimelineEventDisplay = UserTimelineEntry & {
  icon?: string;
  image?: string;
  variant: EventVariant;
  text: Translatable;
};

const props = defineProps<{
  informationManager: InformationManager;
}>();

const sidePopoverMenu = useSidePopoverMenu();
const loading = ref(true);
const rawEvents = ref<Array<UserTimelineEventDisplay>>([]);
const error = ref(false);

const groupedEvents = computed(() => {
  const grouped: Map<DateTime, Array<UserTimelineEventDisplay>> = new Map();
  if (rawEvents.value.length === 0) {
    return grouped;
  }
  let group = { day: rawEvents.value[0].timestamp, events: new Array<UserTimelineEventDisplay>() };
  for (const event of rawEvents.value) {
    if (!event.timestamp.hasSame(group.day, 'day')) {
      if (group.events.length > 0) {
        grouped.set(group.day, group.events);
      }
      group = { day: event.timestamp, events: new Array<UserTimelineEventDisplay>() };
    }
    group.events.push(event);
  }
  if (group.events.length > 0) {
    grouped.set(group.day, group.events);
  }
  return grouped;
});

onMounted(async () => {
  try {
    loading.value = true;
    const result = await getUserTimeline();
    if (result.ok) {
      rawEvents.value = result.value.map((event) => getUserTimelineEventDisplay(event));
    }
  } finally {
    loading.value = false;
  }
});

function getUserTimelineEventDisplay(event: UserTimelineEntry): UserTimelineEventDisplay {
  const entry = event;

  switch (entry.tag) {
    case UserTimelineEntryTag.Joined:
      const inviter = (event as UserTimelineEntryJoined).inviter;
      return {
        ...event,
        icon: personAdd,
        variant: 'success-subtle',
        text: {
          key: inviter ? 'UsersPage.activity.events.joinedBy' : 'UsersPage.activity.events.joined',
          data: { inviter: inviter?.label, user: event.user.label },
        },
      };
    case UserTimelineEntryTag.Revoked:
      const revoker = (event as UserTimelineEntryRevoked).revoker;
      return {
        ...event,
        icon: personRemove,
        variant: 'error-subtle',
        text: {
          key: revoker ? 'UsersPage.activity.events.revokedBy' : 'UsersPage.activity.events.revoked',
          data: { revoker: revoker?.label, user: event.user.label },
        },
      };
  }
}

async function exportUsers(): Promise<void> {
  const modal = await openSpinnerModal('export.exporting');
  const result = await exportAllUsers();
  if (!result) {
    props.informationManager.present(
      new Information({
        message: 'export.exportUsersFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
  modal.dismiss();
}
</script>

<style scoped lang="scss">
.user-activity {
  display: flex;
  flex-direction: column;
  height: 100%;
  position: relative;
  background: ms.color('surface-base-default');

  &::after {
    content: '';
    position: absolute;
    bottom: 0;
    left: 0;
    right: 0;
    height: 5rem;
    background: linear-gradient(to bottom, transparent, ms.color('surface-base-default'));
    pointer-events: none;
  }
}

.user-activity-header {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: ms.spacing('gap-lg');
  flex-shrink: 0;
  padding: ms.spacing('padding-5xl') ms.spacing('padding-4xl') ms.spacing('padding-4xl') ms.spacing('padding-5xl');
  border-bottom: ms.border('thin') solid ms.color('border-neutral-default-subtle');

  &__top {
    display: flex;
    align-items: flex-start;
    gap: ms.spacing('gap-3xl');
    width: 100%;
  }

  &__text {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-sm');
    flex: 1;
    min-width: 0;
    overflow-wrap: break-word;
  }

  &__title {
    @include ms.font('heading-h3');
    color: ms.color('text-base-heading');
  }

  &__subtitle {
    @include ms.font('body-sm-regular');
    color: ms.color('text-base-description');
  }

  &__close {
    --color: #{ms.color('icon-neutral-default')};
    --color-hover: #{ms.color('icon-neutral-default-hover')};
    --background: transparent;
    --background-hover: #{ms.color('surface-neutral-default-subtle-hover')};
    width: ms.size('xl');
    min-width: ms.size('xl');
    height: ms.size('xl');
    min-height: ms.size('xl');
    position: relative;
    right: -0.5rem;
    top: -1rem;

    &::part(native) {
      padding: ms.spacing('padding-xs');
    }

    ion-icon {
      font-size: ms.size('md');
      max-width: none;
    }
  }

  &__export-icon {
    width: 0.875rem;
    height: 0.875rem;
    --fill-color: #{ms.color('icon-neutral-default')};
  }
}

.user-activity-body {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: ms.spacing('padding-4xl') ms.spacing('padding-5xl') ms.spacing('padding-9xl');
}

.user-activity-spinner {
  align-self: center;
}

.user-activity-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: ms.spacing('gap-3xl');
  flex: 1;

  &__text {
    @include ms.font('body-md-medium');
    color: ms.color('text-base-description');
    text-align: center;
  }
}

.activity-group {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-2xl');

  &__title {
    @include ms.font('label-sm-medium');
    color: ms.color('text-base-description');
  }

  &__events {
    display: flex;
    flex-direction: column;
  }
}

.activity-event {
  display: flex;
  align-items: stretch;
  gap: ms.spacing('gap-2xl');

  &-timeline {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: ms.spacing('gap-sm');
    flex-shrink: 0;
    width: ms.size('lg');

    &__icon {
      display: flex;
      align-items: center;
      justify-content: center;
      flex-shrink: 0;
      width: ms.size('lg');
      height: ms.size('lg');
      border-radius: ms.radius('full');
      border: ms.border('thin') solid transparent;

      ion-icon {
        font-size: 1rem;
      }

      .activity-event-timeline__image {
        width: 1rem;
        height: 1rem;
      }

      &--success {
        background: ms.color('surface-success-default');
        border-color: ms.color('border-success-default');
        color: ms.color('icon-on-color-label');
      }

      &--success-subtle {
        background: ms.color('surface-success-default-subtle');
        border-color: ms.color('border-success-default-subtle');
        color: ms.color('icon-success-default');
      }

      &--error {
        background: ms.color('surface-error-default');
        border-color: ms.color('border-error-default');
        color: ms.color('icon-on-color-label');
      }

      &--error-subtle {
        background: ms.color('surface-error-default-subtle');
        border-color: ms.color('border-error-default-subtle');
        color: ms.color('icon-error-default');
      }

      &--brand {
        background: ms.color('surface-brand-default');
        border-color: ms.color('border-brand-default');
        color: ms.color('icon-on-color-label');
      }

      &--brand-subtle {
        background: ms.color('surface-brand-default-subtle');
        border-color: ms.color('border-brand-default-subtle');
        color: ms.color('icon-brand-default');
        --fill-color: #{ms.color('icon-brand-default')};
      }

      &--neutral {
        background: ms.color('surface-neutral-default');
        border-color: ms.color('border-neutral-default-subtle');
        color: ms.color('icon-on-color-label');
      }
    }

    &__line {
      flex: 1;
      width: ms.border('thin');
      background: ms.color('border-base-default');
    }
  }

  &-content {
    display: flex;
    flex-direction: column;
    gap: ms.spacing('gap-xs');
    flex: 1;
    min-width: 0;
    padding: ms.spacing('padding-md') 0 ms.spacing('padding-3xl');
    overflow-wrap: break-word;

    &__text {
      @include ms.font('body-md-regular');
      color: ms.color('text-base-body');
    }

    &__date {
      @include ms.font('body-sm-regular');
      color: ms.color('text-base-description');
    }
  }

  &:last-child {
    .activity-event-timeline__line {
      display: none;
    }

    .activity-event-content {
      padding-bottom: 0;
    }
  }
}
</style>
