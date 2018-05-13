import trio
import random
import string

from parsec.schema import UnknownCheckedSchema, BaseCmdSchema, fields
from parsec.utils import to_jsonb64
from parsec.backend.exceptions import NotFoundError, AlreadyExistsError, UserClaimError


class UserIDSchema(BaseCmdSchema):
    user_id = fields.String(required=True)


class DeviceDeclareSchema(BaseCmdSchema):
    device_name = fields.String(required=True)


class DeviceGetConfigurationTrySchema(BaseCmdSchema):
    configuration_try_id = fields.String(required=True)


class DeviceAcceptConfigurationTrySchema(BaseCmdSchema):
    configuration_try_id = fields.String(required=True)
    cyphered_user_privkey = fields.Base64Bytes(required=True)


class DeviceRefuseConfigurationTrySchema(BaseCmdSchema):
    configuration_try_id = fields.String(required=True)
    reason = fields.String(required=True)


class DeviceConfigureSchema(BaseCmdSchema):
    configure_device_token = fields.String(required=True)
    user_id = fields.String(required=True)
    device_name = fields.String(required=True)
    device_verify_key = fields.Base64Bytes(required=True)
    # TODO: should be itself cyphered with a password-derived key
    # to mitigate man-in-the-middle attack
    user_privkey_cypherkey = fields.Base64Bytes(required=True)


class DeviceSchema(UnknownCheckedSchema):
    created_on = fields.DateTime(required=True)
    revocated_on = fields.DateTime()
    verify_key = fields.Base64Bytes(required=True)


class UserSchema(BaseCmdSchema):
    user_id = fields.String(required=True)
    created_on = fields.DateTime()
    created_by = fields.String()
    broadcast_key = fields.Base64Bytes()
    devices = fields.Map(fields.String(), fields.Nested(DeviceSchema))


class UserClaimSchema(BaseCmdSchema):
    invitation_token = fields.String(required=True)
    user_id = fields.String(required=True)
    broadcast_key = fields.Base64Bytes(required=True)
    device_name = fields.String(required=True)
    device_verify_key = fields.Base64Bytes(required=True)


def _generate_token():
    return "".join([random.choice(string.digits) for _ in range(6)])


