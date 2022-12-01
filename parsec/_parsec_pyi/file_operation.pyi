from __future__ import annotations
from typing import List, Set, Tuple

from parsec._parsec_pyi.local_manifest import LocalFileManifest, Chunk
from parsec._parsec_pyi.ids import ChunkID
from parsec._parsec_pyi.time import DateTime

def prepare_read(manifest: LocalFileManifest, size: int, offset: int) -> Tuple[Chunk, ...]: ...
def prepare_write(
    manifest: LocalFileManifest, size: int, offset: int, timestamp: DateTime
) -> Tuple[LocalFileManifest, List[Tuple[Chunk, int]], Set[ChunkID]]: ...
def prepare_resize(
    manifest: LocalFileManifest, size: int, timestamp: DateTime
) -> Tuple[LocalFileManifest, List[Tuple[Chunk, int]], Set[ChunkID]]: ...
def prepare_reshape(
    manifest: LocalFileManifest,
) -> List[Tuple[int, Tuple[Chunk, ...], Chunk, bool, Set[ChunkID]]]: ...
