# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from typing import Tuple
from hashlib import sha256

from parsec.types import BackendOrganizationAddr, OrganizationID, DeviceID
from parsec.crypto import SecretKey, PrivateKey, SigningKey
from parsec.serde import UnknownCheckedSchema, fields, post_load
from parsec.core.types.base import EntryID, EntryIDField, serializer_factory


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class LocalDevice:

    organization_addr: BackendOrganizationAddr
    device_id: DeviceID
    signing_key: SigningKey
    private_key: PrivateKey
    user_manifest_id: EntryID
    user_manifest_key: SecretKey
    local_symkey: bytes

    def __repr__(self):
        return f"{self.__class__.__name__}({self.device_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    @property
    def slug(self):
        # Add a hash to avoid clash when the backend is reseted
        # and we recreate a device with same organization/device_id
        # organization and device_id than a previous one
        hash_part = sha256(self.root_verify_key.encode()).hexdigest()[:10]
        return f"{hash_part}#{self.organization_id}#{self.device_id}"

    @staticmethod
    def load_slug(slug: str) -> Tuple[OrganizationID, DeviceID]:
        """
        Raises: ValueError
        """
        _, raw_org_id, raw_device_id = slug.split("#")
        return OrganizationID(raw_org_id), DeviceID(raw_device_id)

    @property
    def root_verify_key(self):
        return self.organization_addr.root_verify_key

    @property
    def organization_id(self):
        return self.organization_addr.organization_id

    @property
    def device_name(self):
        return self.device_id.device_name

    @property
    def user_id(self):
        return self.device_id.user_id

    @property
    def verify_key(self):
        return self.signing_key.verify_key

    @property
    def public_key(self):
        return self.private_key.public_key


class LocalDeviceSchema(UnknownCheckedSchema):
    organization_addr = fields.BackendOrganizationAddr(required=True)
    device_id = fields.DeviceID(required=True)
    signing_key = fields.SigningKey(required=True)
    private_key = fields.PrivateKey(required=True)
    user_manifest_id = EntryIDField(required=True)
    user_manifest_key = fields.SecretKey(required=True)
    local_symkey = fields.Bytes(required=True)

    @post_load
    def make_obj(self, data):
        return LocalDevice(**data)


local_device_serializer = serializer_factory(LocalDeviceSchema)
