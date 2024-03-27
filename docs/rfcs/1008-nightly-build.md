<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Nightly build

This RFC proposes adding nightly build to parsec.

- [Goals](#goals)
- [What we already have](#what-we-already-have)
- [What is missing](#what-is-missing)
  - [For the client part](#for-the-client-part)
    - [Missing version argument in the workflow `package-ionic-app.yml`](#missing-version-argument-in-the-workflow-package-ionic-appyml)
    - [Include the client part in the releaser workflow](#include-the-client-part-in-the-releaser-workflow)
  - [Overwriting nightly release \& automatic release](#overwriting-nightly-release--automatic-release)
  - [Snapcraft channel with tracks](#snapcraft-channel-with-tracks)
  - [Schedule the workflow](#schedule-the-workflow)
  - [Slack notification (optional)](#slack-notification-optional)
  - [Improve the unique dev format](#improve-the-unique-dev-format)
- [F.A.Q](#faq)
  - [Why do we need to release the nightly build? Or why we can't directly use the artifact link of a workflow?](#why-do-we-need-to-release-the-nightly-build-or-why-we-cant-directly-use-the-artifact-link-of-a-workflow)
  - [Why do we use the `dev` identifier for a nightly build ?](#why-do-we-use-the-dev-identifier-for-a-nightly-build-)

## Goals

- Periodically test the release pipeline.
- Have a recent build software at the ready (client and server).

## What we already have

We already have a workflow that create a release & build the artifact[^releaser-workflow] for that release.

That workflow[^releaser-workflow] is triggered when we modify the workflow itself but notably when we push a tag on the repository that have the format `v[0-9]+.[0-9].[0-9]+*`.

Currently, the workflow build the server python wheel for Linux, Windows & macOS.

[^releaser-workflow]: https://github.com/Scille/parsec-cloud/blob/05e4deba964c6b68b6fcbca4d458c235c43f6cff/.github/workflows/releaser.yml

The script `releaser.py` can generate unique version that can be used for versioning and tagging nightly build.

## What is missing

From the release, we already have the server part indicated by:

<https://github.com/Scille/parsec-cloud/blob/05e4deba964c6b68b6fcbca4d458c235c43f6cff/.github/workflows/releaser.yml#L74-L79>

### For the client part

#### Missing version argument in the workflow `package-ionic-app.yml`

The workflow ([`package-ionic-app.yml`]) need to be adapted like [`package-server.yml`] to take a version in argument.

[`package-server.yml`]: https://github.com/Scille/parsec-cloud/blob/05e4deba964c6b68b6fcbca4d458c235c43f6cff/.github/workflows/package-server.yml

#### Include the client part in the releaser workflow

We need to add the client to the workflow [`releaser.yml`].
Thankfully we already have the workflow for that ([`package-ionic-app.yml`]).

[`package-ionic-app.yml`]: https://github.com/Scille/parsec-cloud/blob/05e4deba964c6b68b6fcbca4d458c235c43f6cff/.github/workflows/package-server.yml
[`releaser.yml`]: https://github.com/Scille/parsec-cloud/blob/05e4deba964c6b68b6fcbca4d458c235c43f6cff/.github/workflows/releaser.yml

### Overwriting nightly release & automatic release

Currently, the [`releaser.yml`] workflow create a [GitHub release] in draft mode.

[GitHub release]: https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases

This is problematic if the workflow create a `draft-release` for each nightly build (everyday so).

> We would be overwhelmed by the amount of draft releases that would accumulate.

To circumvent that we could make the workflow to remove the previous nightly release.
But we have some limitation, the API to [delete a release] require a `release_id`.

[delete a release]: https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#delete-a-release

This ID could be obtained by [getting a release using its tag name] but a `draft-release` isn't identified by a tag name (the more you know)

[getting a release using its tag name]: https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#get-a-release-by-tag-name

> [!TIP]
> This could explain why `Github CLI` only delete releases identified by a tag only, see [`gh release delete` manual](https://cli.github.com/manual/gh_release_delete).

> We could also [list all releases] to find the one we look for.

[list all releases]: https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#list-releases

So we could update the workflow to directly create the release.

> At this step, you may be asking yourself [Why do we need to release the nightly build ?](#why-do-we-need-to-release-the-nightly-build-or-why-we-cant-directly-use-the-artifact-link-of-a-workflow)

### Snapcraft channel with tracks

Our currently workflow with Snapcraft we only use the default channels `stable` & `edge`.

I would like to introduce a [track] in the channel.

A track allow to more precisely specify which version to install with `snap`, what I have currently in mind is the following track:

- `latest`: We already have this one since it's the default one used by Snapcraft when we don't specify it
- `v{MAJOR}`: A track per major version (in the case we need to maintained multiple version at the same time)
- `nightly`: A track that contain the nightly build

  > I've also considered the following naming instead of `nightly` (Firefox style):
  > - `canary` (Chrome style)
  > - `insider` (VS-Code style)
  >
  > I've made a poll on `2024-03-01` about which naming to use and `nightly` came first.

[track]: https://snapcraft.io/docs/channels#heading--tracks

That would translate to the following usage:

```shell
snap install parsec # This will use `latest/stable`
snap install parsec --channel=latest # will use `latest/stable`
snap install parsec --channel=latest/beta # will use `latest/beta`
snap install parsec --channel=v3 # will use `v3/stable`
snap install parsec --channel=insider # will use `insider/stable`
```

We could simply put the versions that contain a `dev` part into the `insider` track.

### Schedule the workflow

Require [Overwriting nightly release & automatic release](#overwriting-nightly-release--automatic-release)

We need to have a workflow that schedule the nightly build at a specific interval (every day at 22H30 for example).
GitHub already allow to [trigger a workflow on a schedule].

[trigger a workflow on a schedule]: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#schedule

> [!CAUTION]
> Event triggered by a GitHub actions don't trigger a workflow. So everything should be done in a single workflow.

### Slack notification (optional)

We could send a Slack notification when a nightly build finished (successfully or not)
With the link to download the generated build.

Slack provide GitHub action for that: <https://github.com/slackapi/slack-github-action>

### Improve the unique dev format

Currently, `releaser.py` generate a dev version by appending the `dev` part at the end of the current version before the local part.

The dev part is formatted that way `dev.{number}`. The `{number}` value is calculated from the current date with the following formula:

$f(year, month, day) = year * 366 + month * 31 + day$

> [!NOTE]
> If you are careful, the value $372$ ($12 * 31 = 372$) should have been used instead of $366$ (that's a mistake)

For the date `2024-02-27 15:50:28+01:00` that gives `dev.740873`:

$$f(2024, 2, 27) = 2024 * 366 + 2 * 31 + 27$$
$$f(2024, 2, 27) = 740873$$

The formula could be changed to:

- A _timestamp_: For `2024-02-27 15:50:28+01:00` we get `dev.1709045428`

- A _timestamp_ without the time info:

  For `2024-02-27 15:50:28+01:00`, the value is calculated with:

  $$S = 24 * 3600$$
  $$S = 86400$$
  $$timestamp = 1709045428$$
  $$g(timestamp) = floor(timestamp / S)$$
  $$g(1709045428) = 19780$$

  That gives `dev.19780`, converted back to a timestamp $19780 * S = 1708992000$, that give `2024-02-27 01:00:00+01:00`

- The corrected formulas using $372$:

  For `2024-02-27 15:50:28+01:00`, the value is calculated with:

  $$f'(year, month, day) = year * 372 + month * 31 + day$$
  $$f'(2024, 2, 27) = 753017$$

  That would give `dev.753017`

- A date (`dev.YYYYMMDD`): For `2024-02-27 15:50:28+01:00` That gives `dev.20240227`

## F.A.Q

### Why do we need to release the nightly build? Or why we can't directly use the artifact link of a workflow?

When a workflow generate artifacts, those artifacts could be directly downloaded from the workflow summary.

But that has some limitations:

- Those artifacts expire after a configured period (default to 90days).
- The artifacts are wrapped in a zip file, so you need to extract them before using them.
- You could not easily know if the workflow summary you're using is the latest available.
- The artifacts may contain data that is not useful for you.

### Why do we use the `dev` identifier for a nightly build ?

We need to have a version that is compatible with [semver] and [pep440], that indicate that it's a nightly build.

On [semver], we could use any verb like `nightly`, so the choice come from [pep440].

Pep440 only allows the verb `dev` at the end of the version before the local identifiers ([Specify a development release in pep440])

[semver]: https://semver.org/
[pep440]: https://peps.python.org/pep-0440/
[Specify a development release in pep440]: https://peps.python.org/pep-0440/#developmental-releases
