# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import override

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    EmailAddress,
    EmailValidationToken,
    HashDigest,
    SecretKey,
)
from parsec.components.account import (
    AccountCreateAccountWithPasswordBadOutcome,
    AccountCreateEmailValidationTokenBadOutcome,
    AccountVaultItemListBadOutcome,
    AccountVaultItemRecoveryList,
    AccountVaultItemUploadBadOutcome,
    AccountVaultKeyRotation,
    BaseAccountComponent,
    PasswordAlgorithm,
    PasswordAlgorithmArgon2ID,
    VaultItemRecoveryAuthMethodPassword,
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
    async def create_account_with_password(
        self,
        token: EmailValidationToken,
        now: DateTime,
        mac_key: SecretKey,
        vault_key_access: bytes,
        human_label: str,
        created_by_user_agent: str,
        created_by_ip: str | None,
        password_secret_algorithm: PasswordAlgorithm,
        auth_method_id: AccountAuthMethodID,
    ) -> None | AccountCreateAccountWithPasswordBadOutcome:
        # look for an email linked to the provided token
        unverified_email = next(
            (k for (k, v) in self._data.unverified_emails.items() if v[0] == token), None
        )
        if unverified_email is None:
            # the provided token is not in unverified emails
            return AccountCreateAccountWithPasswordBadOutcome.INVALID_TOKEN
        email = unverified_email

        # create authentication method
        auth_method = MemoryAuthenticationMethod(
            created_on=now,
            created_by_ip=created_by_ip,
            created_by_user_agent=created_by_user_agent,
            mac_key=mac_key,
            vault_key_access=vault_key_access,
            password_secret_algorithm=password_secret_algorithm,
            disabled_on=None,
            id=auth_method_id,
        )

        authentication_methods = {auth_method_id: auth_method}
        # check for auth method uniqueness
        if self.auth_method_id_already_exists(auth_method_id):
            return AccountCreateAccountWithPasswordBadOutcome.AUTH_METHOD_ALREADY_EXISTS

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
        created_by_ip: str | None,
        created_by_user_agent: str,
        new_auth_method_id: AccountAuthMethodID,
        new_auth_method_mac_key: SecretKey,
        new_password_algorithm: PasswordAlgorithm,
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
                    password_secret_algorithm=new_password_algorithm,
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
                    VaultItemRecoveryAuthMethodPassword(
                        created_on=auth_method.created_on,
                        created_by_ip=auth_method.created_by_ip,
                        created_by_user_agent=auth_method.created_by_user_agent,
                        vault_key_access=auth_method.vault_key_access,
                        algorithm=PasswordAlgorithmArgon2ID(
                            salt=auth_method.password_secret_algorithm.salt,
                            opslimit=auth_method.password_secret_algorithm.opslimit,
                            memlimit_kb=auth_method.password_secret_algorithm.memlimit_kb,
                            parallelism=auth_method.password_secret_algorithm.parallelism,
                        ),
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
