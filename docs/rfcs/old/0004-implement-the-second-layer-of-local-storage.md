# Implement the second layer of local storage

From [ISSUE-268](https://github.com/Scille/parsec-cloud/issues/268)

Dans la [PR-261](https://github.com/Scille/parsec-cloud/pull/261) on a un cache mémoire au-dessus du local DB pour stocker les users et manifests, ainsi que les blocks. L'optimisation pour les blocks a été limitée au contenu et non aux metadatas (date d'accès), car il faut MAJ la date de dernier accès de manière fiable.

Cependant, on pourrait procéder ainsi pour éviter au maximum les requêtes de MAJ de date d'accès.

Clean blocks (provenant du remote et qu'on peut perdre) :

- Set blocks : pas écrits dans local DB mais uniquement en mémoire
- Get blocks (memory cache hit) : MAJ date dernier accès en mémoire, lecture depuis la mémoire
- Get blocks (memory cache miss) : déplacement du block en mémoire, date dernier accès dans local DB à NULL, lecture depuis la mémoire
- Avant purge du cache OU arrêt soft : MAJ date d'accès dans la DB locale depuis les blocks dans la mémoire
- Après arrêt hard : suppression des blocks avec date d'accès à NULL

Dirty blocks (pas encore sauvegardés) :

- Set nouveaux blocks : écrits dans local DB et aussi en mémoire
- Get blocks (memory cache hit) : MAJ date dernier accès dans local DB, lecture depuis la mémoire
- Get blocks (memory cache miss) : déplacement du block en mémoire, MAJ date dernier accès dans local DB, lecture depuis la mémoire
- Après arrêt hard : rien
- Avant arrêt soft : rien

Avec ces optimisations :

- On ne gagne rien sur les dirty blocks
- On gagne lors de clean blocks cache memory hit
- On perd clean blocks lus pendant la session si arrêt hard
- Reprise sur arrêt hard un poil plus lente
