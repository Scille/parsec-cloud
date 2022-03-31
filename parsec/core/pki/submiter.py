# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import attr
from typing import Iterable, Optional, List, Tuple
from uuid import UUID, uuid4
from pathlib import Path
from pendulum import DateTime

from parsec.api.data import PkiEnrollmentSubmitPayload
from parsec.api.protocol import HumanHandle, DeviceLabel, PkiEnrollmentStatus
from parsec.core.backend_connection import (
    pki_enrollment_submit as cmd_pki_enrollment_submit,
    pki_enrollment_info as cmd_pki_enrollment_info,
)
from parsec.core.types import BackendPkiEnrollmentAddr, BackendOrganizationAddr
from parsec.core.pki.plumbing import (
    X509Certificate,
    pki_enrollment_select_certificate,
    pki_enrollment_sign_payload,
    pki_enrollment_save_local_pending,
    pki_enrollment_remove_local_pending,
    pki_enrollment_list_local_pendings,
    pki_enrollment_load_accept_payload,
)
from parsec.core.types import LocalDevice
from parsec.core.local_device import LocalDeviceError, generate_new_device
from parsec.crypto import PrivateKey, SigningKey


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollementSubmiterInitalCtx:
    addr: BackendPkiEnrollmentAddr
    enrollment_id: UUID
    signing_key: SigningKey
    private_key: PrivateKey
    x509_certificate: X509Certificate

    @classmethod
    def new(cls, addr: BackendPkiEnrollmentAddr) -> "PkiEnrollementSubmiterInitalCtx":
        enrollment_id = uuid4()
        signing_key = SigningKey.generate()
        private_key = PrivateKey.generate()

        x509_certificate = pki_enrollment_select_certificate()

        return cls(
            addr=addr,
            enrollment_id=enrollment_id,
            signing_key=signing_key,
            private_key=private_key,
            x509_certificate=x509_certificate,
        )

    async def submit(
        self,
        config_dir: Path,
        requested_device_label: DeviceLabel,
        requested_human_handle: HumanHandle,
        force: bool,
    ) -> "PkiEnrollmentSubmiterSubmittedCtx":
        # TODO: document exceptions !

        # Build submit payload
        cooked_submit_payload = PkiEnrollmentSubmitPayload(
            verify_key=self.signing_key.verify_key,
            public_key=self.private_key.public_key,
            requested_human_handle=requested_human_handle,
            requested_device_label=requested_device_label,
        )
        raw_submit_payload = cooked_submit_payload.dump()
        payload_signature = pki_enrollment_sign_payload(
            payload=raw_submit_payload, x509_certificate=self.x509_certificate
        )

        try:
            rep = await cmd_pki_enrollment_submit(
                addr=self.addr,
                enrollment_id=self.enrollment_id,
                force=force,
                submitter_der_x509_certificate=self.x509_certificate.der_x509_certificate,
                submit_payload_signature=payload_signature,
                submit_payload=raw_submit_payload,
            )
        except Exception:
            # TODO: exception handling !
            raise

        if rep["status"] != "ok":
            # TODO: error handling !
            raise RuntimeError(f"Backend refused to create enrollment: {rep}")

        # Save the enrollment request on disk.
        # Note there is not atomicity with the request to the backend, but it's
        # considered fine:
        # - if the pending enrollment is not saved, CLI will display an error message (unless
        #   the whole machine has crashed ^^) so user is expected to retry the submit command
        # - in case the enrollment is accepted by a ninja-fast admin before the submit can be
        #   retried, it's no big deal to revoke the newly enrolled user and restart from scratch
        pki_enrollment_save_local_pending(
            config_dir=config_dir,
            x509_certificate=self.x509_certificate,
            addr=self.addr,
            enrollment_id=self.enrollment_id,
            submitted_on=rep["submitted_on"],
            submit_payload=cooked_submit_payload,
            signing_key=self.signing_key,
            private_key=self.private_key,
        )

        return PkiEnrollmentSubmiterSubmittedCtx(
            config_dir=config_dir,
            x509_certificate=self.x509_certificate,
            addr=self.addr,
            submitted_on=rep["submitted_on"],
            enrollment_id=self.enrollment_id,
            submit_payload=cooked_submit_payload,
            signing_key=self.signing_key,
            private_key=self.private_key,
        )


