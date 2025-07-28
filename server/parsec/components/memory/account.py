# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Literal, override

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    EmailAddress,
    HashDigest,
    HumanHandle,
    InvitationToken,
    InvitationType,
    OrganizationID,
    SecretKey,
    UserID,
    ValidationCode,
)
from parsec.components.account import (
    AccountAuthMethodCreateBadOutcome,
    AccountAuthMethodDisableBadOutcome,
    AccountAuthMethodListBadOutcome,
    AccountCreateProceedBadOutcome,
    AccountCreateSendValidationEmailBadOutcome,
    AccountDeleteProceedBadOutcome,
    AccountDeleteSendValidationEmailBadOutcome,
    AccountInfo,
    AccountInfoBadOutcome,
    AccountInviteListBadOutcome,
    AccountOrganizationListBadOutcome,
    AccountOrganizationSelfList,
    AccountOrganizationSelfListActiveUser,
    AccountOrganizationSelfListRevokedUser,
    AccountRecoverProceedBadOutcome,
    AccountRecoverSendValidationEmailBadOutcome,
    AccountVaultItemListBadOutcome,
    AccountVaultItemRecoveryList,
    AccountVaultItemUploadBadOutcome,
    AccountVaultKeyRotation,
    AuthMethod,
    BaseAccountComponent,
    UntrustedPasswordAlgorithm,
    ValidationCodeInfo,
    VaultItemRecoveryList,
    VaultItemRecoveryVault,
    VaultItems,
)
from parsec.components.email import SendEmailBadOutcome
from parsec.components.events import EventBus
from parsec.components.memory.datamodel import (
    MemoryAccount,
    MemoryAccountVault,
    MemoryAuthenticationMethod,
    MemoryDatamodel,
)
from parsec.config import BackendConfig


