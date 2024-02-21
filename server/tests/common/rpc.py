# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

# /!\ Autogenerated by misc/gen_protocol_typings.py, any modification will be lost !

from __future__ import annotations

from parsec._parsec import (
    BlockID,
    BootstrapToken,
    DateTime,
    EnrollmentID,
    HashDigest,
    InvitationToken,
    PublicKey,
    SequesterServiceID,
    UserID,
    VerifyKey,
    VlobID,
    anonymous_cmds,
    authenticated_cmds,
    invited_cmds,
)


class BaseAnonymousRpcClient:
    async def _do_request(self, req: bytes) -> bytes:
        raise NotImplementedError

    async def ping(self, ping: str) -> anonymous_cmds.latest.ping.Rep:
        req = anonymous_cmds.latest.ping.Req(ping=ping)
        raw_rep = await self._do_request(req.dump())
        return anonymous_cmds.latest.ping.Rep.load(raw_rep)

    async def pki_enrollment_info(
        self, enrollment_id: EnrollmentID
    ) -> anonymous_cmds.latest.pki_enrollment_info.Rep:
        req = anonymous_cmds.latest.pki_enrollment_info.Req(enrollment_id=enrollment_id)
        raw_rep = await self._do_request(req.dump())
        return anonymous_cmds.latest.pki_enrollment_info.Rep.load(raw_rep)

    async def organization_bootstrap(
        self,
        bootstrap_token: BootstrapToken | None,
        root_verify_key: VerifyKey,
        user_certificate: bytes,
        device_certificate: bytes,
        redacted_user_certificate: bytes,
        redacted_device_certificate: bytes,
        sequester_authority_certificate: bytes | None,
    ) -> anonymous_cmds.latest.organization_bootstrap.Rep:
        req = anonymous_cmds.latest.organization_bootstrap.Req(
            bootstrap_token=bootstrap_token,
            root_verify_key=root_verify_key,
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
            sequester_authority_certificate=sequester_authority_certificate,
        )
        raw_rep = await self._do_request(req.dump())
        return anonymous_cmds.latest.organization_bootstrap.Rep.load(raw_rep)

    async def pki_enrollment_submit(
        self,
        enrollment_id: EnrollmentID,
        force: bool,
        submitter_der_x509_certificate: bytes,
        submitter_der_x509_certificate_email: str,
        submit_payload_signature: bytes,
        submit_payload: bytes,
    ) -> anonymous_cmds.latest.pki_enrollment_submit.Rep:
        req = anonymous_cmds.latest.pki_enrollment_submit.Req(
            enrollment_id=enrollment_id,
            force=force,
            submitter_der_x509_certificate=submitter_der_x509_certificate,
            submitter_der_x509_certificate_email=submitter_der_x509_certificate_email,
            submit_payload_signature=submit_payload_signature,
            submit_payload=submit_payload,
        )
        raw_rep = await self._do_request(req.dump())
        return anonymous_cmds.latest.pki_enrollment_submit.Rep.load(raw_rep)


