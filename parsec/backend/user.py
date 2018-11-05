import trio
import random
import string

from parsec.schema import UnknownCheckedSchema, BaseCmdSchema, fields
from parsec.utils import to_jsonb64
from parsec.backend.exceptions import NotFoundError, AlreadyExistsError, UserClaimError


class _UserIDSchema(BaseCmdSchema):
    user_id = fields.String(required=True)


UserIDSchema = _UserIDSchema()


class _DeviceDeclareSchema(BaseCmdSchema):
    device_name = fields.String(required=True)


DeviceDeclareSchema = _DeviceDeclareSchema()


class _DeviceGetConfigurationTrySchema(BaseCmdSchema):
    config_try_id = fields.String(required=True)


DeviceGetConfigurationTrySchema = _DeviceGetConfigurationTrySchema()


class _DeviceAcceptConfigurationTrySchema(BaseCmdSchema):
    config_try_id = fields.String(required=True)
    ciphered_user_privkey = fields.Base64Bytes(required=True)
    ciphered_user_manifest_access = fields.Base64Bytes(required=True)


DeviceAcceptConfigurationTrySchema = _DeviceAcceptConfigurationTrySchema()


class _DeviceRefuseConfigurationTrySchema(BaseCmdSchema):
    config_try_id = fields.String(required=True)
    reason = fields.String(required=True)


DeviceRefuseConfigurationTrySchema = _DeviceRefuseConfigurationTrySchema()


class _DeviceConfigureSchema(BaseCmdSchema):
    configure_device_token = fields.String(required=True)
    user_id = fields.String(required=True)
    device_name = fields.String(required=True)
    device_verify_key = fields.Base64Bytes(required=True)
    # TODO: should be itself ciphered with a password-derived key
    # to mitigate man-in-the-middle attack
    exchange_cipherkey = fields.Base64Bytes(required=True)
    salt = fields.Base64Bytes(required=True)


DeviceConfigureSchema = _DeviceConfigureSchema()


class _DeviceSchema(UnknownCheckedSchema):
    created_on = fields.DateTime(required=True)
    revocated_on = fields.DateTime()
    verify_key = fields.Base64Bytes(required=True)


DeviceSchema = _DeviceSchema()


class _UserSchema(BaseCmdSchema):
    user_id = fields.String(required=True)
    created_on = fields.DateTime(required=True)
    broadcast_key = fields.Base64Bytes(required=True)
    devices = fields.Map(fields.String(), fields.Nested(DeviceSchema), required=True)


UserSchema = _UserSchema()


class _UserClaimSchema(BaseCmdSchema):
    invitation_token = fields.String(required=True)
    user_id = fields.String(required=True)
    broadcast_key = fields.Base64Bytes(required=True)
    device_name = fields.String(required=True)
    device_verify_key = fields.Base64Bytes(required=True)


UserClaimSchema = _UserClaimSchema()


def _generate_token():
    return "".join([random.choice(string.digits) for _ in range(6)])


