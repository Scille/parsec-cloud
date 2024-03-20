// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { UserInfo, UserProfile } from '@/parsec';

export enum SortProperty {
  Name,
  JoinedDate,
  Status,
  Profile,
}

export interface UserModel extends UserInfo {
  isSelected: boolean;
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

  constructor() {
    this.users = [];
    this.filters = {
      statusActive: true,
      statusRevoked: true,
      profileAdmin: true,
      profileStandard: true,
      profileOutsider: true,
    };
  }

  hasSelected(): boolean {
    return this.getSelectedUsers().length > 0;
  }

  selectAll(selected: boolean): void {
    for (const entry of this.users) {
      if (this.userIsVisible(entry) && !entry.isRevoked()) {
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
    if ((!this.filters.statusRevoked && user.isRevoked()) || (!this.filters.statusActive && !user.isRevoked())) {
      return false;
    }
    return true;
  }

  getSelectedUsers(): Array<UserModel> {
    return this.users.filter((user) => user.isSelected && this.userIsVisible(user));
  }

  selectableUsersCount(): number {
    return this.users.filter((user) => this.userIsVisible(user) && !user.isRevoked()).length;
  }

  selectedCount(): number {
    return this.getSelectedUsers().length;
  }

  sort(property: SortProperty, ascending: boolean): void {
    this.users.sort((user1, user2) => {
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
            return user2.isRevoked() && !user1.isRevoked() ? -1 : 0;
          }
          return diff;
        case SortProperty.Status:
          if (user2.isRevoked() === user1.isRevoked()) {
            return ascending ? diff : -diff;
          }
          if (ascending) {
            return user2.isRevoked() && !user1.isRevoked() ? -1 : 0;
          } else {
            return user1.isRevoked() && !user2.isRevoked() ? -1 : 0;
          }
        default:
          return 0;
      }
    });
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
