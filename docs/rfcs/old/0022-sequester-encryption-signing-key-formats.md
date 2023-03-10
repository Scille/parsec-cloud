# Sequester - Encryption & Siging key formats

From [ISSUE-2463](https://github.com/Scille/parsec-cloud/issues/2463)

see [RFC-0023](0023-sequester-protocol-data-api-evolutions.md)

Bootstrap org: SequesterSigningKey verified on client or server side (or both ?)

## Key Algorithm, PKCS11 etc

TODO

## Certificate format

### Sequester verify key

```python
@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class SequesterVerifyKeyCertificate(BaseAPISignedData):
    class SCHEMA_CLS(BaseSignedDataSchema):
        # Override author field to always uses None given this certificate can only be signed by the root key
        author = fields.CheckedConstant(None, required=True)

        type = fields.CheckedConstant("sequester_verify_key_certificate", required=True)
        verify_key = fields.Bytes(required=True)
        verify_key_format=fields.Enum(required=True)


    # Override author field to always uses None given this certificate can only be signed by the root key
    author: Literal[None]  # type: ignore[assignment]
    verify_key: bytes
    verify_key_format: SigningKeyFormat
```

Another approaches is to create a custom certificate format instead of sticking to what is done with the user/device/realm_role certifs.
This would save us having this weird always-none author field, but in the other hand this would create confusion between two types of certificates.
However there is also the `SequesterEncryptionKeyCertificate` to take into account (given it is signed by `SequesterSigningKey` and hence work differently from the other certificates...), so it may be worth it to use another wording instead of certificate suffix and be free about a custom format.

@AntoineDupre @vxgmichel what do you think ?

On top of that we want to make sure `verify_key_der` contains valid data (given once bootstrap is done, this key cannot be changed).
So we must have at least one verification, but we can also do two:

- Client side verification: this is good to catch error if the wrong file has been selected when using the bootstrap cli
- Server side verification: do we really want to only trust the client on this ? 😄 (another possible future usecase would be to allow to provide the verify key der in the organization_create administration command, so that we can be sure the first user have done the organization bootstrap with the expected verify key)

### Sequester encryption key

```python
@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class SequesterServiceEncryptionKey(BaseAPIData):
    class SCHEMA_CLS(BaseSchema):

        type = fields.CheckedConstant("sequester_service_encryption_key", required=True)
        encryption_key = fields.Bytes(required=True)
        encryption_key_format=fields.Enum(required=True)
        timestamp = fields.DateTime(required=True)
        service_name=fields.Text(required=True)

    encryption_key: bytes
    encryption_key_format: EncryptionKeyFormat
    timestamp: DateTime
    service_name: str
```

The sequester key payload has to be signed by the Organisation sequester verify key. The signature has to be stored in the backend.

```python
class SequesterRegisterServiceSchema(BaseSchema):
    service_type = SequesterServiceTypeField(required=True)
    service_id = fields.UUID(required=True)
    sequester_payload = fields.Bytes(required=True)  # Byte dump of SequesterServiceEncryptionKey
    sequester_payload_signature = fields.Bytes(required=True)
```
