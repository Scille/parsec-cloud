<!-- Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS -->

# Glossary

<!-- markdownlint-disable no-inline-html -->
<table>
  <tr>
    <th>Noun (EN / FR)</th>
    <th>Verb (EN / FR)</th>
    <th>Definition</th>
    <th>Not to be confused with</th>
    <th>Synonyms to avoid</th>
  </tr>
  <tr>
    <td>User / Utilisateur</td>
    <td></td>
    <td>A Parsec User. They can be standard or outsider.</td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <td>Standard user / Utilisateur standard
    <td></td>
    <td>
      They can have any role in a workspace (Owner, Manager, Contributor, Reader).
      Create workspaces in their organizations.
      Add other devices to their organizations.
      This is set during the onboarding process.
    </td>
    <td>
      Workspaces-specific roles:
      <ul>
        <li>Owner</li>
        <li>Manager</li>
        <li>Contributor</li>
        <li>Reader</li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>Administrator / Administrateur</td>
    <td></td>
    <td>
      Administrator role is set by default for the workspace creator.
      A standard user can be administrator in workspaces.
      This role can be set during the onboarding process for any other user who joins the organization.
      Administrators can invite and revoke other users in the organization.
    </td>
    <td>
      Workspaces-specific roles:
      <ul>
        <li>Owner</li>
        <li>Manager</li>
        <li>Contributor</li>
        <li>Reader</li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>Outsider / Externe</td>
    <td></td>
    <td>
      Outsiders users can read/write on workspaces they are invited to,
      but aren't allowed to create workspaces themselves.
      They cannot see personal information (email & user/device name) of other users.
      They can only be Contributor or Reader in a Workspace.
      This is set during the onboarding process.
    </td>
    <td>
      Workspaces-specific roles:
      <ul>
        <li>Owner</li>
        <li>Manager</li>
        <li>Contributor</li>
        <li>Reader</li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>Organization / Organisation</td>
    <td></td>
    <td>
      The main structure in Parsec, in which users can create and share workspaces,
      and administrators can invite other users.
    </td>
    <td>Workspace: Workspaces are part of an organization</td>
    <td></td>
  </tr>
  <tr>
    <td>Workspace / Espace de travail</td>
    <td></td>
    <td>
      Workspaces can be created by any user inside an organization, and can be shared between users in the same organization.
    </td>
    <td>Organization: A workspace is created in an organization</td>
    <td></td>
  </tr>
  <tr>
    <td>Device / Appareil</td>
    <td></td>
    <td>
      The device(s) registered by the user to access their organization(s),
      each device is linked to a singular user for a specific organization.
    </td>
    <td>
      <ul>
        <li>Workspace</li>
        <li>User</li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>File / Fichier</td>
    <td></td>
    <td></td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <td>Folder / Dossier</td>
    <td></td>
    <td></td>
    <td></td>
    <td>Directory</td>
  </tr>
  <tr>
    <td>Onboard / Accueil</td>
    <td>to onboard / accueillir</td>
    <td>
      The onboarding process happens after an invitation.
      This consists in the several steps between the inviter and invitee for the latter to join an organization.
    </td>
    <td></td>
    <td></td>
  </tr>
  <tr>
    <td>Invitation / Invitation</td>
    <td>to invite / inviter</td>
    <td>Getting a new user into a given organization</td>
    <td>
      <ul>
        <li>
          <strong>The onboarding process:</strong>
          This is specifically used for the part where both the host and the new user are connected simultaneously to exchange information in order to finalize the invitation
        </li>
        <li>
          <strong>Adding a device:</strong>
          Invitation is specifically used for users, the term "adding" should be used instead for devices
        </li>
      </ul>
    </td>
    <td>
      <ul>
        <li>Enroll</li>
        <li>Enlist</li>
        <li>Onboard</li>
        <li>Greet</li>
        <li>Join</li>
      </ul>
    </td>
  </tr>
  <tr>
    <td>Joining / Rejoindre</td>
    <td></td>
    <td>A new user joins an existing organization.</td>
    <td>The onboarding process / invitation</td>
    <td></td>
  </tr>
  <tr>
    <td>Owner / Propriétaire</td>
    <td></td>
    <td>A workspace owner, with manager rights.</td>
    <td>
      Organization roles:
      <ul>
        <li>Standard User</li>
        <li>Administrator</li>
        <li>Outsider</li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>Manager / Gestionnaire</td>
    <td></td>
    <td>
      A workspace role with read/write rights in a workspace, and who can invite, promote or demote others users up to Manager on this workspace.
    </td>
    <td>
      Organization roles:
      <ul>
        <li>Standard User</li>
        <li>Administrator</li>
        <li>Outsider</li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>Contributor / Contributeur</td>
    <td></td>
    <td>A workspace role with read/write rights in a workspace.</td>
    <td>
      Organization roles:
      <ul>
        <li>Standard User</li>
        <li>Administrator</li>
        <li>Outsider</li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>Reader / Lecteur</td>
    <td></td>
    <td>A workspace role with read-only rights in a workspace.</td>
    <td>
      Organization roles:
      <ul>
        <li>Standard User</li>
        <li>Administrator</li>
        <li>Outsider</li>
      </ul>
    </td>
    <td></td>
  </tr>
  <tr>
    <td>Passphrase / Phrase secrète</td>
    <td></td>
    <td>
      A passphrase is equivalent to a password, but instead of being composed of characters (letter, digit, ...)
      it is composed of words (can also include digit and special characters like the password).
    </td>
    <td></td>
    <td> / Phrase de passe</td>
  </tr>
  <tr>
    <td>Server / Serveur</td>
    <td></td>
    <td>
      The Parsec server is the application hosted on a remote computer interacting with the S3 data storage and the metadata PostgreSQL database.
    </td>
    <td></td>
    <td>Backend / </td>
  </tr>
</table>
