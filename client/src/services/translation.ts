// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { InvitationStatus, UserProfile, WorkspaceRole } from '@/parsec';
import { InvoiceStatus } from '@/services/bms';
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

export function getInvoiceStatusTranslationKey(status: InvoiceStatus): Translatable {
  switch (status) {
    case InvoiceStatus.Paid:
      return 'clientArea.invoices.status.paid';
    case InvoiceStatus.Draft:
      return 'clientArea.invoices.status.draft';
    case InvoiceStatus.Open:
      return 'clientArea.invoices.status.toBePaid';
    case InvoiceStatus.Uncollectible:
      return 'clientArea.invoices.status.uncollectible';
    default:
      return 'clientArea.invoices.status.void';
  }
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

export function getInvitationStatusTranslationKey(status: InvitationStatus): Translatable {
  switch (status) {
    case InvitationStatus.Pending:
      return 'UsersPage.invitation.status.pending';
    case InvitationStatus.Finished:
      return 'UsersPage.invitation.status.finished';
    case InvitationStatus.Cancelled:
      return 'UsersPage.invitation.status.cancelled';
  }
}

export function longLocaleCodeToShort(longCode: Locale): string {
  return longCode.split('-')[0];
}
