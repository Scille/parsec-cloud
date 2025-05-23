// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

// `allow-unwrap-in-test` don't behave as expected, see:
// https://github.com/rust-lang/rust-clippy/issues/11119
#![allow(clippy::unwrap_used)]

use std::collections::HashMap;

use pretty_assertions::assert_eq;

use libparsec_serialization_format::generate_protocol_cmds_family_from_contents;

#[path = "./common/libparsec_types_mock.rs"]
mod libparsec_types;

#[test]
fn simple() {
    generate_protocol_cmds_family_from_contents!(
        r#"[
            {
                "major_versions": [
                    1,
                    2,
                    3
                ],
                "cmd": "ping",
                "req": {
                    "fields": [
                        {
                            "name": "ping",
                            "type": "String"
                        }
                    ]
                },
                "reps": [
                    {
                        "status": "ok",
                        "fields": [
                            {
                                "name": "pong",
                                "type": "String"
                            }
                        ]
                    }
                ]
            }
        ]"#
    );

    // Check v1/v2/v3 use the same structure (this won't compile if not)

    assert_eq!(
        family_cmds::v1::ping::Req {
            ping: "foo".to_owned()
        },
        family_cmds::v2::ping::Req {
            ping: "foo".to_owned()
        }
    );
    assert_eq!(
        family_cmds::v1::ping::Req {
            ping: "foo".to_owned()
        },
        family_cmds::v3::ping::Req {
            ping: "foo".to_owned()
        }
    );

    // Check round-trip serialize/deserialize

    let req = family_cmds::v2::ping::Req {
        ping: "foo".to_owned(),
    };
    let dumped = req.dump().unwrap();
    assert_eq!(
        family_cmds::v2::AnyCmdReq::load(&dumped).unwrap(),
        family_cmds::v2::AnyCmdReq::Ping(req)
    );

    let rep = family_cmds::v2::ping::Rep::Ok {
        pong: "foo".to_owned(),
    };
    let dumped = rep.dump().unwrap();
    assert_eq!(
        family_cmds::v2::ping::Rep::load(&dumped).unwrap(),
        family_cmds::v2::ping::Rep::Ok {
            pong: "foo".to_owned()
        }
    );
}

#[test]
fn complex_type() {
    generate_protocol_cmds_family_from_contents!(
        r#"[
            {
                "major_versions": [1],
                "cmd": "ping",
                "req": {
                    "fields": [
                        {
                            "name": "ping",
                            "type": "Map<Integer, (DeviceID, Boolean)>"
                        }
                    ]
                },
                "reps": [
                    {
                        "status": "ok",
                        "fields": [
                            {
                                "name": "pong",
                                "type": "NonRequiredOption<Integer>"
                            }
                        ]
                    }
                ]
            }
        ]"#
    );

    // Check round-trip serialize/deserialize

    let req = family_cmds::v1::ping::Req {
        ping: HashMap::from([(1, (libparsec_types::DeviceID("alice@pc1".to_owned()), true))]),
    };
    let dumped = req.dump().unwrap();
    assert_eq!(
        family_cmds::v1::AnyCmdReq::load(&dumped).unwrap(),
        family_cmds::v1::AnyCmdReq::Ping(req)
    );
    let rep = family_cmds::v1::ping::Rep::Ok { pong: Some(1) };
    let dumped = rep.dump().unwrap();
    assert_eq!(family_cmds::v1::ping::Rep::load(&dumped).unwrap(), rep,);
}

#[test]
fn unknown_rep_status() {
    generate_protocol_cmds_family_from_contents!(
        r#"[
            {
                "major_versions": [1],
                "cmd": "ping",
                "req": {
                    "fields": []
                },
                "reps": [
                    {
                        "status": "ok",
                        "fields": [
                            {
                                "name": "pong",
                                "type": "String"
                            }
                        ]
                    }
                ]
            }
        ]

        [
            {
                "major_versions": [1],
                "cmd": "ping2",
                "req": {
                    "fields": []
                },
                "reps": [
                    {
                        "status": "ok",
                        "fields": [
                            {
                                "name": "dummy",
                                "type": "String"
                            }
                        ]
                    },
                    {
                        "status": "dummy",
                        "fields": [
                            {
                                "name": "pong",
                                "type": "String"
                            }
                        ]
                    }
                ]
            }
        ]"#
    );

    let unknown_status = family_cmds::v1::ping2::Rep::Dummy {
        pong: "foo".to_owned(),
    }
    .dump()
    .unwrap();
    let known_status_but_bad_content = family_cmds::v1::ping2::Rep::Ok {
        dummy: "foo".to_owned(),
    }
    .dump()
    .unwrap();

    assert_eq!(
        family_cmds::v1::ping::Rep::load(&unknown_status).unwrap(),
        family_cmds::v1::ping::Rep::UnknownStatus {
            unknown_status: "dummy".to_owned(),
            reason: None
        }
    );
    assert!(family_cmds::v1::ping::Rep::load(&known_status_but_bad_content).is_err());
}

