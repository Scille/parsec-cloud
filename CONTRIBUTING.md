# Contributing to Parsec

:+1::tada: First off, thanks for taking the time to contribute! :tada::+1:

Contributing to Parsec is welcome! We love open source and community collaboration. All of Parsec development is fully open here on GitHub and there are no private, closed areas that exclude you and the Parsec community.

The guidelines here give some tips on how to best contribute. Please try to follow these. It's short, quick and easy. This will ensure a smooth process. Thanks again for contributing!

## Summary

- [Summary](#summary)
- [Filing Issues](#filing-issues)
  - [Filing Bugs](#filing-bugs)
  - [Filing Features](#filing-features)
- [Contributing Code](#contributing-code)
  - [Send us a Contributor Assignment Agreement](#send-us-a-contributor-assignment-agreement)
    - [What we need you to do](#what-we-need-you-to-do)
- [Contributing Documentation](#contributing-documentation)
- [Acknowledgement](#acknowledgement)

## Filing Issues

We track **issues** in the GitHub issue tracker [here](https://github.com/Scille/parsec-cloud/issues).

An issue is either a **bug** (an unexpected / unwanted behavior of the software or incorrect documentation) or a **feature** (a desire for new functionality / behavior in the software or new documentation).

### Filing Bugs

Use the following link to [Create a bug report](https://github.com/Scille/parsec-cloud/issues/new?template=bug-report.yml&title=[%F0%9F%90%9B+|+Bug]%3A+).

Please fill in the required fields and add any extra information that might help us understand the issue.

> A bug issue should also be only about **one thing** - if you have found multiple related things, please file multiple issues and link the issues (cross-referencing by mentioning the related issues in the descriptions - that will make GitHub automatically add respective links).

### Filing Features

Submit your feature request [here](https://github.com/Scille/parsec-cloud/issues/new?assignees=&labels=B-Triage%2CC-Feature-Request&projects=&template=feature-request.yml&title=%5B%F0%9F%9A%80+%7C+Feature+request%5D%3A+).

Take your time when filling the form.
There are usually many ways to solve a problem: try describing your use case and goals before jumping into specific, concrete proposal for a solution.
This helps us seeing the big picture. If you "only" propose a concrete solution, and we consider it doesn't fit in, we will need to guess what is your actual use case before coming up with a solution that fits.

## Contributing Code

We use the Fork & Pull Model. This means that you fork the repo, make changes to your fork, and then make a pull request here on the main repo.

> This article [Create a branch for an issue] gives more detailed information on how the process works.

1. All development happens on GitHub and we use the usual fork the repository, branch-per-issue, [pull request] and merge workflow, also known as [GitHub Flow].

2. A necessary **requirement** is that an issue needs to exist first. That is, a PR is always for a specific issue.

   We suggest that you create a branch on your fork with the following format:

   ```jinja
   {{ ISSUE_ID }}-{{ DESCRIPTIVE_NAME }}
   ```

   That format is alike to the branch created using [Create a branch for an issue] (if you're a maintainer of a project).

3. Another prerequisite (for merging code) is that you have [signed and sent us a contributor agreement](#send-us-a-contributor-assignment-agreement).
   This only needs to be done once, but we cannot merge code until we have received a CAA.

4. Further, we have a CI system in place running the whole set of unit tests for Parsec on various platforms.
   A necessary condition for a PR to be merged that all of our required workflow to be green.

5. A branch must be rebased to the **master** branch before merging.

   You can squash your commits into a single one if you want (with a nice commit title & message is best :yum:).

   **BUT** Never correct conflict in your branch using a merge commit:

   > Since we use the rebase strategy to merge branch, you merge commit that fix the conflict will be removed during the rebase.

6. If your branch doesn't run green on our CI, or your branch becomes stale, because other things were merged in between, you are responsible for fixing thing on your branch first.

[pull request]: https://help.github.com/articles/using-pull-requests
[Github Flow]: https://docs.github.com/en/issues/tracking-your-work-with-issues/creating-a-branch-for-an-issue
[Create a branch for an issue]: https://docs.github.com/en/issues/tracking-your-work-with-issues/creating-a-branch-for-an-issue

### Send us a Contributor Assignment Agreement

Before you can contribute any changes to the Parsec project, we need a CAA (Contributor Assignment Agreement) from you.

The CAA gives us the rights to your code, which we need e.g. to react to license violations by others, for possible future license changes and for dual-licensing of the code. The CAA closely follows a template established by the [Harmony project](http://harmonyagreements.org/), and CAAs are required by almost all open source projects which are non-trivial in scope.

#### What we need you to do

1. Download the [Contributor agreement CAA (PDF)](https://github.com/scille/parsec-cloud/raw/master/docs/legal/contributor_agreement.pdf).
2. Fill in the required information that identifies you and sign the CAA.
3. Scan the CAA to PNG, JPG or TIFF, or take a photo of the box on page 2 (the **entire box**, including the information identifying the document).
4. Email the scan or photo to `contact@scille.fr` with the subject line "Parsec project contributor assignment agreement"

**You only need to do this once - all future contributions are covered!**

## Contributing Documentation

Contributions to the documentation are highly welcome! Parsec is a complex software, users have varying degrees of background and experience and we have limited resources. Also, developers of Parsec usually don't have a **user perspective** (anymore) - but docs should be for users.

All Parsec documentation is contained in this repository [here](https://github.com/Scille/parsec-cloud/tree/master/docs).

The documentation is built (by us) and deployed [here](http://docs.parsec.cloud).

## Acknowledgement

This document is based on the [contributing one](https://github.com/crossbario/crossbar/blob/master/CONTRIBUTING.md)
of the [Crossbar.io](https://crossbar.io/) project.
This a cool open source project made by nice people, though not related to Parsec in any way, you should go have a look ;-)
