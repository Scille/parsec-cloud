History
=======


.. towncrier release notes start


Parsec 1.7.2 (2020-02-24)
-------------------------

No significant changes.


Parsec 1.7.1 (2020-02-24)
-------------------------

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Fix bug in sdist/bdist_wheel configuration that prevented release on pypi.org
  since 1.4.0 (`#992 <https://github.com/Scille/parsec-cloud/issues/992>`__)


Parsec 1.7.0 (2020-02-22)
-------------------------

Features
~~~~~~~~

* Add a way to copy/paste an internal link to a file  (`#937
  <https://github.com/Scille/parsec-cloud/issues/937>`__)
* Access a file directly using an url  (`#938 <https://github.com/Scille/parsec-
  cloud/issues/938>`__)

Bugfixes
~~~~~~~~

* Disable file operations for a reader  (`#981
  <https://github.com/Scille/parsec-cloud/issues/981>`__)
* Fix files display not being updated automatically  (`#980
  <https://github.com/Scille/parsec-cloud/issues/980>`__)


Parsec 1.6.0 (2020-02-12)
-------------------------

Features
~~~~~~~~

* Added a global menu to the GUI  (`#945 <https://github.com/Scille/parsec-
  cloud/issues/945>`__)
* Add a line under the tab bar  (`#942 <https://github.com/Scille/parsec-
  cloud/issues/942>`__)
* Removed tab title length limit  (`#944 <https://github.com/Scille/parsec-
  cloud/issues/944>`__)

Bugfixes
~~~~~~~~

* Clear password input when switching device on login  (`#946
  <https://github.com/Scille/parsec-cloud/issues/946>`__)
* Fix files display on low horizontal resolutions  (`#926
  <https://github.com/Scille/parsec-cloud/issues/926>`__)
* Display an error when trying to move a folder into itself  (`#935
  <https://github.com/Scille/parsec-cloud/issues/935>`__)
* Fix users and devices being hidden on low resolutions  (`#927
  <https://github.com/Scille/parsec-cloud/issues/927>`__)
* Disable Paste button if nothing has been copied/cut  (`#934
  <https://github.com/Scille/parsec-cloud/issues/934>`__)
* Fix menu bar being resized when changing window size  (`#932
  <https://github.com/Scille/parsec-cloud/issues/932>`__)


Parsec 1.5.0 (2020-01-20)
-------------------------

Features
~~~~~~~~

* Add copy, cut and paste to the Parsec file explorer  (`#855
  <https://github.com/Scille/parsec-cloud/issues/855>`__)

Bugfixes
~~~~~~~~

* Fix unhandled exception in backend when a client connected over ssl disconnect
  during handshake. (`#833 <https://github.com/Scille/parsec-
  cloud/issues/833>`__)
* Fix Organization bootstrap and user/device claim links encoding when their
  corresponding organization ID contains unicode. (`#884
  <https://github.com/Scille/parsec-cloud/issues/884>`__)
* Fix recreation of an organization by the administration as long as it hasn't
  been bootstrapped.  (`#885 <https://github.com/Scille/parsec-
  cloud/issues/885>`__)
* Clear displayed files on stat error  (`#920 <https://github.com/Scille/parsec-
  cloud/issues/920>`__)
* Fix a bug related to broken symlinks in the base directory for mountpoints
  after a hard shutdown.  (`#881 <https://github.com/Scille/parsec-
  cloud/issues/881>`__)
* Used new partial strategy to download manifests when rebuilding history to fix
  it not loading on a heavy workspace.  (`#888
  <https://github.com/Scille/parsec-cloud/issues/888>`__)
* Fix incorrect behavior when the backend accept anonymous connection to expired
  organization. (`#891 <https://github.com/Scille/parsec-cloud/issues/891>`__)
* Prevent winfsp from freezing the application when the mounting operation times
  out.  (`#905 <https://github.com/Scille/parsec-cloud/issues/905>`__)
* Prevent managers from inviting other users as managers  (`#916
  <https://github.com/Scille/parsec-cloud/issues/916>`__)
