History
=======


.. towncrier release notes start


Parsec 1.14.0 (2020-07-21)
--------------------------

Features
~~~~~~~~

* Workspaces can now be enabled/disabled from the application. The workspace
  status is stored in the configuration in order to be restored at the next
  application startup.  (`#1159 <https://github.com/Scille/parsec-
  cloud/issues/1159>`__)
* Updated user list to look more like the device list  (`#1154
  <https://github.com/Scille/parsec-cloud/issues/1154>`__)
* Added additional text for the main menu  (`#1150
  <https://github.com/Scille/parsec-cloud/issues/1150>`__)
* Added some keyboard shortcuts  (`#1151 <https://github.com/Scille/parsec-
  cloud/issues/1151>`__)
* Added a "+" button to add a new tab  (`#1155
  <https://github.com/Scille/parsec-cloud/issues/1155>`__)
* Hide an already connected device from the list of available devices  (`#1139
  <https://github.com/Scille/parsec-cloud/issues/1139>`__)
* Allows join organization to take a bootstrap org link  (`#1170
  <https://github.com/Scille/parsec-cloud/issues/1170>`__)
* Switched app font to Montserrat  (`#1147 <https://github.com/Scille/parsec-
  cloud/issues/1147>`__)
* Added an automated email sending function on user invite to workspace  (`#1177
  <https://github.com/Scille/parsec-cloud/issues/1177>`__)

Bugfixes
~~~~~~~~

* The workspaces are now mounted as separated drives on Windows. Also,
  workspaces with reader access are mounted as read-only volumes. This allows
  proper compatibility with Acrobat Reader and avoid path-length issues.
  (`#1081 <https://github.com/Scille/parsec-cloud/issues/1081>`__)
* Fix Python 3.8 incompatibility (bug in trio_asyncio with postgresql)  (`#1194
  <https://github.com/Scille/parsec-cloud/issues/1194>`__)
* Fixed deadlock when importing a file from a parsec workspace  (`#1188
  <https://github.com/Scille/parsec-cloud/issues/1188>`__)
* Fix error on Windows when using the mountpoint right after (<0.01s) it has
  been mounted. (`#1210 <https://github.com/Scille/parsec-cloud/issues/1210>`__)
* Fixed revoked user exception handling and notification.  (`#1205
  <https://github.com/Scille/parsec-cloud/issues/1205>`__)
* Fixed history window not showing when a file has a source.  (`#1182
  <https://github.com/Scille/parsec-cloud/issues/1182>`__)
* Fixed some hidden windows staying in memory  (`#1156
  <https://github.com/Scille/parsec-cloud/issues/1156>`__)
* Fix sharing error message causing unhandled exception in the GUI  (`#1241
  <https://github.com/Scille/parsec-cloud/issues/1241>`__)
* Updating pynacl to 1.4.0 (`#1172 <https://github.com/Scille/parsec-
  cloud/issues/1172>`__)
* Fix GUI behavior when trying to share a workspace while not connected to the
  backend or wen providing an invalid user name  (`#1242
  <https://github.com/Scille/parsec-cloud/issues/1242>`__)
* Fix realm access check in backend for user who has lost it role to this realm.
  (`#1184 <https://github.com/Scille/parsec-cloud/issues/1184>`__)
* Fix history button in GUI  (`#1243 <https://github.com/Scille/parsec-
  cloud/issues/1243>`__)
* Bootstrap organization widget made more responsive on low resolutions  (`#1169
  <https://github.com/Scille/parsec-cloud/issues/1169>`__)
* Added clearer messages on failure to access a file by its link  (`#1167
  <https://github.com/Scille/parsec-cloud/issues/1167>`__)
* Fixed missing reject method on file history  (`#1239
  <https://github.com/Scille/parsec-cloud/issues/1239>`__)
* Path display no longer makes the window expand  (`#1162
  <https://github.com/Scille/parsec-cloud/issues/1162>`__)
* Improve high DPI support for the parsec application.  (`#1245
  <https://github.com/Scille/parsec-cloud/issues/1245>`__)
* Fixed menu icons alignement and colors  (`#1149
  <https://github.com/Scille/parsec-cloud/issues/1149>`__)

Client/Backend API evolutions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* API can now return stats about workspace such as metadata size and data size.
  (`#1176 <https://github.com/Scille/parsec-cloud/issues/1176>`__)
* Introduce outsider profil for user. Outsider users can read/write on
  workspaces they are invited to, but are not allowed to create workspaces. On
  top of that outsider users cannot see personnal informations (email &
  user/device name) of other users.  (`#1163 <https://github.com/Scille/parsec-
  cloud/issues/1163>`__)
