<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0-only 2016-present Scille SAS -->

<template>
  <div class="workspace-activity">
    <div class="workspace-activity-header">
      <div class="workspace-activity-header__top">
        <div class="workspace-activity-header__text">
          <ion-text class="workspace-activity-header__title">
            {{ $msTranslate('WorkspaceActivity.title') }}
          </ion-text>
          <ion-text class="workspace-activity-header__subtitle">
            {{ workspace.name }}
          </ion-text>
        </div>
        <ion-button
          class="workspace-activity-header__close"
          fill="clear"
          slot="icon-only"
          @click="sidePopoverMenu.close()"
        >
          <ion-icon :icon="close" />
        </ion-button>
      </div>
      <ion-button
        v-show="workspace.selfRole === WorkspaceRole.Owner"
        class="workspace-activity-header__export button-small color-secondary"
        fill="outline"
        @click="downloadWorkspaceExport"
      >
        <ms-image
          class="workspace-activity-header__export-icon button-icon-left"
          :image="DownloadIcon"
        />
        {{ $msTranslate('WorkspaceActivity.exportAll') }}
      </ion-button>
    </div>

    <div class="workspace-activity-body">
      <ms-spinner
        v-if="loading"
        class="workspace-activity-spinner"
      />
      <div
        v-else-if="error"
        class="workspace-activity-empty"
      >
        <ms-report-text :theme="MsReportTheme.Error">
          {{ $msTranslate('WorkspaceActivity.loadError') }}
        </ms-report-text>
      </div>
      <!-- Manage empty state -->
      <div
        v-else-if="groupedEvents.size === 0"
        class="workspace-activity-empty"
      >
        <ion-text class="workspace-activity-empty__text">
          {{ $msTranslate('WorkspaceActivity.noActivity') }}
        </ion-text>
      </div>
      <template v-else>
        <div
          v-for="[day, events] in groupedEvents"
          :key="day.toISO() || ''"
          class="activity-group"
        >
          <ion-text class="activity-group__title">
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
                  {{ $msTranslate(I18n.formatDate(event.entry.timestamp, 'full')) }}
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
import { getWorkspaceTimeline, WorkspaceInfo, WorkspaceRole, WorkspaceTimelineEntryTag, WorkspaceTimelineEvent } from '@/parsec';
import { exportWorkspace } from '@/services/export';
import { Information, InformationLevel, InformationManager, PresentationMode } from '@/services/informationManager';
import { useSidePopoverMenu } from '@/services/sidePopoverMenu';
import { getWorkspaceRoleTranslationKey } from '@/services/translation';
import { IonButton, IonIcon, IonText } from '@ionic/vue';
import { archive, business, close, personAdd, personRemove, reload, shareSocial, trash } from 'ionicons/icons';
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
  RenameIcon,
  Translatable,
} from 'megashark-lib';
import { computed, onMounted, ref } from 'vue';

type EventVariant = 'success' | 'success-subtle' | 'error' | 'error-subtle' | 'brand' | 'brand-subtle' | 'neutral';

interface WorkspaceTimelineEventDisplay extends WorkspaceTimelineEvent {
  icon?: string;
  image?: string;
  variant: EventVariant;
  text: Translatable;
}

const props = defineProps<{
  workspace: WorkspaceInfo;
  informationManager: InformationManager;
}>();

const sidePopoverMenu = useSidePopoverMenu();
const loading = ref(true);
const rawEvents = ref<Array<WorkspaceTimelineEventDisplay>>([]);
const error = ref(false);

