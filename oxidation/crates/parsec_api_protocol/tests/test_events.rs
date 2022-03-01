// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use hex_literal::hex;
use rstest::rstest;

use parsec_api_protocol::*;
use parsec_api_types::RealmRole;

#[rstest]
fn serde_events_listen_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "events_listen"
    //   wait: false
    let data = hex!("82a3636d64ad6576656e74735f6c697374656ea477616974c2");

    let expected = EventsListenReqSchema {
        cmd: "events_listen".to_owned(),
        wait: false,
    };

    let schema = EventsListenReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = EventsListenReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
#[case::pinged(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   event: "pinged"
        //   ping: "foobar"
        //   status: "ok"
        &hex!(
            "83a56576656e74a670696e676564a470696e67a6666f6f626172a6737461747573a26f6b"
        )[..],
        EventsListenRepSchema(APIEvent::Pinged(EventsPingedRepSchema {
            status: Status::Ok,
            ping: "foobar".to_owned(),
        }))
    )
)]
#[case::message_received(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   event: "message.received"
        //   index: 0
        //   status: "ok"
        &hex!(
            "83a56576656e74b06d6573736167652e7265636569766564a5696e64657800a67374617475"
            "73a26f6b"
        )[..],
        EventsListenRepSchema(APIEvent::MessageReceived(EventsMessageReceivedRepSchema {
            status: Status::Ok,
            index: 0,
        }))
    )
)]
#[case::invite_status_changed(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   event: "invite.status_changed"
        //   invitation_status: "IDLE"
        //   status: "ok"
        //   token: ext(2, hex!("d864b93ded264aae9ae583fd3d40c45a"))
        &hex!(
            "84a56576656e74b5696e766974652e7374617475735f6368616e676564b1696e7669746174"
            "696f6e5f737461747573a449444c45a6737461747573a26f6ba5746f6b656ed802d864b93d"
            "ed264aae9ae583fd3d40c45a"
        )[..],
        EventsListenRepSchema(APIEvent::InviteStatusChanged(
            EventsInviteStatusChangedRepSchema {
                status: Status::Ok,
                invitation_status: InvitationStatus::Idle,
                token: "d864b93ded264aae9ae583fd3d40c45a".parse().unwrap(),
            },
        ))
    )
)]
#[case::realm_maintenance_finished(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   encryption_revision: 0
        //   event: "realm.maintenance_finished"
        //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
        //   status: "ok"
        &hex!(
            "84b3656e6372797074696f6e5f7265766973696f6e00a56576656e74ba7265616c6d2e6d61"
            "696e74656e616e63655f66696e6973686564a87265616c6d5f6964d8021d3353157d7d4e95"
            "ad2fdea7b3bd19c5a6737461747573a26f6b"
        )[..],
        EventsListenRepSchema(APIEvent::RealmMaintenanceFinished(
            EventsRealmMaintenanceFinishedRepSchema {
                status: Status::Ok,
                realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
                encryption_revision: 0,
            }
        ))
    )
)]
#[case::realm_maintenance_started(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   encryption_revision: 0
        //   event: "realm.maintenance_started"
        //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
        //   status: "ok"
        &hex!(
            "84b3656e6372797074696f6e5f7265766973696f6e00a56576656e74b97265616c6d2e6d61"
            "696e74656e616e63655f73746172746564a87265616c6d5f6964d8021d3353157d7d4e95ad"
            "2fdea7b3bd19c5a6737461747573a26f6b"
        )[..],
        EventsListenRepSchema(APIEvent::RealmMaintenanceStarted(
            EventsRealmMaintenanceStartedRepSchema {
                status: Status::Ok,
                realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
                encryption_revision: 0,
            }
        ))
    )
)]
#[case::realm_vlobs_updated(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   checkpoint: 0
        //   event: "realm.vlobs_updated"
        //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
        //   src_id: ext(2, hex!("2b5f314728134a12863da1ce49c112f6"))
        //   src_version: 0
        //   status: "ok"
        &hex!(
            "86aa636865636b706f696e7400a56576656e74b37265616c6d2e766c6f62735f7570646174"
            "6564a87265616c6d5f6964d8021d3353157d7d4e95ad2fdea7b3bd19c5a67372635f6964d8"
            "022b5f314728134a12863da1ce49c112f6ab7372635f76657273696f6e00a6737461747573"
            "a26f6b"
        )[..],
        EventsListenRepSchema(APIEvent::RealmVlobsUpdated(
            EventsRealmVlobsUpdatedRepSchema {
                status: Status::Ok,
                realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
                checkpoint: 0,
                src_id: "2b5f314728134a12863da1ce49c112f6".parse().unwrap(),
                src_version: 0,
            }
        ))
    )
)]
#[case::realm_roles_updated(
    (
        // Generated from Python implementation (Parsec v2.6.0+dev)
        // Content:
        //   event: "realm.roles_updated"
        //   realm_id: ext(2, hex!("1d3353157d7d4e95ad2fdea7b3bd19c5"))
        //   role: "OWNER"
        //   status: "ok"
        &hex!(
            "84a56576656e74b37265616c6d2e726f6c65735f75706461746564a87265616c6d5f6964d8"
            "021d3353157d7d4e95ad2fdea7b3bd19c5a4726f6c65a54f574e4552a6737461747573a26f"
            "6b"
        )[..],
        EventsListenRepSchema(APIEvent::RealmRolesUdpated(
            EventsRealmRolesUpdatedRepSchema {
                status: Status::Ok,
                realm_id: "1d3353157d7d4e95ad2fdea7b3bd19c5".parse().unwrap(),
                role: RealmRole::Owner,
            }
        ))
    )
)]
fn serde_events_listen_rep(#[case] data_expected: (&[u8], EventsListenRepSchema)) {
    let (data, expected) = data_expected;

    let schema = EventsListenRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = EventsListenRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_events_subscribe_req() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   cmd: "events_subscribe"
    let data = hex!("81a3636d64b06576656e74735f737562736372696265");

    let expected = EventsSubscribeReqSchema {
        cmd: "events_subscribe".to_owned(),
    };

    let schema = EventsSubscribeReqSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = EventsSubscribeReqSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}

#[rstest]
fn serde_events_subscribe_rep() {
    // Generated from Python implementation (Parsec v2.6.0+dev)
    // Content:
    //   status: "ok"
    let data = hex!("81a6737461747573a26f6b");

    let expected = EventsSubscribeRepSchema { status: Status::Ok };

    let schema = EventsSubscribeRepSchema::load(&data).unwrap();

    assert_eq!(schema, expected);

    // Also test serialization round trip
    let data2 = schema.dump();
    let schema2 = EventsSubscribeRepSchema::load(&data2).unwrap();

    assert_eq!(schema2, expected);
}
