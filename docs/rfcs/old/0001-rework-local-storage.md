# Rework local storage

From [ISSUE-120](https://github.com/Scille/parsec-cloud/issues/120)

## Need

Stocker les données locales: vlob, block, dirty vlob, dirty block, remote user/device info
Conserver les dirty vlob/block sans limite de temps ni de taille
Une fois synchronisés, supprimer les dirty vlob/block
(bonus) migrer les dirty blocks en block pour éviter des réécritures
Nettoyer les vlobs/blocks non utilisés quand le cache est trop important
(bonus) permettre de spécifier qu'un vlob/block ne doit pas être supprimer pour permettre le mode offline
Les données user/device doivent avoir une durée de vie limitée (~1h) vu que celle-ci peuvent être révoqué à tout instant.

## Implementation

- utilisation de SQlite
- stockage des vlobs directement dans sqlite
- stockage des block sous forme de fichiers (==> vérifier les performances auparavant)
- la table blocks contient la taille de chaque élément + flags (dirty, offline) + date dernier accès + chemin fichier
- on ne prend pas en compte les vlobs ni la base sqlite dans la taille du cache
- vu que les blocks font tous ~ la même taille, on calcule le nb de blocks maximum (via la taille de cache max autorisée)
- garbage collection lazy: si plus de place on remplace le plus vieux blocks
