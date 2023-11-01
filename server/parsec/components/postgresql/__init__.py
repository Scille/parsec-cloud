# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

# from parsec.components.postgresql.block import PGBlockComponent, PGBlockStoreComponent
# from parsec.components.postgresql.factory import components_factory
# from parsec.components.postgresql.handler import (
#     MigrationItem,
#     MigrationResult,
#     PGHandler,
#     apply_migrations,
#     retrieve_migrations,
# )
# from parsec.components.postgresql.message import PGMessageComponent
# from parsec.components.postgresql.organization import PGOrganizationComponent
# from parsec.components.postgresql.ping import PGPingComponent
# from parsec.components.postgresql.pki import PGPkiEnrollmentComponent
# from parsec.components.postgresql.realm import PGRealmComponent
# from parsec.components.postgresql.user import PGUserComponent
# from parsec.components.postgresql.vlob import PGVlobComponent

retrieve_migrations = object()
apply_migrations = object()
MigrationItem = object()
MigrationResult = object()
PGHandler = object()
PGOrganizationComponent = object()
PGPingComponent = object()
PGUserComponent = object()
PGMessageComponent = object()
PGRealmComponent = object()
PGVlobComponent = object()
PGBlockComponent = object()
PGBlockStoreComponent = object()
PGPkiEnrollmentComponent = object()
components_factory = object()

__all__ = [
    "retrieve_migrations",
    "apply_migrations",
    "MigrationItem",
    "MigrationResult",
    "PGHandler",
    "PGOrganizationComponent",
    "PGPingComponent",
    "PGUserComponent",
    "PGMessageComponent",
    "PGRealmComponent",
    "PGVlobComponent",
    "PGBlockComponent",
    "PGBlockStoreComponent",
    "PGPkiEnrollmentComponent",
    "components_factory",
]
