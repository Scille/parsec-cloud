# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
"""Re-export of the in-memory editics component for the PostgreSQL backend.

Step 0 has no PostgreSQL persistence for editics sessions (todo step_0 §1.2,
§5): state is in-memory only. This module keeps the import path consistent with
the other PostgreSQL-backed components (`parsec.components.postgresql.<name>`)
so the factory wiring stays uniform.
"""

from __future__ import annotations

from parsec.components.editics.memory import MemoryEditicsComponent

__all__ = ["MemoryEditicsComponent"]
