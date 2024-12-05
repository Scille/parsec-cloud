// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { InvitationStatus, UserProfile, WorkspaceRole } from '@/parsec';
import { CustomOrderStatus } from '@/services/bms';
import { Locale, Translatable } from 'megashark-lib';

export function getProfileTranslationKey(profile: UserProfile): Translatable {
  if (profile === UserProfile.Admin) {
    return 'UsersPage.profile.admin.label';
  } else if (profile === UserProfile.Standard) {
    return 'UsersPage.profile.standard.label';
  } else if (profile === UserProfile.Outsider) {
    return 'UsersPage.profile.outsider.label';
  }
  return '';
}

interface WorkspaceRoleTranslations {
  label: Translatable;
  description?: Translatable;
}

export function getWorkspaceRoleTranslationKey(role: WorkspaceRole | null): WorkspaceRoleTranslations {
  switch (role) {
    case null: {
      return {
        label: 'workspaceRoles.none',
      };
    }
    case WorkspaceRole.Reader: {
      return {
        label: 'workspaceRoles.reader.label',
        description: 'workspaceRoles.reader.description',
      };
    }
    case WorkspaceRole.Contributor: {
      return {
        label: 'workspaceRoles.contributor.label',
        description: 'workspaceRoles.contributor.description',
      };
    }
    case WorkspaceRole.Manager: {
      return {
        label: 'workspaceRoles.manager.label',
        description: 'workspaceRoles.manager.description',
      };
    }
    case WorkspaceRole.Owner: {
      return {
        label: 'workspaceRoles.owner.label',
        description: 'workspaceRoles.owner.description',
      };
    }
  }
}

interface CustomOrderStatusTranslations {
  title: Translatable;
  description?: Translatable;
}

export function getCustomOrderStatusTranslationKey(status: CustomOrderStatus | undefined): CustomOrderStatusTranslations {
  switch (status) {
    case undefined: {
      return {
        title: 'clientArea.dashboard.processing.error.title',
      };
    }
    case CustomOrderStatus.NothingLinked:
      return {
        title: 'clientArea.dashboard.processing.requestSent.title',
        description: 'clientArea.dashboard.processing.requestSent.description',
      };
    case CustomOrderStatus.EstimateLinked:
      return {
        title: 'clientArea.dashboard.processing.estimateLinked.title',
        description: 'clientArea.dashboard.processing.estimateLinked.description',
      };
    case CustomOrderStatus.InvoiceToBePaid:
      return {
        title: 'clientArea.dashboard.processing.invoiceToBePaid.title',
        description: 'clientArea.dashboard.processing.invoiceToBePaid.description',
      };
    case CustomOrderStatus.InvoicePaid:
      return {
        title: 'clientArea.dashboard.processing.organizationAvailable.title',
        description: 'clientArea.dashboard.processing.organizationAvailable.description',
      };
    default:
      return {
        title: 'clientArea.dashboard.processing.error.title',
        description: 'clientArea.dashboard.processing.error.description',
      };
  }
}

export function getInvitationStatusTranslationKey(status: InvitationStatus): Translatable {
  switch (status) {
    case InvitationStatus.Ready:
      return 'UsersPage.invitation.status.ready';
    case InvitationStatus.Idle:
      return 'UsersPage.invitation.status.idle';
    case InvitationStatus.Finished:
      return 'UsersPage.invitation.status.finished';
    case InvitationStatus.Cancelled:
      return 'UsersPage.invitation.status.cancelled';
  }
}

export function longLocaleCodeToShort(longCode: Locale): string {
  return longCode.split('-')[0];
}
