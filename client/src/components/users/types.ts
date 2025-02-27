// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { UserInfo, UserProfile } from '@/parsec';

export enum SortProperty {
  Name = 'sort-name',
  JoinedDate = 'sort-joined-data',
  Status = 'sort-status',
  Profile = 'sort-profile',
}

export enum InvitationAction {
  Greet = 'greet',
  Cancel = 'cancel',
  Invite = 'invite',
}

export interface UserModel extends UserInfo {
  isSelected: boolean;
  isCurrent: boolean;
}

function compareUserProfiles(profile1: UserProfile, profile2: UserProfile): number {
  const WEIGHTS = new Map<UserProfile, number>([
    [UserProfile.Admin, 3],
    [UserProfile.Standard, 2],
    [UserProfile.Outsider, 1],
  ]);

  return WEIGHTS.get(profile2)! - WEIGHTS.get(profile1)!;
}

export interface UserFilterLabels {
  statusActive?: boolean;
  statusRevoked?: boolean;
  statusFrozen?: boolean;
  profileAdmin?: boolean;
  profileStandard?: boolean;
  profileOutsider?: boolean;
}

export interface UserFilterChangeEvent {
  filters: UserFilterLabels;
}

export class UserCollection {
  users: Array<UserModel>;
  filters: UserFilterLabels;
  _searchFilter: string;

  constructor() {
    this.users = [];
    this.filters = {
      statusActive: true,
      statusRevoked: true,
      statusFrozen: true,
      profileAdmin: true,
      profileStandard: true,
      profileOutsider: true,
    };
    this._searchFilter = '';
  }

  public set searchFilter(value: string) {
    this._searchFilter = value.toLocaleLowerCase();
  }

  getFilters(): UserFilterLabels {
    return this.filters;
  }

  setFilters(filters: UserFilterLabels): void {
    this.filters.statusActive = filters.statusActive;
    this.filters.statusRevoked = filters.statusRevoked;
    this.filters.statusFrozen = filters.statusFrozen;
    this.filters.profileAdmin = filters.profileAdmin;
    this.filters.profileStandard = filters.profileStandard;
    this.filters.profileOutsider = filters.profileOutsider;
  }

  hasSelected(): boolean {
    return this.getSelectedUsers().length > 0;
  }

  selectAll(selected: boolean): void {
    for (const entry of this.users) {
      if (this.userIsVisible(entry) && !entry.isRevoked() && !entry.isCurrent) {
        entry.isSelected = selected;
      }
    }
  }

  getUsers(): Array<UserModel> {
    return this.users.filter((user) => {
      return this.userIsVisible(user);
    });
  }

  unselectHiddenUsers(): void {
    for (const entry of this.users) {
      if (!this.userIsVisible(entry)) {
        entry.isSelected = false;
      }
    }
  }

  totalUsersCount(): number {
    return this.users.length;
  }

  usersCount(): number {
    return this.getUsers().length;
  }

  private userIsVisible(user: UserModel): boolean {
    if (
      (!this.filters.profileAdmin && user.currentProfile === UserProfile.Admin) ||
      (!this.filters.profileStandard && user.currentProfile === UserProfile.Standard) ||
      (!this.filters.profileOutsider && user.currentProfile === UserProfile.Outsider)
    ) {
      return false;
    }
    if (
      (!this.filters.statusRevoked && user.isRevoked()) ||
      (!this.filters.statusActive && user.isActive()) ||
      (!this.filters.statusFrozen && user.isFrozen())
    ) {
      return false;
    }
    if (
      !user.humanHandle.label.toLocaleLowerCase().includes(this._searchFilter) &&
      !user.humanHandle.email.toLocaleLowerCase().includes(this._searchFilter)
    ) {
      return false;
    }
    return true;
  }

  getSelectedUsers(): Array<UserModel> {
    return this.users.filter((user) => user.isSelected && this.userIsVisible(user));
  }

  selectableUsersCount(): number {
    return this.users.filter((user) => this.userIsVisible(user) && !user.isRevoked() && !user.isCurrent).length;
  }

  selectedCount(): number {
    return this.getSelectedUsers().length;
  }

  private statusDiff(user1: UserModel, user2: UserModel): number {
    return (
      Number(user2.isActive()) * 4 +
      Number(user2.isRevoked()) * 2 +
      Number(user2.isFrozen()) -
      (Number(user1.isActive()) * 4 + Number(user1.isRevoked()) * 2 + Number(user1.isFrozen()))
    );
  }

  sort(property: SortProperty, ascending: boolean): void {
    this.users.sort((user1, user2) => {
      // Arbitrary value to keep the current user always at the top of the list
      if (user1.isCurrent) {
        return -42;
      } else if (user2.isCurrent) {
        return 42;
      }

      let diff = 0;
      const profile1 = ascending ? user1.currentProfile : user2.currentProfile;
      const profile2 = ascending ? user2.currentProfile : user1.currentProfile;
      diff = compareUserProfiles(profile1, profile2);

      switch (property) {
        case SortProperty.Name:
          return ascending
            ? user1.humanHandle.label.localeCompare(user2.humanHandle.label)
            : user2.humanHandle.label.localeCompare(user1.humanHandle.label);
        case SortProperty.JoinedDate:
          if (ascending) {
            return user2.createdOn < user1.createdOn ? -1 : 0;
          } else {
            return user1.createdOn < user2.createdOn ? -1 : 0;
          }
        case SortProperty.Profile:
          if (profile1 === profile2) {
            return this.statusDiff(user1, user2);
          }
          return diff;
        case SortProperty.Status:
          return ascending ? this.statusDiff(user1, user2) : this.statusDiff(user2, user1);
        default:
          return 0;
      }
    });
  }

  hasInactive(): boolean {
    return this.users.some((u) => u.isRevoked() || u.isFrozen());
  }

  getCurrentUser(): UserModel | undefined {
    const users = this.users.filter((u) => u.isCurrent);
    return users.length ? users[0] : undefined;
  }

  clear(): void {
    this.users = [];
  }

  append(entry: UserModel): void {
    this.users.push(entry);
  }

  replace(entries: UserModel[]): void {
    this.clear();
    this.users = entries;
  }
}