const groupedEvents = computed(() => {
  const grouped: Map<DateTime, Array<WorkspaceTimelineEventDisplay>> = new Map();
  if (rawEvents.value.length === 0) {
    return grouped;
  }
  let group = { day: rawEvents.value[0].entry.timestamp, events: new Array<WorkspaceTimelineEventDisplay>() };
  for (const event of rawEvents.value) {
    if (!event.entry.timestamp.hasSame(group.day, 'day')) {
      if (group.events.length > 0) {
        grouped.set(group.day, group.events);
      }
      group = { day: event.entry.timestamp, events: new Array<WorkspaceTimelineEventDisplay>() };
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
    const result = await getWorkspaceTimeline(props.workspace.id);
    if (!result.ok) {
      error.value = true;
    } else {
      rawEvents.value = result.value.map((event) => getWorkspaceTimelineEventDisplay(event));
    }
  } finally {
    loading.value = false;
  }
});

function getRoleLabel(role: WorkspaceRole): string {
  return I18n.translate(getWorkspaceRoleTranslationKey(role).label);
}

function getWorkspaceTimelineEventDisplay(event: WorkspaceTimelineEvent): WorkspaceTimelineEventDisplay {
  const author = event.author?.label ?? I18n.translate('WorkspaceActivity.unknownUser');
  const entry = event.entry;

  switch (entry.tag) {
    case WorkspaceTimelineEntryTag.Created:
      return {
        ...event,
        icon: business,
        variant: 'brand',
        text: {
          key: 'WorkspaceActivity.events.created',
          data: { author, name: entry.initialName },
        },
      };
    case WorkspaceTimelineEntryTag.Renamed:
      return {
        ...event,
        image: RenameIcon,
        variant: 'brand-subtle',
        text: entry.previousName
          ? { key: 'WorkspaceActivity.events.renamed', data: { author, previousName: entry.previousName, newName: entry.newName ?? '' } }
          : { key: 'WorkspaceActivity.events.renamedTo', data: { author, newName: entry.newName ?? '' } },
      };
    case WorkspaceTimelineEntryTag.Archived:
      return {
        ...event,
        icon: archive,
        variant: 'neutral',
        text: {
          key: 'WorkspaceActivity.events.archived',
          data: { author },
        },
      };
    case WorkspaceTimelineEntryTag.Restored:
      return {
        ...event,
        icon: reload,
        variant: 'success',
        text: {
          key: 'WorkspaceActivity.events.restored',
          data: { author },
        },
      };
    case WorkspaceTimelineEntryTag.DeletionPlanned:
      return {
        ...event,
        icon: trash,
        variant: 'error',
        text: {
          key: 'WorkspaceActivity.events.deletionPlanned',
          data: { when: I18n.translate(I18n.formatDate(entry.deletionDate, 'long')) },
        },
      };
    case WorkspaceTimelineEntryTag.UserRoleChanged: {
      const recipient = event.recipient?.label;
      if (entry.previousRole === null) {
        return {
          ...event,
          icon: personAdd,
          variant: 'success-subtle',
          text: {
            key: 'WorkspaceActivity.events.shared',
            data: { author, with: recipient, role: getRoleLabel(entry.newRole as WorkspaceRole) },
          },
        };
      }
      if (entry.newRole === null) {
        return {
          ...event,
          icon: personRemove,
          variant: 'error-subtle',
          text: {
            key: 'WorkspaceActivity.events.unshared',
            data: { author, with: recipient },
          },
        };
      }
      return {
        ...event,
        icon: shareSocial,
        variant: 'brand-subtle',
        text: {
          key: 'WorkspaceActivity.events.roleChanged',
          data: { author, with: recipient, role: getRoleLabel(entry.newRole) },
        },
      };
    }
  }
}

async function downloadWorkspaceExport(): Promise<void> {
  const modal = await openSpinnerModal('export.exporting');
  const result = await exportWorkspace(props.workspace.handle);
  if (!result) {
    props.informationManager.present(
      new Information({
        message: 'export.exportWorkspaceFailed',
        level: InformationLevel.Error,
      }),
      PresentationMode.Toast,
    );
  }
  modal.dismiss();
}
</script>

<style scoped lang="scss">
.workspace-activity {
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

.workspace-activity-header {
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

.workspace-activity-body {
  display: flex;
  flex-direction: column;
  gap: ms.spacing('gap-4xl');
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: ms.spacing('padding-4xl') ms.spacing('padding-5xl') ms.spacing('padding-9xl');
}

// will be implemented with filter popovers
// .workspace-activity-filters {
//   display: flex;
//   justify-content: flex-end;
//   gap: ms.spacing('gap-lg');

//   &__button {
//     &.opened {
//       --background: #{ms.color('surface-neutral-default-subtle-hover')};
//       --border-color: #{ms.color('border-neutral-default-subtle-hover')};
//       --color: #{ms.color('text-neutral-default-hover')};
//     }
//   }
// }

.workspace-activity-spinner {
  align-self: center;
}

.workspace-activity-empty {
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
