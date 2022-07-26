.. Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

.. _doc_development_release:

===================
Release Cheat-Sheet
===================

Major/minor vs patch versions
-----------------------------

When the need for release comes, the obvious question is:

    what is going to be the next version ?

The flowchart is:

- You want to release what's on the tip of master ⟶ do a minor bump
- Otherwise ⟶ do a patch bump

In other words, the patch bump should only be used to cherry-pick a subset of
changes present on the master on a previous version.

Release major/minor version
---------------------------

In the following we will consider we want to release version ``v2.9.0``.

0 - Create release candidate version
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Release is complex and fails pretty often, so you should create release
candidate versions.

As a matter of fact you should only consider creating a final version only
if you already have a release candidate that doesn't need any correction.

Yes, even this ultra small 1 line typo fix. We've all been there, we all
know how it ends up ;-)

Release candidate versions must have the naming ``v2.9.0-rc1``, ``v2.9.0-rc2`` etc.

1 - Create the release branch
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Master branch being protected, we do our work on a dedicated branch that we call the release branch.
Release branch naming must be ``<major>.<minor>``, so in our case ``2.9``:

.. code-block:: shell

    git checkout master && git pull
    git checkout -b 2.9
    git push --set-upstream origin 2.9

Note at this point the branch is even with master, hence we cannot create a Pull Request yet (see part 4).

2 - Create release commits and tag
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. code-block:: shell

    python misc/releaser.py build v2.9.0

The script will pause just before creating the release commit, this is so you
can open ``HISTORY.rst`` and check the release note.

For instance, if some release candidate versions has been released before us,
now is the time to merge all those release notes into ours.

Once hitting enter, the script:
- creates the release commit with a commit message of the type: ``Bump version v2.8.1+dev -> v2.9.0``.
- create the release tag ``v2.9.0``
- create the ``Bump version v2.9.0 -> v2.9.0+dev`` commit.

Note the release tag is annotated & signed, so you must have your GPG key
at the ready (and this key should be configured in your github account).

You should also check the signature of the commits and tag:

.. code-block:: shell

    git show v2.9.0 --show-signature
    git tag --verify v2.9.0

3 - Push upstream
^^^^^^^^^^^^^^^^^

.. code-block:: shell

    git push
    git push origin v2.9.0

It's better to push the commit before the tags, this way we can detect if
somebody has pushed on the branch in our back.

4 - Create the release Pull Request
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Release Pull Request starts from the release branch (e.g. ``2.9``) and targets the ``master`` branch.
Pull Request title must be ``Release v2.9.0``.

5 - Sign the installers
^^^^^^^^^^^^^^^^^^^^^^^

The CI is going to generate the installers for Linux, Mac and Windows.

On Linux the snap installer is automatically released on the edge channel of snapcraft.

On Windows and Mac, the installers must be downloaded from the CI build artifacts and
signed. See the documentation in ``packaging/`` for more information.

On top of that the Python wheel of the project is going to be uploaded to Pypi.

6 - Create the release on Github
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Once the tag pushed, it can be converted as a release on github using the
["Draft a new release"](https://github.com/Scille/parsec-cloud/releases) button.

The release should contain the Mac and Windows installers that have been signed during step 4.

/!\ Don't forget to check "This is a pre-release" if your creating a release candidate !

Note: The Parsec client's version checker is smart enough to ignore new version
that doesn't contain an installer for there platform. Hence it's safe to create
a new github release without any installer.

7 - Merge (or update) the Pull Request
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

If you were dealing with a final release (e.g. ``v2.9.0``), you can merge the branch in master call it a day ;-)

However if you just release a RC release:
- for quickfix you can commit directly on the version branch
- for bigger fix, open a PR targeting master. Once merged you can then merge back master
  on your version branch to get the changes. Alternatively, if the master contains other
  changes you don't want, you can cherry-pick the merge commit.

Once you're happy with the changes, you can release a new RC.

When you no longer have changes to add (i.e. your current RC is perfect) then you can
do a final release and merge the version branch in master.

Release patch version
---------------------

In the following we will consider we want to release version ``v2.9.1``.

0 - (Re)create the version branch
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The version branch ``2.9`` used to do ``2.9.0`` release has most likely been
removed when merged into master, we must recreate it.
Note the version branch should be set to the ``Bump version v2.9.0 -> v2.9.0+dev``
commit (i.e. the commit right after the release tag) and not release tag itself.

Of course the version branch should be reused if a previous patch release has
already been done (e.g. you're planning to release ``v2.9.2``).

1 - Cherry-pick the changes
^^^^^^^^^^^^^^^^^^^^^^^^^^^

Most of the time, the changes needed on the patch release are also expected to
end up in the master branch.

In this case, a main PR should be opened against master, then once merge it commits
can be cherry-picked to create another PR against the version branch.

2 - Follow the major/minor guide
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

You know the drill, creating the release:

.. code-block:: shell

    python misc/releaser.py build 2.9.1

Pushing upstream:

.. code-block:: shell

    git push  # Here we push the `2.9` branch !
    git push origin v2.9.1

And finally signing the installer and creating the release on Github.

3 - All done !
^^^^^^^^^^^^^^

Unlike the major/minor release, we don't merge back the version branch into master.
This is of course because our version branch is decorrelated from master and merging
would mess things around.

However this has one downside: if a third party repo use git subtree on a patch version,
it won't be able to automatically update the subtree to a newer non-patch version.
