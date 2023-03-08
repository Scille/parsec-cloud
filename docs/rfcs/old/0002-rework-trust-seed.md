# Rework trust seed

From [ISSUE-121](https://github.com/Scille/parsec-cloud/issues/121)

## Need

Pouvoir partager un workspace en lecture seule.
En l'état les trust seed ne servent plus à rien.
Solution ==> rework de la resource beacon dans le backend.
Un beacon sert à lier un ensemble de données opaques:

- on peut s'abonner à un beacon pour être informé de modifications (déjà implémenté)
- on peut configurer qui a accès aux données liées à un beacon (à faire)

## Implementation

- supprimer entièrement les rts/wts
- ajouter la gestion des droits dans l'api beacon (ajout/suppression d'utilisateur pour un beacon donné)
- les api de création de vlob/block doivent contenir un champ beacon (qui sera stocké dans les informations du vlob/block et pas juste utilisé pour envoyer un signal comme actuellement)
- lors de la consultation/modification d'un vlob/block le backend vérifie les droits d'accès

==> À réfléchir: un user manifest aura son propre beacon qui ne contiendra donc qu'un seul utilisateur et un seul vlob. Cela permet à un attaquant de simplement retrouver ces points d'entrées au système et de tenter de les déchiffrer en priorité...
