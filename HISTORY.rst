History
=======

.. towncrier release notes start


Parsec v3.7.1 (2025-12-11)
--------------------------

Features
~~~~~~~~

* Display a loading page when starting Parsec web application
  (`#11673 <https://github.com/Scille/parsec-cloud/issues/11673>`__)

* Update the user interface for workspace items in the list view.
  (`#11574 <https://github.com/Scille/parsec-cloud/issues/11574>`__)

* Application update modal appears automatically when an update is available.
  (`#11740 <https://github.com/Scille/parsec-cloud/issues/11740>`__)


Bugfixes
~~~~~~~~

* Minor GUI improvements such as logout button readability, font size
  harmonization and maximum width for profile name.
  (`#11785 <https://github.com/Scille/parsec-cloud/issues/11785>`__)

* Display an error message on SSO login if the pop-up window cannot be
  displayed.
  (`#11788 <https://github.com/Scille/parsec-cloud/issues/11788>`__)

* Fixed broken navigation when accessing a folder that was not available
  locally.
  (`#11823 <https://github.com/Scille/parsec-cloud/issues/11823>`__)


Parsec v3.7.0 (2025-11-28)
--------------------------

Features
~~~~~~~~

* Update component library (megashark-lib) and show/hide action bar buttons
  based on the screen size
  (`#11391 <https://github.com/Scille/parsec-cloud/issues/11391>`__)

* Remove support for server-side request rejection of web client based on `User-
  Agent` header (CORS configuration should be used instead).
  (`#11628 <https://github.com/Scille/parsec-cloud/issues/11628>`__)

* Added support for Single Sign-On authentication via ProConnect
  (`#10282 <https://github.com/Scille/parsec-cloud/issues/10282>`__)


Bugfixes
~~~~~~~~

* Fixed an issue where a folder could be open twice and prevent further
  navigation
  (`#11640 <https://github.com/Scille/parsec-cloud/issues/11640>`__)

* Fixed broken server CLI commands (export, inspect, sequester
  create/list/revoke)
  (`#11702 <https://github.com/Scille/parsec-cloud/issues/11702>`__)

* Fixed server error on stats route if organization does not exists
  (`#11477 <https://github.com/Scille/parsec-cloud/issues/11477>`__)

* Enhance CryptPad error management
  (`#11697 <https://github.com/Scille/parsec-cloud/issues/11697>`__)



Parsec v3.6.1 (2025-11-14)
--------------------------

Features
~~~~~~~~

* Shows tooltip on file, workspace and user name on mouse over
  (`#10486 <https://github.com/Scille/parsec-cloud/issues/10486>`__)

* Warn the user if they are trying to create a workspace with a similar name to
  one that already exists
  (`#8391 <https://github.com/Scille/parsec-cloud/issues/8391>`__)

* The database's datamodel is now checked during server startup to ensure all
  migrations have been applied.
  (`#8709 <https://github.com/Scille/parsec-cloud/issues/8709>`__)

* Remove support for Unicode in email addresses to avoid homoglyph attacks.
  (`#11336 <https://github.com/Scille/parsec-cloud/issues/11336>`__)

* Display invitations on a dedicated page
  (`#11131 <https://github.com/Scille/parsec-cloud/issues/11131>`__)

* Added a bug report feature directly in the app
  (`#10135 <https://github.com/Scille/parsec-cloud/issues/10135>`__)

* Shows a specific icon when a file is being uploaded
  (`#11057 <https://github.com/Scille/parsec-cloud/issues/11057>`__)

* Added a redirection page to handle Parsec links in both web and desktop mode
  (`#10379 <https://github.com/Scille/parsec-cloud/issues/10379>`__)


Bugfixes
~~~~~~~~

* Fixed a bug where menus and headers would display actions not available for
  certain profiles and roles
  (`#11232 <https://github.com/Scille/parsec-cloud/issues/11232>`__)

* Breaking change: server rejects any invitation enrollment attempt from a
  client using Parsec < 3.6. This is to avoid SAS codes appearing as different
  when doing an enrollment with a client using Parsec >= 3.6.
  (`#11017 <https://github.com/Scille/parsec-cloud/issues/11017>`__)

* Fixed an issue where workspaces name would not be updated in header and
  sidebar when renamed.
  (`#11056 <https://github.com/Scille/parsec-cloud/issues/11056>`__)

* Breaking change: Slightly change the SASCode algorithm to increase security
  during user enrollment.
  (`#11015 <https://github.com/Scille/parsec-cloud/issues/11015>`__)

* Stops the mountpoint properly when closing the app
  (`#11210 <https://github.com/Scille/parsec-cloud/issues/11210>`__)

* Simplified searching and selection of user in the "Copy workspace roles" modal
  by replacing the custom input and dropdown.
  (`#11328 <https://github.com/Scille/parsec-cloud/issues/11328>`__)

* Linux: Fixed an issue that would prevent the application window from
  displaying when installed with snap
  (`#11529 <https://github.com/Scille/parsec-cloud/issues/11529>`__)


Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Removed the warning for small displays shown when the window was made small
  (`#11190 <https://github.com/Scille/parsec-cloud/issues/11190>`__)

* Added an explanation text when trying to copy user workspace roles containing
  Manager or Owner roles to an External user.
  (`#9636 <https://github.com/Scille/parsec-cloud/issues/9636>`__)

* Update megashark-lib dependency and PKI tooltips
  (`#11228 <https://github.com/Scille/parsec-cloud/issues/11228>`__)

* Removed Edit button in the Viewer header for read-only users
  (`#11460 <https://github.com/Scille/parsec-cloud/issues/11460>`__)

* Added information during the invitation sas code exchange to better guide
  users
  (`#11483 <https://github.com/Scille/parsec-cloud/issues/11483>`__)



Parsec v3.5.2 (2025-10-21)
--------------------------

Bugfixes
~~~~~~~~

* Make application more resilient on corrupted certificate
  (`#11331 <https://github.com/Scille/parsec-cloud/issues/11331>`__)



Parsec v3.5.1 (2025-10-21)
--------------------------

Bugfixes
~~~~~~~~

* Improve mountpoint cleanup at startup for orphan mountpoints (Linux/macOS)
  (`#10014 <https://github.com/Scille/parsec-cloud/issues/10014>`__)

* File replace hangs via drag & drop from Finder
  (`#10054 <https://github.com/Scille/parsec-cloud/issues/10054>`__)

* File replace incorrectly deletes original file if cancelled during import
  (`#11107 <https://github.com/Scille/parsec-cloud/issues/11107>`__)

* Sort files by other than filename does not work (Firefox)
  (`#11155 <https://github.com/Scille/parsec-cloud/issues/11155>`__)


Parsec v3.5.0 (2025-09-18)
--------------------------

Features
~~~~~~~~

* On list view, add possibility to sort file/folder by Name, Last update,
  Creation date, and Size by clicking on the header label.
  (`#10921 <https://github.com/Scille/parsec-cloud/issues/10921>`__)

* Display total data size in the organization information page
  (`#10284 <https://github.com/Scille/parsec-cloud/issues/10284>`__)

* Files and folders can now be filtered by creation date.
  (`#10454 <https://github.com/Scille/parsec-cloud/issues/10454>`__)

* Allows opening links in Word Document viewer and ask for confirmation before
  opening them
  (`#10466 <https://github.com/Scille/parsec-cloud/issues/10466>`__)

* Added sort and filter by role options to the workspaces page
  (`#10182 <https://github.com/Scille/parsec-cloud/issues/10182>`__)

* Added custom branding options for web and desktop clients
  (`#5903 <https://github.com/Scille/parsec-cloud/issues/5903>`__)

* Added a checklist of security recommendations
  (`#7142 <https://github.com/Scille/parsec-cloud/issues/7142>`__)

* Display the user who last updated a file/folder
  (`#8333 <https://github.com/Scille/parsec-cloud/issues/8333>`__)

* Display a warning message at startup if long paths support is not enabled on
  Windows.
  (`#10131 <https://github.com/Scille/parsec-cloud/issues/10131>`__)

* Improved web homepage to better guide existing Parsec users on how to start
  using the web app
  (`#10643 <https://github.com/Scille/parsec-cloud/issues/10643>`__)

* File sorting option and order are now persistent
  (`#10693 <https://github.com/Scille/parsec-cloud/issues/10693>`__)

* Reorganization of the sidebar menu to improve comprehension and usability.
  (`#10846 <https://github.com/Scille/parsec-cloud/issues/10846>`__)


Bugfixes
~~~~~~~~

* Fixed generic error when trying to open a file while offline.
  (`#10296 <https://github.com/Scille/parsec-cloud/issues/10296>`__)

* Fixed recent documents not updated if renamed or deleted
  (`#10722 <https://github.com/Scille/parsec-cloud/issues/10722>`__)

* Fixed wrong remaining trial days
  (`#8405 <https://github.com/Scille/parsec-cloud/issues/8405>`__)


Improved Documentation
~~~~~~~~~~~~~~~~~~~~~~

* Add "Security best practices" section to the User Guide
  (`#10537 <https://github.com/Scille/parsec-cloud/issues/10537>`__)



Parsec v3.4.0 (2025-06-11)
--------------------------

This is the first release to include Parsec web app 🎉

Features
~~~~~~~~

* Added page for tracking order (custom order only).
  (`#9111 <https://github.com/Scille/parsec-cloud/issues/9111>`__)

* Update user interface to regroup any user profile and settings in a single
  tabbed view.
  (`#9470 <https://github.com/Scille/parsec-cloud/issues/9470>`__)

* Added picture in picture, loop and playback speed options to video and audio
  viewers
  (`#9750 <https://github.com/Scille/parsec-cloud/issues/9750>`__)

* Switch sidebar to tab menu on mobile
  (`#9805 <https://github.com/Scille/parsec-cloud/issues/9805>`__)

* Added responsive for organization page.
  (`#9878 <https://github.com/Scille/parsec-cloud/issues/9878>`__)

* Automatically scrolls to the file pointed to when using a file link
  (`#8627 <https://github.com/Scille/parsec-cloud/issues/8627>`__)

* Added responsive for files/folders item in list view.
  (`#9902 <https://github.com/Scille/parsec-cloud/issues/9902>`__)

* Added responsive for user item in list view.
  (`#9912 <https://github.com/Scille/parsec-cloud/issues/9912>`__)

* Added top bar responsive for organization page, files page, and users page
  (`#9984 <https://github.com/Scille/parsec-cloud/issues/9984>`__)

* Add new commands in the client CLI: `certificate poll`, `certificate forget-
  all-certificates`, `device change-authentication`, `device overwrite-server-
  url`
  (`#10017 <https://github.com/Scille/parsec-cloud/issues/10017>`__)

* Update responsive for Organization information page
  (`#10061 <https://github.com/Scille/parsec-cloud/issues/10061>`__)

* Added small-display global and specific contextual menus for members and
  files, as well as correct handling of contextual menu when selecting multiple
  members
  (`#9505 <https://github.com/Scille/parsec-cloud/issues/9505>`__)

* Add responsive for create trial organization
  (`#10143 <https://github.com/Scille/parsec-cloud/issues/10143>`__)

* Added responsive for profile menu
  (`#10165 <https://github.com/Scille/parsec-cloud/issues/10165>`__)


Improved Documentation
~~~~~~~~~~~~~~~~~~~~~~

* Fix incompatibility with Outsider users created with Parsec < v3.3 preventing
  them to start a client with Parsec >= 3.3
  (`#10041 <https://github.com/Scille/parsec-cloud/issues/10041>`__)

* Hosting Guide: add database and S3 backup and restore instructions
  (`#6174 <https://github.com/Scille/parsec-cloud/issues/6174>`__)

Bugfixes
~~~~~~~~

* Fixed "Remember me" login for customer area
  (`#9616 <https://github.com/Scille/parsec-cloud/issues/9616>`__)

* Added a minimal window size
  (`#9674 <https://github.com/Scille/parsec-cloud/issues/9674>`__)

* Only count active users in organization stats
  (`#9723 <https://github.com/Scille/parsec-cloud/issues/9723>`__)

* fixes file viewer dropdowns expanding out of the viewport
  (`#9798 <https://github.com/Scille/parsec-cloud/issues/9798>`__)

* Fix crash on Windows when a workspace name contains characters incompatible
  with Windows file path rules.
  (`#9925 <https://github.com/Scille/parsec-cloud/issues/9925>`__)

* Fix an error message when clicking on the dock on MacOS while the app is
  starting
  (`#9950 <https://github.com/Scille/parsec-cloud/issues/9950>`__)

* Fix log messages not being appended in libparsec log file
  (`#10011 <https://github.com/Scille/parsec-cloud/issues/10011>`__)


Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Slightly reworked the billing info page in client area to make billing address
  modifications more intuitive
  (`#9607 <https://github.com/Scille/parsec-cloud/issues/9607>`__)

* Prevent sorting users by Status if all users are active
  (`#9621 <https://github.com/Scille/parsec-cloud/issues/9621>`__)

* Updated how the new version available notification looks
  (`#9853 <https://github.com/Scille/parsec-cloud/issues/9853>`__)

* Slightly improve the workspace card UI.
  (`#9866 <https://github.com/Scille/parsec-cloud/issues/9866>`__)



Parsec v3.3.2 (2025-03-04)
--------------------------

bugfixes
~~~~~~~~

* Fix internal build issues
  (`#9849 <https://github.com/scille/parsec-cloud/issues/9854>`__)



Parsec v3.3.1 (2025-03-04)
--------------------------

bugfixes
~~~~~~~~

* fixes file opening and correctly displays an error message if it fails
  (`#9849 <https://github.com/scille/parsec-cloud/issues/9849>`__)



Parsec v3.3.0 (2025-02-25)
--------------------------

Features
~~~~~~~~

* Port the sequester feature (with realm export and mount) and expose it through
  the CLI.
  (`#6407 <https://github.com/Scille/parsec-cloud/issues/6407>`__)

* Provide the CLI for windows
  (`#9771 <https://github.com/Scille/parsec-cloud/issues/9771>`__)

* Update global user interface for file viewers
  (`#9571 <https://github.com/Scille/parsec-cloud/issues/9571>`__)

