import attr
import random
import string
from typing import List, Iterable, Tuple
import pendulum

from parsec.types import UserID
from parsec.event_bus import EventBus
from parsec.crypto import VerifyKey
from parsec.trustchain import (
    unsecure_certified_device_extract_verify_key,
    unsecure_certified_user_extract_public_key,
)
from parsec.api.user import (
    user_get_req_schema,
    user_get_rep_schema,
    user_find_req_schema,
    user_find_rep_schema,
    user_invite_req_schema,
)
from parsec.backend.exceptions import NotFoundError, AlreadyExistsError


@attr.s(slots=True, frozen=True, repr=False)
class Device:
    def __repr__(self):
        return f"{self.__class__.__name__}({self.device_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    @property
    def device_name(self):
        return self.device_id.device_name

    @property
    def user_id(self):
        return self.device_id.user_id

    @property
    def verify_key(self):
        return unsecure_certified_device_extract_verify_key(self.certified_device)

    device_id = attr.ib()
    certified_device = attr.ib()
    device_certifier = attr.ib()

    created_on = attr.ib(factory=pendulum.now)
    revocated_on = attr.ib(default=None)
    certified_revocation = attr.ib(default=None)
    revocation_certifier = attr.ib(default=None)


class DevicesMapping:
    """
    Basically a frozen dict.
    """

    __slots__ = ("_read_only_mapping",)

    def __init__(self, *devices: Iterable[Device]):
        self._read_only_mapping = {d.device_name: d for d in devices}

    def __repr__(self):
        return f"{self.__class__.__name__}({self._read_only_mapping!r})"

    def __getitem__(self, key):
        return self._read_only_mapping[key]

    def items(self):
        return self._read_only_mapping.items()

    def keys(self):
        return self._read_only_mapping.keys()

    def values(self):
        return self._read_only_mapping.values()

    def __iter__(self):
        return self._read_only_mapping.__iter__()

    def __in__(self, key):
        return self._read_only_mapping.__in__(key)


@attr.s(slots=True, frozen=True, repr=False)
class User:
    def __repr__(self):
        return f"{self.__class__.__name__}({self.user_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    @property
    def public_key(self):
        return unsecure_certified_user_extract_public_key(self.certified_user)

    user_id = attr.ib()
    certified_user = attr.ib()
    user_certifier = attr.ib()
    devices = attr.ib(factory=DevicesMapping)

    created_on = attr.ib(factory=pendulum.now)
    revocated_on = attr.ib(default=None)
    certified_revocation = attr.ib(default=None)
    revocation_certifier = attr.ib(default=None)


def _generate_token():
    return "".join([random.choice(string.digits) for _ in range(6)])


class BaseUserComponent:
    def __init__(self, root_verify_key: VerifyKey, event_bus: EventBus):
        self.root_verify_key = root_verify_key
        self.event_bus = event_bus

    async def api_user_get(self, client_ctx, msg):
        msg = user_get_req_schema.load_or_abort(msg)
        try:
            user = await self.get(msg["user_id"])
        except NotFoundError as exc:
            return {"status": "not_found", "reason": str(exc)}

        data, errors = user_get_rep_schema.dump(user)
        if errors:
            raise RuntimeError(f"Dump error with {user!r}: {errors}")
        return data

    async def api_user_find(self, client_ctx, msg):
        msg = user_find_req_schema.load_or_abort(msg)
        results, total = await self.find(**msg)
        return user_find_rep_schema.dump(
            {
                "status": "ok",
                "results": results,
                "page": msg["page"],
                "per_page": msg["per_page"],
                "total": total,
            }
        ).data

    async def api_user_invite(self, client_ctx, msg):
        msg = user_invite_req_schema.load_or_abort(msg)
        try:
            await self.create_invitation(msg["user_id"])
        except AlreadyExistsError as exc:
            return {"status": "already_exists", "reason": str(exc)}

        return {"status": "ok"}

    async def api_user_claim(self, client_ctx, msg):
        raise NotImplementedError()

    async def api_device_declare(self, client_ctx, msg):
        raise NotImplementedError()

    async def api_device_configure(self, client_ctx, msg):
        raise NotImplementedError()

    async def api_device_get_configuration_try(self, client_ctx, msg):
        raise NotImplementedError()

    async def api_device_accept_configuration_try(self, client_ctx, msg):
        raise NotImplementedError()

    async def api_device_refuse_configuration_try(self, client_ctx, msg):
        raise NotImplementedError()

    async def create(self, user: User) -> None:
        raise NotImplementedError()

    async def create_device(self, device: Device) -> None:
        raise NotImplementedError()

    async def get(self, user_id: UserID) -> User:
        raise NotImplementedError()

    async def find(
        self, query: str = None, page: int = 1, per_page: int = 100
    ) -> Tuple[List[UserID], int]:
        raise NotImplementedError()

    async def create_invitation(self, user: UserID) -> None:
        raise NotImplementedError()

    async def claim_invitation(
        self,
        invitation_token: str,
        user_id: str,
        broadcast_key: bytes,
        device_name: str,
        device_verify_key: bytes,
    ):
        raise NotImplementedError()

    async def declare_unconfigured_device(
        self, token: str, author: str, user_id: str, device_name: str
    ):
        raise NotImplementedError()

    async def get_unconfigured_device(self, user_id: str, device_name: str):
        raise NotImplementedError()

    async def register_device_configuration_try(
        self,
        config_try_id: str,
        user_id: str,
        device_name: str,
        device_verify_key: bytes,
        exchange_cipherkey: bytes,
        salt: bytes,
    ):
        raise NotImplementedError()

    async def retrieve_device_configuration_try(self, config_try_id: str, user_id: str):
        raise NotImplementedError()

    async def accept_device_configuration_try(
        self,
        config_try_id: str,
        user_id: str,
        ciphered_user_privkey: bytes,
        ciphered_user_manifest_access: bytes,
    ):
        raise NotImplementedError()

    async def refuse_device_configuration_try(self, config_try_id: str, user_id: str, reason: str):
        raise NotImplementedError()

    async def revoke_user(self, user_id: str):
        raise NotImplementedError()

    async def revoke_device(self, user_id: str, device_name: str):
        raise NotImplementedError()
