import attr
import json
import trio
from uuid import uuid4
import pendulum
from nacl.public import PrivateKey
from nacl.secret import SecretBox
import nacl.utils

from parsec.utils import from_jsonb64, to_jsonb64


def _generate_sym_key():
    return nacl.utils.random(SecretBox.KEY_SIZE)


def _try_merge_two_patches(p1, p2):
    if (
        (p1.offset < p2.offset and p1.offset + p1.size < p2.offset)
        or (p2.offset < p1.offset and p2.offset + p2.size < p1.offset)
    ):
        return None

    p1buffer = p1.get_buffer()
    p2buffer = p2.get_buffer()
    # Remember p2 has priority over p1
    if p1.offset < p2.offset:
        newbuffer = p1buffer[:p2.offset - p1.offset] + p2buffer + p1buffer[
            p2.offset + p2.size - p1.offset:
        ]
        newsize = len(newbuffer)
        newoffset = p1.offset
    else:
        newbuffer = p2buffer + p1buffer[p2.offset + p2.size - p1.offset:]
        newsize = len(newbuffer)
        newoffset = p2.offset
    return Patch(p1.local_storage, newoffset, newsize, buffer=newbuffer)


def _merge_patches(patches):
    merged = []
    for p2 in patches:
        new_merged = []
        for p1 in merged:
            res = _try_merge_two_patches(p1, p2)
            if res:
                p2 = res
            else:
                new_merged.append(p1)
        new_merged.append(p2)
        merged = new_merged
    return sorted(merged, key=lambda x: x.offset)


@attr.s(slots=True)
class Patch:
    local_storage = attr.ib()
    offset = attr.ib()
    size = attr.ib()
    dirty_block_id = attr.ib(default=None)
    dirty_block_key = attr.ib(default=None)
    _buffer = attr.ib(default=None)

    @property
    def end(self):
        return self.offset + self.size

    def get_buffer(self):
        if self._buffer is None:
            if not self.dirty_block_id:
                raise RuntimeError("This patch has no buffer...")

            ciphered = self.local_storage.fetch_dirty_block(self.dirty_block_id)
            self._buffer = SecretBox(self.dirty_block_key).decrypt(ciphered)
        return self._buffer

    def save_as_dirty_block(self):
        if self.dirty_block_id:
            raise RuntimeError(
                "Cannot modify already existing `%s` dirty block" % self.dirty_block_id
            )

        self.dirty_block_id = uuid4().hex
        self.dirty_block_key = _generate_sym_key()
        ciphered = SecretBox(self.dirty_block_key).encrypt(self._buffer)
        self.local_storage.flush_dirty_block(self.dirty_block_id, ciphered)

    @classmethod
    def build_from_dirty_block(cls, local_storage, id, offset, size, key):
        return cls(local_storage, offset, size, id, key)