#[test]
fn introduce_in_field() {
    generate_protocol_cmds_family_from_contents!(
        r#"[
            {
                "major_versions": [
                    1,
                    2,
                    3
                ],
                "cmd": "ping",
                "req": {
                    "fields": [
                        {
                            "name": "ping",
                            "type": "String",
                            "introduced_in": "2.1"
                        }
                    ]
                },
                "reps": []
            }
        ]

        [
            {
                "major_versions": [
                    1,
                    2,
                    3
                ],
                "cmd": "ping2",
                "req": {
                    "fields": []
                },
                "reps": [
                    {
                        "status": "ok",
                        "fields": [
                            {
                                "name": "pong",
                                "type": "Boolean",
                                "introduced_in": "2.1"
                            }
                        ]
                    }
                ]
            }
        ]"#
    );

    // Test Req

    let v1 = family_cmds::v1::ping::Req {};
    let v2_with = family_cmds::v2::ping::Req {
        ping: libparsec_types::Maybe::Present("foo".to_owned()),
    };
    let v2_without = family_cmds::v2::ping::Req {
        ping: libparsec_types::Maybe::Absent,
    };
    let v3 = family_cmds::v3::ping::Req {
        ping: "foo".to_owned(),
    };

    let v1_dumped = v1.dump().unwrap();
    let v2_with_dumped = v2_with.dump().unwrap();
    // Field is optional in v2...
    assert_eq!(
        family_cmds::v2::AnyCmdReq::load(&v1_dumped).unwrap(),
        family_cmds::v2::AnyCmdReq::Ping(v2_without)
    );
    // ...and becomes required in v3
    assert!(family_cmds::v3::AnyCmdReq::load(&v1_dumped).is_err());
    assert_eq!(
        family_cmds::v3::AnyCmdReq::load(&v2_with_dumped).unwrap(),
        family_cmds::v3::AnyCmdReq::Ping(v3)
    );

    // Test Rep

    let v1 = family_cmds::v1::ping2::Rep::Ok {};
    let v2_with = family_cmds::v2::ping2::Rep::Ok {
        pong: libparsec_types::Maybe::Present(true),
    };
    let v2_without = family_cmds::v2::ping2::Rep::Ok {
        pong: libparsec_types::Maybe::Absent,
    };
    let v3 = family_cmds::v3::ping2::Rep::Ok { pong: true };

    let v1_dumped = v1.dump().unwrap();
    let v2_with_dumped = v2_with.dump().unwrap();
    // Field is optional in v2...
    assert_eq!(
        family_cmds::v2::ping2::Rep::load(&v1_dumped).unwrap(),
        v2_without
    );
    // ...and becomes required in v3
    assert!(family_cmds::v3::ping2::Rep::load(&v1_dumped).is_err());
    assert_eq!(
        family_cmds::v3::ping2::Rep::load(&v2_with_dumped).unwrap(),
        v3
    );
}

#[test]
fn nested_type() {
    generate_protocol_cmds_family_from_contents!(
        r#"[
            {
                "major_versions": [1],
                "cmd": "ping",
                "req": {
                    "fields": [
                        {"name": "e", "type": "EnumNestedType"},
                        {"name": "s", "type": "StructNestedType"}
                    ]
                },
                "reps": [],
                "nested_types": [
                    {
                        "name": "EnumNestedType",
                        "discriminant_field": "discriminant",
                        "variants": [
                            {
                                "name": "E1",
                                "discriminant_value": "e1",
                                "fields": [
                                    {
                                        "name": "f",
                                        "type": "Integer"
                                    }
                                ]
                            },
                            {
                                "name": "E2",
                                "discriminant_value": "e2",
                                "fields": []
                            }
                        ]
                    },
                    {
                        "name": "StructNestedType",
                        "fields": [
                            {
                                "name": "f",
                                "type": "Integer"
                            }
                        ]
                    }
                ]
            }
        ]"#
    );

    // Check round-trip serialize/deserialize

    let req = family_cmds::v1::ping::Req {
        e: family_cmds::v1::ping::EnumNestedType::E1 { f: 42 },
        s: family_cmds::v1::ping::StructNestedType { f: 42 },
    };
    let dumped = req.dump().unwrap();
    assert_eq!(
        family_cmds::v1::AnyCmdReq::load(&dumped).unwrap(),
        family_cmds::v1::AnyCmdReq::Ping(req)
    );
}

#[test]
fn rep_unit() {
    generate_protocol_cmds_family_from_contents!(
        r#"[
            {
                "major_versions": [1],
                "cmd": "ping",
                "req": {
                    "fields": []
                },
                "reps": [
                    {
                        "status": "ok_enum",
                        "unit": "EnumNestedType"
                    },
                    {
                        "status": "ok_struct",
                        "unit": "StructNestedType"
                    }
                ],
                "nested_types": [
                    {
                        "name": "EnumNestedType",
                        "discriminant_field": "discriminant",
                        "variants": [
                            {
                                "name": "E1",
                                "discriminant_value": "yyy",
                                "fields": [
                                    {
                                        "name": "f",
                                        "type": "Integer"
                                    }
                                ]
                            },
                            {
                                "name": "E2",
                                "discriminant_value": "xxxx",
                                "fields": []
                            }
                        ]
                    },
                    {
                        "name": "StructNestedType",
                        "fields": [
                            {
                                "name": "f",
                                "type": "Integer"
                            }
                        ]
                    }
                ]
            }
        ]"#
    );

    // Check round-trip serialize/deserialize

    let nested_enum = family_cmds::v1::ping::EnumNestedType::E2 {};
    let nested_struct = family_cmds::v1::ping::StructNestedType { f: 42 };
    let ok_enum = family_cmds::v1::ping::Rep::OkEnum(nested_enum);
    let ok_struct = family_cmds::v1::ping::Rep::OkStruct(nested_struct);

    let ok_enum_dump = ok_enum.dump().unwrap();
    let ok_struct_dump = ok_struct.dump().unwrap();

    assert_eq!(
        family_cmds::v1::ping::Rep::load(&ok_enum_dump).unwrap(),
        ok_enum
    );
    assert_eq!(
        family_cmds::v1::ping::Rep::load(&ok_struct_dump).unwrap(),
        ok_struct
    );
}
