<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Nightly build

This RFC proposes adding nightly builds to Parsec.

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

We already have a workflow to create a release and build its artifacts[^releaser-workflow]. It is triggered by pushing a tag with the format `v[0-9]+.[0-9]+.[0-9]+*`.

Currently, the workflow builds the server's Python wheel for Linux, Windows & macOS.

[^releaser-workflow]: https://github.com/Scille/parsec-cloud/blob/05e4deba964c6b68b6fcbca4d458c235c43f6cff/.github/workflows/releaser.yml

The script `releaser.py` can generate unique version that can be used for versioning and tagging nightly build.

## What is missing

### Client packaging

The following changes are required for the [`package-ionic-app.yml`] workflow:

- it needs to take a version argument (like [`package-server.yml`]).
- it needs to be added to the [`releaser.yml`] workflow.

[`package-server.yml`]: https://github.com/Scille/parsec-cloud/blob/05e4deba964c6b68b6fcbca4d458c235c43f6cff/.github/workflows/package-server.yml
[`package-ionic-app.yml`]: https://github.com/Scille/parsec-cloud/blob/05e4deba964c6b68b6fcbca4d458c235c43f6cff/.github/workflows/package-server.yml
[`releaser.yml`]: https://github.com/Scille/parsec-cloud/blob/05e4deba964c6b68b6fcbca4d458c235c43f6cff/.github/workflows/releaser.yml

### Overwriting nightly release & automatic release

The [`releaser.yml`] workflow creates a [GitHub release] in draft mode. Creating a `draft-release` for _every_ nightly build (everyday so) is problematic because we might be overwhelmed by the number of draft releases.

We could [override the assets] of the previous nightly release (instead of recreating a new one). The problem with this approach is that GitHub sort the releases by creation date: nightly release will be put behind newer releases.

So the idea is to _remove_ the previous nightly release and recreate a new one. In order to do so, the API to [delete a release] requires a `release_id`.

[override the assets]: https://stackoverflow.com/questions/62934246/github-update-overwrite-existing-asset-of-a-release
[GitHub release]: https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases

[delete a release]: https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#delete-a-release

The `release_id` could be obtained by [getting a release using its tag name] but a `draft-release` isn't identified by a tag name (the more you know).

[getting a release using its tag name]: https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#get-a-release-by-tag-name

Another possibility is to [list all releases] to find the one to be removed. This would allow to update the workflow in order to directly create the release (see [Why do we need to release the nightly build?](#why-do-we-need-to-release-the-nightly-build-or-why-we-cant-directly-use-the-artifact-link-of-a-workflow)).

[list all releases]: https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#list-releases

### Snapcraft channel with tracks

Currently we only use the `stable` & `edge` from the default `latest` track. We could introduce a new [track] in order to include nightly builds, for example:

- `latest`: We already have this one since it's the default track (when no track is specified)
- `v{MAJOR}`: A track per major version (in the case we need to maintain multiple version at the same time)
- `nightly`: A track for the nightly builds

  > The name `nightly` (Firefox style) was chosen by the parsec-dev team based on a poll. Alternatives that were considered:
  > - `canary` (Chrome style)
  > - `insider` (VS-Code style)

[track]: https://snapcraft.io/docs/channels#heading--tracks

That would translate to the following usage:

```shell
snap install parsec # This will use `latest/stable`
snap install parsec --channel=latest # will use `latest/stable`
snap install parsec --channel=latest/beta # will use `latest/beta`
snap install parsec --channel=v3 # will use `v3/stable`
snap install parsec --channel=nightly # will use `nightly/stable`
```

We could simply put the versions that contain a `dev` part into the `insider` track.

### Schedule the workflow


We need to have a workflow that schedule the nightly build at a specific interval (every day at 22H30 for example).
GitHub already allow to [trigger a workflow on a schedule].

[trigger a workflow on a schedule]: https://docs.github.com/en/actions/using-workflows/events-that-trigger-workflows#schedule

> [!CAUTION]
> Event triggered by a GitHub actions don't trigger a workflow. So everything should be done in a single workflow.

### Slack notification (optional)

We could send a Slack notification when a nightly build finished (successfully or not)
With the link to download the generated build.

Slack provide GitHub action for that: <https://github.com/slackapi/slack-github-action>

### Fix the unique dev format

Currently, `releaser.py` generates a dev version by appending the `dev` part at the end of the current version before the local part.

The dev part is formatted `dev.{number}` where `{number}` is calculated with the following formula:

$f(year, month, day) = year * 366 + month * 31 + day$

The constant $366$ is wrong, the value $372$ ($12 * 31 = 372$) should have been used instead.

For the date `2024-02-27 15:50:28+01:00` that gives `dev.740873`:

$$f(2024, 2, 27) = 2024 * 366 + 2 * 31 + 27$$
$$f(2024, 2, 27) = 740873$$

Considering the previous example date, the formula could be changed to:

- A _timestamp_: `dev.1709045428`
- A _timestamp_ without the time info ($floor(timestamp / 24 * 3600)$) : `dev.19780`
- The current formula with fixed value ($year * 372 + month * 31 + day$): `dev.753017`
- A date format (`dev.YYYYMMDD`): `dev.20240227`

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
