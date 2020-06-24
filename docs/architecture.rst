.. _doc_architecture:

============
Architecture
============


Overview
========

Parsec is divided between a client (responsible for exposing data to the user and
providing an encryption layer) and a server (storing the encrypted data and notifying clients about
other users activity such as data modification or new sharing).



.. figure:: figures/architecture_diagram.svg
    :align: center
    :alt: Parsec single server, multi organizations showcase

    Parsec single server, multi organizations showcase

The Parsec server only requires a PostgreSQL DB for metadata (that is encrypted
using devices keys for the most part) and an Amazon S3 or OpenStack Swift object storage for data
blobs (that are all encrypted using Workspaces keys, that never left users' devices).
Redundancy using multiple cloud providers is possible.


Parsec security model
=====================

PARSEC secures sensitive data before they are stored on public clouds, proceeding in 3 steps :

- Splitting of files in blocks before encryption;
- Encryption of each block with a different symmetric key (BLOCK_ENC_KEY);
- Encryption of the metadata (tree structure, composition of files, multiple BLOCK_ENC_KEY, sharing information) with the private key of the user (USER_ENC_S_KEY).



Separation of the actors
************************

- User : represents a natural person in Parsec. An user have an asymmetric key (USER_ENC_S_KEY / USER_ENC_P_KEY) that enables him to encrypt data for him alone, like his User Manifest (see below).
- The Workstation : the physical support -- desktop or mobile computer.
- Device : it is through a Device that the user accesses Parsec.
  Each user potentially has multiple devices (e.g. one for his desktop and one for his laptop).
  Each terminal owns it's own asymmetric signature key (DEVICE_SIG_S_KEY /
  DEVICE_SIG_P_KEY) enabling him to sign the modification he has made.


Parsec data model
*****************

- File Manifest : contains the name of the file, the lost of block composing it and the associated BLOCK_ENC_KEY.
- Folder Manifest : index containing a set of entries, each entry being a File Manifest or another Folder Manifest.
- Workspace Manifest : index similar to the Folder Manifest, but that can be shared between multiple users.
- User Manifest : root index of each user containing the Workspaces Manifests shared with him.


Data sharing model
******************

- Workspace : a


un groupe d’utilisateurs partageant le même niveau de confiance. PARSEC effectue le
partage de données sensibles via le chiffrement de la clé de workspace (WS_ENC_KEY) par la clé du
destinataire du partage (USER_ENC_P_KEY) -- cette étape de chiffrement est répétée autant de fois
qu’il y a de destinataires.
● Organisation : un ensemble des workspaces et un ensemble d'utilisateurs membres de l'organisation.
L'accès à un workspace ne peut être accordé qu'aux membres de l'organisation. Deux organisations
distinctes ne peuvent pas accéder au même workspace.
