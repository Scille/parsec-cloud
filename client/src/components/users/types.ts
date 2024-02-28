// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { UserInfo } from '@/parsec';

export enum SortProperty {
  Name,
  Email,
  JoinedDate,
  Status,
  Profile,
}

export interface UserModel extends UserInfo {
  isSelected: boolean;
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
    for (const entry of this.users) {
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
    return this.users.filter((user) => user.isSelected);
  }

  selectedCount(): number {
    return this.users.filter((user) => user.isSelected).length;
  }

  revokedSelectedCount(): number {
    return this.users.filter((user) => user.isSelected && user.isRevoked()).length;
  }

  sort(_property: SortProperty, _ascending: boolean): void {
    this.users.sort((_item1, _item2) => {
      return 1;
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