@attr.s(slots=True, frozen=True, auto_attribs=True)
class PkiEnrollmentSubmiterSubmittedCtx:
    config_dir: Path
    x509_certificate: X509Certificate
    addr: BackendPkiEnrollmentAddr
    submitted_on: DateTime
    enrollment_id: UUID
    submit_payload: PkiEnrollmentSubmitPayload
    signing_key: SigningKey
    private_key: PrivateKey

    @classmethod
    def list_from_disk(cls, config_dir: Path) -> List["PkiEnrollmentSubmiterSubmittedCtx"]:
        # TODO: document exceptions !
        ctxs = []
        for pending in pki_enrollment_list_local_pendings(config_dir=config_dir):
            ctx = PkiEnrollmentSubmiterSubmittedCtx(
                config_dir=config_dir,
                x509_certificate=pending.x509_certificate,
                addr=pending.addr,
                submitted_on=pending.submitted_on,
                enrollment_id=pending.enrollment_id,
                submit_payload=pending.submit_payload,
                signing_key=pending.signing_key,
                private_key=pending.private_key,
            )
            ctxs.append(ctx)

        return ctxs

    async def poll(
        self, clean_disk: bool, extra_trust_roots: Iterable[Path] = ()
    ) -> Tuple[PkiEnrollmentStatus, DateTime, Optional[LocalDevice]]:
        try:
            rep = await cmd_pki_enrollment_info(addr=self.addr, enrollment_id=self.enrollment_id)
        except Exception:
            # TODO: exception handling !
            raise
        if rep["status"] != "ok":
            # TODO: exception handling !
            raise RuntimeError()

        enrollment_status = rep["type"]
        maybe_new_device = None
        need_remove_local_pending = False

        try:
            if enrollment_status == PkiEnrollmentStatus.SUBMITTED:
                occured_on = self.submitted_on

            elif enrollment_status == PkiEnrollmentStatus.CANCELLED:
                need_remove_local_pending = True
                occured_on = rep["cancelled_on"]

            elif enrollment_status == PkiEnrollmentStatus.REJECTED:
                need_remove_local_pending = True
                occured_on = rep["rejected_on"]

            else:
                assert enrollment_status == PkiEnrollmentStatus.ACCEPTED
                # In case an exception occurs while checking the accepted payload,
                # we still want to remove the local data.
                # This is because in such case it is most likely we cannot do
                # anything with the accepted payload and it's cheap for admin
                # to revoked the user and restart the enrollment from scratch.
                need_remove_local_pending = True

                occured_on = rep["accepted_on"]
                try:
                    (accepter_x509_certif, accept_payload) = pki_enrollment_load_accept_payload(
                        extra_trust_roots=extra_trust_roots,
                        der_x509_certificate=rep["accepter_der_x509_certificate"],
                        payload_signature=rep["accept_payload_signature"],
                        payload=rep["accept_payload"],
                    )

                # Verification failed
                except LocalDeviceError:
                    raise

                # Create the local device
                organization_addr = BackendOrganizationAddr.build(
                    backend_addr=self.addr,
                    organization_id=self.addr.organization_id,
                    root_verify_key=accept_payload.root_verify_key,
                )
                maybe_new_device = generate_new_device(
                    organization_addr=organization_addr,
                    device_id=accept_payload.device_id,
                    profile=accept_payload.profile,
                    human_handle=accept_payload.human_handle,
                    device_label=accept_payload.device_label,
                    signing_key=self.signing_key,
                    private_key=self.private_key,
                )

        finally:
            if clean_disk and need_remove_local_pending:
                # TODO: handle exception in case the file is not found ?
                pki_enrollment_remove_local_pending(
                    config_dir=self.config_dir, enrollment_id=self.enrollment_id
                )

        return (enrollment_status, occured_on, maybe_new_device)
