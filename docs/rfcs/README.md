<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# About RFCs

This directory contains RFC (Request For Comments) for Parsec.

RFCs are technical documents proposing changes to solve specifics problem.
Its main purpose is to find the best way to solve a problem, as a team effort,
before the implementation starts.

- [About RFCs](#about-rfcs)
  - [Writing an RFC](#writing-an-rfc)
    - [RFC filename](#rfc-filename)
    - [RFC document structure (template)](#rfc-document-structure-template)
  - [Writing markdown](#writing-markdown)
    - [Link a RFC in another RFC](#link-a-rfc-in-another-rfc)
    - [Link a GitHub issue or pull request](#link-a-github-issue-or-pull-request)
    - [Suggested vscode extensions](#suggested-vscode-extensions)

## Writing an RFC

### RFC filename

We use the following pattern for RFC filenames: `<id:04d>-<title:s>.md`

- Where `id` is a serial number, it should be greater than the previous ids used
  > If the greater previous id used is `42` the next id should be `43`.
- Where `title` is the title used in the document (slight simplifications are accepted)
  - Use the `kebab-case`
  - only use alpha numeric character `[0-9a-zA-Z]`

### RFC document structure (template)

Please use the document structure from `0000-rfc-template.md` file.

It is inspired by <https://wasp-lang.dev/blog/2023/12/05/writing-rfcs> so take
a look if you have any doubts about it.

## Writing markdown

If you never written `markdown`, take a look at <https://www.markdownguide.org/basic-syntax/>

### Link a RFC in another RFC

To link another RFC in another RFC

```markdown
[RFC-NNNN](<RFC-FILENAME>)
```

> Where `<RFC-FILENAME>` is the filename of the linked rfc, relative to the rfc.

### Link a GitHub issue or pull request

If you want to link an issue or a pull request in the RFC use the following format:

- For issue:

  ```markdown
  [ISSUE-NNNN](https://github.com/Scille/parsec-cloud/issue/NNNN)
  ```

- For pull request:

  ```markdown
  [PR-NNNN](https://github.com/Scille/parsec-cloud/pull/NNNN)
  ```

### Suggested vscode extensions

- `yzhang.markdown-all-in-one`: Provides completion, some shortcuts & commands to simplify writing markdown.
- `DavidAnson.vscode-markdownlint`: Markdown linter providing a base style when writing markdown.
- `bierner.markdown-footnotes`: Allows to render `footnotes` on the preview.
