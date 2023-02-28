# Sequester - Protocol & Data API evolutions

From [ISSUE-2464](https://github.com/Scille/parsec-cloud/issues/2464)

see [RFC-0021](0021-sequester-encryption-signing-key-formats.md)

## 1 - Organization Bootstrap

### API Format

```python
class OrganizationBootstrapReqSchema(BaseReqSchema):
    bootstrap_token = fields.String(required=True)
    root_verify_key = fields.VerifyKey(required=True)
    user_certificate = fields.Bytes(required=True)
    device_certificate = fields.Bytes(required=True)
    redacted_user_certificate = fields.Bytes(required=True)
    redacted_device_certificate = fields.Bytes(required=True)

    sequester_verify_key_certificate = fields.Bytes(required=False, allow_none=True)  # Don't forget to add the a added in revision XXX comment !

# No changes on the rep !
class OrganizationBootstrapRepSchema(BaseRepSchema):
    pass
```

`sequester_verify_key_certificate` should contains a `SequesterVerifyKeyCertificate` (see [RFC-0021](0021-sequester-encryption-signing-key-formats.md))

### Checks

Of course `sequester_verify_key_certificate` must be verified against the root verify key (just like the others certifs), we may also want to verify the content of `SequesterVerifyKeyCertificate`

## 2 - Organization stats

```python
# No changes in req !
class OrganizationStatsReqSchema(BaseReqSchema):
    pass


class OrganizationStatsRepSchema(BaseRepSchema):
    data_size = fields.Integer(required=True)
    metadata_size = fields.Integer(required=True)
    realms = fields.Integer(required=True)
    users = fields.Integer(required=True)
    active_users = fields.Integer(required=True)
    users_per_profile_detail = fields.List(
        fields.Nested(UsersPerProfileDetailItemSchema), required=True
    )

    # Don't forget to add the a added in revision XXX comment !
    sequester_verify_key_certificate = fields.Bytes(allow_none=True, required=False)
    sequester_encryption_keys = fields.List(fields.Bytes(), allow_none=True, required=False)
```

Another approaches is to just provide a `is_sequestered_organization` boolean field (especially given we want to provide the verify & encryption keys as part as the vlob update/create error to simplify client sync code, see below).
This is simpler but provide less information given we don't know if the sequester is actually in use (i.e. there is sequester encryption key that must be used when uploading new manifests). Those information are interesting for the GUI: we could display a popup the first time we detect actual use of sequester (without having to actually wait for sync to be triggered) or have have a tooltip on organization button to inform if the sequester is enabled and in active use.

So I think the slightly more complex api is worth it given it gave more choice for later ;-)

## 3 - Vlob update/create

```python
class VlobCreateReqSchema(BaseReqSchema):
    realm_id = RealmIDField(required=True)
    encryption_revision = fields.Integer(required=True)
    vlob_id = VlobIDField(required=True)
    # If blob contains a signed message, it timestamp cannot be directly enforced
    # by the backend (given the message is probably also encrypted).
    # Hence the timestamp is passed in clear so backend can reject the message
    # if it considers the timestamp invalid. On top of that each client asking
    # for the message will receive the declared timestamp to check against
    # the actual timestamp within the message.
    timestamp = fields.DateTime(required=True)
    blob = fields.Bytes(required=True)

    # Don't forget to add the a added in revision XXX comment !
    sequester_blob = fields.Map(fields.UUID(), fields.Bytes(), required=False, allow_none=True)


class VlobUpdateReqSchema(BaseReqSchema):
    encryption_revision = fields.Integer(required=True)
    vlob_id = VlobIDField(required=True)
    timestamp = fields.DateTime(required=True)
    version = fields.Integer(required=True, validate=_validate_version)
    blob = fields.Bytes(required=True)

    # Don't forget to add the a added in revision XXX comment !
    sequester_blob = fields.Map(fields.UUID(), fields.Bytes(), required=False, allow_none=True)
```

EDIT: change sequester_blob key type to UUID

In case the client send a `vlob_update/vlob_create` command without the right `sequester_blob` parameter (i.e. missing or unknown sequester encryption key) a specific error is returned:

```python
class VlobCreateSequesterBlobErrorSchema(...):
    status = fields.CheckConstant("inconsistent_sequester_blob", required=True)
    sequester_verify_key_certificate = fields.Bytes(required=True)
    sequester_encrypt_key_certificates = fields.List(fields.Bytes(), required=True)
```
