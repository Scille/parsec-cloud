<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS -->

# Release Cheat-Sheet

- [Release Cheat-Sheet](#release-cheat-sheet)
  - [Major/minor vs patch versions](#majorminor-vs-patch-versions)
  - [Release Checklist](#release-checklist)
  - [Release major/minor version](#release-majorminor-version)
    - [Create release candidate version](#create-release-candidate-version)
    - [Create the release](#create-the-release)
    - [Create the release Pull Request](#create-the-release-pull-request)
    - [Sign the installers](#sign-the-installers)
    - [Create the release on Github](#create-the-release-on-github)
    - [Merge (or update) the Pull Request](#merge-or-update-the-pull-request)
  - [Release patch version](#release-patch-version)
    - [(Re)create the version branch](#recreate-the-version-branch)
    - [Cherry-pick the changes](#cherry-pick-the-changes)
    - [Follow the major/minor guide](#follow-the-majorminor-guide)
    - [All done](#all-done)

## Major/minor vs patch versions

When the need for release comes, the obvious question is:

> what is going to be the next version ?

The flowchart is:

- You want to release what's on the tip of master ⟶ do a minor bump
- Otherwise ⟶ do a patch bump

In other words, the patch bump should only be used to cherry-pick a subset of
changes present on the master on a previous version.

## Release Checklist

For each release types, apply the following checklist:

- The newsfragment were squashed to generate the block added to [`HISTORY.rst`](/HISTORY.rst).
- The updated [`HISTORY.rst`](/HISTORY.rst) is correctly formated (Some news fragment may introduce invalid syntax that can break the RST file).
- The translations are up-to-date (check the translations in `docs/`, `parsec/core/gui/tr/`, `client/src/locales/`).
- The `releaser.py` correctly update the version in the expected files (`pyproject.toml`, `licenses/BUSL-Scille.txt`, `parsec/_version.py`).

Note: Most of the work can be done using the workflow [`release-starter`](https://github.com/Scille/parsec-cloud/actions/workflows/release-starter.yml) (_most_ because it won't cherry pick the commit need to patch a release).

## Release major/minor version

In the following we will consider we want to release version ``v2.9.0``.

### Create release candidate version

Release is complex and fails pretty often, so you should create release
candidate versions.

As a matter of fact you should only consider creating a final version only
if you already have a release candidate that doesn't need any correction.

Yes, even this ultra small 1 line typo fix. We've all been there, we all
know how it ends up ;-)

Release candidate versions must have the naming ``v2.9.0-rc1``, ``v2.9.0-rc2`` etc.

### Create the release

The script `misc/releaser.py` provides an automated workflow to generate a new release/build.

Note: The commits and release tag are annotated & signed, so you must have your GPG key
at the ready (and this key should be configured in your github account).

To create a new release, simply execute:

```shell
git fetch
python misc/releaser.py build --base=origin/master v2.9.0
```

> `git fetch` is used to update the remote ref, that is used to get the latest remote change on `master`.
> We specify `--base=origin/master` to be able to use that script from any branch.

The script will:

- Ensure the release version is greater than the current version.
- Ensure _git env_ is clean (no changes to be commited).
- Create the release branch `releases/2.9` (and switch to it).
- Update the license Date & version.
- Update the parsec version across different files in our repository.
- Update `HISTORY.rst` with a new block generated using the news fragments found in `newsfragments/`.

  At this step, the script will ask you to review the changes made so far (mostly to check if [`HISTORY.rst`](/HISTORY.rst) was correctly generated).

  For instance, if some release candidate versions have been released before this one,
  now is the time to merge back the corresponding release notes.

- Create the release commit with the message `Bump version v2.8.1+dev -> v2.9.0`.

  The commit will contains the change made to `HISTORY.rst`, the various files referencing the parsec version, the updated license and the removed news fragments.

- Create the release tag `v2.9.0`.

  The script will display the tag information, it needs to be reviewed before proceeding (check the tag signature).

- Create the dev commit with the message `Bump version v2.9.0 -> v2.9.0+dev`.

- Push the branch `releases/2.9` & the tag `v2.9.0`

  The script will require confirmation before continuing.

### Sign the installers

⚠️ This step is outdated.

The CI is going to generate the installers for Linux, Mac and Windows.

On Linux the snap installer is automatically released on the edge channel of snapcraft.

On Windows and Mac, the installers must be downloaded from the CI build artifacts and
signed. See the documentation in ``packaging/`` for more information.

On top of that the Python wheel of the project is going to be uploaded to Pypi.

### Create the release on Github

⚠️ This step is outdated.

Once the tag pushed, it can be converted as a release on github using the
[Draft a new release](https://github.com/Scille/parsec-cloud/releases/new).

The release should contain the Mac and Windows installers that have been signed during step 4.

⚠️ Don't forget to check `Set as a pre-release` if your creating a release candidate !

> Note: The Parsec client's version checker is smart enough to ignore new version
> that doesn't contain an installer for there platform. Hence it's safe to create
> a new github release without any installer.

### Merge (or update) the Pull Request

⚠️ This step is outdated.

If you were dealing with a final release (e.g. ``v2.9.0``), you can merge the branch in master call it a day ;-)

However if you just release a RC release:

- for quickfix you can commit directly on the version branch

- for bigger fix, open a PR targeting master. Once merged you can then merge back master
  on your version branch to get the changes. Alternatively, if the master contains other
  changes you don't want, you can cherry-pick the merge commit.

Once you're happy with the changes, you can release a new RC.

When you no longer have changes to add (i.e. your current RC is perfect) then you can
do a final release and merge the version branch in master.

## Release patch version

In the following we will consider we want to release version ``v2.9.1``.

### (Re)create the version branch

If the release branch ``releases/2.9`` used for the ``2.9.0`` release has been
removed, it needs to be recreated.

> Note: the release branch should live on their own and should not be deleted

In that case, the branch MUST be recreated at the commit ``Bump version v2.9.0 -> v2.9.0+dev``
(i.e. the commit right after the release tag) and not release tag itself.

Of course the version branch should be reused if a previous patch release has
already been done (e.g. you're planning to release ``v2.9.2``).

### Cherry-pick the changes

Most of the time, the changes needed on the patch release are also expected to
end up in the master branch.

In this case, a main PR should be opened against master, then once merge it commits
can be cherry-picked to create another PR against the version branch.

### Follow the major/minor guide

You know the drill, creating the release:

```shell
python misc/releaser.py build 2.9.1
```

> From the step above, your current git branch MUST be `releases/2.9`.

### All done

Unlike the major/minor release, we don't merge back the version branch into master.
This is of course because our version branch is decorrelated from master and merging
would mess things around.

However this has one downside: if a third party repo use git subtree on a patch version,
it won't be able to automatically update the subtree to a newer non-patch version.
