// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { statFolderChildren } from '@/parsec/file';
import { EntryName, EntryStat, FsPath, WorkspaceHandle } from '@/parsec/types';
import DOMPurify from 'dompurify';
import * as FastFuzzy from 'fast-fuzzy';

export interface SearchResult {
  stats: EntryStat;
  parent: FsPath;
  titleMatch: boolean;
  highlightedName?: string;
  highlightedPath?: string;
}

function sanitize(name: EntryName | FsPath): string {
  // Highlight is done using v-html, and the file names are user inputs.
  return DOMPurify.sanitize(name, { ALLOWED_TAGS: ['span'], ALLOWED_ATTR: ['class'] });
}

interface MatchResult {
  index: number;
  length: number;
}

function match(pattern: string, value: FsPath | EntryName): Array<MatchResult> {
  const matches = [];
  let index = 0;
  // It matches the first occurrence, so we call it multiple times, moving the index in the string
  while (index < value.length) {
    const result = FastFuzzy.search(pattern, [value.slice(index)], {
      returnMatchData: true,
      ignoreCase: true,
      ignoreSymbols: false,
      threshold: pattern.length < 5 ? 0.75 : 0.5,
    });
    if (result && result.length > 0) {
      matches.push({ index: result[0].match.index + index, length: result[0].match.length });
      index += result[0].match.index + result[0].match.length;
    } else {
      index += value.length;
    }
  }
  return matches;
}

function highlight(value: FsPath | EntryName, matches: Array<MatchResult>): string {
  let result = '';
  let lastIndex = 0;

  for (const matchResult of matches) {
    result += value.slice(lastIndex, matchResult.index);
    const matchText = value.slice(matchResult.index, matchResult.index + matchResult.length);
    result += `<span class="highlight">${matchText}</span>`;
    lastIndex = matchResult.index + matchResult.length;
  }
  result += value.slice(lastIndex);
  return result;
}

export async function fileSearch(
  workspaceHandle: WorkspaceHandle,
  root: FsPath,
  pattern: string,
  results: Array<SearchResult>,
  signal: AbortSignal,
): Promise<void> {
  const rootSanitized = sanitize(root);
  const rootResult = match(pattern, rootSanitized);
  const stack: Array<{ path: FsPath; highlightedParent?: string }> = [
    {
      path: root,
      highlightedParent: rootResult.length ? highlight(rootSanitized, rootResult) : undefined,
    },
  ];

  // Going through the stack of folders
  while (stack.length) {
    if (signal.aborted) {
      return;
    }
    const { path, highlightedParent } = stack.pop()!;
    const statsResult = await statFolderChildren(workspaceHandle, path, true, true);
    if (!statsResult.ok) {
      continue;
    }
    for (const entry of statsResult.value) {
      const sanitized = sanitize(entry.name);
      const result = match(pattern, sanitized);

      if (result.length) {
        results.push({
          stats: entry,
          parent: path,
          titleMatch: true,
          highlightedName: highlight(sanitized, result),
          highlightedPath: highlightedParent,
        });
      } else if (highlightedParent) {
        results.push({
          stats: entry,
          parent: path,
          titleMatch: false,
          highlightedName: undefined,
          highlightedPath: highlightedParent,
        });
      }
      if (!entry.isFile()) {
        const pathSanitized = sanitize(entry.path);
        const pathResult = match(pattern, pathSanitized);
        stack.push({
          path: entry.path,
          highlightedParent: pathResult.length ? highlight(pathSanitized, pathResult) : undefined,
        });
      }

      if (signal.aborted) {
        return;
      }
    }
  }
}