class MemoryAccountComponent(BaseAccountComponent):
    def __init__(
        self,
        data: MemoryDatamodel,
        config: BackendConfig,
        event_bus: EventBus,
    ) -> None:
        super().__init__(config)
        self._data = data
        self._event_bus = event_bus

    def _auth_method_id_already_exists(self, auth_method_id: AccountAuthMethodID) -> bool:
        return self._data.get_account_from_any_auth_method(auth_method_id) is not None

    @override
    async def get_password_algorithm_or_fake_it(
        self, email: EmailAddress
    ) -> UntrustedPasswordAlgorithm:
        try:
            account = self._data.accounts[email]
            active_authentication_methods = account.current_vault.active_authentication_methods
        except KeyError:
            active_authentication_methods = ()

        for auth_method in active_authentication_methods:
            if auth_method.password_algorithm is not None:
                return auth_method.password_algorithm

        # Account does not exists or it does not have an active password algorithm.
        # In either case, a fake configuration is returned.
        return self._generate_fake_password_algorithm(email)

    @override
    async def account_info(
        self,
        auth_method_id: AccountAuthMethodID,
    ) -> AccountInfo | AccountInfoBadOutcome:
        match self._data.get_account_from_active_auth_method(auth_method_id=auth_method_id):
            case (account, _):
                pass
            case None:
                return AccountInfoBadOutcome.ACCOUNT_NOT_FOUND

        return AccountInfo(
            human_handle=account.human_handle,
        )

    @override
    async def create_send_validation_email(
        self,
        now: DateTime,
        email: EmailAddress,
    ) -> ValidationCode | SendEmailBadOutcome | AccountCreateSendValidationEmailBadOutcome:
        async with self._data.account_creation_lock:
            if email in self._data.accounts:
                assert email not in self._data.account_create_validation_emails  # Sanity check
                return AccountCreateSendValidationEmailBadOutcome.ACCOUNT_ALREADY_EXISTS

            # Note we simply overwrite any existing previous validation code
            validation_code = ValidationCode.generate()
            self._data.account_create_validation_emails[email] = ValidationCodeInfo(
                validation_code, now
            )

        # Note we send the email after the lock has been released, this is to
        # simulation what is done in PostgreSQL where the transaction has to be
        # finished before sending the email since this operation can be long.
        outcome = await self._send_account_create_validation_email(
            email=email,
            validation_code=validation_code,
        )
        match outcome:
            case None:
                return validation_code
            case error:
                return error

    @override
    async def create_check_validation_code(
        self,
        now: DateTime,
        email: EmailAddress,
        validation_code: ValidationCode,
    ) -> None | AccountCreateProceedBadOutcome:
        async with self._data.account_creation_lock:
            return self._create_account_check_code(now, email, validation_code)

    def _create_account_check_code(
        self,
        now: DateTime,
        email: EmailAddress,
        validation_code: ValidationCode,
    ) -> None | AccountCreateProceedBadOutcome:
        try:
            last_mail_info = self._data.account_create_validation_emails[email]
        except KeyError:
            return AccountCreateProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED

        if not last_mail_info.can_be_used(now):
            return AccountCreateProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED

        if last_mail_info.validation_code != validation_code:
            # Register the failed attempt
            last_mail_info.failed_attempts += 1
            return AccountCreateProceedBadOutcome.INVALID_VALIDATION_CODE

        # Sanity check
        assert email not in self._data.accounts

        return None

    @override
    async def create_proceed(
        self,
        now: DateTime,
        validation_code: ValidationCode,
        vault_key_access: bytes,
        human_handle: HumanHandle,
        created_by_user_agent: str,
        created_by_ip: str | Literal[""],
        auth_method_id: AccountAuthMethodID,
        auth_method_mac_key: SecretKey,
        auth_method_password_algorithm: UntrustedPasswordAlgorithm | None,
    ) -> None | AccountCreateProceedBadOutcome:
        async with self._data.account_creation_lock:
            outcome = self._create_account_check_code(now, human_handle.email, validation_code)
            match outcome:
                case None:
                    pass
                case error:
                    return error

            # Check for auth method uniqueness
            if self._auth_method_id_already_exists(auth_method_id):
                return AccountCreateProceedBadOutcome.AUTH_METHOD_ID_ALREADY_EXISTS

            # Create authentication method
            auth_method = MemoryAuthenticationMethod(
                id=auth_method_id,
                created_on=now,
                created_by_ip=created_by_ip,
                created_by_user_agent=created_by_user_agent,
                mac_key=auth_method_mac_key,
                vault_key_access=vault_key_access,
                password_algorithm=auth_method_password_algorithm,
                disabled_on=None,
            )

            vault = MemoryAccountVault(
                items={}, authentication_methods={auth_method_id: auth_method}
            )

            self._data.accounts[human_handle.email] = MemoryAccount(
                account_email=human_handle.email,
                human_handle=human_handle,
                previous_vaults=[],
                current_vault=vault,
            )

            # And finally discard the used validation email
            del self._data.account_create_validation_emails[human_handle.email]

    @override
    async def delete_send_validation_email(
        self,
        now: DateTime,
        auth_method_id: AccountAuthMethodID,
    ) -> ValidationCode | SendEmailBadOutcome | AccountDeleteSendValidationEmailBadOutcome:
        async with self._data.account_creation_lock:
            match self._data.get_account_from_active_auth_method(auth_method_id=auth_method_id):
                case (account, _):
                    pass
                case None:
                    return AccountDeleteSendValidationEmailBadOutcome.ACCOUNT_NOT_FOUND

            # Note we simply overwrite any existing previous validation code
            validation_code = ValidationCode.generate()
            self._data.account_delete_validation_emails[account.account_email] = ValidationCodeInfo(
                validation_code, now
            )

        # Note we send the email after the lock has been released, this is to
        # simulation what is done in PostgreSQL where the transaction has to be
        # finished before sending the email since this operation can be long.
        outcome = await self._send_account_delete_validation_email(
            email=account.account_email,
            validation_code=validation_code,
        )
        match outcome:
            case None:
                return validation_code
            case error:
                return error

    @override
    async def delete_proceed(
        self,
        now: DateTime,
        auth_method_id: AccountAuthMethodID,
        validation_code: ValidationCode,
    ) -> None | AccountDeleteProceedBadOutcome:
        async with self._data.account_creation_lock:
            # 1) Retrieve the account

            match self._data.get_account_from_active_auth_method(auth_method_id=auth_method_id):
                case (account, _):
                    pass
                case None:
                    return AccountDeleteProceedBadOutcome.ACCOUNT_NOT_FOUND

            assert account.deleted_on is None  # Sanity check

            # 2) Check validation code

            try:
                last_mail_info = self._data.account_delete_validation_emails[account.account_email]
            except KeyError:
                return AccountDeleteProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED

            if not last_mail_info.can_be_used(now):
                return AccountDeleteProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED

            if last_mail_info.validation_code != validation_code:
                # Register the failed attempt
                last_mail_info.failed_attempts += 1
                return AccountDeleteProceedBadOutcome.INVALID_VALIDATION_CODE

            # 3) All good, we can do the actual deletion

            account.deleted_on = now
            for auth_method in account.current_vault.authentication_methods.values():
                if auth_method.disabled_on is None:
                    auth_method.disabled_on = now

            # 4) And finally discard the used validation email

            del self._data.account_delete_validation_emails[account.account_email]

    @override
    async def recover_send_validation_email(
        self, now: DateTime, email: EmailAddress
    ) -> ValidationCode | SendEmailBadOutcome | AccountRecoverSendValidationEmailBadOutcome:
        async with self._data.account_creation_lock:
            try:
                account = self._data.accounts[email]
            except KeyError:
                return AccountRecoverSendValidationEmailBadOutcome.ACCOUNT_NOT_FOUND
            if account.deleted_on is not None:
                return AccountRecoverSendValidationEmailBadOutcome.ACCOUNT_NOT_FOUND

            # Note we simply overwrite any existing previous validation code
            validation_code = ValidationCode.generate()
            self._data.account_recover_validation_emails[email] = ValidationCodeInfo(
                validation_code, now
            )

        # Note we send the email after the lock has been released, this is to
        # simulation what is done in PostgreSQL where the transaction has to be
        # finished before sending the email since this operation can be long.
        outcome = await self._send_account_recover_validation_email(
            email=email,
            validation_code=validation_code,
        )
        match outcome:
            case None:
                return validation_code
            case error:
                return error

    @override
    async def recover_proceed(
        self,
        now: DateTime,
        validation_code: ValidationCode,
        email: EmailAddress,
        created_by_user_agent: str,
        created_by_ip: str | Literal[""],
        new_vault_key_access: bytes,
        new_auth_method_id: AccountAuthMethodID,
        new_auth_method_mac_key: SecretKey,
        new_auth_method_password_algorithm: UntrustedPasswordAlgorithm | None,
    ) -> None | AccountRecoverProceedBadOutcome:
        async with self._data.account_creation_lock:
            # 1) Check validation code

            try:
                last_mail_info = self._data.account_recover_validation_emails[email]
            except KeyError:
                return AccountRecoverProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED

            if not last_mail_info.can_be_used(now):
                return AccountRecoverProceedBadOutcome.SEND_VALIDATION_EMAIL_REQUIRED

            if last_mail_info.validation_code != validation_code:
                # Register the failed attempt
                last_mail_info.failed_attempts += 1
                return AccountRecoverProceedBadOutcome.INVALID_VALIDATION_CODE

            # Since the validation code is valid, the account is guaranteed to exist
            # (and not be deleted).

            account = self._data.accounts[email]
            assert account.deleted_on is None

            # 2) Check for auth method uniqueness

            if self._auth_method_id_already_exists(new_auth_method_id):
                return AccountRecoverProceedBadOutcome.AUTH_METHOD_ID_ALREADY_EXISTS

            # 3) Disable previous authentication methods

            for auth_method in account.current_vault.active_authentication_methods:
                auth_method.disabled_on = now

            # 4) Create the new vault with its authentication method

            new_auth_method = MemoryAuthenticationMethod(
                id=new_auth_method_id,
                created_on=now,
                created_by_ip=created_by_ip,
                created_by_user_agent=created_by_user_agent,
                mac_key=new_auth_method_mac_key,
                vault_key_access=new_vault_key_access,
                password_algorithm=new_auth_method_password_algorithm,
                disabled_on=None,
            )
            new_vault = MemoryAccountVault(
                items={}, authentication_methods={new_auth_method_id: new_auth_method}
            )
            account.previous_vaults.append(account.current_vault)
            account.current_vault = new_vault

            # 5) And finally discard the used validation email

            del self._data.account_recover_validation_emails[email]

    @override
    async def vault_item_upload(
        self, auth_method_id: AccountAuthMethodID, item_fingerprint: HashDigest, item: bytes
    ) -> None | AccountVaultItemUploadBadOutcome:
        match self._data.get_account_from_active_auth_method(auth_method_id=auth_method_id):
            case (account, _):
                pass
            case None:
                return AccountVaultItemUploadBadOutcome.ACCOUNT_NOT_FOUND

        if item_fingerprint in account.current_vault.items:
            return AccountVaultItemUploadBadOutcome.FINGERPRINT_ALREADY_EXISTS

        account.current_vault.items[item_fingerprint] = item

    @override
    async def vault_item_list(
        self,
        auth_method_id: AccountAuthMethodID,
    ) -> VaultItems | AccountVaultItemListBadOutcome:
        match self._data.get_account_from_active_auth_method(auth_method_id=auth_method_id):
            case (account, auth_method):
                pass
            case None:
                return AccountVaultItemListBadOutcome.ACCOUNT_NOT_FOUND

        return VaultItems(
            key_access=auth_method.vault_key_access,
            items=account.current_vault.items,
        )

    @override
    async def vault_key_rotation(
        self,
        now: DateTime,
        auth_method_id: AccountAuthMethodID,
        created_by_ip: str | Literal[""],
        created_by_user_agent: str,
        new_auth_method_id: AccountAuthMethodID,
        new_auth_method_mac_key: SecretKey,
        new_auth_method_password_algorithm: UntrustedPasswordAlgorithm | None,
        new_vault_key_access: bytes,
        items: dict[HashDigest, bytes],
    ) -> None | AccountVaultKeyRotation:
        match self._data.get_account_from_active_auth_method(auth_method_id=auth_method_id):
            case (account, _):
                pass
            case None:
                return AccountVaultKeyRotation.ACCOUNT_NOT_FOUND

        if self._auth_method_id_already_exists(new_auth_method_id):
            return AccountVaultKeyRotation.NEW_AUTH_METHOD_ID_ALREADY_EXISTS

        if account.current_vault.items.keys() != items.keys():
            return AccountVaultKeyRotation.ITEMS_MISMATCH

        for auth_method in account.current_vault.authentication_methods.values():
            auth_method.disabled_on = now
        account.previous_vaults.append(account.current_vault)
        account.current_vault = MemoryAccountVault(
            items=items,
            authentication_methods={
                new_auth_method_id: MemoryAuthenticationMethod(
                    id=new_auth_method_id,
                    created_on=now,
                    created_by_ip=created_by_ip,
                    created_by_user_agent=created_by_user_agent,
                    mac_key=new_auth_method_mac_key,
                    vault_key_access=new_vault_key_access,
                    password_algorithm=new_auth_method_password_algorithm,
                )
            },
        )

        return None

    @override
    async def vault_item_recovery_list(
        self,
        auth_method_id: AccountAuthMethodID,
    ) -> VaultItemRecoveryList | AccountVaultItemRecoveryList:
        match self._data.get_account_from_active_auth_method(auth_method_id=auth_method_id):
            case (account, _):
                pass
            case None:
                return AccountVaultItemRecoveryList.ACCOUNT_NOT_FOUND

        def _convert_vault(vault: MemoryAccountVault) -> VaultItemRecoveryVault:
            return VaultItemRecoveryVault(
                auth_methods=[
                    AuthMethod(
                        auth_method_id=auth_method.id,
                        created_on=auth_method.created_on,
                        created_by_ip=auth_method.created_by_ip,
                        created_by_user_agent=auth_method.created_by_user_agent,
                        vault_key_access=auth_method.vault_key_access,
                        password_algorithm=auth_method.password_algorithm,
                        disabled_on=auth_method.disabled_on,
                    )
                    for auth_method in vault.authentication_methods.values()
                ],
                vault_items=vault.items,
            )

        return VaultItemRecoveryList(
            current_vault=_convert_vault(account.current_vault),
            previous_vaults=[_convert_vault(v) for v in account.previous_vaults],
        )

    @override
    async def invite_self_list(
        self, auth_method_id: AccountAuthMethodID
    ) -> list[tuple[OrganizationID, InvitationToken, InvitationType]] | AccountInviteListBadOutcome:
        match self._data.get_account_from_active_auth_method(auth_method_id=auth_method_id):
            case (account, _):
                pass
            case None:
                return AccountInviteListBadOutcome.ACCOUNT_NOT_FOUND

        res = []
        for org in self._data.organizations.values():
            # For each organization, we must first determine if the account's email
            # address corresponds to an active user.
            # - If that's the case, we list only the invitations related to this user's ID
            #   (i.e. we will return only device&shamir invitations).
            # - Otherwise, we list only the user invitations related to the account's email.
            maybe_active_user = next(
                (
                    u.cooked.user_id
                    for u in org.active_users()
                    if u.cooked.human_handle.email == account.account_email
                ),
                None,
            )
            match maybe_active_user:
                case UserID() as active_user_id:
                    invitations = (
                        invitation
                        for invitation in org.invitations.values()
                        if invitation.claimer_user_id == active_user_id
                    )
                case None:
                    invitations = (
                        invitation
                        for invitation in org.invitations.values()
                        if invitation.claimer_email == account.account_email
                    )

            res.extend(
                (org.organization_id, invitation.token, invitation.type)
                for invitation in invitations
            )

        return res

    async def organization_self_list(
        self, auth_method_id: AccountAuthMethodID
    ) -> AccountOrganizationSelfList | AccountOrganizationListBadOutcome:
        match self._data.get_account_from_active_auth_method(auth_method_id=auth_method_id):
            case (account, _):
                pass
            case None:
                return AccountOrganizationListBadOutcome.ACCOUNT_NOT_FOUND

        active = []
        revoked = []
        for org in self._data.organizations.values():
            for user in org.users.values():
                if user.cooked.human_handle.email != account.account_email:
                    # The email is not part of this organization
                    continue
                if user.revoked_on is not None:
                    revoked.append(
                        AccountOrganizationSelfListRevokedUser(
                            user_id=user.cooked.user_id,
                            created_on=user.cooked.timestamp,
                            revoked_on=user.revoked_on,
                            current_profile=user.current_profile,
                            organization_id=org.organization_id,
                        )
                    )
                else:
                    active.append(
                        AccountOrganizationSelfListActiveUser(
                            user_id=user.cooked.user_id,
                            created_on=user.cooked.timestamp,
                            current_profile=user.current_profile,
                            is_frozen=user.is_frozen,
                            organization_id=org.organization_id,
                            organization_is_expired=org.is_expired,
                            organization_user_profile_outsider_allowed=org.user_profile_outsider_allowed,
                            organization_active_users_limit=org.active_users_limit,
                            organization_allowed_client_agent=org.allowed_client_agent,
                            organization_account_vault_strategy=org.account_vault_strategy,
                        )
                    )

        return AccountOrganizationSelfList(
            active=active,
            revoked=revoked,
        )

    @override
    async def auth_method_create(
        self,
        now: DateTime,
        auth_method_id: AccountAuthMethodID,
        created_by_user_agent: str,
        created_by_ip: str | Literal[""],
        new_auth_method_id: AccountAuthMethodID,
        new_auth_method_mac_key: SecretKey,
        new_auth_method_password_algorithm: UntrustedPasswordAlgorithm | None,
        new_vault_key_access: bytes,
    ) -> None | AccountAuthMethodCreateBadOutcome:
        # Verify the requesting account exists and is active
        match self._data.get_account_from_active_auth_method(auth_method_id=auth_method_id):
            case (account, _):
                pass
            case None:
                return AccountAuthMethodCreateBadOutcome.ACCOUNT_NOT_FOUND

        # Check for auth method uniqueness
        if self._auth_method_id_already_exists(new_auth_method_id):
            return AccountAuthMethodCreateBadOutcome.AUTH_METHOD_ID_ALREADY_EXISTS

        # If we're creating a password authentication method, disable any existing
        # password authentication methods in the current vault
        if new_auth_method_password_algorithm is not None:
            for existing_auth_method in account.current_vault.authentication_methods.values():
                if (
                    existing_auth_method.password_algorithm is not None
                    and existing_auth_method.disabled_on is None
                ):
                    existing_auth_method.disabled_on = now

        # Create the new authentication method

        new_auth_method = MemoryAuthenticationMethod(
            id=new_auth_method_id,
            created_on=now,
            created_by_ip=created_by_ip,
            created_by_user_agent=created_by_user_agent,
            mac_key=new_auth_method_mac_key,
            vault_key_access=new_vault_key_access,
            password_algorithm=new_auth_method_password_algorithm,
            disabled_on=None,
        )

        account.current_vault.authentication_methods[new_auth_method_id] = new_auth_method

        return None

    @override
    async def auth_method_list(
        self,
        auth_method_id: AccountAuthMethodID,
    ) -> list[AuthMethod] | AccountAuthMethodListBadOutcome:
        # Verify the requesting account exists and is active
        match self._data.get_account_from_active_auth_method(auth_method_id=auth_method_id):
            case (account, _):
                pass
            case None:
                return AccountAuthMethodListBadOutcome.ACCOUNT_NOT_FOUND

        # Return all active authentication methods for the account's current vault
        items = [
            AuthMethod(
                auth_method_id=auth_method.id,
                created_on=auth_method.created_on,
                created_by_ip=auth_method.created_by_ip,
                created_by_user_agent=auth_method.created_by_user_agent,
                vault_key_access=auth_method.vault_key_access,
                password_algorithm=auth_method.password_algorithm,
                disabled_on=None,
            )
            for auth_method in account.current_vault.active_authentication_methods
        ]
        items.sort(key=lambda x: x.created_on)
        return items

    @override
    async def auth_method_disable(
        self,
        now: DateTime,
        auth_method_id: AccountAuthMethodID,
        to_disable_auth_method_id: AccountAuthMethodID,
    ) -> None | AccountAuthMethodDisableBadOutcome:
        if auth_method_id == to_disable_auth_method_id:
            return AccountAuthMethodDisableBadOutcome.SELF_DISABLE_NOT_ALLOWED

        # Verify the requesting account exists and is active
        match self._data.get_account_from_active_auth_method(auth_method_id=auth_method_id):
            case (account, _):
                pass
            case None:
                return AccountAuthMethodDisableBadOutcome.ACCOUNT_NOT_FOUND

        # Find the target authentication method
        match self._data.get_account_from_any_auth_method(to_disable_auth_method_id):
            case None:
                return AccountAuthMethodDisableBadOutcome.AUTH_METHOD_NOT_FOUND
            case (target_account, target_auth_method):
                if target_account is not account:
                    return AccountAuthMethodDisableBadOutcome.CROSS_ACCOUNT_NOT_ALLOWED
                if target_auth_method.disabled_on is not None:
                    return AccountAuthMethodDisableBadOutcome.AUTH_METHOD_ALREADY_DISABLED

        # Disable the authentication method
        target_auth_method.disabled_on = now
