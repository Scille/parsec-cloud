# Parsec Development Workflow

This document describes the Parsec Development Workflow and includes some
guidelines and best practices to create, review and merge pull requests in
Parsec.

- [Parsec Development Workflow](#parsec-development-workflow)
  - [Overview](#overview)
  - [Opening a pull request](#opening-a-pull-request)
    - [What makes a good pull request](#what-makes-a-good-pull-request)
    - [How to write a useful description](#how-to-write-a-useful-description)
    - [How to keep a sanitized commit history](#how-to-keep-a-sanitized-commit-history)
    - [Pull request categories](#pull-request-categories)
    - [The pull request checklist](#the-pull-request-checklist)
  - [Reviewing a pull request](#reviewing-a-pull-request)
    - [The automated review](#the-automated-review)
    - [The peer review](#the-peer-review)
      - [The code owner](#the-code-owner)
    - [Specific pull request reviews](#specific-pull-request-reviews)
      - [Pull request: fix typos](#pull-request-fix-typos)
      - [Pull request: update dependencies](#pull-request-update-dependencies)
  - [Merging a pull request](#merging-a-pull-request)
    - [Why do we want a linear git history](#why-do-we-want-a-linear-git-history)
    - [What is a merge queue](#what-is-a-merge-queue)
  - [Interesting sources](#interesting-sources)

## Overview

All development happens on [GitHub](https://github.com/Scille/) following a
branch-based workflow:

1. create a branch
2. make changes
3. open a pull request
4. discuss and review your code
5. merge

Please read the [GitHub Flow quickstart](https://docs.github.com/en/get-started/quickstart/github-flow)
for general tips and best practices on these steps.

## Opening a pull request

You open a pull request to ask collaborators for feedback on your changes:

- when you have little or no code but want to share some general ideas,
- when you’re stuck and need help or advice,
- when you’re ready for someone to review your work

If you want early feedback or advice before you complete your changes, **please
mark your pull request as a draft**.

### What makes a good pull request

When you create a pull request, please follow the following guidelines:

1. **Smaller is better**. You should include the minimal set of changes required
   to address the subject of the pull request.
   - If the issue you are working on requires changing several software
     components at once, take some time after the "job is done" to split those
     changes into several small, coherent, pull request.
   - This reduces the time and effort required to review and merge the pull
     request
2. Include a **meaningful title**, describing what is being solved at a high
   level. Pull request are scanned and triaged by people, a good title will save
   some time otherwise spent reading detailed descriptions and understanding what
   is it about.
3. For non-trivial changes, **write a useful description** to explain your
   changes to the reviewer(s). See [How to write a useful description](#how-to-write-a-useful-description).
4. If the pull request addresses an issue, please **link the issue** in the
   description.
5. If some other pull requests should be merged before, **list dependencies** in
   the description.
6. Ensure the [commit history is sanitized](#how-to-keep-a-sanitized-commit-history).
7. It should be affected to the related categories listed in [Pull request categories](#pull-request-categories).
8. Complete [the Pull-request checklist](#the-pull-request-checklist).

### How to write a useful description

A useful description is one that give all the hints necessary for the
reviewer(s) to understand what is being changed and to reason about your
approach.

Here are some tips:

 Give some **context**: Why this pull request exists ? Why it was created ?
- Explain **details that should not be overlooked**: Have you made any design
  choices that should be highlighted ? Are you trying to optimize something ?
- Describe **components are being impacted**: The pull request changes many
  files ? group them logically (API, Client, Crate X ? etc.) in the description

> Note: you should not *explain* your code in the pull request description. If
> you need to do so, consider making it an actual, in-code comment instead.

Also, for *draft* pull requests:

- Are you stuck or need help with something specifically ? let them know that !
- Draw reviewer(s) attention to **what is working and what is not**

### How to keep a sanitized commit history

Since pull requests are merged using the rebase method, all the commits created
in your branch will be present in the main branch. This is why we want to
sanitize commit history before merging it into the base branch.

- Give each commit a meaningful message to help you and future contributors
  understand changes included in the commit
  > For example, `fix typo` or `increase rate limit`

- Ideally, each commit contains an isolated, coherent and complete change.
  > This makes it easy to revert your changes if you decide to take a different
  > approach.
  > See [Github flow - Make changes](https://docs.github.com/en/get-started/quickstart/github-flow#make-changes).

- Squash (merge/fussion/combine) any commit that
  - fixes or revert something that was introduced *in your branch*
    > For example if you have a commit `fix typo` that fix a typo that is introduced in your
    > branch, you should squash this commit with the commit that introduced the typo.
  - is only relevant in the branch context (for example if you applied a
    suggestion from a reviewer)

- You could also squash every commits in your branch into a single one. In this
  case you must carefully write the commit title and commit body for them to be
  meaningful (they will be most likely the PR title/description).

The commit history sanitization could be done at the end.

### Pull request categories

A pull request might affect different parts of the codebase in different ways.

The following table lists the categories that can be manually or automatically
assigned to a pull request.

> Some categories could be automaticaly detected: for example if the pull
> request modifies `.rs` files we can assign the `Rust` category to it.

| Category     | Meaning                                        |
| ------------ | ---------------------------------------------- |
| Bug          | The PR fixes a bug                             |
| Security     | The PR is related to a security bug            |
| Feature      | The PR adds a new feature                      |
| Typo         | The PR fixes a typo                            |
| Dependencies | The PR updates one or more dependencies        |
| Release      | The PR is a release                            |
| RFC          | The PR is related to the RFCs documentation    |
| Docs         | The PR is related to the user documentation    |
| Gui          | The PR is related to the GUI                   |
| Test         | The PR is related to the test codebase         |
| **PER CODE** |                                                |
| CI           | The PR affects the CI                          |
| Rust         | The PR affect the Rust code                    |
| Python       | The PR affect the Python code                  |
| Js           | The PR affect the Javascript / Typescript code |
| Java         | The PR affect the Java / Kotlin / Android code |
| **PER OS**   |                                                |
| Linux        | The PR is specific to Linux                    |
| MacOS        | The PR is specific to MacOS                    |
| Windows      | The PR is specific to Windows                  |

Hints:

- If the pull request is related to more than one category in `PER CODE`, this
  may suggests that it needs to be splitted into smaller pull requests.
  > This would not be enforced because sometime it is required to change
  > multiple parts of the code at the same time.

- If the pull request is related to more than one category in `PER OS`, this
  will certainly indicate that the change is not OS-specific so spliting the
  pull request is recommended.

### The pull request checklist

The checklist is displayed before submitting a pull request:

```markdown
<!-- You can erase any parts of this template not applicable to your pull request -->

## Describe your changes

<!-- Why this PR exist ? what is its goal ?
     Give some context about the changes
     Draw attention to the details that should not be overlooked -->

## Checklist

Before you submit this pull request, please make sure to:

- [ ] Keep changes in the pull request as small as possible
   <!-- Move unrelated changes to a new pull request -->
- [ ] Ensure the commit history is sanitized
   <!-- Commits should have a meaningful title and contain a coherent set of changes
        Squash commits that are only relevant to the branch context -->
- [ ] Give a meaningful title to your PR
- [ ] Describe your changes
   <!-- Give some context about the changes
        Draw attention to the details that should not be overlooked -->
- [ ] Link any related issue in the description
   <!-- You can add `closes #<IssueID>` to close the issue when the PR is merged -->
- [ ] Link any dependent pull request in the description

If this PR adds a new feature or fixes a bug:

- [ ] I have added or updated relevant tests

If this PR modifies installation or updates packages:

- [ ] I have checked that installation works on all platforms (Windows, Linux, Mac ...)
    <!-- If you don't have a specific platform, seek help of someone that have it. -->

- [ ] I have updated related documentations (installation commands, fix to specific problems ...)

If this PR is related to GUI:

- [ ] I have checked that GUIs affected by my changes are working properly.

If this PR affects the user:

<!-- Changes that have an impact for the user:
     - UI has been changed
     - A message displayed to the user has been changed
     - A new feature has been added
     - A use case has been modified
     - ... -->

- [ ] I have created a news fragment
   <!-- The news fragments are used to generate the changelog -->
- [ ] I have updated the user documentation
```

## Reviewing a pull request

Before a pull request can be merged, it needs to be reviewed to meet our quality
standards.

The review consists in two steps:

- [The automated review](#the-automated-review)
- [The peer review](#the-peer-review)

Also, there are some extra steps for [specific pull requests reviews](#specific-pull-request-reviews).

### The automated review

The goal is to automate the systematic and tedious checks that can be easily
forgotten by a peer.

This step is responsible for:

- Ensuring codebase compiles (or equivalent)
- Executing tests and verifying their result (cargo test, pytest, ...)
- Verifying coding style using the configured linters (rustfmt, clippy, ruff,
  eslint, ...)
- Checking spell issues (cspell)
- Ensuring the app can still be packaged with the proposed change
- Ensuring commit signatures are valid (gpg signed commit)
- Verifying simple checks from the [Pull-request checklist](#the-pull-request-checklist)
- Verifying simple check on commit titles

### The peer review

The goal is not only to ensure quality standards missed in the automated review,
but also to encourage **team collaboration**, **application security** and **knowledge sharing**.

Anyone can review a pull request and provide feedback on some or all of the changes.
However, it is necessary to have at least one approving review from a [code owner]((#the-code-owner)) in order to be able to merge the pull request.

> You may need more than one approving review from different code owners if your change modify different code base (i.e. If you modify both rust & python for example).

As a reviewer, your approval involves some extra responsibilities since it is a
pledge of trust to merge the proposed changes in the master branch. Remember to:

- Read and *understand* the proposed changes
- Ask questions (what is the purpose of this ? can you explain me this ?)
- Take the opportunity to provide valuable feedback
- Suggests improvements like a simpler or more effective implementation, code
  refactoring, code comments, fix typos, etc.
- Ensure the changes are well tested
- If necessary, pull the code locally and perform some extra testing

Also:

- Ensure changes are coherent and related to the linked issue or the pull
  request description
  - If necessary, propose to split the changes into smaller pull requests
- Ensure the author followed the [Pull-request checklist](#the-pull-request-checklist)
  (if applicable: check news fragment, user documentation, etc.)
- Ensure that [commit history is sanitized](#how-to-keep-a-sanitized-commit-history)

#### The code owner

A *code owner* is a person or a team assigned to be responsible for a specific
component or code parts, usually because it has a deep knownledge about it.

> For example the Python dev are the owner of the python code.

### Specific pull request reviews

This section describes extra steps or things to pay attention to for specific
types of pull requests.

#### Pull request: fix typos

We want the codebase to not be cluttered with typos (syntax, grammar and others).
Pull requests that solves those issues are welcome!

When reviewing this type of pull request, ensure that:

- It does not introduce new typos
- The original meaning is not changed

Also, take the opportunity to consider rephrasing to better convey the initial
meaning.

> We don't want to introduce new typos when correcting some that are already present. (ouroboros)

#### Pull request: update dependencies

Likely opened by a bot (like dependabot) these are pull requests that update
one or more required dependencies and are small (few files are changed, most
likely one line per file).

In addition to the standard process, we would like to inspect the changes
introduced by the newer version of the dependency.

> What have changed between the current version and the updated version of the dependency

Here the burden remains on the reviewer (considering the update was created by a
bot). Take the time to read dependency changelog, blog posts, or similar. Keep
in mind that this step could prevent (to a certain limit) a potential
supply-chain attack where a fake update is released with malicious code.

## Merging a pull request

After the pull request has been reviewed and accepted, the author is allowed to
merge it into the main branch.

The merge should be done in a specific way:

- The commits should appear on top of the git history (see [Why do we want a linear git history](#why-do-we-want-a-linear-git-history)).
- It should be tested one last time if changes are compatible with the main
  branch.

    This is done by the [merge queue](#what-is-a-merge-queue) that will rebase the
    commits onto the main branch.

### Why do we want a linear git history

Why want a linear git history ? because it allows to:

- Answer the question: *Did **feature A** get introduced before of after **bugfix B** ?*
- Simpler git history graph (everything is in-line).
- `git bisect` work best with a linear history.
- Remove unnecessary commit `Merge branch X into Y` that clutters the history.

How to achieve a linear git history?

Git provides multiple methods for that

- `git fast forward` (`git merge --ff-only`):

  When commits are already ahead of the base branch, it will only update the
  branch pointer.

- `git squash` (`git merge --squash`):

  Apply the changes onto the destination branch but wont make a commit, this
  allows to create a single commit on top of the current branch.

- `git rebase`

  Will replay commits that are not already on the `upstream` branch on top of
  the `upstream` branch's commit.

### What is a merge queue

A merge queue is a system that will queue pull requests to be merge in a queue :p

A merge queue typically works by creating a temporary branch that include:

- The latest change from the base branch.
- The changes introduced by the PRs ahead of the queue.
- The changes added by your PR.

This ensures that we merge pull requests that work with the latest state of the
base branch and the previous PRs that are in the process to be merge thus already in the queue

## Interesting sources

- [10 Proven techniques for More Effective Code Reviews](https://www.dainemawer.com/articles/ten-proven-techniques-for-more-effective-code-reviews)
- [Introducing code owners](https://github.blog/2017-07-06-introducing-code-owners/)
- [Avoid messy git history, use linear history](https://dev.to/bladesensei/avoid-messy-git-history-3g26)
- [A tidy, linear Git history](https://www.bitsnbites.eu/a-tidy-linear-git-history/)
