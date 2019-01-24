import attr

from parsec.types import BackendOrganizationAddr, DeviceID
from parsec.crypto import PrivateKey, SigningKey
from parsec.serde import UnknownCheckedSchema, fields, post_load
from parsec.core.types.base import serializer_factory
from parsec.core.types.access import ManifestAccessSchema


@attr.s(slots=True, frozen=True, repr=False, auto_attribs=True)
class LocalDevice:

    organization_addr: BackendOrganizationAddr
    device_id: DeviceID
    signing_key: SigningKey
    private_key: PrivateKey
    user_manifest_access: dict  # TODO: Better typing
    local_symkey: bytes

    def __repr__(self):
        return f"{self.__class__.__name__}({self.device_id})"

    def evolve(self, **kwargs):
        return attr.evolve(self, **kwargs)

    # @property
    # def full_device_id(self):
    #     return FullDeviceID.build(self.organization_id, self.device_id)

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
    def user_name(self):
        return self.device_id.user_name

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
    user_manifest_access = fields.Nested(ManifestAccessSchema, required=True)
    local_symkey = fields.Bytes(required=True)

    @post_load
    def make_obj(self, data):
        return LocalDevice(**data)


local_device_serializer = serializer_factory(LocalDeviceSchema)
