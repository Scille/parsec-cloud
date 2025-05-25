# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from typing import override

from pydantic import EmailStr

from parsec._parsec import (
    AccountAuthMethodID,
    DateTime,
    EmailValidationToken,
    HashDigest,
    SecretKey,
)
from parsec.components.account import (
    AccountCreateAccountWithPasswordBadOutcome,
    AccountCreateEmailValidationTokenBadOutcome,
    AccountGetPasswordSecretKeyBadOutcome,
    AccountVaultItemListBadOutcome,
    AccountVaultItemUploadBadOutcome,
    BaseAccountComponent,
    PasswordAlgorithm,
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
        self, email: EmailStr, now: DateTime
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
    async def check_signature(self):
        pass

    @override
    def get_password_mac_key(
        self, user_email: EmailStr
    ) -> SecretKey | AccountGetPasswordSecretKeyBadOutcome:
        account = self._data.accounts.get(user_email)
        if account is None:
            return AccountGetPasswordSecretKeyBadOutcome.USER_NOT_FOUND
        assert isinstance(account, MemoryAccount)
        # No need to check if the authentication method is password because
        # currently it is the only authentication method supported

        secret_key = next(
            (
                v.mac_key
                for v in account.current_vault.authentication_methods.values()
                if v.disabled_on is None
            ),
            None,
        )
        if secret_key is None:
            return AccountGetPasswordSecretKeyBadOutcome.UNABLE_TO_GET_SECRET_KEY
        return secret_key

    @override
    def test_get_token_by_email(self, email: str) -> EmailValidationToken | None:
        """Use only in tests, nothing is checked."""
        return self._data.unverified_emails[email][0]

    def auth_method_id_already_exists(self, id: AccountAuthMethodID) -> bool:
        for account in self._data.accounts.values():
            for auth_id in account.current_vault.authentication_methods.keys():
                if auth_id == id:
                    return True
            for vault in account.previous_vaults:
                for auth_id in vault.authentication_methods.keys():
                    if auth_id == id:
                        return True
        return False

    @override
    async def vault_item_upload(
        self, auth_method_id: AccountAuthMethodID, item_fingerprint: HashDigest, item: bytes
    ) -> None | AccountVaultItemUploadBadOutcome:
        match self._data.get_account_from_auth_method(auth_method_id=auth_method_id):
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
        match self._data.get_account_from_auth_method(auth_method_id=auth_method_id):
            case (account, auth_method):
                pass
            case None:
                return AccountVaultItemListBadOutcome.ACCOUNT_NOT_FOUND

        return VaultItems(
            key_access=auth_method.vault_key_access,
            items=account.current_vault.items,
        )
