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

  const locale = 'clientArea.dashboard.processing';

  switch (status) {
    case undefined: {
      return {
        title: `${locale}.error.title`,
      };
    }
    case CustomOrderStatus.NothingLinked:
      return {
        title: `${locale}.requestSent.title`,
        description: `${locale}.requestSent.description`,
      };
    case CustomOrderStatus.EstimateLinked:
      return {
        title: `${locale}.estimateLinked.title`,
        description: `${locale}.estimateLinked.description`,
      };
    case CustomOrderStatus.InvoiceToBePaid:
      return {
        title: `${locale}.invoiceToBePaid.title`,
        description: `${locale}.invoiceToBePaid.description`,
      };
    case CustomOrderStatus.InvoicePaid:
      return {
        title: `${locale}.organizationAvailable.title`,
        description: `${locale}.organizationAvailable.description`,
      };
    default:
      return {
        title: `${locale}.error.title`,
        description: `${locale}.error.description`,
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
