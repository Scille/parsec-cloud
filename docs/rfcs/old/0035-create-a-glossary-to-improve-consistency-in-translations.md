# Create a glossary to improve consistency in translations

From [ISSUE-3852](https://github.com/Scille/parsec-cloud/issues/3852)

We should write and maintain a glossary that will serve as a reference for the different concepts that appear in the application. In particular we should settle on consistent and specific wordings for user translations. A possible template could look like this:

```yaml
Noun: English/French
Verb: English/French
Definition: ...
Not to be confused with: ...
Synonyms to avoid: ...
```

Example:

```yaml
Noun: invitation/invitation
Verb: to invite/inviter
Definition: getting a new user into a given organization
Not to be confused with:
 - the on-boarding process: this is specifically used for the part where both the host and the new user are connected simultaneously to exchange information in order to finalize the invitation
 - adding a device: invitation is specifically used for users, the term "adding" should be used instead for devices
Synonyms to avoid: enroll, enlist, on-board, greet, join
```

A list of concept that might go in the glossary:

- Organization/Organisation
- User/Utilisateur
- Device/Appareil
- Invitation/Invitation
- Joining/Rejoindre
- On-boarding/Accueil
- Workspace/Espace de travail
- Snapshots/Vue instantanée (timestamped workspaces)
- File/Fichier
- Folder/Dossier
- Administrator/Administrateur
- Standard user/Utilisateur standard
- Outsider/Externe
- Owner/Propriétaire
- Manager/Gestionnaire
- Contributor/Contributeur
- Reader/Lecteur

List of entries that already need a fix in the translations:

- ~enrôlement~ : procédure d’accueil
- ~greet~: onboard
- ~donner votre avis~: nous contacter
- ~directory~: folder
- ~voir l'état~: détails

Current version in the wiki <https://github.com/Scille/parsec-cloud/wiki/Glossary>