class BaseUserComponent:
    def __init__(self, event_bus):
        self.event_bus = event_bus

    async def api_user_get(self, client_ctx, msg):
        msg = UserIDSchema.load_or_abort(msg)
        try:
            user = await self.get(msg["user_id"])
        except NotFoundError:
            return {"status": "not_found", "reason": f"No user with id `{msg['user_id']}`."}

        data, errors = UserSchema.dump(user)
        if errors:
            raise RuntimeError(f"Dump error with {user!r}: {errors}")

        return {"status": "ok", **data}

    async def api_user_invite(self, client_ctx, msg):
        msg = UserIDSchema.load_or_abort(msg)
        token = _generate_token()
        try:
            await self.create_invitation(token, msg["user_id"])
        except AlreadyExistsError:
            return {
                "status": "already_exists",
                "reason": f"User `{msg['user_id']}` already exists.",
            }

        return {"status": "ok", "user_id": msg["user_id"], "invitation_token": token}

    async def api_user_claim(self, client_ctx, msg):
        msg = UserClaimSchema.load_or_abort(msg)
        try:
            await self.claim_invitation(**msg)
        except UserClaimError as exc:
            return {"status": "claim_error", "reason": str(exc)}

        return {"status": "ok"}

    async def api_device_declare(self, client_ctx, msg):
        msg = DeviceDeclareSchema.load_or_abort(msg)
        configure_device_token = _generate_token()
        try:
            await self.declare_unconfigured_device(
                configure_device_token, client_ctx.user_id, msg["device_name"]
            )
        except AlreadyExistsError:
            return {
                "status": "already_exists",
                "reason": f"Device `{msg['device_name']}` already exists.",
            }

        return {"status": "ok", "configure_device_token": configure_device_token}

    async def api_device_configure(self, client_ctx, msg):
        msg = DeviceConfigureSchema.load_or_abort(msg)
        user_id = msg["user_id"]
        device_name = msg["device_name"]
        try:
            device = await self.get_unconfigured_device(user_id, device_name)
        except NotFoundError:
            return {
                "status": "not_found",
                "reason": f"No unconfigured device `{user_id}@{device_name}`.",
            }

        if device["configure_token"] != msg["configure_device_token"]:
            return {"status": "invalid_token", "reason": "Wrong device configuration token."}

        config_try_id = _generate_token()
        await self.register_device_configuration_try(
            config_try_id,
            user_id,
            msg["device_name"],
            msg["device_verify_key"],
            msg["exchange_cipherkey"],
            msg["salt"],
        )

        claim_answered = trio.Event()

        def _on_claim_answered(event, subject, config_try_id):
            if subject == user_id and config_try_id == config_try_id:
                claim_answered.set()

        self.event_bus.connect("device.try_claim_answered", _on_claim_answered, weak=True)

        self.event_bus.send(
            "device.try_claim_submitted",
            author=client_ctx.id,
            user_id=user_id,
            device_name=device_name,
            config_try_id=config_try_id,
        )
        with trio.move_on_after(5 * 60) as cancel_scope:
            await claim_answered.wait()
        if cancel_scope.cancelled_caught:
            return {
                "status": "timeout",
                "reason": (
                    "Timeout while waiting for existing device " "to validate our configuration."
                ),
            }

        # Should not raise NotFoundError given we created this just above
        config_try = await self.retrieve_device_configuration_try(config_try_id, user_id)

        if config_try["status"] != "accepted":
            return {"status": "configuration_refused", "reason": config_try["refused_reason"]}

        await self.configure_device(user_id, device_name, msg["device_verify_key"])

        return {
            "status": "ok",
            "ciphered_user_privkey": to_jsonb64(config_try["ciphered_user_privkey"]),
            "ciphered_user_manifest_access": to_jsonb64(
                config_try["ciphered_user_manifest_access"]
            ),
        }

    async def api_device_get_configuration_try(self, client_ctx, msg):
        msg = DeviceGetConfigurationTrySchema.load_or_abort(msg)
        try:
            config_try = await self.retrieve_device_configuration_try(
                msg["config_try_id"], client_ctx.user_id
            )
        except NotFoundError:
            return {"status": "not_found", "reason": "Unknown device configuration try."}

        return {
            "status": "ok",
            "device_name": config_try["device_name"],
            "configuration_status": config_try["status"],
            "device_verify_key": to_jsonb64(config_try["device_verify_key"]),
            "exchange_cipherkey": to_jsonb64(config_try["exchange_cipherkey"]),
            "salt": to_jsonb64(config_try["salt"]),
        }

    async def api_device_accept_configuration_try(self, client_ctx, msg):
        msg = DeviceAcceptConfigurationTrySchema.load_or_abort(msg)
        try:
            await self.accept_device_configuration_try(
                msg["config_try_id"],
                client_ctx.user_id,
                msg["ciphered_user_privkey"],
                msg["ciphered_user_manifest_access"],
            )
        except NotFoundError:
            return {"status": "not_found", "reason": "Unknown device configuration try."}

        except AlreadyExistsError:
            return {"status": "already_done", "reason": "Device configuration try already done."}

        self.event_bus.send(
            "device.try_claim_answered",
            subject=client_ctx.user_id,
            config_try_id=msg["config_try_id"],
        )
        return {"status": "ok"}

    async def api_device_refuse_configuration_try(self, client_ctx, msg):
        msg = DeviceRefuseConfigurationTrySchema.load_or_abort(msg)
        try:
            await self.refuse_device_configuration_try(
                msg["config_try_id"], client_ctx.user_id, msg["reason"]
            )
        except NotFoundError:
            return {"status": "not_found", "reason": "Unknown device configuration try."}

        except AlreadyExistsError:
            return {"status": "already_done", "reason": "Device configuration try already done."}

        self.event_bus.send(
            "device.try_claim_answered",
            subject=client_ctx.user_id,
            config_try_id=msg["config_try_id"],
        )
        return {"status": "ok"}

    async def create_invitation(self, invitation_token, author, user_id):
        raise NotImplementedError()

    async def claim_invitation(
        self, invitation_token, user_id, broadcast_key, device_name, device_verify_key
    ):
        raise NotImplementedError()

    async def declare_unconfigured_device(self, token, author, user_id, device_name):
        raise NotImplementedError()

    async def get_unconfigured_device(self, user_id, device_name):
        raise NotImplementedError()

    async def register_device_configuration_try(
        self, config_try_id, id, device_name, device_verify_key, exchange_cipherkey
    ):
        raise NotImplementedError()

    async def retrieve_device_configuration_try(self, config_try_id, user_id):
        raise NotImplementedError()

    async def accept_device_configuration_try(
        self, config_try_id, user_id, ciphered_user_privkey, ciphered_user_manifest_access
    ):
        raise NotImplementedError()

    async def refuse_device_configuration_try(self, config_try_id, user_id, reason):
        raise NotImplementedError()

    async def create(self, user_id, broadcast_key, devices):
        raise NotImplementedError()

    async def create_device(self, user_id, device_name, verify_key):
        raise NotImplementedError()

    async def get(self, id):
        raise NotImplementedError()