* Rename the option ``--organization-id`` to ``--organization`` in the CLI
  command ``organization status``
  (`#9192 <https://github.com/Scille/parsec-cloud/issues/9192>`__)

* The CLI command ``invite cancel`` now take the token as an argument instead of
  an option
  (`#9188 <https://github.com/Scille/parsec-cloud/issues/9188>`__)

* Show/hide submenu (recent files, workspaces, favorite workspaces) on sidebar
  (`#9088 <https://github.com/Scille/parsec-cloud/issues/9088>`__)

* Remember sort and filter options on user list
  (`#8969 <https://github.com/Scille/parsec-cloud/issues/8969>`__)

* Displays the technical identifiers of users, devices and files
  (`#9299 <https://github.com/Scille/parsec-cloud/issues/9299>`__)

* Rename the option ``--organization-id`` to ``--organization`` in the CLI
  command ``organization stats``
  (`#9191 <https://github.com/Scille/parsec-cloud/issues/9191>`__)

* Added a `See in explorer` option for files and folders
  (`#8961 <https://github.com/Scille/parsec-cloud/issues/8961>`__)

* Added a search input for the organization user list
  (`#8964 <https://github.com/Scille/parsec-cloud/issues/8964>`__)

* Invitation token is now passed as an argument instead of an option the CLI
  command ``invite greet``
  (`#9118 <https://github.com/Scille/parsec-cloud/issues/9118>`__)

* The CLI command ``user revoke`` now take the email as an argument instead of
  an option
  (`#9193 <https://github.com/Scille/parsec-cloud/issues/9193>`__)

* Add CLI command ``user revoke`` to revoke a user
  (`#9143 <https://github.com/Scille/parsec-cloud/issues/9143>`__)

* Added the ability to import recovery files directly from the starting page
  (`#9417 <https://github.com/Scille/parsec-cloud/issues/9417>`__)

* Add file viewers for some common file types (audio, image, pdf & docx)
  (`#8854 <https://github.com/Scille/parsec-cloud/issues/8854>`__)

* Replaced the list of workspaces in the sidebar by a list of the most recently
  accessed workspaces
  (`#9345 <https://github.com/Scille/parsec-cloud/issues/9345>`__)

* Redesign home page UI
  (`#9453 <https://github.com/Scille/parsec-cloud/issues/9453>`__)

* Display the user role on a workspace in grid mode and while viewing files
  (`#9174 <https://github.com/Scille/parsec-cloud/issues/9174>`__)

* Preview some file types when exploring a workspace history
  (`#9482 <https://github.com/Scille/parsec-cloud/issues/9482>`__)

* The parsec invitation is now passed as an argument instead of an option in the
  CLI command ``invite claim``
  (`#9117 <https://github.com/Scille/parsec-cloud/issues/9117>`__)

* Display suspended users
  (`#8694 <https://github.com/Scille/parsec-cloud/issues/8694>`__)

* Pass workspace name as an argument instead of an option to the CLI command
  ``workspace create``
  (`#9119 <https://github.com/Scille/parsec-cloud/issues/9119>`__)

* The CLI command ``invite user`` now take the email as an argument instead of
  an option
  (`#9190 <https://github.com/Scille/parsec-cloud/issues/9190>`__)

* User invitations are now shared between all administrators, and any available
  administrator can greet a new user.
  (`#9413 <https://github.com/Scille/parsec-cloud/issues/9413>`__)

* Port the shared recovery feature (based on the Shamir algorithm) and expose it
  through the CLI.
  (`#6090 <https://github.com/Scille/parsec-cloud/issues/6090>`__)

* Added a batch workspace sharing option to assign a chosen workspace role to
  several organization members at once.
  (`#8326 <https://github.com/Scille/parsec-cloud/issues/8326>`__)

* Displays the last opened file in the sidebar
  (`#8973 <https://github.com/Scille/parsec-cloud/issues/8973>`__)

* Added the option to update a user's profile
  (`#8248 <https://github.com/Scille/parsec-cloud/issues/8248>`__)

Bugfixes
~~~~~~~~

* Fix an issue causing the client to not synchronize properly after the user has
  accepted the Terms of Service
  (`#9614 <https://github.com/Scille/parsec-cloud/issues/9614>`__)

* Detect when the client is no longer receiving events from the server and
  report an offline status
  (`#8359 <https://github.com/Scille/parsec-cloud/issues/8359>`__)

* Fix file changed timestamp (ctime) on Unix systems
  (`#8942 <https://github.com/Scille/parsec-cloud/issues/8942>`__)

* Fixes a bug when trying to greet a new device while offline
  (`#9298 <https://github.com/Scille/parsec-cloud/issues/9298>`__)

* Ensure that the mountpoint folder exists before adding a link to it in the
  sidebar
  (`#8820 <https://github.com/Scille/parsec-cloud/issues/8820>`__)

* Fix the ``/administration/stats`` endpoint when the provided ``at`` parameter
  corresponds to a date before the creation of an existing organization.
  (`#9427 <https://github.com/Scille/parsec-cloud/issues/9427>`__)

* Fixed a few visual and functional bugs with the sidebar
  (`#8851 <https://github.com/Scille/parsec-cloud/issues/8851>`__)

* Fix a race condition producing errors when creating the same workspace twice
  in parallel.
  (`#9428 <https://github.com/Scille/parsec-cloud/issues/9428>`__)

* Correctly refreshes the displayed size of a file when copying from the file
  explorer
  (`#9388 <https://github.com/Scille/parsec-cloud/issues/9388>`__)

* Fixed the file list not being properly updated after copying a file
  (`#8970 <https://github.com/Scille/parsec-cloud/issues/8970>`__)

* Stop spinners with check mark.
  (`#9109 <https://github.com/Scille/parsec-cloud/issues/9109>`__)

* Properly formats the amount of money due when displaying invoices
  (`#9284 <https://github.com/Scille/parsec-cloud/issues/9284>`__)

* Do not report write permissions on linux and macOS when a workspace is read-
  only.
  (`#9389 <https://github.com/Scille/parsec-cloud/issues/9389>`__)

* Fixed a rare bug that could occur when switching organizations
  (`#9287 <https://github.com/Scille/parsec-cloud/issues/9287>`__)

* Fix handling of files containing a tilde in their name in the Windows file
  explorer
  (`#9171 <https://github.com/Scille/parsec-cloud/issues/9171>`__)

* Fix GUI sometime not updating folder view when new file/folder is being
  remotely added.
  (`#9102 <https://github.com/Scille/parsec-cloud/issues/9102>`__)

* Fixed a bug when logging in to the customer account to create a new
  organization
  (`#9406 <https://github.com/Scille/parsec-cloud/issues/9406>`__)

* Fix crash when reading a Parsec URL containing tab or newline characters in
  the scheme part.
  (`#9316 <https://github.com/Scille/parsec-cloud/issues/9316>`__)



Parsec v3.2.4 (2025-01-21)
--------------------------

Bugfixes
~~~~~~~~

* Fix missing error message in Windows Explorer when trying to delete a file in
  a read-only mountpoint.
  (`#9382 <https://github.com/Scille/parsec-cloud/issues/9382>`__)

* Fix wrong error showed in Windows File Explorer when moving a file to a
  destination where its name already exists.
  (`#9226 <https://github.com/Scille/parsec-cloud/issues/9226>`__)

* Fix an issue when modifying a file using TextEdit on macOS
  (`#8991 <https://github.com/Scille/parsec-cloud/issues/8991>`__)

* Fix an issue when creating a new file from the mountpoint on macOS
  (`#8976 <https://github.com/Scille/parsec-cloud/issues/8976>`__)

* Fix a panic when copying a file using the mountpoint on macOS
  (`#9173 <https://github.com/Scille/parsec-cloud/issues/9173>`__)

* Improve mountpoint behavior for read-only workspace on Linux & Windows.
  (`#9197 <https://github.com/Scille/parsec-cloud/issues/9197>`__)



Parsec v3.2.3 (2024-12-18)
--------------------------

Bugfixes
~~~~~~~~

* Fix a freeze when moving a file from the Windows File Explorer
  (`#9012 <https://github.com/Scille/parsec-cloud/issues/9012>`__)


Parsec v3.2.2 (2024-12-16)
--------------------------

Features
~~~~~~~~

* Allows overwriting default config & data directory during CLI command
  ``invite claim``
  (`#8937 <https://github.com/Scille/parsec-cloud/issues/8937>`__)

Bugfixes
~~~~~~~~

* Fix infinite loop during synchronization of a workspace when importing a file
  using the CLI command ``workspace import``
  (`#8941 <https://github.com/Scille/parsec-cloud/issues/8941>`__)

* Fix rare freeze when the folder displayed by the GUI is remotely modified.
  (`#9002 <https://github.com/Scille/parsec-cloud/issues/9002>`__)

* Fix CLI command ``invite user`` generating a device invitation link instead of
  a user invitation link.
  (`#9186 <https://github.com/Scille/parsec-cloud/issues/9186>`__)

* Fix CLI command ``invite list`` crashing when listing more than one entry
  (`#9176 <https://github.com/Scille/parsec-cloud/issues/9176>`__)

Parsec v3.2.1 (2024-12-03)
--------------------------

Bugfixes
~~~~~~~~

* Fix the server shutdown hanging indefinitely while a client is connected
  through SSE.
  (`#9099 <https://github.com/Scille/parsec-cloud/issues/9099>`__)



Parsec v3.2.0 (2024-11-22)
--------------------------

Features
~~~~~~~~

* The list of users in the organization now shows the date the user joined
  (`#5941 <https://github.com/Scille/parsec-cloud/issues/5941>`__)

* Show which users are External when sharing a workspace
  (`#8245 <https://github.com/Scille/parsec-cloud/issues/8245>`__)

* Improve Parsec update dialog and show the version about to be installed
  (`#8572 <https://github.com/Scille/parsec-cloud/issues/8572>`__)

* Added workspace history: can now browse and restore files at any given time
  from the workspace history
  (`#8663 <https://github.com/Scille/parsec-cloud/issues/8663>`__)

* Added CLI commands ``tos config``, ``tos list`` and ``tos accept`` to manage
  an organization's Term of Service
  (`#8664 <https://github.com/Scille/parsec-cloud/issues/8664>`__)

* Added full support for macOS
  (`#8669 <https://github.com/Scille/parsec-cloud/issues/8669>`__)

* Added support for recovery files to recover access in case the password is
  lost or the user loses access to its computer
  (`#8667 <https://github.com/Scille/parsec-cloud/issues/8667>`__)

* Displays the revocation date in the user details modal
  (`#8862 <https://github.com/Scille/parsec-cloud/issues/8862>`__)


Bugfixes
~~~~~~~~

* Improve snap packaging to prevent some user applications to crash when trying
  to open a file from Parsec
  (`#8737 <https://github.com/Scille/parsec-cloud/issues/8737>`__)

* Fixed a bug allowing the user to access unintended steps during organization
  creation from a bootstrap link
  (`#8771 <https://github.com/Scille/parsec-cloud/issues/8771>`__)

* Fixed a bug on Windows causing much delayed synchronization on files
  created/modified from the mount point.
  (`#8818 <https://github.com/Scille/parsec-cloud/issues/8818>`__)

* Prompt the user for administrator permissions when winfsp is being installed,
  instead of failing silently if the installer wasn't already running in
  administrator mode.
  (`#8934 <https://github.com/Scille/parsec-cloud/issues/8934>`__)

* Confined files are no longer included in selection and file count
  (`#8913 <https://github.com/Scille/parsec-cloud/issues/8913>`__)