* Add --spontaneous-organization-bootstrap option to backend to allow
  bootstrapping an organization that haven't been created by administration
  beforehand. Add --oganization-bootstrap-webhook option to backend to notify a
  webhook URL on organization bootstrap.  (`#1281
  <https://github.com/Scille/parsec-cloud/issues/1281>`__)
* Update API to version 2.0 which improve handshake system and rework enrollment
  system for a SAS-based asynchronous one (better usability and security)
  (`#1119 <https://github.com/Scille/parsec-cloud/issues/1119>`__)
* Adding some http request managment.  (`#1171
  <https://github.com/Scille/parsec-cloud/issues/1171>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Restrict read access for parsec directories to the current user. This includes
  configuration, data, config and workspace directories.  (`#940
  <https://github.com/Scille/parsec-cloud/issues/940>`__)
* Added docker-compose as a backend deployment option  (`#1233
  <https://github.com/Scille/parsec-cloud/issues/1233>`__)
* Turn user_id and device_name fields into UUID to anonymize them. Personal
  informations are instead stored in human_handle and device_label fields which
  are not available to users with OUTSIDER profile.  (`#1174
  <https://github.com/Scille/parsec-cloud/issues/1174>`__)
* Use 4 symbols from a 32-symbol alphabet as SAS code. The alphatbet is:
  ``ABCDEFGHJKLMNPQRSTUVWXYZ23456789``.  (`#1165
  <https://github.com/Scille/parsec-cloud/issues/1165>`__)
* Backend now able to retry first db connection  (`#1258
  <https://github.com/Scille/parsec-cloud/issues/1258>`__)
* Remove noop --db-drop-deleted-data option from backend run command  (`#1246
  <https://github.com/Scille/parsec-cloud/issues/1246>`__)
* Update WinFSP embedded package  (`#1223 <https://github.com/Scille/parsec-
  cloud/issues/1223>`__)
* Add a high-level interface for workspace files.  (`#1190
  <https://github.com/Scille/parsec-cloud/issues/1190>`__)
* Update windows installer to be less verbose. In particular: skip the
  components panel, hide installation details and advance automatically after
  completion.  (`#1126 <https://github.com/Scille/parsec-cloud/issues/1126>`__)
* Consider https as default endpoint scheme for blockstore config in backend run
  cli (`#1143 <https://github.com/Scille/parsec-cloud/issues/1143>`__)
* Change bytes symbol in English  (`#1221 <https://github.com/Scille/parsec-
  cloud/issues/1221>`__)
* Add DPI aware option in the Windows installer options to fix blurry texts on
  some high-DPI screens.  (`#1203 <https://github.com/Scille/parsec-
  cloud/issues/1203>`__)
* Remove ``(shared by X)`` messages from workspace name.  (`#928
  <https://github.com/Scille/parsec-cloud/issues/928>`__)


Parsec 1.13.0 (2020-04-29)
--------------------------

Features
~~~~~~~~

* Added a way to create an organization on the business website directly from
  the GUI  (`#1014 <https://github.com/Scille/parsec-cloud/issues/1014>`__)
* Add one migration tool in the cli.  (`#1116 <https://github.com/Scille/parsec-
  cloud/issues/1116>`__)
* Add an action to open the current directory in file explorer  (`#1107
  <https://github.com/Scille/parsec-cloud/issues/1107>`__)
* Add a contextual menu on workspace buttons  (`#1085
  <https://github.com/Scille/parsec-cloud/issues/1085>`__)
* Updated file icons to reflect the file format  (`#1093
  <https://github.com/Scille/parsec-cloud/issues/1093>`__)

Bugfixes
~~~~~~~~

* Allow closing of login in tab  (`#1101 <https://github.com/Scille/parsec-
  cloud/issues/1101>`__)
* Fixed GUI staying minimized when an URL is clicked  (`#1100
  <https://github.com/Scille/parsec-cloud/issues/1100>`__)
* Fix internal behavior involving cancelled tasks that could lead to unhandled
  errors logs.  (`#1123 <https://github.com/Scille/parsec-cloud/issues/1123>`__)
* Fix save operations on windows for some third party applications.  This is
  related to the mechanism used by third party applications to safely save
  files. This mechanism might use the `replace_if_exists` flag in the `rename`
  winfsp operation. This flag is now supported.  (`#1128
  <https://github.com/Scille/parsec-cloud/issues/1128>`__)
* Allows workspace owners to change the role of other owners  (`#870
  <https://github.com/Scille/parsec-cloud/issues/870>`__)
* Fixed alignment problem when displaying users  (`#1127
  <https://github.com/Scille/parsec-cloud/issues/1127>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Improve high CPU usage and blocking IO detection.  (`#1124
  <https://github.com/Scille/parsec-cloud/issues/1124>`__)
* Update API to version 1.2 which add human handle system  (`#1104
  <https://github.com/Scille/parsec-cloud/issues/1104>`__)


Parsec 1.12.0 (2020-04-14)
--------------------------

Bugfixes
~~~~~~~~

* Fix forbidden error during backend startup when some custom S3 providers
  (`#1094 <https://github.com/Scille/parsec-cloud/issues/1094>`__)
* Use "localhost" as the default hostname in the cli.  (`#1075
  <https://github.com/Scille/parsec-cloud/issues/1075>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Add `fs.entry.file_conflict_resolved` internal event to be notified when a
  file conflict has been resolved by copying and renaming the file with the
  local changes.  (`#1095 <https://github.com/Scille/parsec-
  cloud/issues/1095>`__)
* Add cancel button to "Parsec is already running, please close it" prompt in
  windows installer. (`#1103 <https://github.com/Scille/parsec-
  cloud/issues/1103>`__)
* Update the windows installer to be less verbose. In particular, the Winfsp
  installation becomes silent.  (`#1112 <https://github.com/Scille/parsec-
  cloud/issues/1112>`__)


Parsec 1.11.4 (2020-03-31)
--------------------------

No significant changes.


Parsec 1.11.3 (2020-03-31)
--------------------------

No significant changes.


Parsec 1.11.2 (2020-03-31)
--------------------------

No significant changes.


Parsec 1.11.1 (2020-03-31)
--------------------------

No significant changes.


Parsec 1.11.0 (2020-03-30)
--------------------------

Features
~~~~~~~~

* The overall appearance of the GUI has changed: new icons, new colors, new
  texts, and a few fixes  (`#952 <https://github.com/Scille/parsec-
  cloud/issues/952>`__)


Parsec 1.10.0 (2020-03-26)
--------------------------

Features
~~~~~~~~

* Improved updater now selects the right latest exe file on Windows  (`#1054
  <https://github.com/Scille/parsec-cloud/issues/1054>`__)

Bugfixes
~~~~~~~~

* Fix ``parsec backend init`` cli command crashing due to a missing
  ``init_tables.sql`` resource. (`#1052 <https://github.com/Scille/parsec-
  cloud/issues/1052>`__)
* Fix unhandled error message in GUI that could occur during sync with poor
  connection. (`#1055 <https://github.com/Scille/parsec-cloud/issues/1055>`__)
* Fix marker issue when listing many files in a directory.  (`#1039
  <https://github.com/Scille/parsec-cloud/issues/1039>`__)


Parsec 1.9.1 (2020-03-13)
-------------------------

Bugfixes
~~~~~~~~

* Added missing organization_update to admin cmds  (`#1032
  <https://github.com/Scille/parsec-cloud/issues/1032>`__)


Parsec 1.9.0 (2020-03-06)
-------------------------

Features
~~~~~~~~

* Only allows one log in tab in all situations  (`#963
  <https://github.com/Scille/parsec-cloud/issues/963>`__)

Bugfixes
~~~~~~~~

* Fixed invalid access to file table item  (`#1021
  <https://github.com/Scille/parsec-cloud/issues/1021>`__)
* Fix error handling during workspace reencryption detection when offline.
  (`#1016 <https://github.com/Scille/parsec-cloud/issues/1016>`__)
* Fix an error on linux when mounting a workspace when the workspace manifest is
  absent and the session is offline.  (`#1018 <https://github.com/Scille/parsec-
  cloud/issues/1018>`__)
* Fix invalid access to workspace_id on entry_updated  (`#1022
  <https://github.com/Scille/parsec-cloud/issues/1022>`__)
* Fix workspace_fs not available on event  (`#1001
  <https://github.com/Scille/parsec-cloud/issues/1001>`__)
* Fix access to invalid attribute on timestamped workspace  (`#1020
  <https://github.com/Scille/parsec-cloud/issues/1020>`__)
* Fix synchronization not triggered for newly created workspaces until they get
  files. (`#1023 <https://github.com/Scille/parsec-cloud/issues/1023>`__)


Parsec 1.8.0 (2020-03-03)
-------------------------

Features
~~~~~~~~

* Added a link to the documentation  (`#999 <https://github.com/Scille/parsec-
  cloud/issues/999>`__)
* Removed confirmation when opening a new tab  (`#993
  <https://github.com/Scille/parsec-cloud/issues/993>`__)

Bugfixes
~~~~~~~~

* Fix French translation for changelog  (`#994
  <https://github.com/Scille/parsec-cloud/issues/994>`__)
* Case insensitive extension matching when displaying file icon  (`#1007
  <https://github.com/Scille/parsec-cloud/issues/1007>`__)

Improved Documentation
~~~~~~~~~~~~~~~~~~~~~~

* Add french translation to the documentation (`#1005
  <https://github.com/Scille/parsec-cloud/issues/1005>`__)


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
