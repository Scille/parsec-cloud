# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import Literal, override

from parsec._parsec import (
    AccountAuthMethodID,
    AccountDeletionToken,
    DateTime,
    EmailAddress,
    EmailValidationToken,
    HashDigest,
    SecretKey,
)
from parsec.components.account import (
    AccountCreateAccountBadOutcome,
    AccountCreateAccountDeletionTokenBadOutcome,
    AccountCreateEmailValidationTokenBadOutcome,
    AccountVaultItemListBadOutcome,
    AccountVaultItemRecoveryList,
    AccountVaultItemUploadBadOutcome,
    AccountVaultKeyRotation,
    BaseAccountComponent,
    PasswordAlgorithm,
    VaultItemRecoveryAuthMethod,
    VaultItemRecoveryList,
    VaultItemRecoveryVault,
    VaultItems,
)
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

    @override
    async def get_password_algorithm_or_fake_it(self, email: EmailAddress) -> PasswordAlgorithm:
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
    async def create_email_validation_token(
        self, email: EmailAddress, now: DateTime
    ) -> EmailValidationToken | AccountCreateEmailValidationTokenBadOutcome:
        if email in self._data.accounts:
            return AccountCreateEmailValidationTokenBadOutcome.ACCOUNT_ALREADY_EXISTS
        elif email in self._data.unverified_emails:
            (_, last_email_datetime) = self._data.unverified_emails[email]
            if not self.should_resend_token(now, last_email_datetime):
                return AccountCreateEmailValidationTokenBadOutcome.TOO_SOON_AFTER_PREVIOUS_DEMAND

        token = EmailValidationToken.new()
        self._data.unverified_emails[email] = (token, now)
        return token

    @override
    async def create_account(
        self,
        token: EmailValidationToken,
        now: DateTime,
        mac_key: SecretKey,
        vault_key_access: bytes,
        human_label: str,
        created_by_user_agent: str,
        created_by_ip: str | Literal[""],
        auth_method_id: AccountAuthMethodID,
        auth_method_password_algorithm: PasswordAlgorithm | None,
    ) -> None | AccountCreateAccountBadOutcome:
        # look for an email linked to the provided token
        (unverified_email, token_created_at) = next(
            ((k, v[1]) for (k, v) in self._data.unverified_emails.items() if v[0] == token),
            (None, None),
        )
        if (
            unverified_email is None
            or token_created_at is None
            or not self.is_creation_token_still_valid(token_created_at, now)
        ):
            # the provided token is not in unverified emails or no longer valid
            return AccountCreateAccountBadOutcome.INVALID_TOKEN
        email = unverified_email

        # create authentication method
        auth_method = MemoryAuthenticationMethod(
            id=auth_method_id,
            created_on=now,
            created_by_ip=created_by_ip,
            created_by_user_agent=created_by_user_agent,
            mac_key=mac_key,
            vault_key_access=vault_key_access,
            password_algorithm=auth_method_password_algorithm,
            disabled_on=None,
        )

        authentication_methods = {auth_method_id: auth_method}
        # check for auth method uniqueness
        if self.auth_method_id_already_exists(auth_method_id):
            return AccountCreateAccountBadOutcome.AUTH_METHOD_ALREADY_EXISTS

        vault = MemoryAccountVault(items={}, authentication_methods=authentication_methods)

        self._data.accounts[email] = MemoryAccount(
            account_email=email,
            human_label=human_label,
            previous_vaults=[],
            current_vault=vault,
        )

        # remove from unverified emails at the end
        self._data.unverified_emails.pop(email)

    @override
    def test_get_token_by_email(self, email: EmailAddress) -> EmailValidationToken | None:
        """Use only in tests, nothing is checked."""
        return self._data.unverified_emails[email][0]

    def auth_method_id_already_exists(self, auth_method_id: AccountAuthMethodID) -> bool:
        return self._data.get_account_from_any_auth_method(auth_method_id) is not None

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
        new_auth_method_password_algorithm: PasswordAlgorithm | None,
        new_vault_key_access: bytes,
        items: dict[HashDigest, bytes],
    ) -> None | AccountVaultKeyRotation:
        match self._data.get_account_from_active_auth_method(auth_method_id=auth_method_id):
            case (account, _):
                pass
            case None:
                return AccountVaultKeyRotation.ACCOUNT_NOT_FOUND

        if self.auth_method_id_already_exists(new_auth_method_id):
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
                    VaultItemRecoveryAuthMethod(
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
    async def create_email_deletion_token(
        self, email: EmailAddress, now: DateTime
    ) -> AccountDeletionToken | AccountCreateAccountDeletionTokenBadOutcome:
        try:
            (_, last_email_datetime) = self._data.accounts_deletion_requested[email]
            if not self.should_resend_token(now, last_email_datetime):
                return AccountCreateAccountDeletionTokenBadOutcome.TOO_SOON_AFTER_PREVIOUS_DEMAND
        except KeyError:
            pass
        token = AccountDeletionToken.new()
        self._data.accounts_deletion_requested[email] = (token, now)
        return token
