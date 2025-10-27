<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

<template>
  <ion-item
    button
    id="click-trigger"
    class="profile-header ion-no-padding"
    :class="{ 'profile-header-clicked': isPopoverOpen }"
    @click="openHomepagePopover($event)"
  >
    <div
      class="avatar small"
      :class="{ online: isOnline, offline: !isOnline }"
    >
      <ion-icon
        :icon="personCircle"
        class="avatar-icon"
      />
    </div>
    <div class="text-content">
      <div class="text-content-name">
        <ion-text class="text-content-name__text subtitles-normal">
          {{ name }}
        </ion-text>
        <ion-icon
          :class="{ 'popover-is-open': isPopoverOpen }"
          slot="end"
          :icon="chevronDown"
        />
      </div>
    </div>
  </ion-item>
</template>

<script setup lang="ts">
import { Env } from '@/services/environment';
import { openAboutModal } from '@/views/about';
import { AccountSettingsTabs } from '@/views/account/types';
import ProfileHeaderHomepagePopover, { ProfilePopoverHomepageOption } from '@/views/header/ProfileHeaderHomepagePopover.vue';
import { IonIcon, IonItem, IonText, popoverController } from '@ionic/vue';
import { chevronDown, personCircle } from 'ionicons/icons';
import { ref, Ref } from 'vue';

const isPopoverOpen = ref(false);

const tab: Ref<AccountSettingsTabs> = ref(AccountSettingsTabs.Settings);

const props = defineProps<{
  name: string;
  email: string;
  isOnline: boolean;
}>();

async function openHomepagePopover(event: Event): Promise<void> {
  isPopoverOpen.value = !isPopoverOpen.value;
  const popover = await popoverController.create({
    component: ProfileHeaderHomepagePopover,
    cssClass: 'profile-header-homepage-popover',
    componentProps: {
      email: props.email,
    },
    event: event,
    showBackdrop: false,
    alignment: 'end',
  });
  await popover.present();

  const { data } = await popover.onDidDismiss();
  isPopoverOpen.value = false;

  if (data === undefined) {
    return;
  }
  if (
    data.option === ProfilePopoverHomepageOption.Settings ||
    data.option === ProfilePopoverHomepageOption.Account ||
    data.option === ProfilePopoverHomepageOption.Authentication
  ) {
    if (data.option === ProfilePopoverHomepageOption.Account) {
      tab.value = AccountSettingsTabs.Account;
    } else if (data.option === ProfilePopoverHomepageOption.Authentication) {
      tab.value = AccountSettingsTabs.Authentication;
    }
    if (data?.tab) {
      emit('changeTab', data.tab);
    }
  } else if (data.option === ProfilePopoverHomepageOption.Documentation) {
    await Env.Links.openDocumentationLink();
  } else if (data.option === ProfilePopoverHomepageOption.Feedback) {
    await Env.Links.openContactLink();
  } else if (data.option === ProfilePopoverHomepageOption.About) {
    await openAboutModal();
  }
}

const emit = defineEmits<{
  (e: 'changeTab', tab: AccountSettingsTabs): void;
}>();
</script>

<style lang="scss" scoped>
.profile-header {
  --background: none;
  --background-hover: none;
  border-radius: var(--parsec-radius-12);
  padding: 0.375rem 0.5rem 0.375rem 0.25rem;
  border: 1px solid transparent;
  transition: all ease-in-out 200ms;
  flex-shrink: 0;

  & * {
    pointer-events: none;
  }

  &:hover {
    --background-hover: none;
    border-color: var(--parsec-color-light-secondary-medium);
    background: var(--parsec-color-light-secondary-background);
  }

  &-clicked {
    border-color: var(--parsec-color-light-secondary-medium);
    background: var(--parsec-color-light-secondary-background);
  }
}

.avatar {
  position: relative;
  color: var(--parsec-color-light-primary-600);
  display: flex;
  margin-right: 0.5rem;

  @include ms.responsive-breakpoint('md') {
    margin: 0;
  }

  &-icon {
    font-size: 2rem;
  }

  &::after {
    content: '';
    position: absolute;
    bottom: 0px;
    right: 0px;
    height: 0.5rem;
    width: 0.5rem;
    border-radius: 50%;
    border: var(--parsec-color-light-secondary-white) solid 2px;
    z-index: 2;
  }

  &.online::after {
    background-color: var(--parsec-color-light-success-500);
  }

  &.offline::after {
    background-color: var(--parsec-color-light-secondary-light);
  }
}

.text-content {
  display: flex;
  flex-direction: column;
  width: 100%;

  &-name {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--parsec-color-light-secondary-soft-text);

    &__text {
      font-size: 0.9375rem;

      @include ms.responsive-breakpoint('md') {
        display: none;
      }
    }

    ion-icon {
      transition: transform ease-out 300ms;
      font-size: 1rem;

      &.popover-is-open {
        transform: rotate(180deg);
      }
    }
  }
}
</style>
