# WiP

def _verify_certificates(root_verify_key, uv_devices, uv_users, known_devices, known_users):
    unsecure_users = {u[user_id]: u for u in uv_users.map(_unsecure_read_user)}
    unsecure_devices = {d[device_id]: d for d in uv_devices.map(_unsecure_read_device)}
    verified_users = {}
    verified_devices = OrderedDict()
    updatable_users_id = []
    updatable_devices_id = []

    def _recursive_verify_device(device_id, path):
        # Try to just get it in verified_devices. In that case, the chained users, devices, and
        # revocations have already been verified through this trustchain.
        try:
            return verified_devices[device_id]
        except KeyError:
            pass

        # If we can't find it in unsecure_devices, the trustchain is broken
        try:
            unsecure_device = unsecure_devices[device_id]
        except KeyError:
            raise RemoteDevicesManagerInvalidTrustchainError(
                f"{path}: Device not provided by backend"
            )

        # Define d_certif as the device certificate for easier access, define other variables
        d_certif = unsecure_device.device_certificate
        # This field being None means certified by root
        if unsecure_device.certified_by is None:
            certifier_verify_key = root_verify_key
            certifier_revoked_on = None
            sub_path = f"{path} <-create- <Root Key>"
        else:
            sub_path = f"{path} <-create- `{unsecure_device.certified_by}`"
            certifier = _recursive_verify_device(unsecure_device.certified_by, sub_path)
            certifier_verify_key = certifier.verify_key

        # Handles the user if it hasn't been found in the trustchain already
        if unsecure_device.user_id not in verified_users:
            unsecure_user = unsecure_users[unsecure_device.user_id]
            # Verify user revoke certif if any. Better do it before the actual user verification.
            if unsecure_user.revoked_by:
                sub_path = f"{path} <-revoke- `{unsecure_user.revoked_by}`"
                revoker = _recursive_verify_device(unsecure_user.revoked_by, sub_path)
                try:
                    RevokedUserCertificateContent.verify_and_load(
                        unsecure_user.revoked_user_certificate,
                        author_verify_key=revoker.verify_key,
                        expected_author=unsecure_user.revoked_by,
                    )
                except DataError as exc:
                    raise RemoteDevicesManagerInvalidTrustchainError(
                        f"{sub_path}: invalid certificate: {exc}"
                    ) from exc

                if revoker.revoked_on and unsecure_user.certified_on > revoker.revoked_on:
                    raise RemoteDevicesManagerInvalidTrustchainError(
                        f"{sub_path}: Signature ({unsecure_user.certified_on}) is "
                        f"posterior to user revocation {revoker.revoked_on})"
                    )

            # Now verify user : raises exception if values are incorrect, list id in updateables
            # only if we need to update or add this one
            if _verify_user(
                root_verify_key,
                unsecure_user,
                verified_devices,
                verified_users,
                known_devices,
                known_users,
            ):
                updatable_users_id += [unsecure_device.user_id]
            verified_users[unsecure_user.user_id] = unsecure_user

        certifier_revoked_on = verified_users[unsecure_device.user_id].revoked_on

        # Verify device certificate. Check revocation time.
        _verify_device_certificate(d_certif, certifier_verify_key, unsecure_device.certified_by)
        if certifier_revoked_on and unsecure_user.certified_on > certifier_revoked_on:
            raise RemoteDevicesManagerInvalidTrustchainError(
                f"{sub_path}: Signature ({unsecure_user.certified_on}) is "
                f"posterior to device revocation {certifier_revoked_on})"
            )

        verified_devices[unsecure_device.device_id] = unsecure_device
        updatable_devices_id += [unsecure_device.device_id]  # TODO : select only needed ones
        return unsecure_user

    for d_certif, _ in all_devices.values():
        # verify each device here
        _recursive_verify_device(d_certif.device_id, f"`{d_certif.device_id}`")

    return (
        {d_id: verified_devices[d_id] for d_id in updatable_devices_id},
        {u_id: verified_users[u_id] for u_id in updatable_users_id},
    )


def _unsecure_read_device(uv_device):
    """
    Convert to VerifiedRemoteDevice to easily access metadata
    (obviously this VerifiedRemoteDevice is not verified at all so far !)
    Raises:
        RemoteDevicesManagerInvalidTrustchainError
    """
    try:
        d_certif = DeviceCertificateContent.unsecure_load(uv_device.device_certificate)

    except DataError as exc:
        raise RemoteDevicesManagerInvalidTrustchainError(
            f"Invalid format for device certificate: {exc}"
        ) from exc

    params = {
        "fetched_on": uv_device.fetched_on,
        "device_id": d_certif.device_id,
        "verify_key": d_certif.verify_key,
        "device_certificate": uv_device.device_certificate,
        # TODO: rework naming
        "certified_by": d_certif.author,
        "certified_on": d_certif.timestamp,
    }

    d_certif = VerifiedRemoteDevice(**params)
    return d_certif


# Do we need it?.. Could be useful to have this fields when manipulating a
# VerifiedRemoteDevice through Parsec
def _get_revoked_device(device, revoked_by, revoked_on):
    return VerifiedRemoteDevice(
        fetched_on=device.fetched_on,
        device_id=device.device_id,
        verify_key=device.verify_key,
        device_certificate=device.device_certificate,
        certified_by=device.author,
        certified_on=device.timestamp,
        revoked_by=revoked_by,
        revoked_on=revoked_on,
    )


