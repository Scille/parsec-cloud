# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS


from parsec.backend.postgresql.handler import PGHandler
from parsec.backend.sequester import BaseSequesterComponent


class PGPSequesterComponent(BaseSequesterComponent):
    def __init__(self, dbh: PGHandler):
        self.dbh = dbh
