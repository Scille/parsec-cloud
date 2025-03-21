<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->
<!-- cSpell:ignore myworkspace, Resana -->

# Workspace mountpoint integration

## Overview

The goal of this RFC is to describe and standardize how Parsec supports
mountpoint integration across different platforms (Linux, macOS, Windows).

## Background & Motivation

In Parsec V2, mountpoint integration was steadily improved over the years to a
point of maturity where things "just worked" without much complaint from users.

Recently, some changes came to disrupt this stability:

- **Parsec V3 migration**: mountpoint support was completely re-written in Rust
  which introduced some regressions because:
  - the new implementation was not identical to the previous one
  - the tests needed migration too, so edge-cases could have been missed
  - the 3rd party libraries changed, new issues were detected (particularily on
    macOS)

- **Mountpoint strategy**: Parsec workspaces can be mounted either as *Drive* (only on Windows) or
  as *Directory*. This added some uncertainty as which should be the preferred/
  default strategy to use on Windows.

- **User mount/unmount**: the ability to mount/unmount workspaces was considered
  not so user-friendly and was removed from the GUI. This introduced some
  unexpected friction:
  - a user with too many workspaces is now forced to have all of its workspaces
    mounted on disk (performance & security concerns arose).
  - the workspaces "manually" unmounted by the user (via the system's file
    explorer) are currently not managed by Parsec, leaving space for errors and
    risk of data loss.

## Goals and Non-Goals

In short, this RFC aims at describing how Parsec should support mountpoint on
each platform, therefore answering questions such as:

- How workspaces should be mounted by default?
- What is the expected behavior on startup/shutdown of the application?
- How should Parsec recover from a mountpoint crash or an application
  crash?

> The **need** for user function to mount/unmount workspaces can be discussed
> here. However, we should probably leave the **how to do it** for another RFC.

## Design

### Choosing a Mountpoint Strategy

The mountpoint strategy defines how Parsec mounts a given Workspace on the user
file system:

- **DriveLetter**: A system Drive (such as `Z:` in Windows) is created for each
  workspace to be mounted. The workspace content is available at the root of
  each drive (e.g. `Z:\myfile.txt_`).
- **Directory**: A *base directory* is created at a specific location (e.g.
  `$HOME/Parsec3` on Linux). A sub-directory is created for each workspace to be
  mounted. The workspace content is available at the root of each sub-directory
  (e.g. `$HOME/Parsec3/myworkspace/myfile.txt`).
- **Disabled**: No mountpoint is created for any workspaces.

> Note that the *workspace name* is not part of the path in the Drive strategy.

#### Comparison: Drive vs Directory

- Mountpoint as Drive
  - :+1: Pros
    - Good native integration with File Explorer across platforms
  - :-1: Cons
    - **(Windows) Can easily run out of drive letters (only 26 available)**
    - Drive can be manually unmounted from File Explorer

- Mountpoint as Directory
  - :+1: Pros
    - Base mountpoint directory can be changed if needed (used by Resana Secure)
    - Allows to mount different workspaces in different base directories
      (not yet implemented in V3; available in V2 and used by Resana Secure)
  - :-1: Cons
    - Needs custom code for integration with File Explorer to support things
      such as sidebar for quick access, icons, etc.
    - Directory (either base or workspace dir) can be manually removed as the
      user sees it as a regular directory

#### Default strategy

On Unix system (Linux & MacOS), there is not much difference between Drive and Directory strategies.

On Windows, the limitation on the number of drive letters is what ends up
tipping balance in favor of the *Mountpoint as Directory* strategy.

> In Parsec V3, the **Mountpoint as Directory** strategy is the current default
> for all platforms.
>
> The default base directory is `Parsec3` in the user's home (see [home_dir]).
> This can be changed with the `PARSEC_BASE_HOME_DIR` environment variable.

### Mounting workspaces

<!-- For the moment, this only covers the mountpoint as Directory strategy -->

#### When to mount?

**Organization login**: by default, all the workspaces that the user has access
to, need to be mounted according to the mountpoint strategy. Note that the user
can login into an organization either:

- by selecting an existing organization from the login page
- by completing the joining process (which triggers a login)

**Workspace access**: If a mountpoint crashes or is manually unmounted by the
user (via File Explorer) it may not be a good idea to automatically re-mount it
since this could lead to possibly infinite error loops. Instead, Parsec should
try to re-mount the workspace whenever the user access the workspace on the GUI.

- this allow to recover from errors without having a dedicated user action.

#### How to mount?

Each workspace will be mounted in a `workspace_dir_path` constructed by joining
`base_mountpoint_dir` and the `workspace_name`.

If the `workspace_dir_path` already exists, it may be "reused" to mount the
workspace:

1. if the existing directory was manually created by the user
   - if empty, should be **safe to mount here**
   - if not empty, **do not mount here** (it contains user data)
2. if the existing directory contains a valid mounted workspace (probably from a
   different organization) **do not mount here**
   - On Linux this can be checked by getting directory metadata (`stat`) and
     looking if the device that created it (`st_dev`) is different from the
     device that created  `base_mountpoint_dir`. If they are the same, then it's
     a regular empty directory.
3. if the existing directory is an artifact of a previously mounted workspace,
   should be **safe to mount here** after removing the artifact.
   - On Linux this happens when metadata cannot be obtained for the directory.
     The artifact can be removed by force unmount (`fusermount -u <directory>`).

If the directory cannot be reused for any reason, then a different path should
be selected. The more-or-less standard approach of adding a suffix to the name
is preferred (e.g. try `my workspace (1)`, `my workspace (2)` and so on).

#### Error Management

| Error | Expected behavior |
| ----- | ----------------- |
| **Unable to create workspace dir** | If either `base_mountpoint_dir` or `workspace_dir_path` cannot be created (e.g. permissions, disk space, other), the mount should be aborted as there is no way to correctly mount the workspace. |
| **Workspace unmounted** | If the workspace crashes, or is manually unmounted by the user, no action is performed. The workspace should be re-mounted when the user attemps to access the workspace again from Parsec GUI. |
| **Workspace directory removed** | If the workspace directory is manually removed by the user, no action is performed. The workspace should be re-mounted when the user attemps to access the workspace again from Parsec GUI. |

In either of the previous cases, Parsec should inform the user about the issue.
This could be done in a non-intrusive manner such as displaying a warning sign
on top of the workspace that will display a tooltip on hover.

### Unmounting workspaces

<!-- For the moment, this only covers mountpoint as Directory strategy -->

Unmounting a workspace is an important operation because it allows the user to
reduce the perimeter of data exposed to the file system (and thus to external
applications).

#### When to unmount?

Unmount should **always** be performed in the following cases:

- Organization logout: when the user logout from an organization all mounted
  workspaces from that organization should be unmounted.

> NOTE: Issue [#8381](https://github.com/Scille/parsec-cloud/issues/8381) describe
> some erratic behavior regarding cleanup on logout (sometimes workspaces are
> removed and sometimes not)

- Application shutdown: when the user exits Parsec, all mounted workspaces from
  all organizations should be unmounted.
  - On Linux, this also applies to shutdown via signals (`SIGINT`, `SIGTERM`).

## Security/Privacy/Compliance

- Note that if Parsec application crashes, user data is not exposed because the
  mounted file systems are no longer running. Only the workspaces names may
  remain as directories (artifacts). These will be removed the next time Parsec
  application starts.

[home_dir]: https://docs.rs/dirs/latest/dirs/fn.home_dir.html
