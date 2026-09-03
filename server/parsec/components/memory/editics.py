# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
"""Re-export of the in-memory editics component.

The actual implementation lives in `parsec.components.editics.memory`; this
module mirrors the convention used by the other components (which expose
their in-memory implementation as `parsec.components.memory.<name>`).
"""

from __future__ import annotations

from parsec.components.editics.memory import MemoryEditicsComponent

__all__ = ["MemoryEditicsComponent"]
