const descrUser1 = [
  'File author by default',
  "Doesn't belong to any group",
  'Can review all the changes',
  'Can perform all actions with comments',
  'The file favorite state is undefined',
  'Can create files from templates using data from the editor'
  // 'Can submit forms'
];

const descrUser2 = [
  'Belongs to Group2',
  'Can review only his own changes or changes made by users with no group',
  'Can view comments, edit his own comments and comments left by users with no group. Can remove his own comments only',
  'This file is marked as favorite',
  'Can create new files from the editor'
  // 'Can't submit forms'
];

const descrUser3 = [
  'Belongs to Group3',
  'Can review changes made by Group2 users',
  'Can view comments left by Group2 and Group3 users. Can edit comments left by the Group2 users',
  "This file isn't marked as favorite",
  "Can't copy data from the file to clipboard",
  "Can't download the file",
  "Can't print the file",
  'Can create new files from the editor'
  // 'Can't submit forms'
];

const descrUser0 = [
  'The name is requested when the editor is opened',
  "Doesn't belong to any group",
  'Can review all the changes',
  'Can perform all actions with comments',
  'The file favorite state is undefined',
  "Can't mention others in comments",
  "Can't create new files from the editor"
  // 'Can't submit forms'
];

class User {
  id: string;
  name: string|null;
  email: string|null;
  group: string|null;
  reviewGroups: string[]|null;
  commentGroups: any;
  favorite: boolean|null;
  deniedPermissions: any[];
  descriptions: string[];
  templates: boolean;

  constructor(
    id: string,
    name: string|null,
    email: string|null,
    group: string|null,
    reviewGroups: string[]|null,
    commentGroups: any,
    favorite: boolean|null,
    deniedPermissions: any[],
    descriptions: string[],
    templates: boolean) {
    this.id = id;
    this.name = name;
    this.email = email;
    this.group = group;
    this.reviewGroups = reviewGroups;
    this.commentGroups = commentGroups;
    this.favorite = favorite;
    this.deniedPermissions = deniedPermissions;
    this.descriptions = descriptions;
    this.templates = templates;
  }
}

export const users = [
  new User('uid-1', 'John Smith', 'smith@example.com',
    null, null, {},
    null, [], descrUser1, true),
  new User('uid-2', 'Mark Pottato', 'pottato@example.com',
    'group-2', ['group-2', ''], {
      view: '',
      edit: ['group-2', ''],
      remove: ['group-2']
    },
    true, [], descrUser2, false),  // own and without group
  new User('uid-3', 'Hamish Mitchell', 'mitchell@example.com',
    'group-3', ['group-2'], {
      view: ['group-3', 'group-2'],
      edit: ['group-2'],
      remove: []
    },
    false, ['copy', 'download', 'print'], descrUser3, false),  // other group only
  new User('uid-0', null, null,
    null, null, {},
    null, [], descrUser0, false)
];

// get a list of all the users
(users as any).getAllUsers = function (): User[] {
  return users;
};

// get a user by id specified
(users as any).getUser = function (id: string): User {
  let result = null;
  this.forEach((user: User) => {
    if (user.id === id) {
      result = user;
    }
  });
  return result ? result : this[0];
};

// get a list of users with their names and emails for mentions
(users as any).getUsersForMentions = function (id: string): any[] {
  const result: any[] = [];
  this.forEach((user: User) => {
    if (user.id !== id && user.name !== null && user.email !== null) {
      result.push({ name: user.name, email: user.email });
    }
  });
  return result;
};