class BaseAuthenticatedRpcClient:
    async def _do_request(self, req: bytes) -> bytes:
        raise NotImplementedError

    async def invite_1_greeter_wait_peer(
        self, token: InvitationToken, greeter_public_key: PublicKey
    ) -> authenticated_cmds.latest.invite_1_greeter_wait_peer.Rep:
        req = authenticated_cmds.latest.invite_1_greeter_wait_peer.Req(
            token=token, greeter_public_key=greeter_public_key
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.invite_1_greeter_wait_peer.Rep.load(raw_rep)

    async def vlob_poll_changes(
        self, realm_id: VlobID, last_checkpoint: int
    ) -> authenticated_cmds.latest.vlob_poll_changes.Rep:
        req = authenticated_cmds.latest.vlob_poll_changes.Req(
            realm_id=realm_id, last_checkpoint=last_checkpoint
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.vlob_poll_changes.Rep.load(raw_rep)

    async def pki_enrollment_list(
        self,
    ) -> authenticated_cmds.latest.pki_enrollment_list.Rep:
        req = authenticated_cmds.latest.pki_enrollment_list.Req()
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.pki_enrollment_list.Rep.load(raw_rep)

    async def invite_cancel(
        self, token: InvitationToken
    ) -> authenticated_cmds.latest.invite_cancel.Rep:
        req = authenticated_cmds.latest.invite_cancel.Req(token=token)
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.invite_cancel.Rep.load(raw_rep)

    async def realm_get_keys_bundle(
        self, realm_id: VlobID, key_index: int
    ) -> authenticated_cmds.latest.realm_get_keys_bundle.Rep:
        req = authenticated_cmds.latest.realm_get_keys_bundle.Req(
            realm_id=realm_id, key_index=key_index
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.realm_get_keys_bundle.Rep.load(raw_rep)

    async def user_revoke(
        self, revoked_user_certificate: bytes
    ) -> authenticated_cmds.latest.user_revoke.Rep:
        req = authenticated_cmds.latest.user_revoke.Req(
            revoked_user_certificate=revoked_user_certificate
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.user_revoke.Rep.load(raw_rep)

    async def invite_new_user(
        self, claimer_email: str, send_email: bool
    ) -> authenticated_cmds.latest.invite_new_user.Rep:
        req = authenticated_cmds.latest.invite_new_user.Req(
            claimer_email=claimer_email, send_email=send_email
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.invite_new_user.Rep.load(raw_rep)

    async def realm_create(
        self, realm_role_certificate: bytes
    ) -> authenticated_cmds.latest.realm_create.Rep:
        req = authenticated_cmds.latest.realm_create.Req(
            realm_role_certificate=realm_role_certificate
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.realm_create.Rep.load(raw_rep)

    async def vlob_update(
        self,
        vlob_id: VlobID,
        key_index: int,
        timestamp: DateTime,
        version: int,
        blob: bytes,
        sequester_blob: dict[SequesterServiceID, bytes] | None,
    ) -> authenticated_cmds.latest.vlob_update.Rep:
        req = authenticated_cmds.latest.vlob_update.Req(
            vlob_id=vlob_id,
            key_index=key_index,
            timestamp=timestamp,
            version=version,
            blob=blob,
            sequester_blob=sequester_blob,
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.vlob_update.Rep.load(raw_rep)

    async def ping(self, ping: str) -> authenticated_cmds.latest.ping.Rep:
        req = authenticated_cmds.latest.ping.Req(ping=ping)
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.ping.Rep.load(raw_rep)

    async def realm_rotate_key(
        self,
        realm_key_rotation_certificate: bytes,
        per_participant_keys_bundle_access: dict[UserID, bytes],
        keys_bundle: bytes,
        never_legacy_reencrypted_or_fail: bool,
    ) -> authenticated_cmds.latest.realm_rotate_key.Rep:
        req = authenticated_cmds.latest.realm_rotate_key.Req(
            realm_key_rotation_certificate=realm_key_rotation_certificate,
            per_participant_keys_bundle_access=per_participant_keys_bundle_access,
            keys_bundle=keys_bundle,
            never_legacy_reencrypted_or_fail=never_legacy_reencrypted_or_fail,
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.realm_rotate_key.Rep.load(raw_rep)

    async def user_create(
        self,
        user_certificate: bytes,
        device_certificate: bytes,
        redacted_user_certificate: bytes,
        redacted_device_certificate: bytes,
    ) -> authenticated_cmds.latest.user_create.Rep:
        req = authenticated_cmds.latest.user_create.Req(
            user_certificate=user_certificate,
            device_certificate=device_certificate,
            redacted_user_certificate=redacted_user_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.user_create.Rep.load(raw_rep)

    async def vlob_create(
        self,
        realm_id: VlobID,
        vlob_id: VlobID,
        key_index: int,
        timestamp: DateTime,
        blob: bytes,
        sequester_blob: dict[SequesterServiceID, bytes] | None,
    ) -> authenticated_cmds.latest.vlob_create.Rep:
        req = authenticated_cmds.latest.vlob_create.Req(
            realm_id=realm_id,
            vlob_id=vlob_id,
            key_index=key_index,
            timestamp=timestamp,
            blob=blob,
            sequester_blob=sequester_blob,
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.vlob_create.Rep.load(raw_rep)

    async def invite_3b_greeter_signify_trust(
        self, token: InvitationToken
    ) -> authenticated_cmds.latest.invite_3b_greeter_signify_trust.Rep:
        req = authenticated_cmds.latest.invite_3b_greeter_signify_trust.Req(token=token)
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.invite_3b_greeter_signify_trust.Rep.load(raw_rep)

    async def invite_2a_greeter_get_hashed_nonce(
        self, token: InvitationToken
    ) -> authenticated_cmds.latest.invite_2a_greeter_get_hashed_nonce.Rep:
        req = authenticated_cmds.latest.invite_2a_greeter_get_hashed_nonce.Req(token=token)
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.invite_2a_greeter_get_hashed_nonce.Rep.load(raw_rep)

    async def vlob_read_batch(
        self, realm_id: VlobID, vlobs: list[VlobID], at: DateTime | None
    ) -> authenticated_cmds.latest.vlob_read_batch.Rep:
        req = authenticated_cmds.latest.vlob_read_batch.Req(realm_id=realm_id, vlobs=vlobs, at=at)
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.vlob_read_batch.Rep.load(raw_rep)

    async def user_update(
        self, user_update_certificate: bytes
    ) -> authenticated_cmds.latest.user_update.Rep:
        req = authenticated_cmds.latest.user_update.Req(
            user_update_certificate=user_update_certificate
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.user_update.Rep.load(raw_rep)

    async def invite_4_greeter_communicate(
        self, token: InvitationToken, payload: bytes, last: bool
    ) -> authenticated_cmds.latest.invite_4_greeter_communicate.Rep:
        req = authenticated_cmds.latest.invite_4_greeter_communicate.Req(
            token=token, payload=payload, last=last
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.invite_4_greeter_communicate.Rep.load(raw_rep)

    async def certificate_get(
        self,
        common_after: DateTime | None,
        sequester_after: DateTime | None,
        shamir_recovery_after: DateTime | None,
        realm_after: dict[VlobID, DateTime],
    ) -> authenticated_cmds.latest.certificate_get.Rep:
        req = authenticated_cmds.latest.certificate_get.Req(
            common_after=common_after,
            sequester_after=sequester_after,
            shamir_recovery_after=shamir_recovery_after,
            realm_after=realm_after,
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.certificate_get.Rep.load(raw_rep)

    async def realm_rename(
        self, realm_name_certificate: bytes, initial_name_or_fail: bool
    ) -> authenticated_cmds.latest.realm_rename.Rep:
        req = authenticated_cmds.latest.realm_rename.Req(
            realm_name_certificate=realm_name_certificate, initial_name_or_fail=initial_name_or_fail
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.realm_rename.Rep.load(raw_rep)

    async def invite_new_device(
        self, send_email: bool
    ) -> authenticated_cmds.latest.invite_new_device.Rep:
        req = authenticated_cmds.latest.invite_new_device.Req(send_email=send_email)
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.invite_new_device.Rep.load(raw_rep)

    async def block_read(self, block_id: BlockID) -> authenticated_cmds.latest.block_read.Rep:
        req = authenticated_cmds.latest.block_read.Req(block_id=block_id)
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.block_read.Rep.load(raw_rep)

    async def invite_2b_greeter_send_nonce(
        self, token: InvitationToken, greeter_nonce: bytes
    ) -> authenticated_cmds.latest.invite_2b_greeter_send_nonce.Rep:
        req = authenticated_cmds.latest.invite_2b_greeter_send_nonce.Req(
            token=token, greeter_nonce=greeter_nonce
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.invite_2b_greeter_send_nonce.Rep.load(raw_rep)

    async def pki_enrollment_reject(
        self, enrollment_id: EnrollmentID
    ) -> authenticated_cmds.latest.pki_enrollment_reject.Rep:
        req = authenticated_cmds.latest.pki_enrollment_reject.Req(enrollment_id=enrollment_id)
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.pki_enrollment_reject.Rep.load(raw_rep)

    async def vlob_read_versions(
        self, realm_id: VlobID, items: list[tuple[VlobID, int]]
    ) -> authenticated_cmds.latest.vlob_read_versions.Rep:
        req = authenticated_cmds.latest.vlob_read_versions.Req(realm_id=realm_id, items=items)
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.vlob_read_versions.Rep.load(raw_rep)

    async def realm_unshare(
        self, realm_role_certificate: bytes
    ) -> authenticated_cmds.latest.realm_unshare.Rep:
        req = authenticated_cmds.latest.realm_unshare.Req(
            realm_role_certificate=realm_role_certificate
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.realm_unshare.Rep.load(raw_rep)

    async def device_create(
        self, device_certificate: bytes, redacted_device_certificate: bytes
    ) -> authenticated_cmds.latest.device_create.Rep:
        req = authenticated_cmds.latest.device_create.Req(
            device_certificate=device_certificate,
            redacted_device_certificate=redacted_device_certificate,
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.device_create.Rep.load(raw_rep)

    async def block_create(
        self, block_id: BlockID, realm_id: VlobID, block: bytes
    ) -> authenticated_cmds.latest.block_create.Rep:
        req = authenticated_cmds.latest.block_create.Req(
            block_id=block_id, realm_id=realm_id, block=block
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.block_create.Rep.load(raw_rep)

    async def realm_share(
        self, realm_role_certificate: bytes, recipient_keys_bundle_access: bytes, key_index: int
    ) -> authenticated_cmds.latest.realm_share.Rep:
        req = authenticated_cmds.latest.realm_share.Req(
            realm_role_certificate=realm_role_certificate,
            recipient_keys_bundle_access=recipient_keys_bundle_access,
            key_index=key_index,
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.realm_share.Rep.load(raw_rep)

    async def events_listen(
        self,
    ) -> authenticated_cmds.latest.events_listen.Rep:
        req = authenticated_cmds.latest.events_listen.Req()
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.events_listen.Rep.load(raw_rep)

    async def invite_list(
        self,
    ) -> authenticated_cmds.latest.invite_list.Rep:
        req = authenticated_cmds.latest.invite_list.Req()
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.invite_list.Rep.load(raw_rep)

    async def invite_3a_greeter_wait_peer_trust(
        self, token: InvitationToken
    ) -> authenticated_cmds.latest.invite_3a_greeter_wait_peer_trust.Rep:
        req = authenticated_cmds.latest.invite_3a_greeter_wait_peer_trust.Req(token=token)
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.invite_3a_greeter_wait_peer_trust.Rep.load(raw_rep)

    async def pki_enrollment_accept(
        self,
        accept_payload: bytes,
        accept_payload_signature: bytes,
        accepter_der_x509_certificate: bytes,
        enrollment_id: EnrollmentID,
        device_certificate: bytes,
        user_certificate: bytes,
        redacted_device_certificate: bytes,
        redacted_user_certificate: bytes,
    ) -> authenticated_cmds.latest.pki_enrollment_accept.Rep:
        req = authenticated_cmds.latest.pki_enrollment_accept.Req(
            accept_payload=accept_payload,
            accept_payload_signature=accept_payload_signature,
            accepter_der_x509_certificate=accepter_der_x509_certificate,
            enrollment_id=enrollment_id,
            device_certificate=device_certificate,
            user_certificate=user_certificate,
            redacted_device_certificate=redacted_device_certificate,
            redacted_user_certificate=redacted_user_certificate,
        )
        raw_rep = await self._do_request(req.dump())
        return authenticated_cmds.latest.pki_enrollment_accept.Rep.load(raw_rep)


class BaseInvitedRpcClient:
    async def _do_request(self, req: bytes) -> bytes:
        raise NotImplementedError

    async def invite_3b_claimer_wait_peer_trust(
        self,
    ) -> invited_cmds.latest.invite_3b_claimer_wait_peer_trust.Rep:
        req = invited_cmds.latest.invite_3b_claimer_wait_peer_trust.Req()
        raw_rep = await self._do_request(req.dump())
        return invited_cmds.latest.invite_3b_claimer_wait_peer_trust.Rep.load(raw_rep)

    async def invite_4_claimer_communicate(
        self, payload: bytes
    ) -> invited_cmds.latest.invite_4_claimer_communicate.Rep:
        req = invited_cmds.latest.invite_4_claimer_communicate.Req(payload=payload)
        raw_rep = await self._do_request(req.dump())
        return invited_cmds.latest.invite_4_claimer_communicate.Rep.load(raw_rep)

    async def invite_3a_claimer_signify_trust(
        self,
    ) -> invited_cmds.latest.invite_3a_claimer_signify_trust.Rep:
        req = invited_cmds.latest.invite_3a_claimer_signify_trust.Req()
        raw_rep = await self._do_request(req.dump())
        return invited_cmds.latest.invite_3a_claimer_signify_trust.Rep.load(raw_rep)

    async def ping(self, ping: str) -> invited_cmds.latest.ping.Rep:
        req = invited_cmds.latest.ping.Req(ping=ping)
        raw_rep = await self._do_request(req.dump())
        return invited_cmds.latest.ping.Rep.load(raw_rep)

    async def invite_info(
        self,
    ) -> invited_cmds.latest.invite_info.Rep:
        req = invited_cmds.latest.invite_info.Req()
        raw_rep = await self._do_request(req.dump())
        return invited_cmds.latest.invite_info.Rep.load(raw_rep)

    async def invite_2a_claimer_send_hashed_nonce(
        self, claimer_hashed_nonce: HashDigest
    ) -> invited_cmds.latest.invite_2a_claimer_send_hashed_nonce.Rep:
        req = invited_cmds.latest.invite_2a_claimer_send_hashed_nonce.Req(
            claimer_hashed_nonce=claimer_hashed_nonce
        )
        raw_rep = await self._do_request(req.dump())
        return invited_cmds.latest.invite_2a_claimer_send_hashed_nonce.Rep.load(raw_rep)

    async def invite_2b_claimer_send_nonce(
        self, claimer_nonce: bytes
    ) -> invited_cmds.latest.invite_2b_claimer_send_nonce.Rep:
        req = invited_cmds.latest.invite_2b_claimer_send_nonce.Req(claimer_nonce=claimer_nonce)
        raw_rep = await self._do_request(req.dump())
        return invited_cmds.latest.invite_2b_claimer_send_nonce.Rep.load(raw_rep)

    async def invite_1_claimer_wait_peer(
        self, claimer_public_key: PublicKey
    ) -> invited_cmds.latest.invite_1_claimer_wait_peer.Rep:
        req = invited_cmds.latest.invite_1_claimer_wait_peer.Req(
            claimer_public_key=claimer_public_key
        )
        raw_rep = await self._do_request(req.dump())
        return invited_cmds.latest.invite_1_claimer_wait_peer.Rep.load(raw_rep)
