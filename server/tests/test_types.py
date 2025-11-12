# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest
from pydantic import BaseModel, ConfigDict, ValidationError

from parsec._parsec import (
    ActiveUsersLimit,
    DateTime,
    DeviceID,
    EmailAddress,
    GreetingAttemptID,
    InvitationStatus,
    OrganizationID,
    RealmRole,
    SequesterServiceID,
    UserID,
    UserProfile,
    VlobID,
)
from parsec.types import (
    ActiveUsersLimitField,
    Base64BytesField,
    DateTimeField,
    DeviceIDField,
    EmailAddressField,
    GreetingAttemptIDField,
    InvitationStatusField,
    InvitationToken,
    InvitationTokenField,
    OrganizationIDField,
    RealmRoleField,
    SequesterServiceIDField,
    Unset,
    UnsetType,
    UserIDField,
    UserProfileField,
    VlobIDField,
)


# Pydantic models are super slow to build, so we only create a single one
# which is common for all tests
class AllTypesSchema(BaseModel):
    model_config = ConfigDict(arbitrary_types_allowed=True, strict=True)

    base64_bytes: Base64BytesField | UnsetType = Unset
    organization_id: OrganizationIDField | UnsetType = Unset
    user_id: UserIDField | UnsetType = Unset
    device_id: DeviceIDField | UnsetType = Unset
    sequester_service_id: SequesterServiceIDField | UnsetType = Unset
    invitation_token: InvitationTokenField | UnsetType = Unset
    greeting_attempt_id: GreetingAttemptIDField | UnsetType = Unset
    invitation_status: InvitationStatusField | UnsetType = Unset
    realm_role: RealmRoleField | UnsetType = Unset
    vlob_id: VlobIDField | UnsetType = Unset
    date_time: DateTimeField | UnsetType = Unset
    user_profile: UserProfileField | UnsetType = Unset
    active_users_limit: ActiveUsersLimitField | UnsetType = Unset
    email_address: EmailAddressField | UnsetType = Unset


@pytest.mark.parametrize(
    "field", (*AllTypesSchema.model_fields.keys(), "active_users_limit_no_limit")
)
def test_good(field: str):
    match field:
        case "base64_bytes":
            data = {"base64_bytes": "aGVsbG8="}
            expected = b"hello"
        case "organization_id":
            data = {"organization_id": "CoolOrg"}
            expected = OrganizationID("CoolOrg")
        case "user_id":
            data = {"user_id": "4263b5fb763b48e897bbef8228f71a45"}
            expected = UserID.from_hex("4263b5fb763b48e897bbef8228f71a45")
        case "device_id":
            data = {"device_id": "4263b5fb763b48e897bbef8228f71a45"}
            expected = DeviceID.from_hex("4263b5fb763b48e897bbef8228f71a45")
        case "sequester_service_id":
            data = {"sequester_service_id": "4263b5fb763b48e897bbef8228f71a45"}
            expected = SequesterServiceID.from_hex("4263b5fb763b48e897bbef8228f71a45")
        case "invitation_token":
            data = {"invitation_token": "4263b5fb763b48e897bbef8228f71a45"}
            expected = InvitationToken.from_hex("4263b5fb763b48e897bbef8228f71a45")
        case "greeting_attempt_id":
            data = {"greeting_attempt_id": "4263b5fb763b48e897bbef8228f71a45"}
            expected = GreetingAttemptID.from_hex("4263b5fb763b48e897bbef8228f71a45")
        case "invitation_status":
            data = {"invitation_status": "CANCELLED"}
            expected = InvitationStatus.CANCELLED
        case "realm_role":
            data = {"realm_role": "CONTRIBUTOR"}
            expected = RealmRole.CONTRIBUTOR
        case "vlob_id":
            data = {"vlob_id": "4263b5fb763b48e897bbef8228f71a45"}
            expected = VlobID.from_hex("4263b5fb763b48e897bbef8228f71a45")
        case "date_time":
            data = {"date_time": "2000-01-01T00:00:00Z"}
            expected = DateTime(2000, 1, 1)
        case "user_profile":
            data = {"user_profile": "ADMIN"}
            expected = UserProfile.ADMIN
        case "active_users_limit":
            data = {"active_users_limit": 42}
            expected = ActiveUsersLimit.limited_to(42)
        case "active_users_limit_no_limit":
            field = "active_users_limit"
            data = {"active_users_limit": None}
            expected = ActiveUsersLimit.NO_LIMIT
        case "email_address":
            data = {"email_address": "zack@example.com"}
            expected = EmailAddress("zack@example.com")
        case unknown:
            assert False, unknown

    # deserialization
    out = AllTypesSchema(**data)  # type: ignore
    assert getattr(out, field) == expected
    # serialization
    dump = {k: v for k, v in out.model_dump().items() if k in data.keys()}
    assert dump == data


@pytest.mark.parametrize("field", AllTypesSchema.model_fields.keys())
def test_bad_type(field: str):
    match field:
        case "base64_bytes":
            data = {"base64_bytes": 42}
        case "organization_id":
            data = {"organization_id": 42}
        case "user_id":
            data = {"user_id": 42}
        case "device_id":
            data = {"device_id": 42}
        case "sequester_service_id":
            data = {"sequester_service_id": 42}
        case "invitation_token":
            data = {"invitation_token": 42}
        case "greeting_attempt_id":
            data = {"greeting_attempt_id": 42}
        case "invitation_status":
            data = {"invitation_status": 42}
        case "realm_role":
            data = {"realm_role": 42}
        case "vlob_id":
            data = {"vlob_id": 42}
        case "date_time":
            data = {"date_time": 42}
        case "user_profile":
            data = {"user_profile": 42}
        case "active_users_limit":
            data = {"active_users_limit": ""}
        case "email_address":
            data = {"email_address": 42}
        case unknown:
            assert False, unknown

    with pytest.raises(ValidationError):
        AllTypesSchema(**data)  # type: ignore


@pytest.mark.parametrize("field", AllTypesSchema.model_fields.keys())
def test_bad_value(field: str):
    match field:
        case "base64_bytes":
            data = {"base64_bytes": "<>"}
        case "organization_id":
            data = {"organization_id": "<>"}
        case "user_id":
            data = {"user_id": "<>"}
        case "device_id":
            data = {"device_id": "<>"}
        case "sequester_service_id":
            data = {"sequester_service_id": "<>"}
        case "invitation_token":
            data = {"invitation_token": "<>"}
        case "greeting_attempt_id":
            data = {"greeting_attempt_id": "<>"}
        case "invitation_status":
            data = {"invitation_status": "<>"}
        case "realm_role":
            data = {"realm_role": "<>"}
        case "vlob_id":
            data = {"vlob_id": "<>"}
        case "date_time":
            data = {"date_time": "<>"}
        case "user_profile":
            data = {"user_profile": "<>"}
        case "active_users_limit":
            data = {"active_users_limit": -1}
        case "email_address":
            data = {"email_address": "<>"}
        case unknown:
            assert False, unknown

    with pytest.raises(ValidationError):
        AllTypesSchema(**data)  # type: ignore


def test_active_users_limit_out_of_bounds():
    for bad in (-1, 1 << 65):
        try:
            ActiveUsersLimit.limited_to(bad)
        except ValueError:
            pass
