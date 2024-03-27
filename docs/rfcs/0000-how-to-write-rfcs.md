<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# How to write RFCs

This document indicates the `do` & `don't` when writing an RFC file.

- [How to name the RFC file](#how-to-name-the-rfc-file)
- [How to write markdown](#how-to-write-markdown)
- [Link an RFC in another RFC](#link-an-rfc-in-another-rfc)
- [Link a GitHub issue or pull request](#link-a-github-issue-or-pull-request)
- [Suggested vscode extensions](#suggested-vscode-extensions)

## How to name the RFC file

We use a specific filename format for RFC files: `<id:04d>-<title:s>.md`

- Where `id` is a serial number, it should be greater than the previous IDs used
  > If the greater previous ID used is `42` the next ID should be `43`.
- Where `title` should be the title used in the document
  - The title could change a little
  - Use the `kebab-case`
  - only use alphanumeric character `[0-9a-zA-Z]`

## How to write markdown

If you have never written `markdown`, take a look at <https://www.markdownguide.org/basic-syntax/>

## Link an RFC in another RFC

To link another RFC in another RFC

```markdown
[RFC-NNNN](<RFC-FILENAME>)
```

> Where `<RFC-FILENAME>` is the filename of the linked RFC, relative to the RFC.

## Link a GitHub issue or pull request

If you want to link an issue or a pull request in the RFC use the following format:

- For issue

  ```markdown
  [ISSUE-NNNN](https://github.com/foo/bar/issue/NNNN)
  ```

- For pull request

  ```markdown
  [PR-NNNN](https://github.com/foo/bar/pull/NNNN)
  ```

## Suggested vscode extensions

I recommend using these VS-Code extensions:

- `yzhang.markdown-all-in-one`: Provide completion, some shortcut & command to simplify writing Markdown.
- `DavidAnson.vscode-markdownlint`: Add a markdown linter to provide a base style when writing markdown file.
- `bierner.markdown-footnotes`: Allow to render the `footnotes` on the preview.