* No longer opens an empty context menu with right clicking in the list of files
  (`#8915 <https://github.com/Scille/parsec-cloud/issues/8915>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Added tooltip explaining server synchronization status in file details
  (`#8725 <https://github.com/Scille/parsec-cloud/issues/8725>`__)

* Improve how connection issues are handled during the greeting process
  (`#8817 <https://github.com/Scille/parsec-cloud/issues/8817>`__)



Parsec v3.1.0 (2024-10-15)
--------------------------

Features
~~~~~~~~

* Allow creating devices using the keyring in the CLI.
  (`#8554 <https://github.com/Scille/parsec-cloud/issues/8554>`__)

* Added the ability to archive a device from an expired trial organization
  (`#8593 <https://github.com/Scille/parsec-cloud/issues/8593>`__)

* Remove passphrase from CLI options of ``device import-recovery-device``, it is
  now read from the standard input like the password.
  (`#8567 <https://github.com/Scille/parsec-cloud/issues/8567>`__)

* Make CLI command ``organization create`` directly take the organization ID
  instead of using the option `--organization-id`
  (`#8620 <https://github.com/Scille/parsec-cloud/issues/8620>`__)

* Rename the option ``workspace-id`` to ``workspace`` in the CLI command
  ``workspace import``.
  (`#8622 <https://github.com/Scille/parsec-cloud/issues/8622>`__)

* Rename the option ``user-id`` to ``user`` in the CLI command ``workspace
  share``.
  (`#8628 <https://github.com/Scille/parsec-cloud/issues/8628>`__)

* Rename the option ``workspace-id`` to ``workspace`` in the CLI command ``ls``.
  (`#8629 <https://github.com/Scille/parsec-cloud/issues/8629>`__)

* Rename the option ``workspace-id`` to ``workspace`` in the CLI command ``rm``.
  (`#8630 <https://github.com/Scille/parsec-cloud/issues/8630>`__)

* On the CLI when removing a device (``device remove``), it will now also remove
  its associated data (workspaces, certificates, etc).
  (`#8601 <https://github.com/Scille/parsec-cloud/issues/8601>`__)

* Terms of Services (ToS) can now be defined in the server. Users are requested
  to accept them on first connection and whenever they are updated.
  (`#7633 <https://github.com/Scille/parsec-cloud/issues/7633>`__)

* Add server option ``--proxy-trusted-addresses`` to enable parsing of proxy
  headers from trusted addresses. By default, the server will trust the proxy
  headers from localhost.
  (`#8626 <https://github.com/Scille/parsec-cloud/issues/8626>`__)

* Added right click to open context menu on users, files and workspaces
  (`#8645 <https://github.com/Scille/parsec-cloud/issues/8645>`__)


Bugfixes
~~~~~~~~

* Fixed a navigation loop when clicking the Back button after opening a
  workspace link
  (`#8571 <https://github.com/Scille/parsec-cloud/issues/8571>`__)

* Fixed incorrect toast sometimes displayed when logging out while importing
  files
  (`#8584 <https://github.com/Scille/parsec-cloud/issues/8584>`__)

* Fixed synchronization icon displaying the wrong status in file details dialog
  (`#8597 <https://github.com/Scille/parsec-cloud/issues/8597>`__)

* Fixed mocked values being sometimes displayed when logging out
  (`#8652 <https://github.com/Scille/parsec-cloud/issues/8652>`__)

* Fixed logout and go home buttons not appearing in customer area
  (`#8733 <https://github.com/Scille/parsec-cloud/issues/8733>`__)


Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Hide the suggested users when sharing a workspace with a role that cannot
  share the workspace
  (`#8538 <https://github.com/Scille/parsec-cloud/issues/8538>`__)

* Document running the server behind a reverse proxy
  (`#8427 <https://github.com/Scille/parsec-cloud/issues/8427>`__)

* Remind the users of the Terms of Service when creating a new organization
  (`#8681 <https://github.com/Scille/parsec-cloud/issues/8681>`__)

* Added an indication of the number of operations done and failed in the file
  operation menu
  (`#8557 <https://github.com/Scille/parsec-cloud/issues/8557>`__)



Parsec v3.0.3 (2024-10-03)
--------------------------

Bugfixes
~~~~~~~~

* Fix link to download parsec client in invitation email
  (`#8599 <https://github.com/Scille/parsec-cloud/issues/8599>`__)

* Fixed unhandled error when creating a new organization on SAAS
  (`#8588 <https://github.com/Scille/parsec-cloud/issues/8588>`__)

* Fixed customer area not showing a newly created organization
  (`#8578 <https://github.com/Scille/parsec-cloud/issues/8578>`__)


Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Added raw invitation URL in the invitation email
  (`#8367 <https://github.com/Scille/parsec-cloud/issues/8367>`__)


Parsec v3.0.2 (2024-10-01)
--------------------------

Features
~~~~~~~~

* Added a button in the advanced settings to open the configuration folder
  (`#8455 <https://github.com/Scille/parsec-cloud/issues/8455>`__)


Bugfixes
~~~~~~~~

* Fix incorrect error message at the end of the device creation process on the
  greeting client side.
  (`#8573 <https://github.com/Scille/parsec-cloud/issues/8573>`__)

* Fix a bug causing the size limit on the workspace cache database to be
  ignored.
  (`#8517 <https://github.com/Scille/parsec-cloud/issues/8517>`__)

* Make sure than when the server asks for a strictly greater timestamp, the
  client offsets the timestamp sent by the server.
  (`#8270 <https://github.com/Scille/parsec-cloud/issues/8270>`__)

* Fix CLI where possible identical short device ID can be displayed in command
  ``device list``. Now the size of the short ID depends on the actual devices ID
  and not the device list size.
  (`#8041 <https://github.com/Scille/parsec-cloud/issues/8041>`__)

* Fix the access to the Windows mountpoints after a hard shutdown.
  (`#8362 <https://github.com/Scille/parsec-cloud/issues/8362>`__)

* Fixed an error that could cause logging out to fail
  (`#8366 <https://github.com/Scille/parsec-cloud/issues/8366>`__)

* Enable auto-vacuum for the local sqlite databases.
  (`#8516 <https://github.com/Scille/parsec-cloud/issues/8516>`__)

* Snap: Remove script that migrate device location to ``$HOME/.config/parsec..``
  (`#8530 <https://github.com/Scille/parsec-cloud/issues/8530>`__)

* Fix potential file corruption on linux when using an application with a
  complex file saving strategy, such as LibreOffice.
  (`#8521 <https://github.com/Scille/parsec-cloud/issues/8521>`__)

* Fixed a performance issue when leaving the file import menu open
  (`#8468 <https://github.com/Scille/parsec-cloud/issues/8468>`__)


Improved Documentation
~~~~~~~~~~~~~~~~~~~~~~

* Update hosting guide (previously administration guide) for version 3
  (`#7844 <https://github.com/Scille/parsec-cloud/issues/7844>`__)


Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Removed join date option from user sort
  (`#8332 <https://github.com/Scille/parsec-cloud/issues/8332>`__)

* Added the option to go back to server type selection after choosing an option
  when creating an organization
  (`#8129 <https://github.com/Scille/parsec-cloud/issues/8129>`__)



Parsec v3.0.1 (2024-09-26)
--------------------------

Bugfixes
~~~~~~~~

* Fix for lost devices when parsec snap is removed: devices are now stored in
  ``$HOME/.config`` and are independent from snap. Existing devices are
  migrated.
  (`#8464 <https://github.com/Scille/parsec-cloud/issues/8464>`__)


Improved Documentation
~~~~~~~~~~~~~~~~~~~~~~

* Major update of user documentation for version 3
  (`#7634 <https://github.com/Scille/parsec-cloud/issues/7634>`__)



Parsec v3.0.0 (2024-09-18)
--------------------------

The 3.0 release is a major rewrite of Parsec Client from Python to Rust.
Parsec Client is now based on a Rust core library (libparsec) and a
Vue/Ionic GUI. Parsec Server is still written in Python but includes some
major changes required to support Parsec APIv4.

These changes were motivated by the need to make Parsec more easily portable
to web and mobile platforms as well as to enable integrations with external
services.

This version has no backward compatibility with the v2.x branch. Also, some
features are still to be ported to v3 (see
`v3-porting <https://github.com/Scille/parsec-cloud/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc+label%3Av3-porting>`__
issues)

Parsec v2.17.0 (2024-02-13)
---------------------------

Features
~~~~~~~~

* Add new administration routes to freeze users and prevent them from connecting
  to the server.
  (`#5811 <https://github.com/Scille/parsec-cloud/issues/5811>`__)

* Added bulk reassignment of workspace roles
  (`#5938 <https://github.com/Scille/parsec-cloud/issues/5938>`__)

* Added a `Reencrypt all` action on workspace page
  (`#5939 <https://github.com/Scille/parsec-cloud/issues/5939>`__)

* Added a small message to encourage users to promote additional owners on their
  workspaces
  (`#5940 <https://github.com/Scille/parsec-cloud/issues/5940>`__)

Bugfixes
~~~~~~~~

* Make write access to the Windows registry more robust.
  (`#6142 <https://github.com/Scille/parsec-cloud/issues/6142>`__)

* Fix the filter list when filtering user profiles.
  (`#6369 <https://github.com/Scille/parsec-cloud/issues/6369>`__)

* Add missing translation for offline availability dialog.
  (`#6224 <https://github.com/Scille/parsec-cloud/issues/6224>`__)

Improved Documentation
~~~~~~~~~~~~~~~~~~~~~~

* Add documentation for the ``recovery device`` feature. This feature was
  introduced in v2.6.0 and it was not documented until now.
  (`#5630 <https://github.com/Scille/parsec-cloud/issues/5630>`__)



Parsec v2.16.3 (2023-12-01)
---------------------------

Bugfixes
~~~~~~~~

* Check that the Parsec application is not running when executing the
  uninstaller (and other improvements to the Windows installer)
  (`#5893 <https://github.com/Scille/parsec-cloud/issues/5893>`__)



Parsec v2.16.2 (2023-11-28)
---------------------------

Bugfixes
~~~~~~~~

* Fix the Windows installer to properly uninstall the previous Parsec version (a
  regression introduced in the Parsec 2.16.0 installer prevented Parsec 2.15.0 and
  before to be properly uninstalled)
  (`#5845 <https://github.com/Scille/parsec-cloud/issues/5845>`__)



Parsec v2.16.1 (2023-11-13)
---------------------------

Bugfixes
~~~~~~~~

* Fix file creation/modification times as shown in the file explorer on Windows
  (`#5693 <https://github.com/Scille/parsec-cloud/issues/5693>`__)

* Fix progress bar issue for offline availability when the workspace contains
  more than 2GB of data
  (`#5759 <https://github.com/Scille/parsec-cloud/issues/5759>`__)



Parsec v2.16.0 (2023-10-31)
---------------------------

Features
~~~~~~~~

* Add support for archiving workspaces and planning their deletion.
  (`#5061 <https://github.com/Scille/parsec-cloud/issues/5061>`__)
* Add a dialog to manage offline availability for workspaces (i.e a user can now
  choose keep all the data available locally for offline access)
  (`#2266 <https://github.com/Scille/parsec-cloud/issues/2266>`__)
* Added user filtering by profile
  (`#4033 <https://github.com/Scille/parsec-cloud/issues/4033>`__)
* Handle the CLI flag ``--version`` on parsec sub-command. You can now type
  ``parsec --version`` to get the current version of parsec in a terminal.
  (`#4363 <https://github.com/Scille/parsec-cloud/issues/4363>`__)

Bugfixes
~~~~~~~~

* Fix a regression causing the offline availability feature to be unavailable
  for workspaces other than the first one in the list.
  (`#5556 <https://github.com/Scille/parsec-cloud/issues/5556>`__)
* Fix confusing numbering of versions in the file history display.
  (`#5561 <https://github.com/Scille/parsec-cloud/issues/5561>`__)
* Fix a concurrency issue causing connection loss when logging in after a re-
  encryption.
  (`#5542 <https://github.com/Scille/parsec-cloud/issues/5542>`__)
* Fixed an issue on MacOS where a file would be deleted when trying to save it
  from some specific native softwares.
  (`#2330 <https://github.com/Scille/parsec-cloud/issues/2330>`__)
* On linux, if you're using something other than ubuntu, you may need to install
  ``libfuse2`` or ``fuse2`` using the package manager of your system.
  (`#4760 <https://github.com/Scille/parsec-cloud/issues/4760>`__)
* Prevent accidental creation of multiple parsec organizations at the same time
  (`#3698 <https://github.com/Scille/parsec-cloud/issues/3698>`__)
* Accept parenthesis in HumanHandle's label
  (`#3749 <https://github.com/Scille/parsec-cloud/issues/3749>`__)
* Fix user invitation email sending error when the greeter's name contains
  special characters
  (`#3752 <https://github.com/Scille/parsec-cloud/issues/3752>`__)
* Fixed error when opening the QRCode for device invitation
  (`#3769 <https://github.com/Scille/parsec-cloud/issues/3769>`__)
* Hide spinner during an enrollment using PKI
  (`#3846 <https://github.com/Scille/parsec-cloud/issues/3846>`__)
* The window now takes the whole screen by default
  (`#3946 <https://github.com/Scille/parsec-cloud/issues/3946>`__)
* No longer hides user invitations if they match the search string
  (`#3967 <https://github.com/Scille/parsec-cloud/issues/3967>`__)
* Unmounted workspaces are no longer hidden by default and the state is saved
  when the application exits
  (`#3969 <https://github.com/Scille/parsec-cloud/issues/3969>`__)
* Fixed link to user guide not clickable after creation an organization
  (`#4367 <https://github.com/Scille/parsec-cloud/issues/4367>`__)
* Fix `parsec backend sequester list_services` CLI output when an error is
  returned
  (`#4368 <https://github.com/Scille/parsec-cloud/issues/4368>`__)

Improved Documentation
~~~~~~~~~~~~~~~~~~~~~~

* Add SBOM (Software Bills Of Materials) generation on software packaging. This
  provides the list of dependencies used to build the software.
  (`#4770 <https://github.com/Scille/parsec-cloud/issues/4770>`__)
* Correct the favicon displayed on the provided documentation at
  <https://docs.parsec.cloud>
  (`#4127 <https://github.com/Scille/parsec-cloud/issues/4127>`__)

Client/Backend API evolutions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* APIv1 has been removed, which cut off compatibility with Parsec client < 2.15
  and Parsec server < 2.10
  (`#3706 <https://github.com/Scille/parsec-cloud/issues/3706>`__)
* Drop API V2 support on the client side.
  (`#3837 <https://github.com/Scille/parsec-cloud/issues/3837>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Correct some typo found in PO files used by QT for english/french translation
  (`#3757 <https://github.com/Scille/parsec-cloud/issues/3757>`__)
* Added better distinction between file and folder actions in contextual menus.
  (`#3855 <https://github.com/Scille/parsec-cloud/issues/3855>`__)
* Fix an underlying issue that caused an overall performance degradation
  (`#3925 <https://github.com/Scille/parsec-cloud/issues/3925>`__)
* Added instructions to the macFUSE installation pop-up window, and updated a
  deprecated link to the installation guide.
  (`#3958 <https://github.com/Scille/parsec-cloud/issues/3958>`__)
* Fix french typography
  (`#4059 <https://github.com/Scille/parsec-cloud/issues/4059>`__)



Parsec v2.15.0 (2022-12-12)
---------------------------

Features
~~~~~~~~

* Improved the enrolment/onboarding of new users and devices
  (`#3663 <https://github.com/Scille/parsec-cloud/issues/3663>`__)

Bugfixes
~~~~~~~~

* Fix import or drag&drop of an empty folder
  (`#3534 <https://github.com/Scille/parsec-cloud/issues/3534>`__)
* Prevent accidental creation of multiple parsec organizations at the same time
  (`#3698 <https://github.com/Scille/parsec-cloud/issues/3698>`__)
* Fix user invitation email sending error when the greeter's name contains
  special characters
  (`#3752 <https://github.com/Scille/parsec-cloud/issues/3752>`__)
* Accept parenthesis in HumanHandle's label
  (`#3749 <https://github.com/Scille/parsec-cloud/issues/3749>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Updated macFUSE installation guide for MacOS 13
  (`#3588 <https://github.com/Scille/parsec-cloud/issues/3588>`__)
* Fix various typos throughout our codebase
  (`#3700 <https://github.com/Scille/parsec-cloud/issues/3700>`__)
* Recovery devices can now be renamed when created.
  (`#3537 <https://github.com/Scille/parsec-cloud/issues/3537>`__)
* Correct some typo found in PO files used by QT for english/french translation
  (`#3757 <https://github.com/Scille/parsec-cloud/issues/3757>`__)


Parsec v2.14.1 (2022-11-24)
---------------------------

Bugfixes
~~~~~~~~

* Fix an issue that prevented proper reconnection of the client in some specific
  cases
  (`#3595 <https://github.com/Scille/parsec-cloud/issues/3595>`__)
* Improved some texts for device and user invitation
  (`#3601 <https://github.com/Scille/parsec-cloud/issues/3601>`__)


Parsec v2.14.0 (2022-11-10)
---------------------------

Features
~~~~~~~~

* Make CLI case insensitive on option `--log-level` & `--log-format`. Thus
  allowing to provide the argument in upper or lower case.
  (`#3268 <https://github.com/Scille/parsec-cloud/issues/3268>`__)
* Introduce ``generate_service_certificate`` & ``import_service_certificate``
  CLI commands  (corresponding to the ends of ``create_service_certificate``
  command).  This allows to keep the sequester private keys on an offline
  machine at all time.
  (`#3198 <https://github.com/Scille/parsec-cloud/issues/3198>`__)

Bugfixes
~~~~~~~~

* Managers can no longer try to promote other users to manager
  (`#3269 <https://github.com/Scille/parsec-cloud/issues/3269>`__)
* Scrollbar should stay in place when enabling/disabling workspaces
  (`#3301 <https://github.com/Scille/parsec-cloud/issues/3301>`__)
* Fixed how workspaces are displayed when offline
  (`#3302 <https://github.com/Scille/parsec-cloud/issues/3302>`__)
* Fixes greeting a new user into an organization with a legacy user (user that
  does not have an email set)
  (`#3507 <https://github.com/Scille/parsec-cloud/issues/3507>`__)
* Disabled workspaces not longer appear enabled when logging in
  (`#3300 <https://github.com/Scille/parsec-cloud/issues/3300>`__)
* The device instance now gets logged off when an error occurs due to the disk
  being full
  (`#3335 <https://github.com/Scille/parsec-cloud/issues/3335>`__)
* Fix user search to include legacy users
  (`#3538 <https://github.com/Scille/parsec-cloud/issues/3538>`__)
* Fix email validation in the application for some special cases
  (`#3555 <https://github.com/Scille/parsec-cloud/issues/3555>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Add `parsec core stats_server` CLI command to provide per-organization usage
  statistics for the whole server.
  (`#3109 <https://github.com/Scille/parsec-cloud/issues/3109>`__)
* Remove noop `--timestamp` param from `parsec core run` CLI command
  (`#3345 <https://github.com/Scille/parsec-cloud/issues/3345>`__)


Parsec v2.13.0 (2022-10-19)
---------------------------

Features
~~~~~~~~

* Introduce ``generate_service_certificate`` & ``import_service_certificate``
  CLI commands  (corresponding to the ends of ``create_service_certificate``
  command).  This allows to keep the sequester private keys on an offline
  machine at all time.
  (`#3198 <https://github.com/Scille/parsec-cloud/issues/3198>`__)
* Add the webhook sequester service type that pushes encrypted workspace data to
  3rd party throughout HTTP. A webhook services is a sequester service with an
  encryption key. Metadata are also encrypted with the service key and send to
  the parsec metadata server. A webhook service does not store the sequester
  metadata to a storage, but it posted the metadata to an http service.
  (`#2799 <https://github.com/Scille/parsec-cloud/issues/2799>`__)
* Add a way to filter out unmounted workspaces
  (`#3056 <https://github.com/Scille/parsec-cloud/issues/3056>`__)
* Updated the file link not found error message with the name of the file
  (`#3122 <https://github.com/Scille/parsec-cloud/issues/3122>`__)
* Added new context menu option to generate file links that point to a specific
  version of a file.
  (`#3123 <https://github.com/Scille/parsec-cloud/issues/3123>`__)
* Made workspace listing faster
  (`#3125 <https://github.com/Scille/parsec-cloud/issues/3125>`__)

Bugfixes
~~~~~~~~

* Remove cut status if cut is overridden by a copy
  (`#3022 <https://github.com/Scille/parsec-cloud/issues/3022>`__)
* Fixed links in About window not opening
  (`#3023 <https://github.com/Scille/parsec-cloud/issues/3023>`__)
* Fix an error that de-sync postgresql timestamp type
  (`#3071 <https://github.com/Scille/parsec-cloud/issues/3071>`__)
* Fix a bug that caused the login time to be larger than expected for large
  organizations.
  (`#3145 <https://github.com/Scille/parsec-cloud/issues/3145>`__)
* Scrollbar should stay in place when enabling/disabling workspaces
  (`#3301 <https://github.com/Scille/parsec-cloud/issues/3301>`__)
* Fixed how workspaces are displayed when offline
  (`#3302 <https://github.com/Scille/parsec-cloud/issues/3302>`__)
* Disabled workspaces not longer appear enabled when logging in
  (`#3300 <https://github.com/Scille/parsec-cloud/issues/3300>`__)


Parsec v2.12.1 (2022-09-13)
---------------------------

Bugfixes
~~~~~~~~

* Fix an error that de-sync postgresql timestamp type
  (`#3071 <https://github.com/Scille/parsec-cloud/issues/3071>`__)


Parsec v2.12.0 (2022-09-08)
---------------------------

Features
~~~~~~~~

* Added a new folder menu in the file explorer
  (`#2672 <https://github.com/Scille/parsec-cloud/issues/2672>`__)
* Use clipboard content as default URL when joining an org in the GUI
  (`#1598 <https://github.com/Scille/parsec-cloud/issues/1598>`__)
* Add authenticated api entrypoint to execute authenticated commands.
  (`#2624 <https://github.com/Scille/parsec-cloud/issues/2624>`__)
* Added a sort menu in the file explorer
  (`#2673 <https://github.com/Scille/parsec-cloud/issues/2673>`__)
* Added a show status/show history to the current directory in the explorer
  (`#2674 <https://github.com/Scille/parsec-cloud/issues/2674>`__)

Bugfixes
~~~~~~~~

* Correct Content-Type and add User-Agent headers for client to server HTTP
  request
  (`#2979 <https://github.com/Scille/parsec-cloud/issues/2979>`__)
* Fix an error that could occur when selecting a file
  (`#2960 <https://github.com/Scille/parsec-cloud/issues/2960>`__)
* Fix rare segfault in the GUI when an event occurs at the wrong time
  (`#2839 <https://github.com/Scille/parsec-cloud/issues/2839>`__)
* Update re-encryption status when role is changed
  (`#2699 <https://github.com/Scille/parsec-cloud/issues/2699>`__)
* Fix an error that could happen when looking at a file status
  (`#2779 <https://github.com/Scille/parsec-cloud/issues/2779>`__)
* Fixed links in About window not opening
  (`#3023 <https://github.com/Scille/parsec-cloud/issues/3023>`__)

Parsec v2.11.1 (2022-08-16)
---------------------------

Bugfixes
~~~~~~~~

* Fixed a regression which prevented to launch the server in 2.11.0
  (`#2812 <https://github.com/Scille/parsec-cloud/pull/2812>`__)


Parsec v2.11.0 (2022-08-01)
---------------------------

Features
~~~~~~~~

* Added reencrypt_workspace CLI
  (`#2708 <https://github.com/Scille/parsec-cloud/issues/2708>`__)
* Display additional information on an organization
  (`#2607 <https://github.com/Scille/parsec-cloud/issues/2607>`__)
* Display in the GUI in a dialog if the organization logged into is a
  sequestered organization.
  (`#2476 <https://github.com/Scille/parsec-cloud/issues/2476>`__)
* Added an option to copy a user's email
  (`#2534 <https://github.com/Scille/parsec-cloud/issues/2534>`__)
* Introduce the sequester feature
  (`#2462 <https://github.com/Scille/parsec-cloud/issues/2462>`__)
* Display user name and device name before finishing user enrollment
  (`#2533 <https://github.com/Scille/parsec-cloud/issues/2533>`__)
* Select the file name by default when renaming a file
  (`#2531 <https://github.com/Scille/parsec-cloud/issues/2531>`__)

Bugfixes
~~~~~~~~

* Fix error in server when client searches for users with a query containing an
  invalid regex
  (`#2508 <https://github.com/Scille/parsec-cloud/issues/2508>`__)


Parsec v2.10.0 (2022-06-30)
---------------------------

Features
~~~~~~~~

* Display the number of files in the current folder
  (`#2416 <https://github.com/Scille/parsec-cloud/issues/2416>`__)

Bugfixes
~~~~~~~~

* Fix the snackbar not being hidden when the window loses the focus
  (`#2530 <https://github.com/Scille/parsec-cloud/issues/2530>`__)
* Fix duplicated files appearing in workspace sneak peek
  (`#2503 <https://github.com/Scille/parsec-cloud/issues/2503>`__)
* Prevent change of role for a user after their revocation
  (`#2505 <https://github.com/Scille/parsec-cloud/issues/2505>`__)
* Don't overwrite files when importing a new file with the same name
  (`#2504 <https://github.com/Scille/parsec-cloud/issues/2504>`__)
* Fixed invalid email error with some weird user names
  (`#2502 <https://github.com/Scille/parsec-cloud/issues/2502>`__)
* Ignore entries that are not files (i.e socket) when importing a directory
  (`#2512 <https://github.com/Scille/parsec-cloud/issues/2512>`__)
* Fix GUI sometime not detecting invitation deletion or peer has left during
  invitation greet Fix GUI workspace history date picker from being clickable
  too soon
  (`#2337 <https://github.com/Scille/parsec-cloud/issues/2337>`__)
* Prevent closing the dialog while creating an organization
  (`#2360 <https://github.com/Scille/parsec-cloud/issues/2360>`__)
* Fixed performance issues when loading a folder
  (`#2411 <https://github.com/Scille/parsec-cloud/issues/2411>`__)
* Do not display old files when entering a new workspace
  (`#2426 <https://github.com/Scille/parsec-cloud/issues/2426>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Added some feedback when entering a file name to avoid invalid names
  (`#2331 <https://github.com/Scille/parsec-cloud/issues/2331>`__)


Parsec v2.9.2 (2022-05-23)
--------------------------

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Fix 500 error in the server when using redirection with an organization
  containing non-ascii characters
  (`#2339 <https://github.com/Scille/parsec-cloud/issues/2339>`__)


Parsec v2.9.1 (2022-05-23)
--------------------------

* Fix rare error when loading remote data under load
  (`#2336 <https://github.com/Scille/parsec-cloud/issues/2336>`__)


Parsec v2.9.0 (2022-05-20)
--------------------------

Features
~~~~~~~~

* Improved how users and workspaces are filtered
  (`#2082 <https://github.com/Scille/parsec-cloud/issues/2082>`__)
* Added a "Recover device" button when the login devices list is empty
  (`#2184 <https://github.com/Scille/parsec-cloud/issues/2184>`__)
* Added a spinner while retrieving the information during a claim
  (`#2215 <https://github.com/Scille/parsec-cloud/issues/2215>`__)

Bugfixes
~~~~~~~~

* Fix the print dialog pop-up in snap builds (introduced in 2.7.0).
  (`#2161 <https://github.com/Scille/parsec-cloud/issues/2161>`__)
* Fixed small error when pressing Enter on the log in screen
  (`#2178 <https://github.com/Scille/parsec-cloud/issues/2178>`__)
* Fixed a bug preventing file saving with Apple software such as TextEdit or
  Preview.
  (`#2211 <https://github.com/Scille/parsec-cloud/issues/2211>`__)
* Fixed clicking on a file link in some instances
  (`#2223 <https://github.com/Scille/parsec-cloud/issues/2223>`__)
* Fixed an error message when dragging files
  (`#2237 <https://github.com/Scille/parsec-cloud/issues/2237>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Updated links to term and conditions
  (`#1692 <https://github.com/Scille/parsec-cloud/issues/1692>`__)
* Improve server-side error logging when S3/Swift blockstore is not working
  (`#2160 <https://github.com/Scille/parsec-cloud/issues/2160>`__)
* Added an installation guide for FUSE for MacOS.
  (`#2210 <https://github.com/Scille/parsec-cloud/issues/2210>`__)
* Updated error message when the backend cannot be reached during an invitation
  (`#2216 <https://github.com/Scille/parsec-cloud/issues/2216>`__)
* Add support for HTTP proxy
  (`#2217 <https://github.com/Scille/parsec-cloud/issues/2217>`__)
* Added an error message when failing to load a certificate
  (`#2235 <https://github.com/Scille/parsec-cloud/issues/2235>`__)
* Improve handling of keyboard interrupt in client CLI
  (`#2240 <https://github.com/Scille/parsec-cloud/issues/2240>`__)
* Stacktraces in error messages are more consistent
  (`#2253 <https://github.com/Scille/parsec-cloud/issues/2253>`__)
* Add pki certificate email address check in parsec backend on pki enrollment
  submit.
  (`#2255 <https://github.com/Scille/parsec-cloud/issues/2255>`__)
* Gives more details when an error occurs
  (`#2275 <https://github.com/Scille/parsec-cloud/issues/2275>`__)


Parsec v2.8.1 (2022-04-08)
--------------------------

Bugfixes
~~~~~~~~

* Fix regression in Parsec server introduced in version 2.7.0 leading to block
  being stored and fetched with an incorrect ID
  (`#2153 <https://github.com/Scille/parsec-cloud/issues/2153>`__)


Parsec v2.8.0 (2022-04-06)
--------------------------

Bugfixes
~~~~~~~~

* Fixed a bug in the navigation bar where clicking a parent folder didn't change
  the current directory.
  (`#2138 <https://github.com/Scille/parsec-cloud/issues/2138>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Add support for PKI based enrollment
  (`#2113 <https://github.com/Scille/parsec-cloud/issues/2113>`__)


Parsec v2.7.0 (2022-04-01)
--------------------------

Features
~~~~~~~~

* Reduces the size of users and devices
  (`#1445 <https://github.com/Scille/parsec-cloud/issues/1445>`__)
* Added an explicit message for a revoked user
  (`#1464 <https://github.com/Scille/parsec-cloud/issues/1464>`__)
* Better display disabled items in combo boxes
  (`#1864 <https://github.com/Scille/parsec-cloud/issues/1864>`__)
* Added option to copy the server address
  (`#1882 <https://github.com/Scille/parsec-cloud/issues/1882>`__)
* Added snackbar for quick messages
  (`#1885 <https://github.com/Scille/parsec-cloud/issues/1885>`__)
* Improve the detection and management of clients that have out-of-sync clocks
  compare to the backend.
  (`#1894 <https://github.com/Scille/parsec-cloud/issues/1894>`__)
* Added a navigation bar to help navigate through a workspace
  (`#1916 <https://github.com/Scille/parsec-cloud/issues/1916>`__)
* Added --recipient option to share_workspace command and the human_find command
  in the CLI
  (`#1940 <https://github.com/Scille/parsec-cloud/issues/1940>`__)
* Added new right click option for a file in a workspace, that shows the status
  of a file (created and updated when and by whom, type, location, number of
  part of a file present locally / remotely)
  (`#1941 <https://github.com/Scille/parsec-cloud/issues/1941>`__)
* The cache size per workspace can now be configured through the entry
  `workspace_storage_cache_size` in the configuration file. The default value is
  524288000, i.e. 512 MB.
  (`#1983 <https://github.com/Scille/parsec-cloud/issues/1983>`__)
* Display some messages using snackbar instead of dialogs
  (`#2014 <https://github.com/Scille/parsec-cloud/issues/2014>`__)
* Force the choice of a role when greeting a new user
  (`#2057 <https://github.com/Scille/parsec-cloud/issues/2057>`__)

Bugfixes
~~~~~~~~

* Logging out of one tab now correctly update the list of available login device
  (`#1896 <https://github.com/Scille/parsec-cloud/issues/1896>`__)
* Fix refresh in workspaces when displaying a timestamped workspace on the gui
  (`#1909 <https://github.com/Scille/parsec-cloud/issues/1909>`__)
* Improve client/server handshake mechanism
  (`#2047 <https://github.com/Scille/parsec-cloud/issues/2047>`__)
* Updated the documentation (in particular, a lot of missing French translations
  were added)
  (`#2080 <https://github.com/Scille/parsec-cloud/issues/2080>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Changed user icons depending on their role
  (`#1889 <https://github.com/Scille/parsec-cloud/issues/1889>`__)
* Fixed some graphical glitches on MacOS when using dark mode.
  (`#2037 <https://github.com/Scille/parsec-cloud/issues/2037>`__)
* Show a dedicated message when getting a local storage error during file import
  indicating that the disk might be full
  (`#2083 <https://github.com/Scille/parsec-cloud/issues/2083>`__)
* Fix connection reset on client side in case of internal server errors.
  (`#2100 <https://github.com/Scille/parsec-cloud/issues/2100>`__)
* Fix telemetry support, replace --sentry-url by --sentry-dsn/--sentry-
  environment parameters for client and server.
  (`#2102 <https://github.com/Scille/parsec-cloud/issues/2102>`__)


Parsec v2.6.0 (2021-11-09)
--------------------------

Features
~~~~~~~~

* Add filters for revoked users and pending invitations in User gui
  (`#1744 <https://github.com/Scille/parsec-cloud/issues/1744>`__)
* Show a window with the invitation link when an invitation is failing to be
  sent via email
  (`#1751 <https://github.com/Scille/parsec-cloud/issues/1751>`__)
* Added the ability to recover device via exporting recovery device and
  importing them when needed
  (`#1855 <https://github.com/Scille/parsec-cloud/issues/1855>`__)

Bugfixes
~~~~~~~~

* Fix sporadic errors that might occur when clients with slightly different
  clocks perform actions sur as reading/writing to workspaces and changing roles
  concurrently.
  (`#1811 <https://github.com/Scille/parsec-cloud/issues/1811>`__)
* Fix rare error when offline while accessing a workspace mountpoint for the
  first time.
  (`#1812 <https://github.com/Scille/parsec-cloud/issues/1812>`__)
* Fix the logic to solve the conflict in the core
  (`#1820 <https://github.com/Scille/parsec-cloud/issues/1820>`__)
* Do not refresh workspace list when mounting/unmounting a workspace
  (`#1858 <https://github.com/Scille/parsec-cloud/issues/1858>`__)
* Fixed app behavior on MacOS when closing with command-Q and red X.
  (`#1860 <https://github.com/Scille/parsec-cloud/issues/1860>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Create a pop-up notification when trying to edit a workspace as a Reader
  (`#1479 <https://github.com/Scille/parsec-cloud/issues/1479>`__)
* Add GUI support for extension-based smartcard authentication.
  (`#1878 <https://github.com/Scille/parsec-cloud/issues/1878>`__)


Parsec v2.5.5 (2021-09-30)
--------------------------

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Bundle Mozilla’s carefully curated collection of SSL Root Certificates and use
  them instead of system certificates to improve reliability.
  (`#1871 <https://github.com/Scille/parsec-cloud/issues/1871>`__)


Parsec v2.5.4 (2021-09-21)
--------------------------

Bugfixes
~~~~~~~~

* Fix 400 errors when accessing organization containing non-ascii characters
  with REST API.
  (`#1849 <https://github.com/Scille/parsec-cloud/issues/1849>`__)
* Fix incorrect maximum file length detection on linux, e.g in the Nautilus file
  explorer.
  (`#1854 <https://github.com/Scille/parsec-cloud/issues/1854>`__)
* Fix a freeze when a user import a file or a folder from another parsec
  workspace through the file explorer provided by the application.
  (`#1856 <https://github.com/Scille/parsec-cloud/issues/1856>`__)


Parsec v2.5.3 (2021-09-10)
--------------------------

Bugfixes
~~~~~~~~

* Fix mountpoint bug on Linux where some text editor could still edit files in
  a workspace as reader
  (`#1836 <https://github.com/Scille/parsec-cloud/issues/1836>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Added a native menu and keyboard shortcuts for MacOS
  (`#1838 <https://github.com/Scille/parsec-cloud/issues/1838>`__)


Parsec v2.5.2 (2021-09-08)
--------------------------

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Make Standard profile the default choice when greeting new user in GUI.
  (`#1830 <https://github.com/Scille/parsec-cloud/issues/1830>`__)


Parsec v2.5.1 (2021-09-02)
--------------------------

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Update Linux Snap packaging to base image core20
  (`#1826 <https://github.com/Scille/parsec-cloud/issues/1826>`__)


Parsec v2.5.0 (2021-09-02)
--------------------------

Bugfixes
~~~~~~~~

* Fixed a bug on MacOS where the window would freeze after the invitation
  process
  (`#1786 <https://github.com/Scille/parsec-cloud/issues/1786>`__)
* Made the QR code easier to read by removing the logo and changing its color
  (`#1787 <https://github.com/Scille/parsec-cloud/issues/1787>`__)
* Generate the proper error when creating a file with a name larger than 255
  bytes on linux
  (`#1813 <https://github.com/Scille/parsec-cloud/issues/1813>`__)
* Fix file opening on Windows and MacOS
  (`#1822 <https://github.com/Scille/parsec-cloud/issues/1822>`__)

Client/Backend API evolutions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Add active user limit configurable on a per-organization basis. Also add
  --organization-initial-user-profile-outsider-allowed and --organization-
  initial-active-users-limit options in `backend run` command.
  (`#1766 <https://github.com/Scille/parsec-cloud/issues/1766>`__)
* Remove most parts of APIv1 (only `organization_bootstrap` command is kept from
  APIv1 for backward compatibility). Remove `expiration_date` from
  `organization_config` command. Introduce the administration REST api to create
  & get information on organizations.
  (`#1810 <https://github.com/Scille/parsec-cloud/issues/1810>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Images from email invitations are now hosted directly on the Parsec server
  instead of relying on parsec.cloud website.
  (`#1780 <https://github.com/Scille/parsec-cloud/issues/1780>`__)
* Change Parsec server license to Business Source License 1.1 (BSLv1.1).
  (`#1785 <https://github.com/Scille/parsec-cloud/issues/1785>`__)
* Improve claim/greet dialog in GUI when invitation is deleted.
  (`#1806 <https://github.com/Scille/parsec-cloud/issues/1806>`__)
* Improve the file size formatting by displaying for significant figures when
  needed.
  (`#1808 <https://github.com/Scille/parsec-cloud/issues/1808>`__)
* Improve error reports sent by telemetry and CLI arguments documentation.
  (`#1823 <https://github.com/Scille/parsec-cloud/issues/1823>`__)


Parsec v2.4.2 (2021-07-06)
--------------------------

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Made the macFUSE pop-up during MacOS installation more user-friendly
  (`#1777 <https://github.com/Scille/parsec-cloud/issues/1777>`__)


Parsec v2.4.1 (2021-06-29)
--------------------------

Bugfixes
~~~~~~~~

* Fix database migration script n°6.
  (`#1774 <https://github.com/Scille/parsec-cloud/issues/1774>`__)


Parsec v2.4.0 (2021-06-29)
--------------------------

Features
~~~~~~~~

* Adds the outsider profile management in the GUI
  (`#1720 <https://github.com/Scille/parsec-cloud/issues/1720>`__)
* Add QR code on device invitation
  (`#1652 <https://github.com/Scille/parsec-cloud/issues/1652>`__)
* Introduce OUTSIDER organization user profile: an outsider cannot see the
  identity of other users within the organization. On top of that it is only
  allowed to be READER/CONTRIBUTOR on shared workspaces.
  (`#1727 <https://github.com/Scille/parsec-cloud/issues/1727>`__)
* Add `.sb-` temporary directories to the confined pattern list. Those
  directories appear on MacOS when editing `.doc` and `.docx` files.
  (`#1764 <https://github.com/Scille/parsec-cloud/issues/1764>`__)

Bugfixes
~~~~~~~~

* Added the pop-up widget to download latest app version on MacOS
  (`#1736 <https://github.com/Scille/parsec-cloud/issues/1736>`__)
* Fix some alignments issues with the workspace widgets.
  (`#1761 <https://github.com/Scille/parsec-cloud/issues/1761>`__)
* Fix error handling for drag&drop in GUI.
  (`#1732 <https://github.com/Scille/parsec-cloud/issues/1732>`__)
* Fix possible crash when sync occurs right after a workspace reencryption.
  (`#1730 <https://github.com/Scille/parsec-cloud/issues/1730>`__)

Deprecations and Removals
~~~~~~~~~~~~~~~~~~~~~~~~~

* Change the file link URL format so that file path is encrypted. This change
  breaks compatibility with previous file url format.
  (`#1637 <https://github.com/Scille/parsec-cloud/issues/1637>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Server on-organization-bootstrap webhook now allow 2xx return status instead
  of only 200.
  (`#1750 <https://github.com/Scille/parsec-cloud/issues/1750>`__)
* Add red color to remove widget dialogue confirmation button in GUI.
  (`#1758 <https://github.com/Scille/parsec-cloud/issues/1758>`__)
* Reword telemetry related dialogue in GUI.
  (`#1759 <https://github.com/Scille/parsec-cloud/issues/1759>`__)


Parsec v2.3.1 (2021-05-10)
--------------------------

Bugfixes
~~~~~~~~

* Fix blocking calls related to the local storage that might slow down the
  application.
  (`#1713 <https://github.com/Scille/parsec-cloud/issues/1713>`__)
* Fix a regression that broke the "Remount workspace at a given timestamp"
  button.
  (`#1723 <https://github.com/Scille/parsec-cloud/issues/1723>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Update recommended macFUSE version to 4.1.0 for mountpoint on macOS.
  (`#1718 <https://github.com/Scille/parsec-cloud/issues/1718>`__)


Parsec v2.3.0 (2021-05-04)
--------------------------

Features
~~~~~~~~

* Allow read access to a workspace during a re-encryption.
  (`#1650 <https://github.com/Scille/parsec-cloud/issues/1650>`__)

Bugfixes
~~~~~~~~

* Fixed Dock icon behavior on MacOS when app was closed with red X.
  (`#1519 <https://github.com/Scille/parsec-cloud/issues/1519>`__)
* Fix the server blockage while it waits for a peer.
  (`#1625 <https://github.com/Scille/parsec-cloud/issues/1625>`__)
* Added filename normalization to fix conflicts on special characters on MacOS.
  (`#1645 <https://github.com/Scille/parsec-cloud/issues/1645>`__)
* Fix confusing dialog when logging out with an on-going reencryption.
  (`#1663 <https://github.com/Scille/parsec-cloud/issues/1663>`__)
* Fix some blinking with the workspace buttons, especially while doing a
  reencryption.
  (`#1665 <https://github.com/Scille/parsec-cloud/issues/1665>`__)
* Enforce NFC string normalization for organization/device/user/entry id and
  human handle.
  (`#1708 <https://github.com/Scille/parsec-cloud/issues/1708>`__)
* Fix an issue with fuse mountpoint on linux where the shutdown procedure might
  block forever
  (`#1716 <https://github.com/Scille/parsec-cloud/issues/1716>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Update CLI command `parsec core bootstrap_organization` to accept params for
  human/device label/email.
  (`#1674 <https://github.com/Scille/parsec-cloud/issues/1674>`__)
* Improve synchronization performance by running the block uploads in parallel
  (`#1678 <https://github.com/Scille/parsec-cloud/issues/1678>`__)
* Improve Windows installer for smaller size and faster install time. Also fix
  uninstall when previous version has been installed in a custom location.
  (`#1690 <https://github.com/Scille/parsec-cloud/issues/1690>`__)


Parsec v2.2.4 (2021-03-18)
--------------------------

Features
~~~~~~~~

* Made password validation stronger in the GUI
  (`#1601 <https://github.com/Scille/parsec-cloud/issues/1601>`__)
* Added MacOS Big Sur compatibility
  (`#1640 <https://github.com/Scille/parsec-cloud/issues/1640>`__)

Bugfixes
~~~~~~~~

* Fix server event dispatching when a PostgreSQL database connection terminates
  unexpectedly.
  (`#1634 <https://github.com/Scille/parsec-cloud/issues/1634>`__)
* Fix unhandled exception in GUI when offline and workspace author UserInfo is
  not in cache. Fix view on inconsistent files in GUI.
  (`#1641 <https://github.com/Scille/parsec-cloud/issues/1641>`__)
* Fixed a mountpoint issue in MacOS that could cause errors during login or
  unmounting a workspace.
  (`#1644 <https://github.com/Scille/parsec-cloud/issues/1644>`__)
* Fixed style issues on dark mode MacOS
  (`#1646 <https://github.com/Scille/parsec-cloud/issues/1646>`__)
* Fix issue where workspace preview does not update when changes are made while
  on maintenance.
  (`#1658 <https://github.com/Scille/parsec-cloud/issues/1658>`__)

Deprecations and Removals
~~~~~~~~~~~~~~~~~~~~~~~~~

* Remove massively unused `--log-filter` option from `core gui` and `backend
  run` commands.
  (`#1639 <https://github.com/Scille/parsec-cloud/issues/1639>`__)

Client/Backend API evolutions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Bump api version to 1.3; Add the number of workspaces in the organization
  stats
  (`#1655 <https://github.com/Scille/parsec-cloud/issues/1655>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Fix backend server infinite wait on HTTP-invalid incoming request.
  (`#1611 <https://github.com/Scille/parsec-cloud/issues/1611>`__)
* Disable logging to file by default when running the GUI client.
  (`#1638 <https://github.com/Scille/parsec-cloud/issues/1638>`__)


Parsec v2.2.3 (2021-01-29)
--------------------------

Features
--------

* Added MacOS version for release

Bugfixes
~~~~~~~~

* Improved workspace loading performance (less query for reencryption)
  (`#1619 <https://github.com/Scille/parsec-cloud/issues/1619>`__)


Parsec v2.2.2 (2020-12-15)
--------------------------

No significant changes.


Parsec v2.2.1 (2020-12-15)
--------------------------

Features
--------

* Improve backend HTTP welcome page, we no longer use html like it's 1997
  (`#1603 <https://github.com/Scille/parsec-cloud/issues/1603>`__)

Bugfixes
~~~~~~~~

* Fix unhandled error on linux/macOS when logout occurs during mountpoint
  processing.
  (`#1607 <https://github.com/Scille/parsec-cloud/issues/1607>`__)


Parsec v2.2.0 (2020-12-14)
--------------------------

Features
~~~~~~~~

* Added email in workspace sharing dialog
  (`#1514 <https://github.com/Scille/parsec-cloud/issues/1514>`__)
* Reworked the dialog to see a workspace as it was to make it a little bit
  sexier
  (`#1512 <https://github.com/Scille/parsec-cloud/issues/1512>`__)
* Allow copy/cut/paste files from different workspaces.
  (`#1183 <https://github.com/Scille/parsec-cloud/issues/1183>`__)
* Backend can now force https redirection (see `--forward-proto-enforce-https`
  parameter).
  (`#1466 <https://github.com/Scille/parsec-cloud/issues/1466>`__)
* Add a spinner when opening a folder in the gui
  (`#1442 <https://github.com/Scille/parsec-cloud/issues/1442>`__)
* Add macOS compatibility
  (`#1441 <https://github.com/Scille/parsec-cloud/issues/1441>`__)
* Inviting a user already member of an organization is no longer allowed by the
  backend server
  (`#1332 <https://github.com/Scille/parsec-cloud/issues/1332>`__)
* Add widget to import and export keys
  (`#1520 <https://github.com/Scille/parsec-cloud/issues/1520>`__)
* Added a warning message when a user choses their password
  (`#525 <https://github.com/Scille/parsec-cloud/issues/525>`__)

Bugfixes
~~~~~~~~

* Fix the go back in time for workspace.
  (`#1568 <https://github.com/Scille/parsec-cloud/issues/1568>`__)
* Made copy and cut of files asynchronous in the GUI
  (`#1560 <https://github.com/Scille/parsec-cloud/issues/1560>`__)
* Cleaned choices when creating an organization in the GUI
  (`#1596 <https://github.com/Scille/parsec-cloud/issues/1596>`__)
* Mount workspace if needed when a file link is clicked
  (`#1531 <https://github.com/Scille/parsec-cloud/issues/1531>`__)
* Displays an error message when failing to open a file
  (`#1525 <https://github.com/Scille/parsec-cloud/issues/1525>`__)
* Fix an error when opening a workspace in the file explorer
  (`#1541 <https://github.com/Scille/parsec-cloud/issues/1541>`__)
* Fixed overflow error in loading dialog
  (`#1543 <https://github.com/Scille/parsec-cloud/issues/1543>`__)
* Fix uncatched error in GUI when bootstrapping organization with an invalid url
  (`#1593 <https://github.com/Scille/parsec-cloud/issues/1593>`__)
* Improved GUI style on MacOS
  (`#1447 <https://github.com/Scille/parsec-cloud/issues/1447>`__)
* Trim the user name
  (`#1544 <https://github.com/Scille/parsec-cloud/issues/1544>`__)
* Improved import error messages
  (`#1491 <https://github.com/Scille/parsec-cloud/issues/1491>`__)
* Display a correct error message if the time on the machine is not correctly
  set when creating a new org
  (`#1475 <https://github.com/Scille/parsec-cloud/issues/1475>`__)
* Clear workspace list when spinner is displayed
  (`#1515 <https://github.com/Scille/parsec-cloud/issues/1515>`__)
* Fixed crash on MacOS when closing a dialog
  (`#1538 <https://github.com/Scille/parsec-cloud/issues/1538>`__)
* Improved error message when trying to mount a workspace with no drives
  available on Windows
  (`#1542 <https://github.com/Scille/parsec-cloud/issues/1542>`__)
* Fix synchronization potentially not triggered after a file resize
  (`#1579 <https://github.com/Scille/parsec-cloud/issues/1579>`__)
* Hide return button on login screen when there's only one device
  (`#1505 <https://github.com/Scille/parsec-cloud/issues/1505>`__)

Client/Backend API evolutions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Fix incorrect definitions of entry name type for workspace and folder
  manifests in api.
  (`#1571 <https://github.com/Scille/parsec-cloud/issues/1571>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Log exceptions occurring in Qt slots
  (`#1520 <https://github.com/Scille/parsec-cloud/issues/1520>`__)
* Moved password change location in the same menu as the logout button
  (`#621 <https://github.com/Scille/parsec-cloud/issues/621>`__)
* Make OSXFUSE download link clickable in GUI
  (`#1585 <https://github.com/Scille/parsec-cloud/issues/1585>`__)
* Add support for macOS
  (`#1572 <https://github.com/Scille/parsec-cloud/issues/1572>`__)


Parsec v2.1.0 (2020-10-08)
--------------------------

Features
~~~~~~~~

* Ask directly for password if only one device is registered on the machine
  (`#1456 <https://github.com/Scille/parsec-cloud/issues/1456>`__)
* Better display for temporary workspaces
  (`#1463 <https://github.com/Scille/parsec-cloud/issues/1463>`__)
* Show a spinner while workspaces are loaded
  (`#1432 <https://github.com/Scille/parsec-cloud/issues/1432>`__)
* Add feature to display shared workspaces between two users
  (`#1454 <https://github.com/Scille/parsec-cloud/issues/1454>`__)
* Better display when user role on a workspace has been changed
  (`#1418 <https://github.com/Scille/parsec-cloud/issues/1418>`__)
* Adding Users Pagination for GUI.
  (`#1452 <https://github.com/Scille/parsec-cloud/issues/1452>`__)
* Better display of workspace reencryption
  (`#1423 <https://github.com/Scille/parsec-cloud/issues/1423>`__)
* Display login and follow link on not logged organization file link click.
  (`#1405 <https://github.com/Scille/parsec-cloud/issues/1405>`__)
* Display the volume of an organization to admins
  (`#1487 <https://github.com/Scille/parsec-cloud/issues/1487>`__)
* Better indication of the role of a user on a workspace
  (`#1478 <https://github.com/Scille/parsec-cloud/issues/1478>`__)
* Remember the previous position and size of the window
  (`#1486 <https://github.com/Scille/parsec-cloud/issues/1486>`__)
* Add parsec core cli envvar support
  (`#1473 <https://github.com/Scille/parsec-cloud/issues/1473>`__)
* Display server address in user info tooltip
  (`#1474 <https://github.com/Scille/parsec-cloud/issues/1474>`__)

Bugfixes
~~~~~~~~

* Fix the reporting of exceptions with very long traces from the backend
  connection module.
  (`#1340 <https://github.com/Scille/parsec-cloud/issues/1340>`__)
* Fix batch size in workspace reencryption leading to very slow operation.
  (`#1431 <https://github.com/Scille/parsec-cloud/issues/1431>`__)
* Fix a possible deadlock when cancelling the mounting of a workspace on linux.
  (`#1500 <https://github.com/Scille/parsec-cloud/issues/1500>`__)
* Avoid unnecessary scrolling when displaying users and devices
  (`#1449 <https://github.com/Scille/parsec-cloud/issues/1449>`__)
* Improved workspaces loading
  (`#1436 <https://github.com/Scille/parsec-cloud/issues/1436>`__)
* Fixed error message when the chosen org name already exists
  (`#1345 <https://github.com/Scille/parsec-cloud/issues/1345>`__)
* Fix an issue causing workspace files to not be closed properly.
  (`#1391 <https://github.com/Scille/parsec-cloud/issues/1391>`__)
* Refresh device list when logging out
  (`#1453 <https://github.com/Scille/parsec-cloud/issues/1453>`__)
* Validate button is disabled by default when choosing a password
  (`#1459 <https://github.com/Scille/parsec-cloud/issues/1459>`__)
* Refresh workspace list when closing the sharing dialog
  (`#1495 <https://github.com/Scille/parsec-cloud/issues/1495>`__)
* Improve client disconnection handling in the backend.
  (`#1461 <https://github.com/Scille/parsec-cloud/issues/1461>`__)
* Fixed blinking reencryption button
  (`#1485 <https://github.com/Scille/parsec-cloud/issues/1485>`__)
* Fixed opening the GUI with a file link containing an unknown org
  (`#1455 <https://github.com/Scille/parsec-cloud/issues/1455>`__)

Deprecations and Removals
~~~~~~~~~~~~~~~~~~~~~~~~~

* Remove deprecated `parsec core apiv1` commands from the cli.
  (`#1440 <https://github.com/Scille/parsec-cloud/issues/1440>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Improve error message in GUI on unexpected error.
  (`#1481 <https://github.com/Scille/parsec-cloud/issues/1481>`__)


Parsec v2.0.0 (2020-09-03)
--------------------------

No significant changes.


Parsec v1.15.2 (2020-09-02)
---------------------------

Bugfixes
~~~~~~~~

* Fix uncatched exception in GUI when listing workspaces while offline
  (`#1412 <https://github.com/Scille/parsec-cloud/issues/1412>`__)
* Fix error on Linux when using chmod/chown on mountpoint
  (`#1409 <https://github.com/Scille/parsec-cloud/issues/1409>`__)
* Contract and CGV link opens up properly
  (`#1416 <https://github.com/Scille/parsec-cloud/issues/1416>`__)
* Fixed timestamped workspace window not closing correctly on error
  (`#1421 <https://github.com/Scille/parsec-cloud/issues/1421>`__)
* Fix --backend-addr incorrectly always using localhost host in backend run
  command
  (`#1425 <https://github.com/Scille/parsec-cloud/issues/1425>`__)
* Prevent unhandled exception when trying to open an unmounted workspace
  (`#1414 <https://github.com/Scille/parsec-cloud/issues/1414>`__)
* Allow to continue reencryption from the GUI if reencryption has already been
  started
  (`#1422 <https://github.com/Scille/parsec-cloud/issues/1422>`__)
* Fix invite email in backend when not mocked
  (`#1410 <https://github.com/Scille/parsec-cloud/issues/1410>`__)


Parsec v1.15.0 (2020-08-29)
---------------------------

Features
~~~~~~~~

* Updated the logos
  (`#1316 <https://github.com/Scille/parsec-cloud/issues/1316>`__)
* Add a warning when choosing user role during the greet process
  (`#1352 <https://github.com/Scille/parsec-cloud/issues/1352>`__)
* Add support for confined (i.e temporary) files and directories. In this
  context, confined means files that are not meant to be synchronized with other
  clients
  (`#990 <https://github.com/Scille/parsec-cloud/issues/990>`__)
* Moved user info to the top right
  (`#1153 <https://github.com/Scille/parsec-cloud/issues/1153>`__)
* Explain password and confirmation mismatch
  (`#1265 <https://github.com/Scille/parsec-cloud/issues/1265>`__)
* Notify user when the current in used organization has expired
  (`#1206 <https://github.com/Scille/parsec-cloud/issues/1206>`__)
* Updated workspace sharing to be easier to use
  (`#1138 <https://github.com/Scille/parsec-cloud/issues/1138>`__)
* New organization creation process
  (`#1257 <https://github.com/Scille/parsec-cloud/issues/1257>`__)
* Sexier login screen
  (`#1130 <https://github.com/Scille/parsec-cloud/issues/1130>`__)
* Allows creating an organization on a custom metadata server
  (`#1390 <https://github.com/Scille/parsec-cloud/issues/1390>`__)
* Add one custom rsync to parsec
  (`#953 <https://github.com/Scille/parsec-cloud/issues/953>`__)
* GUI allows organization creation on a custom backend
  (`#1133 <https://github.com/Scille/parsec-cloud/issues/1133>`__)

Bugfixes
~~~~~~~~

* Do not open new login tab in the gui if a file linked is clicked with an
  already opened organization
  (`#1398 <https://github.com/Scille/parsec-cloud/issues/1398>`__)
* Do not display disconnected notification when login in
  (`#1353 <https://github.com/Scille/parsec-cloud/issues/1353>`__)
* Display the correct message when closing a connected tab
  (`#1382 <https://github.com/Scille/parsec-cloud/issues/1382>`__)
* Prevent spaces in organization name
  (`#1256 <https://github.com/Scille/parsec-cloud/issues/1256>`__)
* Check email validity when creating an organization/inviting a user
  (`#1377 <https://github.com/Scille/parsec-cloud/issues/1377>`__)
* Fixed organization creation window closing when passwords mismatch
  (`#1376 <https://github.com/Scille/parsec-cloud/issues/1376>`__)
* Do not restart claimer invitation process on an InviteAlreadyUsedError
  (`#1363 <https://github.com/Scille/parsec-cloud/issues/1363>`__)
* Fix email user invite generation
  (`#1400 <https://github.com/Scille/parsec-cloud/issues/1400>`__)
* Fix inconsistent backend replies from an cancelled invite command
  (`#1365 <https://github.com/Scille/parsec-cloud/issues/1365>`__)
* Added workspace name in error message when removed from a workspace
  (`#1385 <https://github.com/Scille/parsec-cloud/issues/1385>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Devices keys filenames are no longer meaningful.  Device key files used to be
  stored in a directory named after the device slug in a file also named after
  the same device slug. As a result, the device path used to be very long (about
  200 characters).  Device key files are now stored directly in the devices
  directory using the device slughash and the `.keys` extension. The path is now
  much shorter
  (`#1366 <https://github.com/Scille/parsec-cloud/issues/1366>`__)
* In order to simplify url validation in the GUI, parsec:// url without hostname
  part are now considered invalid instead of defaulting to localhost.
  (`#1402 <https://github.com/Scille/parsec-cloud/issues/1402>`__)
* Inviting an user to join organization now display a confirmation pop-up.
  (`#1346 <https://github.com/Scille/parsec-cloud/issues/1346>`__)
* Invited users is now displayed before the organization users
  (`#1351 <https://github.com/Scille/parsec-cloud/issues/1351>`__)
* The winfsp and fuse mountpoint now always report 0 MB used over a 1 TB
  capacity. Those values are arbitrary but useful to the operating system,
  especially OSX.
  (`#1401 <https://github.com/Scille/parsec-cloud/issues/1401>`__)


Parsec v1.14.0 (2020-08-06)
---------------------------

Features
~~~~~~~~

* Added some keyboard shortcuts
  (`#1151 <https://github.com/Scille/parsec-cloud/issues/1151>`__)
* Added a "+" button to add a new tab
  (`#1155 <https://github.com/Scille/parsec-cloud/issues/1155>`__)
* Switched app font to Montserrat
  (`#1147 <https://github.com/Scille/parsec-cloud/issues/1147>`__)
* Workspaces can now be enabled/disabled from the application. The workspace
  status is stored in the configuration in order to be restored at the next
  application startup.
  (`#1159 <https://github.com/Scille/parsec-cloud/issues/1159>`__)
* Updated user list to look more like the device list
  (`#1154 <https://github.com/Scille/parsec-cloud/issues/1154>`__)
* Allows join organization to take a bootstrap org link
  (`#1170 <https://github.com/Scille/parsec-cloud/issues/1170>`__)
* Hide an already connected device from the list of available devices
  (`#1139 <https://github.com/Scille/parsec-cloud/issues/1139>`__)
* Added an automated email sending function on user invite to workspace
  (`#1177 <https://github.com/Scille/parsec-cloud/issues/1177>`__)
* Added additional text for the main menu
  (`#1150 <https://github.com/Scille/parsec-cloud/issues/1150>`__)
* Added optional RC channel updater
  (`#1324 <https://github.com/Scille/parsec-cloud/issues/1324>`__)
* Display systray notification to make offline mode more obvious to the users
  (`#1330 <https://github.com/Scille/parsec-cloud/issues/1330>`__)

Bugfixes
~~~~~~~~

* Display author name in file history instead of DeviceID
  (`#1270 <https://github.com/Scille/parsec-cloud/issues/1270>`__)
* Fix GUI behavior when trying to share a workspace while not connected to the
  backend or wen providing an invalid user name
  (`#1242 <https://github.com/Scille/parsec-cloud/issues/1242>`__)
* Fixed revoked user exception handling and notification.
  (`#1205 <https://github.com/Scille/parsec-cloud/issues/1205>`__)
* Bootstrap organization widget made more responsive on low resolutions
  (`#1169 <https://github.com/Scille/parsec-cloud/issues/1169>`__)
* Fixed menu icons alignment and colors
  (`#1149 <https://github.com/Scille/parsec-cloud/issues/1149>`__)
* Fixed missing reject method on file history
  (`#1239 <https://github.com/Scille/parsec-cloud/issues/1239>`__)
* Fixed history window not showing when a file has a source.
  (`#1182 <https://github.com/Scille/parsec-cloud/issues/1182>`__)
* Fix realm access check in backend for user who has lost it role to this realm.
  (`#1184 <https://github.com/Scille/parsec-cloud/issues/1184>`__)
* Fix sharing error message causing unhandled exception in the GUI
  (`#1241 <https://github.com/Scille/parsec-cloud/issues/1241>`__)
* Fix Python 3.8 incompatibility (bug in trio_asyncio with postgresql)
  (`#1194 <https://github.com/Scille/parsec-cloud/issues/1194>`__)
* Fixed some hidden windows staying in memory
  (`#1156 <https://github.com/Scille/parsec-cloud/issues/1156>`__)
* Added clearer messages on failure to access a file by its link
  (`#1167 <https://github.com/Scille/parsec-cloud/issues/1167>`__)
* Improve high DPI support for the parsec application.
  (`#1245 <https://github.com/Scille/parsec-cloud/issues/1245>`__)
* Updating pynacl to 1.4.0
  (`#1172 <https://github.com/Scille/parsec-cloud/issues/1172>`__)
* Fix history button in GUI
  (`#1243 <https://github.com/Scille/parsec-cloud/issues/1243>`__)
* Fix error on Windows when using the mountpoint right after (<0.01s) it has
  been mounted.
  (`#1210 <https://github.com/Scille/parsec-cloud/issues/1210>`__)
* Path display no longer makes the window expand
  (`#1162 <https://github.com/Scille/parsec-cloud/issues/1162>`__)
* The workspaces are now mounted as separated drives on Windows. Also,
  workspaces with reader access are mounted as read-only volumes. This allows
  proper compatibility with Acrobat Reader and avoid path-length issues.
  (`#1081 <https://github.com/Scille/parsec-cloud/issues/1081>`__)
* Fixed deadlock when importing a file from a parsec workspace
  (`#1188 <https://github.com/Scille/parsec-cloud/issues/1188>`__)
* Fix GUI main windows not showing when use close button from the systray. Notification
  explaining Parsec is still running on GUI windows close only triggered once.
  (`#1295 <https://github.com/Scille/parsec-cloud/issues/1295>`__)
* Fix backend side connection auto-close on user revocation when the connection
  has been used to listen events.
  (`#1314 <https://github.com/Scille/parsec-cloud/issues/1314>`__)
* Fixed workspace title showing id instead of name
  (`#1321 <https://github.com/Scille/parsec-cloud/issues/1321>`__)
* Fix internal exception handling of the remote devices manager errors.
  (`#1335 <https://github.com/Scille/parsec-cloud/issues/1335>`__)

Client/Backend API evolutions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Add --spontaneous-organization-bootstrap option to backend to allow
  bootstrapping an organization that haven't been created by administration
  beforehand. Add --organization-bootstrap-webhook option to backend to notify a
  webhook URL on organization bootstrap.
  (`#1281 <https://github.com/Scille/parsec-cloud/issues/1281>`__)
* Update API to version 2.0 which improve handshake system and rework enrollment
  system for a SAS-based asynchronous one (better usability and security)
  (`#1119 <https://github.com/Scille/parsec-cloud/issues/1119>`__)
* API can now return stats about workspace such as metadata size and data size.
  (`#1176 <https://github.com/Scille/parsec-cloud/issues/1176>`__)
* Introduce outsider profile for user. Outsider users can read/write on
  workspaces they are invited to, but are not allowed to create workspaces. On
  top of that outsider users cannot see personal information (email &
  user/device name) of other users.
  (`#1163 <https://github.com/Scille/parsec-cloud/issues/1163>`__)
* Adding some http request management.
  (`#1171 <https://github.com/Scille/parsec-cloud/issues/1171>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Remove ``(shared by X)`` messages from workspace name.
  (`#928 <https://github.com/Scille/parsec-cloud/issues/928>`__)
* Add a high-level interface for workspace files.
  (`#1190 <https://github.com/Scille/parsec-cloud/issues/1190>`__)
* Consider https as default endpoint scheme for blockstore config in backend run
  cli
  (`#1143 <https://github.com/Scille/parsec-cloud/issues/1143>`__)
* Turn user_id and device_name fields into UUID to anonymize them. Personal
  information are instead stored in human_handle and device_label fields which
  are not available to users with OUTSIDER profile.
  (`#1174 <https://github.com/Scille/parsec-cloud/issues/1174>`__)
* Change bytes symbol in English
  (`#1221 <https://github.com/Scille/parsec-cloud/issues/1221>`__)
* Update WinFSP embedded package
  (`#1223 <https://github.com/Scille/parsec-cloud/issues/1223>`__)
* Use 4 symbols from a 32-symbol alphabet as SAS code. The alphabet is:
  ``ABCDEFGHJKLMNPQRSTUVWXYZ23456789``.
  (`#1165 <https://github.com/Scille/parsec-cloud/issues/1165>`__)
* Backend now able to retry first db connection
  (`#1258 <https://github.com/Scille/parsec-cloud/issues/1258>`__)
* Remove noop --db-drop-deleted-data option from backend run command
  (`#1246 <https://github.com/Scille/parsec-cloud/issues/1246>`__)
* Added docker-compose as a backend deployment option
  (`#1233 <https://github.com/Scille/parsec-cloud/issues/1233>`__)
* Add DPI aware option in the Windows installer options to fix blurry texts on
  some high-DPI screens.
  (`#1203 <https://github.com/Scille/parsec-cloud/issues/1203>`__)
* Update windows installer to be less verbose. In particular: skip the
  components panel, hide installation details and advance automatically after
  completion.
  (`#1126 <https://github.com/Scille/parsec-cloud/issues/1126>`__)
* Restrict read access for parsec directories to the current user. This includes
  configuration, data, config and workspace directories.
  (`#940 <https://github.com/Scille/parsec-cloud/issues/940>`__)
* Fix mount error when using Snap package on Debian when fuse is not installed.
  (`#1296 <https://github.com/Scille/parsec-cloud/issues/1296>`__)
* Run Parsec with regular user privileges when the "Run Parsec" checkbox is
  ticked at the end of the windows installation.
  (`#1303 <https://github.com/Scille/parsec-cloud/issues/1303>`__)
* Updated instructions texts for the device invitation process
  (`#1304 <https://github.com/Scille/parsec-cloud/issues/1304>`__)


Parsec 1.13.0 (2020-04-29)
--------------------------

Features
~~~~~~~~

* Added a way to create an organization on the business website directly from
  the GUI
  (`#1014 <https://github.com/Scille/parsec-cloud/issues/1014>`__)
* Add one migration tool in the cli.
  (`#1116 <https://github.com/Scille/parsec-cloud/issues/1116>`__)
* Add an action to open the current directory in file explorer
  (`#1107 <https://github.com/Scille/parsec-cloud/issues/1107>`__)
* Add a contextual menu on workspace buttons
  (`#1085 <https://github.com/Scille/parsec-cloud/issues/1085>`__)
* Updated file icons to reflect the file format
  (`#1093 <https://github.com/Scille/parsec-cloud/issues/1093>`__)

Bugfixes
~~~~~~~~

* Allow closing of login in tab
  (`#1101 <https://github.com/Scille/parsec-cloud/issues/1101>`__)
* Fixed GUI staying minimized when an URL is clicked
  (`#1100 <https://github.com/Scille/parsec-cloud/issues/1100>`__)
* Fix internal behavior involving cancelled tasks that could lead to unhandled
  errors logs.
  (`#1123 <https://github.com/Scille/parsec-cloud/issues/1123>`__)
* Fix save operations on windows for some third party applications.  This is
  related to the mechanism used by third party applications to safely save
  files. This mechanism might use the `replace_if_exists` flag in the `rename`
  winfsp operation. This flag is now supported.
  (`#1128 <https://github.com/Scille/parsec-cloud/issues/1128>`__)
* Allows workspace owners to change the role of other owners
  (`#870 <https://github.com/Scille/parsec-cloud/issues/870>`__)
* Fixed alignment problem when displaying users
  (`#1127 <https://github.com/Scille/parsec-cloud/issues/1127>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Improve high CPU usage and blocking IO detection.
  (`#1124 <https://github.com/Scille/parsec-cloud/issues/1124>`__)
* Update API to version 1.2 which add human handle system
  (`#1104 <https://github.com/Scille/parsec-cloud/issues/1104>`__)


Parsec 1.12.0 (2020-04-14)
--------------------------

Bugfixes
~~~~~~~~

* Fix forbidden error during backend startup when some custom S3 providers
  (`#1094 <https://github.com/Scille/parsec-cloud/issues/1094>`__)
* Use "localhost" as the default hostname in the cli.
  (`#1075 <https://github.com/Scille/parsec-cloud/issues/1075>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Add `fs.entry.file_conflict_resolved` internal event to be notified when a
  file conflict has been resolved by copying and renaming the file with the
  local changes.
  (`#1095 <https://github.com/Scille/parsec-cloud/issues/1095>`__)
* Add cancel button to "Parsec is already running, please close it" prompt in
  windows installer.
  (`#1103 <https://github.com/Scille/parsec-cloud/issues/1103>`__)
* Update the windows installer to be less verbose. In particular, the Winfsp
  installation becomes silent.
  (`#1112 <https://github.com/Scille/parsec-cloud/issues/1112>`__)


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
  texts, and a few fixes
  (`#952 <https://github.com/Scille/parsec-cloud/issues/952>`__)


Parsec 1.10.0 (2020-03-26)
--------------------------

Features
~~~~~~~~

* Improved updater now selects the right latest exe file on Windows
  (`#1054 <https://github.com/Scille/parsec-cloud/issues/1054>`__)

Bugfixes
~~~~~~~~

* Fix ``parsec backend init`` cli command crashing due to a missing
  ``init_tables.sql`` resource.
  (`#1052 <https://github.com/Scille/parsec-cloud/issues/1052>`__)
* Fix unhandled error message in GUI that could occur during sync with poor
  connection.
  (`#1055 <https://github.com/Scille/parsec-cloud/issues/1055>`__)
* Fix marker issue when listing many files in a directory.
  (`#1039 <https://github.com/Scille/parsec-cloud/issues/1039>`__)


Parsec 1.9.1 (2020-03-13)
-------------------------

Bugfixes
~~~~~~~~

* Added missing organization_update to admin cmds
  (`#1032 <https://github.com/Scille/parsec-cloud/issues/1032>`__)


Parsec 1.9.0 (2020-03-06)
-------------------------

Features
~~~~~~~~

* Only allows one log in tab in all situations
  (`#963 <https://github.com/Scille/parsec-cloud/issues/963>`__)

Bugfixes
~~~~~~~~

* Fixed invalid access to file table item
  (`#1021 <https://github.com/Scille/parsec-cloud/issues/1021>`__)
* Fix error handling during workspace reencryption detection when offline.
  (`#1016 <https://github.com/Scille/parsec-cloud/issues/1016>`__)
* Fix an error on linux when mounting a workspace when the workspace manifest is
  absent and the session is offline.
  (`#1018 <https://github.com/Scille/parsec-cloud/issues/1018>`__)
* Fix invalid access to workspace_id on entry_updated
  (`#1022 <https://github.com/Scille/parsec-cloud/issues/1022>`__)
* Fix workspace_fs not available on event
  (`#1001 <https://github.com/Scille/parsec-cloud/issues/1001>`__)
* Fix access to invalid attribute on timestamped workspace
  (`#1020 <https://github.com/Scille/parsec-cloud/issues/1020>`__)
* Fix synchronization not triggered for newly created workspaces until they get
  files.
  (`#1023 <https://github.com/Scille/parsec-cloud/issues/1023>`__)


Parsec 1.8.0 (2020-03-03)
-------------------------

Features
~~~~~~~~

* Added a link to the documentation
  (`#999 <https://github.com/Scille/parsec-cloud/issues/999>`__)
* Removed confirmation when opening a new tab
  (`#993 <https://github.com/Scille/parsec-cloud/issues/993>`__)

Bugfixes
~~~~~~~~

* Fix French translation for changelog
  (`#994 <https://github.com/Scille/parsec-cloud/issues/994>`__)
* Case insensitive extension matching when displaying file icon
  (`#1007 <https://github.com/Scille/parsec-cloud/issues/1007>`__)

Improved Documentation
~~~~~~~~~~~~~~~~~~~~~~

* Add french translation to the documentation
  (`#1005 <https://github.com/Scille/parsec-cloud/issues/1005>`__)


Parsec 1.7.2 (2020-02-24)
-------------------------

No significant changes.


Parsec 1.7.1 (2020-02-24)
-------------------------

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Fix bug in sdist/bdist_wheel configuration that prevented release on pypi.org
  since 1.4.0
  (`#992 <https://github.com/Scille/parsec-cloud/issues/992>`__)


Parsec 1.7.0 (2020-02-22)
-------------------------

Features
~~~~~~~~

* Add a way to copy/paste an internal link to a file
  (`#937 <https://github.com/Scille/parsec-cloud/issues/937>`__)
* Access a file directly using an url
  (`#938 <https://github.com/Scille/parsec-cloud/issues/938>`__)

Bugfixes
~~~~~~~~

* Disable file operations for a reader
  (`#981 <https://github.com/Scille/parsec-cloud/issues/981>`__)
* Fix files display not being updated automatically
  (`#980 <https://github.com/Scille/parsec-cloud/issues/980>`__)


Parsec 1.6.0 (2020-02-12)
-------------------------

Features
~~~~~~~~

* Added a global menu to the GUI
  (`#945 <https://github.com/Scille/parsec-cloud/issues/945>`__)
* Add a line under the tab bar
  (`#942 <https://github.com/Scille/parsec-cloud/issues/942>`__)
* Removed tab title length limit
  (`#944 <https://github.com/Scille/parsec-cloud/issues/944>`__)

Bugfixes
~~~~~~~~

* Clear password input when switching device on login
  (`#946 <https://github.com/Scille/parsec-cloud/issues/946>`__)
* Fix files display on low horizontal resolutions
  (`#926 <https://github.com/Scille/parsec-cloud/issues/926>`__)
* Display an error when trying to move a folder into itself
  (`#935 <https://github.com/Scille/parsec-cloud/issues/935>`__)
* Fix users and devices being hidden on low resolutions
  (`#927 <https://github.com/Scille/parsec-cloud/issues/927>`__)
* Disable Paste button if nothing has been copied/cut
  (`#934 <https://github.com/Scille/parsec-cloud/issues/934>`__)
* Fix menu bar being resized when changing window size
  (`#932 <https://github.com/Scille/parsec-cloud/issues/932>`__)


Parsec 1.5.0 (2020-01-20)
-------------------------

Features
~~~~~~~~

* Add copy, cut and paste to the Parsec file explorer
  (`#855 <https://github.com/Scille/parsec-cloud/issues/855>`__)

Bugfixes
~~~~~~~~

* Fix unhandled exception in backend when a client connected over ssl disconnect
  during handshake.
  (`#833 <https://github.com/Scille/parsec-cloud/issues/833>`__)
* Fix Organization bootstrap and user/device claim links encoding when their
  corresponding organization ID contains unicode.
  (`#884 <https://github.com/Scille/parsec-cloud/issues/884>`__)
* Fix recreation of an organization by the administration as long as it hasn't
  been bootstrapped.
  (`#885 <https://github.com/Scille/parsec-cloud/issues/885>`__)
* Clear displayed files on stat error
  (`#920 <https://github.com/Scille/parsec-cloud/issues/920>`__)
* Fix a bug related to broken symlinks in the base directory for mountpoints
  after a hard shutdown.
  (`#881 <https://github.com/Scille/parsec-cloud/issues/881>`__)
* Used new partial strategy to download manifests when rebuilding history to fix
  it not loading on a heavy workspace.
  (`#888 <https://github.com/Scille/parsec-cloud/issues/888>`__)
* Fix incorrect behavior when the backend accept anonymous connection to expired
  organization.
  (`#891 <https://github.com/Scille/parsec-cloud/issues/891>`__)
* Prevent winfsp from freezing the application when the mounting operation times
  out.
  (`#905 <https://github.com/Scille/parsec-cloud/issues/905>`__)
* Prevent managers from inviting other users as managers
  (`#916 <https://github.com/Scille/parsec-cloud/issues/916>`__)
* Deal with special dash paths in fuse operations.
  (`#904 <https://github.com/Scille/parsec-cloud/issues/904>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Allow owners to switch the role of other owners
  (`#870 <https://github.com/Scille/parsec-cloud/issues/870>`__)


Parsec 1.4.0 (2019-12-06)
-------------------------

Bugfixes
~~~~~~~~

* Fix error handling of list&revoke user in GUI.
  (`#834 <https://github.com/Scille/parsec-cloud/issues/834>`__)
* Fix mount error on Windows when workspace name is too long
  (`#838 <https://github.com/Scille/parsec-cloud/issues/838>`__)
* Fix colored workspace button display
  (`#851 <https://github.com/Scille/parsec-cloud/issues/851>`__)
* Fix bug when the workspaces doesn't show up on new device creation until the
  user manifest is actually modified.
  (`#854 <https://github.com/Scille/parsec-cloud/issues/854>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Provide fusepy with the file system encoding. Also use EINVAL as fallback
  error code.
  (`#827 <https://github.com/Scille/parsec-cloud/issues/827>`__)


Parsec 1.3.0 (2019-11-28)
-------------------------

Features
~~~~~~~~

* Add a button to manually add a new tab Do not open a new tab when launching
  the app without any parameters
  (`#774 <https://github.com/Scille/parsec-cloud/issues/774>`__)
* Allow only one Log-In tab
  (`#777 <https://github.com/Scille/parsec-cloud/issues/777>`__)
* Hide revoked users in workspace sharing dialog
  (`#780 <https://github.com/Scille/parsec-cloud/issues/780>`__)
* Prevent tab change if a modal is open
  (`#820 <https://github.com/Scille/parsec-cloud/issues/820>`__)
* Tab color changes when an instance receives a notification
  (`#821 <https://github.com/Scille/parsec-cloud/issues/821>`__)

Bugfixes
~~~~~~~~

* Now handles inconsistent directories accessed from the GUI, tested mountpoint
  behavior
  (`#782 <https://github.com/Scille/parsec-cloud/issues/782>`__)
* Fix infinite loop in IPC server
  (`#813 <https://github.com/Scille/parsec-cloud/issues/813>`__)
* Fix config not saved when updating from the settings tab when logged in.
  (`#815 <https://github.com/Scille/parsec-cloud/issues/815>`__)
* Fix duplication and infinite loading in view on directories containing many
  entries under Windows.
  (`#835 <https://github.com/Scille/parsec-cloud/issues/835>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Change the invitation token format to 6 random digits.
  (`#819 <https://github.com/Scille/parsec-cloud/issues/819>`__)


Parsec 1.2.1 (2019-11-20)
-------------------------

* Add view to Display changelog history in the GUI
  (`#788 <https://github.com/Scille/parsec-cloud/issues/788>`__)


Parsec 1.2.0 (2019-11-15)
-------------------------

Features
~~~~~~~~

* Backend now checks if timestamp is not inferior of existent on vlob update, if
  it is, sends an error to client which temporarily goes offline to avoid the
  handling of this event in a retry loop.
  (`#758 <https://github.com/Scille/parsec-cloud/issues/758>`__)
* Add notification in GUI when an operation in the mountpoint failed in an
  unexpected manner.
  (`#759 <https://github.com/Scille/parsec-cloud/issues/759>`__)
* Limit a tab title to a few characters and add a tooltip to tabs
  (`#775 <https://github.com/Scille/parsec-cloud/issues/775>`__)
* Add tooltips to taskbar buttons
  (`#776 <https://github.com/Scille/parsec-cloud/issues/776>`__)
* Removed duplicates and supposed minimal sync when listing versions of a path
  (`#784 <https://github.com/Scille/parsec-cloud/issues/784>`__)

Bugfixes
~~~~~~~~

* Fix crash on Linux when the ipc server lock file is located in a non existent
  directory
  (`#760 <https://github.com/Scille/parsec-cloud/issues/760>`__)
* Fix crash in ipc server when socket file path contains missing folder (only on
  windows).
  (`#765 <https://github.com/Scille/parsec-cloud/issues/765>`__)
* Fix rights checking in winfsp operations. This issue used to cause a cffi
  crash on windows when some operations were performed on the file system.
  (`#770 <https://github.com/Scille/parsec-cloud/issues/770>`__)
* Fix len check in ``OrganizationID``/``UserID``/``DeviceName``/``DeviceID``
  when containing multi-bytes unicode characters.
  (`#794 <https://github.com/Scille/parsec-cloud/issues/794>`__)
* Improve support of unicode in the mountpoint on Windows.
  (`#799 <https://github.com/Scille/parsec-cloud/issues/799>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Improve logging output on backend server
  (`#753 <https://github.com/Scille/parsec-cloud/issues/753>`__)


Parsec 1.1.2 (2019-10-22)
-------------------------

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Small GUI improvements on white border around main tab and url
  error message display
* Remove dependency on pywin32 under Windows which cause packaging issue on
  previous version
  (`#750 <https://github.com/Scille/parsec-cloud/issues/750>`__)


Parsec 1.1.1 (2019-10-21)
-------------------------

Bugfixes
~~~~~~~~

* Fix argument parsing in backend cli commands (``PARSEC_CMD_ARGS`` env var, db
  param and S3 entry point default value)
  (`#749 <https://github.com/Scille/parsec-cloud/issues/749>`__)


Parsec 1.1.0 (2019-10-21)
-------------------------

Features
~~~~~~~~

* Add support for IPC communication in GUI to have a single instance running.
  Also add tab support & handle parsec:// url as start argument.
  (`#684 <https://github.com/Scille/parsec-cloud/issues/684>`__)
* Rework backend cli argument and environ variable handling
  (`#701 <https://github.com/Scille/parsec-cloud/issues/701>`__)

Bugfixes
~~~~~~~~

* Fix pure HTTP query handling in backend
  (`#699 <https://github.com/Scille/parsec-cloud/issues/699>`__)
* Fix long wait on GUI login with poor connection to the backend
  (`#706 <https://github.com/Scille/parsec-cloud/issues/706>`__)
* Add missing check in core to enforce consistency of timestamps between a
  manifest and it author's role certificate
  (`#734 <https://github.com/Scille/parsec-cloud/issues/734>`__)
* Fix fonts scaling on wayland
  (`#735 <https://github.com/Scille/parsec-cloud/issues/735>`__)
* Fix bug causing workspace mountpoint directory not being removed on
  application shutdown
  (`#737 <https://github.com/Scille/parsec-cloud/issues/737>`__)

Miscellaneous internal changes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

* Allow dash character (i.e. ``-``) in OrganizationID, UserID & DeviceName
  (`#728 <https://github.com/Scille/parsec-cloud/issues/728>`__)


Parsec 1.0.2 (2019-10-01)
-------------------------

* Upgrade PyQt5 to 5.13.1
  (`#690 <https://github.com/Scille/parsec-cloud/issues/690>`__)
* Add keepalive pings on invite/claim requests
  (`#693 <https://github.com/Scille/parsec-cloud/issues/693>`__)


Parsec 1.0.1 (2019-09-25)
-------------------------

* Upgrade wsproto to 0.15.0 to improve websocket compatibility
  (`#686 <https://github.com/Scille/parsec-cloud/issues/686>`__)
* Replace CXFreeze by a custom script to generate win32 builds
  (`#685 <https://github.com/Scille/parsec-cloud/issues/685>`__)
* Add organization status command in cli
  (`#683 <https://github.com/Scille/parsec-cloud/issues/683>`__)
* User/device invitation get cancelled on server side when the user use the
  cancel button
  (`#682 <https://github.com/Scille/parsec-cloud/issues/682>`__)
* Add organization expiration date support in backend
  (`#680 <https://github.com/Scille/parsec-cloud/issues/680>`__)
* Client connection to Backend specify a `/ws` resource endpoint
  (`#678 <https://github.com/Scille/parsec-cloud/issues/678>`__)


Parsec 1.0.0 (2019-09-10)
-------------------------

* First stable release
