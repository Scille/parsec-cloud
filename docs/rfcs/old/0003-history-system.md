# History system

From [ISSUE-129](https://github.com/Scille/parsec-cloud/issues/129)

## Need

Naviguer dans l'historique des données
Lire un fichier à une version donnée
Restaurer un fichier/répertoire
(bonus) comparer des versions entre elles

## Features

- bouton historique dans la gui
- une fois activé
  - fuse/gui passent en read-only
  - champ date dans la gui permettant de naviguer dans le temps
  - bouton restorer à cette version pour les fichiers/répertoires

## Implementation

- fuse support
  - fuse component can hot-swap FS class
  - HistoryFS wrap regular FS with statemachine for date, read only mode
- backend api
  - allow date based request
  - allow list vlob versions + pagination retrieval ?
