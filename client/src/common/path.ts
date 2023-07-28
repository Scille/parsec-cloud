// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

export function parse(path: string): string[] {
  if (path.length === 0 || path[0] !== '/') {
    throw new Error('Path should be at least "/" and start with "/"');
  }

  // Remove two "/" following each other
  path = path.replace(/\/\/+/g, '/');
  const splitted = path.split('/');

  // path is only "/" which transforms into an array with two empty strings
  if (splitted.length === 2 && splitted[0] === '' && splitted[1] === '') {
    return ['/'];
  }
  // Absolute path get an empty string as their first element, convert it to "/"
  if (splitted.length > 0 && splitted[0] === '') {
    splitted[0] = '/';
  }
  // Path ending with a "/" get an empty string as their last element, convert it to "/"
  if (splitted.length > 0 && splitted[splitted.length - 1] === '') {
    splitted.pop();
  }

  return splitted;
}

export function join(path: string, elem: string): string {
  return path.endsWith('/') ? `${path}${elem}` : `${path}/${elem}`;
}