def _unsecure_read_user(uv_user):
    """
    Convert to VerifiedRemoteUser to easily access metadata
    (obviously those VerifiedRemoteUser is not verified at all so far !)
    Raises:
        RemoteDevicesManagerInvalidTrustchainError
    """
    try:
        u_certif = UserCertificateContent.unsecure_load(uv_user.user_certificate)

    except DataError as exc:
        raise RemoteDevicesManagerInvalidTrustchainError(
            f"Invalid format for user certificate: {exc}"
        ) from exc

    params = {
        "fetched_on": uv_user.fetched_on,
        "user_id": u_certif.user_id,
        "public_key": u_certif.public_key,
        "user_certificate": uv_user.device_certificate,
        # TODO: rework naming
        "certified_by": u_certif.author,
        "certified_on": u_certif.timestamp,
        "is_admin": u_certif.is_admin,
        "revoked_user_certificate": uv_user.revoked_user_certificate,
    }
    if uv_user.revoked_user_certificate:
        try:
            r_certif = RevokedUserCertificateContent.unsecure_load(
                uv_device.revoked_device_certificate
            )

        except DataError as exc:
            raise RemoteDevicesManagerInvalidTrustchainError(
                f"Invalid format for revoked device certificate: {exc}"
            ) from exc

        if r_certif.user_id != d_certif.user_id:
            raise RemoteDevicesManagerInvalidTrustchainError(
                f"Mismatch device_id in creation (`{d_certif.device_id}`)"
                f" and revocation (`{r_certif.device_id}`) certificates"
            )

        params["revoked_by"] = r_certif.author
        params["revoked_on"] = r_certif.timestamp

    u_certif = VerifiedRemoteUser(**params)
    return u_certif


def _verify_device_certificate(
    device_certificate: bytes, author_verify_key: VerifyKey, expected_author_id: DeviceID
):
    """
    Raises:
        RemoteDevicesManagerInvalidTrustchainError
    """
    try:
        DeviceCertificateContent.verify_and_load(
            uv_device.device_certificate, author_verify_key, expected_author_id
        )
    except CryptoError as exc:
        raise RemoteDevicesManagerInvalidTrustchainError(
            f"{sub_path}: invalid certificate: {exc}"
        ) from exc


def _verify_user(
    root_verify_key, unsecure_user, verified_devices, verified_users, known_devices, known_users
) -> Boolean:
    """
    Return a boolean indicating if the user must be updated from known ones
    Raises:
        RemoteDevicesManagerInvalidTrustchainError
    """
    if unsecure_user.author is None:
        # Certified by root
        certifier_verify_key = root_verify_key
        certifier_revoked_on = None
        sub_path = f"`{unsecure_user.user_id}` <-create- <Root Key>"

    else:
        sub_path = f"`{unsecure_user.user_id}` <-create- `{unsecure_user.author}`"
        try:
            certifier = verified_devices[unsecure_user.author]
            if not verified_users[certifier.user_id].is_admin:
                if certifier.user_id != unsecure_user.user_id:
                    raise Exception  ################

        except KeyError:
            raise RemoteDevicesManagerInvalidTrustchainError(
                f"{sub_path}: Device not provided by backend"
            )
        certifier_verify_key = certifier.verify_key
        certifier_revoked_on = certifier.revoked_on

    try:
        UserCertificateContent.verify_and_load(
            unsecure_user.user_certificate,
            author_verify_key=certifier_verify_key,
            expected_author=unsecure_user.author,
        )

    except DataError as exc:
        raise RemoteDevicesManagerInvalidTrustchainError(
            f"{sub_path}: invalid certificate: {exc}"
        ) from exc

    if certifier_revoked_on and unsecure_user.timestamp > certifier_revoked_on:
        raise RemoteDevicesManagerInvalidTrustchainError(
            f"{sub_path}: Signature ({unsecure_user.timestamp}) is posterior "
            f"to user revocation {certifier_revoked_on})"
        )

    if uv_device.user_id in known_users:
        _check_users_are_equal_without_revocation(unsecure_user, known_user[uv_device.user_id])
        if _users_revocation_status_are_equal(unsecure_user, known_user[uv_device.user_id]):
            return False
    return True


def _check_users_are_equal_without_revocation(user0, user1):
    for attribute_to_compare in [
        "user_id", "public_key", "user_certificate", "certified_by", "certified_on", "is_admin"
    ]:
        if user0[attribute_to_compare] != user1[attribute_to_compare]:
            raise RemoteDevicesManagerInvalidTrustchainError(
                f"User {user0.user_id} obtained from backend not corresponding with user already"
                f" in cache : field {attribute_to_compare} mismatch between "
                f"{user0['field']} from backend and {user1['field']} from cache"
            )


def _users_revocation_status_are_equal(new_user, old_user):
    for attribute_to_compare in ["revoked_user_certificate", "revoked_by", "revoked_on"]:
        if new_user[attribute_to_compare] != old_use[attribute_to_compare]:
            if new_user[attribute_to_compare] is None:
                raise RemoteDevicesManagerInvalidTrustchainError(
                    f"User {new_user.user_id} obtained from backend contains field "
                    f"{attribute_to_compare} set to None while user obtained from backend already "
                    f"has this field set to {old_user[attribute_to_compare]}"
                )
            return False
        return True