class BaseUserComponent:

    def __init__(self, signal_ns):
        self._signal_user_claimed = signal_ns.signal("user_claimed")
        self._signal_device_try_claim_submitted = signal_ns.signal("device_try_claim_submitted")
        self._signal_device_try_claim_answered = signal_ns.signal("device_try_claim_answered")

    async def api_user_get(self, client_ctx, msg):
        msg = UserIDSchema().load_or_abort(msg)
        try:
            user = await self.get(msg["user_id"])
        except NotFoundError:
            return {"status": "not_found", "reason": "No user with id `%s`." % msg["user_id"]}

        data, errors = UserSchema().dump(user)
        if errors:
            raise RuntimeError("Dump error with %r: %s" % (user, errors))

        return {"status": "ok", **data}

    async def api_user_invite(self, client_ctx, msg):
        msg = UserIDSchema().load_or_abort(msg)
        token = _generate_token()
        try:
            await self.create_invitation(token, client_ctx.id, msg["user_id"])
        except AlreadyExistsError:
            return {
                "status": "already_exists", "reason": "User `%s` already exists." % msg["user_id"]
            }

        return {"status": "ok", "user_id": msg["user_id"], "invitation_token": token}

    async def api_user_claim(self, client_ctx, msg):
        msg = UserClaimSchema().load_or_abort(msg)
        try:
            await self.claim_invitation(**msg)
        except UserClaimError as exc:
            return {"status": "claim_error", "reason": str(exc)}

        return {"status": "ok"}

    async def api_device_declare(self, client_ctx, msg):
        msg = DeviceDeclareSchema().load_or_abort(msg)
        configure_device_token = _generate_token()
        try:
            await self.declare_unconfigured_device(
                configure_device_token, client_ctx.user_id, msg["device_name"]
            )
        except AlreadyExistsError:
            return {
                "status": "already_exists",
                "reason": "Device `%s` already exists." % msg["device_name"],
            }

        return {"status": "ok", "configure_device_token": configure_device_token}

    async def api_device_configure(self, client_ctx, msg):
        msg = DeviceConfigureSchema().load_or_abort(msg)
        user_id = msg["user_id"]
        try:
            user = await self.get(user_id)
        except NotFoundError:
            return {"status": "not_found", "reason": "No user with id `%s`." % msg["user_id"]}

        device = user["devices"].get(msg["device_name"])
        if not device:
            return {
                "status": "not_found", "reason": "Device `%s` doesn't exists." % msg["device_name"]
            }

        if device["configure_token"] != msg["configure_device_token"]:
            return {"status": "invalid_token", "reason": "Wrong device configuration token."}

        config_try_id = _generate_token()
        await self.register_device_configuration_try(
            config_try_id,
            user_id,
            msg["device_name"],
            msg["device_verify_key"],
            msg["user_privkey_cypherkey"],
        )

        claim_answered = trio.Event()

        def _on_claim_answered(sender):
            claim_answered.set()

        with self._signal_device_try_claim_answered.temporarily_connected_to(
            _on_claim_answered, sender=config_try_id
        ):
            self._signal_device_try_claim_submitted.send(config_try_id)
            with trio.move_on_after(5 * 60) as cancel_scope:
                await claim_answered.wait()
            if cancel_scope.cancelled_caught:
                return {
                    "status": "timeout",
                    "reason": (
                        "Timeout while waiting for existing device "
                        "to validate our configuration."
                    ),
                }

        # Should not raise NotFoundError given we created this just above
        config_try = await self.retrieve_device_configuration_try(config_try_id, user_id)

        if config_try["status"] != "accepted":
            return {"status": "configuration_refused", "reason": config_try["refused_reason"]}

        await self.configure_device(user_id, msg["device_name"], msg["device_verify_key"])

        return {
            "status": "ok", "cyphered_user_privkey": to_jsonb64(config_try["cyphered_user_privkey"])
        }

    async def api_device_get_configuration_try(self, client_ctx, msg):
        msg = DeviceGetConfigurationTrySchema().load_or_abort(msg)
        try:
            config_try = await self.retrieve_device_configuration_try(
                msg["configuration_try_id"], client_ctx.user_id
            )
        except NotFoundError:
            return {"status": "not_found", "reason": "Unknown device configuration try."}

        return {
            "status": "ok",
            "device_name": config_try["device_name"],
            "configuration_status": config_try["status"],
            "device_verify_key": to_jsonb64(config_try["device_verify_key"]),
            "user_privkey_cypherkey": to_jsonb64(config_try["user_privkey_cypherkey"]),
        }

    async def api_device_accept_configuration_try(self, client_ctx, msg):
        msg = DeviceAcceptConfigurationTrySchema().load_or_abort(msg)
        try:
            await self.accept_device_configuration_try(
                msg["configuration_try_id"], client_ctx.user_id, msg["cyphered_user_privkey"]
            )
        except NotFoundError:
            return {"status": "not_found", "reason": "Unknown device configuration try."}

        except AlreadyExistsError:
            return {"status": "already_done", "reason": "Device configuration try already done."}

        self._signal_device_try_claim_answered.send(msg["configuration_try_id"])
        return {"status": "ok"}

    async def api_device_refuse_configuration_try(self, client_ctx, msg):
        msg = DeviceRefuseConfigurationTrySchema().load_or_abort(msg)
        try:
            await self.refuse_device_configuration_try(
                msg["configuration_try_id"], client_ctx.user_id, msg["reason"]
            )
        except NotFoundError:
            return {"status": "not_found", "reason": "Unknown device configuration try."}

        except AlreadyExistsError:
            return {"status": "already_done", "reason": "Device configuration try already done."}

        self._signal_device_try_claim_answered.send(msg["configuration_try_id"])
        return {"status": "ok"}

    async def claim_invitation(
        self, invitation_token, user_id, broadcast_key, device_name, device_verify_key
    ):
        raise NotImplementedError()

    async def create_invitation(self, invitation_token, author, user_id):
        raise NotImplementedError()

    async def declare_unconfigured_device(self, token, user_id, device_name):
        raise NotImplementedError()

    async def register_device_configuration_try(
        self, config_try_id, id, device_name, device_verify_key, user_privkey_cypherkey
    ):
        raise NotImplementedError()

    async def retrieve_device_configuration_try(self, config_try_id, user_id):
        raise NotImplementedError()

    async def accept_device_configuration_try(self, config_try_id, user_id, cyphered_user_privkey):
        raise NotImplementedError()

    async def refuse_device_configuration_try(self, config_try_id, user_id, reason):
        raise NotImplementedError()

    async def create(self, author, user_id, broadcast_key, devices):
        raise NotImplementedError()

    async def create_device(self, user_id, device_name, verify_key):
        raise NotImplementedError()

    async def get(self, id):
        raise NotImplementedError()