* Deal with special dash paths in fuse operations.  (`#904
  <https://github.com/Scille/parsec-cloud/issues/904>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Allow owners to switch the role of other owners  (`#870
  <https://github.com/Scille/parsec-cloud/issues/870>`__)


Parsec 1.4.0 (2019-12-06)
-------------------------

Bugfixes
~~~~~~~~

* Fix error handling of list&revoke user in GUI. (`#834
  <https://github.com/Scille/parsec-cloud/issues/834>`__)
* Fix mount error on Windows when workspace name is too long (`#838
  <https://github.com/Scille/parsec-cloud/issues/838>`__)
* Fix colored workspace button display  (`#851
  <https://github.com/Scille/parsec-cloud/issues/851>`__)
* Fix bug when the workspaces doesn't show up on new device creation until the
  user manifest is actually modified. (`#854 <https://github.com/Scille/parsec-
  cloud/issues/854>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Provide fusepy with the file system encoding. Also use EINVAL as fallback
  error code.  (`#827 <https://github.com/Scille/parsec-cloud/issues/827>`__)


Parsec 1.3.0 (2019-11-28)
-------------------------

Features
~~~~~~~~

* Add a button to manually add a new tab Do not open a new tab when launching
  the app without any parameters (`#774 <https://github.com/Scille/parsec-
  cloud/issues/774>`__)
* Allow only one Log-In tab (`#777 <https://github.com/Scille/parsec-
  cloud/issues/777>`__)
* Hide revoked users in workspace sharing dialog (`#780
  <https://github.com/Scille/parsec-cloud/issues/780>`__)
* Prevent tab change if a modal is open (`#820
  <https://github.com/Scille/parsec-cloud/issues/820>`__)
* Tab color changes when an instance receives a notification (`#821
  <https://github.com/Scille/parsec-cloud/issues/821>`__)

Bugfixes
~~~~~~~~

* Now handles inconsistent directories accessed from the GUI, tested mountpoint
  behaviour (`#782 <https://github.com/Scille/parsec-cloud/issues/782>`__)
* Fix infinite loop in IPC server (`#813 <https://github.com/Scille/parsec-
  cloud/issues/813>`__)
* Fix config not saved when updating from the settings tab when logged in.
  (`#815 <https://github.com/Scille/parsec-cloud/issues/815>`__)
* Fix duplication and infinite loading in view on directories containing many
  entries under Windows. (`#835 <https://github.com/Scille/parsec-
  cloud/issues/835>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Change the invitation token format to 6 random digits.  (`#819
  <https://github.com/Scille/parsec-cloud/issues/819>`__)


Parsec 1.2.1 (2019-11-20)
-------------------------

* Add view to Display changelog history in the GUI (`#788
  <https://github.com/Scille/parsec-cloud/issues/788>`__)


Parsec 1.2.0 (2019-11-15)
-------------------------

Features
~~~~~~~~

* Backend now checks if timestamp is not inferior of existant on vlob update, if
  it is, sends an error to client which temporarily goes offline to avoid the
  handling of this event in a retry loop.  (`#758
  <https://github.com/Scille/parsec-cloud/issues/758>`__)
* Add notification in GUI when an operation in the mountpoint failed in an
  unexpected manner. (`#759 <https://github.com/Scille/parsec-
  cloud/issues/759>`__)
* Limit a tab title to a few characters and add a tooltip to tabs  (`#775
  <https://github.com/Scille/parsec-cloud/issues/775>`__)
* Add tooltips to taskbar buttons  (`#776 <https://github.com/Scille/parsec-
  cloud/issues/776>`__)
* Removed duplicates and supposed minimal sync when listing versions of a path
  (`#784 <https://github.com/Scille/parsec-cloud/issues/784>`__)

Bugfixes
~~~~~~~~

* Fix crash on Linux when the ipc server lock file is located in a non existant
  directory (`#760 <https://github.com/Scille/parsec-cloud/issues/760>`__)
* Fix crash in ipc server when socket file path contains missing folder (only on
  windows).  (`#765 <https://github.com/Scille/parsec-cloud/issues/765>`__)
* Fix rights checking in winfsp operations. This issue used to cause a cffi
  crash on windows when some operations were performed on the file system.
  (`#770 <https://github.com/Scille/parsec-cloud/issues/770>`__)
* Fix len check in ``OrganizationID``/``UserID``/``DeviceName``/``DeviceID``
  when containing multibytes unicode characters. (`#794
  <https://github.com/Scille/parsec-cloud/issues/794>`__)
* Improve support of unicode in the mountpoint on Windows. (`#799
  <https://github.com/Scille/parsec-cloud/issues/799>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Improve logging output on backend server  (`#753
  <https://github.com/Scille/parsec-cloud/issues/753>`__)


Parsec 1.1.2 (2019-10-22)
-------------------------

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Small GUI improvements on white border around main tab and url
  error message display
* Remove dependency on pywin32 under Windows which cause packaging issue on
  previous version (`#750 <https://github.com/Scille/parsec-
  cloud/issues/750>`__)


Parsec 1.1.1 (2019-10-21)
-------------------------

Bugfixes
~~~~~~~~

* Fix argument parsing in backend cli commands (``PARSEC_CMD_ARGS`` env var, db
  param and S3 entry point default value) (`#749
  <https://github.com/Scille/parsec-cloud/issues/749>`__)


Parsec 1.1.0 (2019-10-21)
-------------------------

Features
~~~~~~~~

* Add support for IPC communication in GUI to have a single instance running.
  Also add tab support & handle parsec:// url as start argument.  (`#684
  <https://github.com/Scille/parsec-cloud/issues/684>`__)
* Rework backend cli argument and environ variable handling  (`#701
  <https://github.com/Scille/parsec-cloud/issues/701>`__)

Bugfixes
~~~~~~~~

* Fix pure HTTP query handling in backend (`#699
  <https://github.com/Scille/parsec-cloud/issues/699>`__)
* Fix long wait on GUI login with poor connection to the backend (`#706
  <https://github.com/Scille/parsec-cloud/issues/706>`__)
* Add missing check in core to enforce consistency of timestamps between a
  manifest and it author's role certificate (`#734
  <https://github.com/Scille/parsec-cloud/issues/734>`__)
* Fix fonts scaling on wayland (`#735 <https://github.com/Scille/parsec-
  cloud/issues/735>`__)
* Fix bug causing workspace mountpoint directory not being removed on
  application shutdown (`#737 <https://github.com/Scille/parsec-
  cloud/issues/737>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Allow dash character (i.e. ``-``) in OrganizationID, UserID & DeviceName
  (`#728 <https://github.com/Scille/parsec-cloud/issues/728>`__)


Parsec 1.0.2 (2019-10-01)
-------------------------

* Upgrade PyQt5 to 5.13.1 (`#690
  <https://github.com/Scille/parsec-cloud/issues/690>`__)
* Add keepalive pings on invite/claim requests (`#693
  <https://github.com/Scille/parsec-cloud/issues/693>`__)


Parsec 1.0.1 (2019-09-25)
-------------------------

* Upgrade wsproto to 0.15.0 to improve websocket compatibility (`#686
  <https://github.com/Scille/parsec-cloud/issues/686>`__)
* Replace CXFreeze by a custom script to generate win32 builds (`#685
  <https://github.com/Scille/parsec-cloud/issues/685>`__)
* Add organization status command in cli (`#683
  <https://github.com/Scille/parsec-cloud/issues/683>`__)
* User/device invitation get cancelled on server side when the user use the
  cancel button (`#682 <https://github.com/Scille/parsec-cloud/issues/682>`__)
* Add organization expiration date support in backend (`#680
  <https://github.com/Scille/parsec-cloud/issues/680>`__)
* Client connection to Backend specify a `/ws` resource endpoint (`#678
  <https://github.com/Scille/parsec-cloud/issues/678>`__)


Parsec 1.0.0 (2019-09-10)
-------------------------

* First stable release
