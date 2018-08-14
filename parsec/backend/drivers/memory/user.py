from parsec.backend.user import BaseUserComponent
import pendulum

from parsec.backend.exceptions import (
    AlreadyExistsError,
    NotFoundError,
    OutOfDateError,
    UserClaimError,
)


class MemoryUserComponent(BaseUserComponent):
    def __init__(self, signal_ns):
        self._users = {}
        self._invitations = {}
        self._device_configuration_tries = {}
        self._unconfigured_devices = {}

    async def claim_invitation(
        self, invitation_token, user_id, broadcast_key, device_name, device_verify_key
    ):
        assert isinstance(broadcast_key, (bytes, bytearray))
        assert isinstance(device_verify_key, (bytes, bytearray))

        invitation = self._invitations.get(user_id)
        if not invitation:
            raise NotFoundError("No invitation for user `%s`" % user_id)

        try:
            if user_id in self._users:
                raise AlreadyExistsError("User `%s` has already been registered" % user_id)

            now = pendulum.utcnow()
            if (now - invitation["date"]) > pendulum.interval(hours=1):
                raise OutOfDateError("Claim code is too old.")

            if invitation["invitation_token"] != invitation_token:
                raise UserClaimError("Invalid invitation token")

        except UserClaimError:
            invitation["claim_tries"] += 1
            if invitation["claim_tries"] > 3:
                del self._invitations[user_id]
            raise

        await self.create(
            invitation["author"], user_id, broadcast_key, devices=[(device_name, device_verify_key)]
        )

    async def create_invitation(self, invitation_token, author, user_id):
        if user_id in self._users:
            raise AlreadyExistsError("User `%s` already exists" % user_id)

        # Overwrite previous invitation if any
        self._invitations[user_id] = {
            "date": pendulum.utcnow(),
            "author": author,
            "invitation_token": invitation_token,
            "claim_tries": 0,
        }

    async def create(self, author, user_id, broadcast_key, devices):
        assert isinstance(broadcast_key, (bytes, bytearray))

        if isinstance(devices, dict):
            devices = list(devices.items())

        for _, key in devices:
            assert isinstance(key, (bytes, bytearray))

        if user_id in self._users:
            raise AlreadyExistsError("User `%s` already exists" % user_id)

        now = pendulum.utcnow()
        self._users[user_id] = {
            "user_id": user_id,
            "created_on": now,
            "created_by": author,
            "broadcast_key": broadcast_key,
            "devices": {
                name: {"created_on": now, "verify_key": key, "revocated_on": None}
                for name, key in devices
            },
        }

    async def get(self, user_id):
        try:
            return self._users[user_id]

        except KeyError:
            raise NotFoundError(user_id)

    async def create_device(self, user_id, device_name, verify_key):
        if user_id not in self._users:
            raise NotFoundError("User `%s` doesn't exists" % user_id)

        user = self._users[user_id]
        if device_name in user["devices"]:
            raise AlreadyExistsError("Device `%s@%s` already exists" % (user_id, device_name))

        user["devices"][device_name] = {
            "created_on": pendulum.utcnow(),
            "verify_key": verify_key,
            "revocated_on": None,
            "configured": True,
        }

    async def configure_device(self, user_id, device_name, device_verify_key):
        key = (user_id, device_name)
        try:
            device = self._unconfigured_devices[key]
        except KeyError:
            raise NotFoundError(f"Device `{user_id}@{device_name}` doesn't exists")
        device["verify_key"] = device_verify_key

        try:
            user = self._users[user_id]
        except KeyError:
            raise NotFoundError("User `%s` doesn't exists" % user_id)

        user["devices"][device_name] = device
        del self._unconfigured_devices[key]

    async def declare_unconfigured_device(self, token, user_id, device_name):
        if user_id not in self._users:
            raise NotFoundError("User `%s` doesn't exists" % user_id)

        user = self._users[user_id]
        if device_name in user["devices"]:
            raise AlreadyExistsError("Device `%s@%s` already exists" % (user_id, device_name))

        key = (user_id, device_name)
        self._unconfigured_devices[key] = {
            "created_on": pendulum.utcnow(),
            "configure_token": token,
        }

    async def get_unconfigured_device(self, user_id, device_name):
        try:
            return self._unconfigured_devices[(user_id, device_name)]
        except KeyError:
            raise NotFoundError(f"Unconfigured device `{user_id}@{device_name}` doesn't exists")

    async def register_device_configuration_try(
        self, config_try_id, user_id, device_name, device_verify_key, exchange_cipherkey, salt
    ):
        self._device_configuration_tries[(user_id, config_try_id)] = {
            "status": "waiting_answer",
            "device_name": device_name,
            "device_verify_key": device_verify_key,
            "exchange_cipherkey": exchange_cipherkey,
            "salt": salt,
        }
        return config_try_id

    async def retrieve_device_configuration_try(self, config_try_id, user_id):
        config_try = self._device_configuration_tries.get((user_id, config_try_id))
        if not config_try:
            raise NotFoundError()

        return config_try

    async def accept_device_configuration_try(
        self, config_try_id, user_id, ciphered_user_privkey, ciphered_user_manifest_access
    ):
        config_try = self._device_configuration_tries.get((user_id, config_try_id))
        if not config_try:
            raise NotFoundError()

        if config_try["status"] != "waiting_answer":
            raise AlreadyExistsError("Device configuration try already done.")

        config_try["status"] = "accepted"
        config_try["ciphered_user_privkey"] = ciphered_user_privkey
        config_try["ciphered_user_manifest_access"] = ciphered_user_manifest_access

    async def refuse_device_configuration_try(self, config_try_id, user_id, reason):
        config_try = self._device_configuration_tries.get((user_id, config_try_id))
        if not config_try:
            raise NotFoundError()

        if config_try["status"] != "waiting_answer":
            raise AlreadyExistsError("Device configuration try already done.")

        config_try["status"] = "refused"
        config_try["refused_reason"] = reason
