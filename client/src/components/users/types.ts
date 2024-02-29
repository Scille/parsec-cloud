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

export class UserCollection {
  users: Array<UserModel>;

  constructor() {
    this.users = [];
  }

  hasSelected(): boolean {
    return this.users.find((user) => user.isSelected) !== undefined;
  }

  selectAll(selected: boolean): void {
    for (const entry of this.users.filter((user) => !user.isRevoked())) {
      entry.isSelected = selected;
    }
  }

  getUsers(): Array<UserModel> {
    return this.users;
  }

  usersCount(): number {
    return this.users.length;
  }

  getSelectedUsers(): Array<UserModel> {
    return this.users.filter((user) => user.isSelected && !user.isRevoked());
  }

  selectedCount(): number {
    return this.users.filter((user) => user.isSelected && !user.isRevoked()).length;
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
