.. Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

.. _doc_functional_architecture:

=========================================
WiP : Parsec client functional components
=========================================

Missing translations and figures

Mountpoint
**********
Le composant Mountpoint est responsable de l’interaction avec le système de fichiers pour
communiquer avec les applications tiers selon une logique très simple consistant à transmettre toutes les
requêtes natives au système de fichiers virtuel défini dans le composant Core.

GUI
***
Le composant GUI gérant l'interface utilisateur interagit avec le composant Core ou directement avec le
système de fichiers natif, mais pas avec le composant Mountpoint. Contrairement au composant Mountpoint,
la GUI souscrit aux événements exposés par le serveur de métadonnées, tels que les notifications en cas
d’actions concurrentes, de suppression d'un groupe etc. cela afin d’en informer l’utilisateur.

CORE
****
Outre les deux fonctions précédentes, le composant Core intègre toute la logique du client et contient
cinq sous-modules (Figure 3) dont les rôles sont les suivants :

● Le Système de Fichiers Virtuels (VFS) reçoit toutes les requêtes relatives au système de fichiers en
provenance du composant Mountpoint et, à cette fin, dispose d’une API qui simule une interface de
système de fichiers. Pour optimiser ses performances, ce composant ne cherche pas à pousser les
modifications jusqu’au serveur de métadonnées; il se contente de stocker de manière chiffrées les
modifications sur le disque dur de la machine locale.

● Le Synchroniseur est le composant qui transfère périodiquement les données modifiées stockées sur la
machine locale vers le serveur de métadonnées. Il s’occupe d’écouter les notifications du serveur de
métadonnées en cas de modification des données par un autre terminal ainsi que de résoudre les conflits
de version entre les données locales et celles du serveur de métadonnées.

● Le Gestionnaire d’Identités stocke dans la mémoire locale l’identité de l’utilisateur connecté (sous la
forme d’une session). La phrase de passe de l’utilisateur, qui n’est pas stockée, à laquelle on a rajouté un sel,
chiffre la clé privée du terminal (DEVICE_SIG_S_KEY), ainsi que celle de l’utilisateur
(USER_ENC_S_KEY) qui est partagée entre tous les terminaux de l’utilisateur. La DEVICE_SIG_S_KEY
sert à signer une modification, et la USER_ENC_S_KEY sert à déchiffrer les métadonnées personnelles
de l’utilisateur.

● La Messagerie a pour rôle d’écouter les messages techniques (notifications) en provenance du serveur
de métadonnées (le Core est lié via une socket TCP au serveur de métadonnées). Ces messages
peuvent soit demander une action du Core (par exemple en cas de partage de fichier), soit être à but
purement informatif et affichés sous la forme d’une notification.

● Le Partage gère les opérations de partage. S’il reçoit un message de partage de workspace, il déchiffre
ce message et ajoute les entrées correspondantes dans le user manifest de l’utilisateur. Si des messages
ont été envoyés alors qu’aucun des terminaux de l’utilisateur n’étaient connectés, ils sont gardés en file
d’attente sur le serveur de métadonnées et traités à la connexion de l’utilisateur.


Le serveur de métadonnées
*************************
Le serveur de métadonnées est dans un environnement distant et contient trois sous-modules :

● La Messagerie permet d’envoyer des notifications techniques aux utilisateurs

● Le Stockage des Données/Métadonnées. Les données des fichiers (les Blocks) sont stockées sur un
service de stockages objet (AWS S3 ou OpenStack Swift), les métadonnées (les Vlobs, pour Versioned
Blobs) sont stockées dans une base PostgreSQL

● Le module Notification des Terminaux - s’occupe d’envoyer des notifications aux terminaux connectés lors de
modifications des données ou bien de la réception d’un nouveau message

● Le Gestionnaire de Clés Publiques contient une correspondance entre l’identité des utilisateurs/terminaux
et leur clé publique (USER_ENC_P_KEY et DEVICE_SIG_P_KEY).
