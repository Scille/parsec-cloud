# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

# flake8: noqa

from protocol.utils import *

from parsec.api.protocol import *
from parsec.api.data import *


iud = InviteUserData(
    requested_device_label=BOB.device_label,
    requested_human_handle=BOB.human_handle,
    public_key=BOB.public_key,
    verify_key=BOB.verify_key,
).dump_and_encrypt(key=KEY)
display("invite user data", iud, [KEY, "zip"])

iud = InviteUserData(
    requested_device_label=None,
    requested_human_handle=BOB.human_handle,
    public_key=BOB.public_key,
    verify_key=BOB.verify_key,
).dump_and_encrypt(key=KEY)
display("invite user data no device label", iud, [KEY, "zip"])

iud = InviteUserData(
    requested_device_label=BOB.device_label,
    requested_human_handle=None,
    public_key=BOB.public_key,
    verify_key=BOB.verify_key,
).dump_and_encrypt(key=KEY)
display("invite user data no human handle", iud, [KEY, "zip"])

iud = InviteUserData(
    requested_device_label=None,
    requested_human_handle=None,
    public_key=BOB.public_key,
    verify_key=BOB.verify_key,
).dump_and_encrypt(key=KEY)
display("invite user data no human handle no device label", iud, [KEY, "zip"])

iuc = InviteUserConfirmation(
    device_id=BOB.device_id,
    device_label=BOB.device_label,
    human_handle=BOB.human_handle,
    profile=BOB.profile,
    root_verify_key=BOB.root_verify_key,
).dump_and_encrypt(key=KEY)
display("invite user confirmation", iuc, [KEY, "zip"])

iuc = InviteUserConfirmation(
    device_id=BOB.device_id,
    device_label=None,
    human_handle=BOB.human_handle,
    profile=BOB.profile,
    root_verify_key=BOB.root_verify_key,
).dump_and_encrypt(key=KEY)
display("invite user confirmation no device_label", iuc, [KEY, "zip"])

iuc = InviteUserConfirmation(
    device_id=BOB.device_id,
    device_label=BOB.device_label,
    human_handle=None,
    profile=BOB.profile,
    root_verify_key=BOB.root_verify_key,
).dump_and_encrypt(key=KEY)
display("invite user confirmation no human_handle", iuc, [KEY, "zip"])

iuc = InviteUserConfirmation(
    device_id=BOB.device_id,
    device_label=None,
    human_handle=None,
    profile=BOB.profile,
    root_verify_key=BOB.root_verify_key,
).dump_and_encrypt(key=KEY)
display("invite user confirmation no device_label/human_handle", iuc, [KEY, "zip"])

#################

idd = InviteDeviceData(
    requested_device_label=BOB.device_label, verify_key=BOB.verify_key
).dump_and_encrypt(key=KEY)
display("invite device data", idd, [KEY, "zip"])

idd = InviteDeviceData(requested_device_label=None, verify_key=BOB.verify_key).dump_and_encrypt(
    key=KEY
)
display("invite device data no device label", idd, [KEY, "zip"])

idc = InviteDeviceConfirmation(
    device_id=BOB.device_id,
    device_label=BOB.device_label,
    human_handle=BOB.human_handle,
    profile=BOB.profile,
    private_key=BOB.private_key,
    user_manifest_id=BOB.user_manifest_id,
    user_manifest_key=BOB.user_manifest_key,
    root_verify_key=BOB.root_verify_key,
).dump_and_encrypt(key=KEY)
display("invite device confirmation", idc, [KEY, "zip"])

idc = InviteDeviceConfirmation(
    device_id=BOB.device_id,
    device_label=None,
    human_handle=BOB.human_handle,
    profile=BOB.profile,
    private_key=BOB.private_key,
    user_manifest_id=BOB.user_manifest_id,
    user_manifest_key=BOB.user_manifest_key,
    root_verify_key=BOB.root_verify_key,
).dump_and_encrypt(key=KEY)
display("invite device confirmation no device_label", idc, [KEY, "zip"])

idc = InviteDeviceConfirmation(
    device_id=BOB.device_id,
    device_label=BOB.device_label,
    human_handle=None,
    profile=BOB.profile,
    private_key=BOB.private_key,
    user_manifest_id=BOB.user_manifest_id,
    user_manifest_key=BOB.user_manifest_key,
    root_verify_key=BOB.root_verify_key,
).dump_and_encrypt(key=KEY)
display("invite device confirmation no human_handle", idc, [KEY, "zip"])

idc = InviteDeviceConfirmation(
    device_id=BOB.device_id,
    device_label=None,
    human_handle=None,
    profile=BOB.profile,
    private_key=BOB.private_key,
    user_manifest_id=BOB.user_manifest_id,
    user_manifest_key=BOB.user_manifest_key,
    root_verify_key=BOB.root_verify_key,
).dump_and_encrypt(key=KEY)
display("invite device confirmation no device_label/human_handle", idc, [KEY, "zip"])
