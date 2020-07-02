# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import re


class Q:
    """
    Dead simple SQL query composition framework (◠﹏◠)
    """

    def __init__(self, src, **kwargs):
        # retrieve variables
        variables = {}
        for candidate in re.findall(r"\$([a-zA-Z0-9_]+)", src):
            if candidate in variables:
                continue
            if candidate.isdigit():
                raise ValueError(f"Invalid variable name `{candidate}`")
            variables[candidate] = f"${len(variables) + 1}"
        self._variables = variables

        # Replace variables with their order-based equivalent
        for variable, order_based in variables.items():
            src = src.replace(f"${variable}", order_based)

        self._sql = src
        self._stripped_sql = " ".join([x.strip() for x in src.split()])

    @property
    def sql(self):
        return self._sql

    def __call__(self, **kwargs):
        if kwargs.keys() != self._variables.keys():
            missing = self._variables.keys() - kwargs.keys()
            unknown = kwargs.keys() - self._variables.keys()
            raise ValueError(f"Invalid paramaters, missing: {missing}, unknown: {unknown}")
        args = [self._stripped_sql]
        for variable in self._variables:
            args.append(kwargs[variable])
        return args


def q_organization(organization_id=None, _id=None, table="organization", select="*"):
    assert organization_id is not None or _id is not None
    if _id is not None:
        condition = f"{table}._id = {_id}"
    else:
        condition = f"{table}.organization_id = {organization_id}"
    return f"(SELECT {select} FROM {table} WHERE {condition})"


def q_organization_internal_id(organization_id, **kwargs):
    return q_organization(organization_id=organization_id, select="_id", **kwargs)


def _table_q_factory(table, public_id_field):
    def _q(
        organization_id=None,
        organization=None,
        _id=None,
        table_alias=None,
        select="*",
        suffix=None,
        **kwargs,
    ):
        if table_alias:
            from_table = f"{table} as {table_alias}"
            select_table = table_alias
        else:
            from_table = table
            select_table = table
        if _id is not None:
            condition = f"{select_table}._id = {_id}"
        else:
            public_id = kwargs.pop(public_id_field, None)
            assert public_id is not None
            assert organization_id is not None or organization is not None
            if not organization:
                organization = q_organization_internal_id(organization_id)
            condition = f"{select_table}.organization = {organization} AND {select_table}.{ public_id_field } = { public_id }"
        assert not kwargs
        suffix = suffix or ""
        return f"(SELECT {select} FROM {from_table} WHERE {condition} {suffix})"

    def _q_internal_id(**kwargs):
        return _q(select="_id", **kwargs)

    _q.__name__ = f"q_{table}"
    _q_internal_id.__name__ = f"q_{table}_internal_id"

    return _q, _q_internal_id


q_device, q_device_internal_id = _table_q_factory("device", "device_id")
q_user, q_user_internal_id = _table_q_factory("user_", "user_id")
q_realm, q_realm_internal_id = _table_q_factory("realm", "realm_id")
q_block, q_block_internal_id = _table_q_factory("block", "block_id")
q_vlob, q_vlob_internal_id = _table_q_factory("vlob", "vlob_id")
q_human, q_human_internal_id = _table_q_factory("human", "email")


def q_user_can_read_vlob(
    user=None,
    user_id=None,
    realm=None,
    realm_id=None,
    organization=None,
    organization_id=None,
    table="realm_user_role",
):
    if user is None:
        assert organization_id is not None and user_id is not None
        _q_user = q_user_internal_id(
            organization=organization, organization_id=organization_id, user_id=user_id
        )
    else:
        _q_user = user

    if realm is None:
        assert organization_id is not None and realm_id is not None
        _q_realm = q_realm_internal_id(
            organization=organization, organization_id=organization_id, realm_id=realm_id
        )
    else:
        _q_realm = realm

    return f"""
COALESCE(
    (
        SELECT { table }.role IS NOT NULL
        FROM { table }
        WHERE
            { table }.realm = { _q_realm }
            AND { table }.user_ = { _q_user }
        ORDER BY certified_on DESC
        LIMIT 1
    ),
    False
)
"""


def q_user_can_write_vlob(
    user=None,
    user_id=None,
    realm=None,
    realm_id=None,
    organization=None,
    organization_id=None,
    table="realm_user_role",
):
    if user is None:
        assert organization_id is not None and user_id is not None
        _q_user = q_user_internal_id(
            organization=organization, organization_id=organization_id, user_id=user_id
        )
    else:
        _q_user = user

    if realm is None:
        assert organization_id is not None and realm_id is not None
        _q_realm = q_realm_internal_id(
            organization=organization, organization_id=organization_id, realm_id=realm_id
        )
    else:
        _q_realm = realm

    return f"""
COALESCE(
    (
        SELECT { table }.role IN ('CONTRIBUTOR', 'MANAGER', 'OWNER')
        FROM { table }
        WHERE
            { table }.realm = { _q_realm }
            AND { table }.user_ = { _q_user }
        ORDER BY certified_on DESC
        LIMIT 1
    ),
    False
)
"""
