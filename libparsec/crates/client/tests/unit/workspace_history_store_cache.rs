// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_tests_fixtures::prelude::*;
use libparsec_types::prelude::*;

use super::{
    CachePopulateManifestAtError, CachePopulateManifestEntry, CacheResolvedEntry,
    InvalidManifestHistoryError, WorkspaceHistoryStoreCache,
};

#[parsec_test]
fn block() {
    let mut cache = WorkspaceHistoryStoreCache::default();

    let block_id = BlockID::default();
    let block_data = Bytes::from_static(b"data");

    p_assert_eq!(cache.get_block(block_id), None);

    cache.populate_block(block_id, block_data.clone());
    p_assert_eq!(cache.get_block(block_id), Some(block_data));

    let overwrite_block_data = Bytes::from_static(b"data");
    cache.populate_block(block_id, overwrite_block_data.clone());
    p_assert_eq!(cache.get_block(block_id), Some(overwrite_block_data));
}

#[derive(Debug)]
struct Resolve {
    at: DateTime,
    entry_id: VlobID,
    resolution: CacheResolvedEntry,
}

impl std::str::FromStr for Resolve {
    type Err = &'static str;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        const BAD_FORMAT_MSG: &str =
            "Expected format `<at>:[<ID*>@]CM|NF|(<v*>@<timestamp>)`, ex: `t1:V2@t2`, `t2:ID1@CM`";

        let parse_prefix_and_u8 = |raw: &str, expected_prefix: &str| -> Option<u8> {
            if !raw.starts_with(expected_prefix) {
                return None;
            }
            raw[expected_prefix.len()..].parse().ok()
        };

        let parse_timestamp = |raw: &str| -> Result<DateTime, &'static str> {
            let timestamp_index = parse_prefix_and_u8(raw, "t").ok_or(BAD_FORMAT_MSG)?;
            let timestamp = DateTime::from_timestamp_seconds(
                DateTime::from_rfc3339("2000-01-01T00:00:00Z")
                    .unwrap()
                    .as_timestamp_seconds()
                    + timestamp_index as i64,
            )
            .unwrap();
            Ok(timestamp)
        };

        // Parse `at`

        let (at, raw) = match raw.split_once(':') {
            Some((at_raw, remain_raw)) => {
                let at = parse_timestamp(at_raw)?;
                (at, remain_raw)
            }
            None => return Err(BAD_FORMAT_MSG),
        };

        // Parse optional `entry_id` or default to `ID0`

        let (entry_id_index, raw) = match raw.split_once('@') {
            Some((maybe_id, remain)) => match parse_prefix_and_u8(maybe_id, "ID") {
                Some(entry_id_index) => (entry_id_index, remain),
                None => (0, raw),
            },
            None => (0, raw),
        };
        let entry_id = VlobID::from([entry_id_index; 16]);

        // Parse resolution object

        let resolution = match raw {
            "NF" => CacheResolvedEntry::NotFound,
            "CM" => CacheResolvedEntry::CacheMiss,

            // e.g. `V1@t2`
            _ => match raw.split_once('@') {
                Some((raw_version, raw_timestamp)) => {
                    let timestamp = parse_timestamp(raw_timestamp)?;
                    let version = match parse_prefix_and_u8(raw_version, "v") {
                        Some(version) => version as VersionInt,
                        None => return Err(BAD_FORMAT_MSG),
                    };
                    let manifest = ArcChildManifest::File(Arc::new(FileManifest {
                        author: "alice@dev1".parse().unwrap(),
                        timestamp,
                        id: entry_id,
                        parent: VlobID::from_hex("aa00000000000000000000000000f11e").unwrap(),
                        version,
                        created: timestamp,
                        updated: timestamp,
                        size: 0,
                        blocksize: 512.try_into().unwrap(),
                        blocks: vec![],
                    }));

                    CacheResolvedEntry::Exists(manifest)
                }
                None => return Err(BAD_FORMAT_MSG),
            },
        };

        Ok(Resolve {
            at,
            entry_id,
            resolution,
        })
    }
}

#[derive(Debug)]
struct Populate {
    at: DateTime,
    populate: CachePopulateManifestEntry,
}

impl std::str::FromStr for Populate {
    type Err = &'static str;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        const BAD_FORMAT_MSG: &str =
            "Expected format `<at>:[<ID*>@]NF|(<v*>@<timestamp>)`, ex: `t1:V2@t2`, `t2:ID1@NF`";

        // Re-use `Resolve`` parsing since the format of `Populate` is the same (except it doesn't have `CM`)
        match raw.parse::<Resolve>() {
            Ok(resolve) => match resolve.resolution {
                CacheResolvedEntry::Exists(manifest) => Ok(Populate {
                    at: resolve.at,
                    populate: CachePopulateManifestEntry::Exists(manifest),
                }),
                CacheResolvedEntry::NotFound => Ok(Populate {
                    at: resolve.at,
                    populate: CachePopulateManifestEntry::NotFound(resolve.entry_id),
                }),
                CacheResolvedEntry::CacheMiss => Err(BAD_FORMAT_MSG),
            },
            Err(_) => Err(BAD_FORMAT_MSG),
        }
    }
}

enum Outcome {
    Ok {
        expected_resolves: Vec<Resolve>,
    },
    Ko {
        check_error: Box<dyn Fn(&InvalidManifestHistoryError)>,
    },
}

impl std::str::FromStr for Outcome {
    type Err = &'static str;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        const BAD_FORMAT_MSG: &str = "Expected format `Ok=>(<lookup>,)*` or `Ko=>ErrorType`, ex: `Ok=>t2:V1@t1,t3:CM`, `Ko=>TooRecent`";

        match raw.split_once("=>") {
            Some(("Ok", remain_raw)) => {
                let expected_resolves = remain_raw
                    .split(',')
                    .map(|raw| raw.parse::<Resolve>())
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Outcome::Ok { expected_resolves })
            }
            Some(("Ko", error_type)) => {
                let check_error = match error_type {
                    "TooRecent" => |e: &InvalidManifestHistoryError| {
                        p_assert_matches!(e, InvalidManifestHistoryError::TooRecent { .. });
                    },
                    "AlreadyKnownToExist" => |e: &InvalidManifestHistoryError| {
                        p_assert_matches!(
                            e,
                            InvalidManifestHistoryError::AlreadyKnownToExist { .. }
                        );
                    },
                    "AlreadyKnownToBeNotFound" => |e: &InvalidManifestHistoryError| {
                        p_assert_matches!(
                            e,
                            InvalidManifestHistoryError::AlreadyKnownToBeNotFound { .. }
                        );
                    },
                    "AlreadyKnownAndDiffers" => |e: &InvalidManifestHistoryError| {
                        p_assert_matches!(
                            e,
                            InvalidManifestHistoryError::AlreadyKnownAndDiffers { .. }
                        );
                    },
                    "AlreadyKnownToHaveMoreRecentSmallerVersion" => {
                        |e: &InvalidManifestHistoryError| {
                            p_assert_matches!(e, InvalidManifestHistoryError::AlreadyKnownToHaveMoreRecentSmallerVersion { .. });
                        }
                    }
                    _ => return Err("Ko error type is not part of `InvalidManifestHistoryError`"),
                };
                Ok(Outcome::Ko {
                    check_error: Box::new(check_error),
                })
            }
            _ => Err(BAD_FORMAT_MSG),
        }
    }
}

#[parsec_test]
// "THE NUMBERS MASON, WHAT DO THEY MEAN ????"
//
// Each test is divided into three parts:
// - A list of existing populates
// - A single new populate (this is the thing we are testing !)
// - An expected outcome
//
// Each populate is described as a string in the following format:
// - `t0:v1@t1` stands for `populate_manifest_at(t0, Exists { manifest: { version: 1, timestamp: t1} })`
// - `t0:NF` stands for `populate_manifest_at(t0, NotFound)`
//
// The outcome can be either Ok or Ko:
// - For Ok it is followed by a list of expected resolves in the cache
//   e.g. `Ok=>t0:NF,t2:CM` for `resolve_manifest_at(t0) == NotFound` and `resolve_manifest_at(t2) == CacheMiss`
// - For Ko it is followed by the expected error type, e.g. `Ko=>TooRecent`

// New populate provides an already known info
#[case(vec!["t1:NF"], "t1:NF", "Ok=>t0:NF,t1:NF,t2:CM")]
#[case(vec!["t1:v1@t1"], "t1:v1@t1", "Ok=>t0:NF,t1:v1@t1,t2:CM")]
#[case(vec!["t1:v2@t1"], "t1:v2@t1", "Ok=>t0:CM,t1:v2@t1,t2:CM")]
#[case(vec!["t2:v1@t2"], "t0:NF", "Ok=>t0:NF,t1:NF,t2:v1@t2,t3:CM")]
#[case(vec!["t3:v1@t1"], "t2:v1@t1", "Ok=>t0:NF,t1:v1@t1,t2:v1@t1,t3:v1@t1,t4:CM")]
// New populate create a new info without updating existing ones
#[case(vec![], "t2:v1@t1", "Ok=>t0:NF,t1:v1@t1,t2:v1@t1,t3:CM")]
#[case(vec![], "t2:v2@t1", "Ok=>t0:CM,t1:v2@t1,t2:v2@t1,t3:CM")]
#[case(vec!["t1:v1@t1"], "t3:v3@t3", "Ok=>t1:v1@t1,t2:CM,t3:v3@t3")]
// New populate update an existing info
#[case(vec!["t1:NF"], "t2:NF", "Ok=>t0:NF,t1:NF,t2:NF,t3:CM")]
#[case(vec!["t1:v1@t1"], "t3:v1@t1", "Ok=>t0:NF,t1:v1@t1,t2:v1@t1,t3:v1@t1,t5:CM")]
#[case(vec!["t1:v1@t1"], "t3:v2@t3", "Ok=>t0:NF,t1:v1@t1,t2:v1@t1,t3:v2@t3,t4:CM")]
#[case(vec!["t3:v2@t3"], "t1:v1@t1", "Ok=>t0:NF,t1:v1@t1,t2:v1@t1,t3:v2@t3,t4:CM")]
// New populate conflicts with existing info
#[case(vec![], "t1:v1@t2", "Ko=>TooRecent")]
#[case(vec!["t1:v2@t1"], "t1:NF", "Ko=>AlreadyKnownToExist")]
#[case(vec!["t3:v2@t1"], "t2:NF", "Ko=>AlreadyKnownToExist")]
#[case(vec!["t1:NF"], "t0:v2@t0", "Ko=>AlreadyKnownToBeNotFound")]
#[case(vec!["t1:NF"], "t1:v2@t1", "Ko=>AlreadyKnownToBeNotFound")]
#[case(vec!["t1:NF"], "t2:v2@t1", "Ko=>AlreadyKnownToBeNotFound")]
#[case(vec!["t1:v2@t0"], "t1:v2@t1", "Ko=>AlreadyKnownAndDiffers")]
#[case(vec!["t0:v2@t0"], "t1:v2@t1", "Ko=>AlreadyKnownAndDiffers")]
#[case(vec!["t1:v2@t1"], "t0:v2@t0", "Ko=>AlreadyKnownAndDiffers")]
#[case(vec!["t2:v2@t2"], "t1:v3@t1", "Ko=>AlreadyKnownToHaveMoreRecentSmallerVersion")]
#[case(vec!["t2:v2@t2"], "t3:v3@t1", "Ko=>AlreadyKnownToHaveMoreRecentSmallerVersion")]
#[case(vec!["t2:v2@t2"], "t3:v1@t1", "Ko=>AlreadyKnownToHaveMoreRecentSmallerVersion")]
fn brute_force(#[case] existing: Vec<&str>, #[case] new: &str, #[case] expected_outcome: &str) {
    println!("existing: {existing:?}");
    println!("new: {new:?}");
    println!("expected_outcome: {expected_outcome:?}");

    let existing = existing
        .into_iter()
        .map(|raw| raw.parse::<Populate>().unwrap())
        .collect::<Vec<_>>();
    let new = new.parse::<Populate>().unwrap();
    let expected_outcome = expected_outcome.parse::<Outcome>().unwrap();

    let mut cache = WorkspaceHistoryStoreCache::default();

    for item in existing {
        cache.populate_manifest_at(item.at, item.populate).unwrap();
    }

    let outcome = cache.populate_manifest_at(new.at, new.populate);

    match expected_outcome {
        Outcome::Ok { expected_resolves } => {
            for expected_resolve in expected_resolves {
                let resolved_entry =
                    cache.resolve_manifest_at(expected_resolve.at, expected_resolve.entry_id);
                match (expected_resolve.resolution, resolved_entry) {
                    (CacheResolvedEntry::NotFound, CacheResolvedEntry::NotFound)
                    | (CacheResolvedEntry::CacheMiss, CacheResolvedEntry::CacheMiss) => (),
                    (
                        CacheResolvedEntry::Exists(ArcChildManifest::File(expected)),
                        CacheResolvedEntry::Exists(ArcChildManifest::File(got)),
                    ) if expected == got => (),
                    (expected, got) => panic!(
                        "Resolution at {}: Expected {:?}, got {:?}",
                        expected_resolve.at, expected, got
                    ),
                }
            }
        }

        Outcome::Ko { check_error } => match outcome {
            Err(CachePopulateManifestAtError::InvalidHistory(err)) => check_error(&err),
            outcome => {
                panic!("Expected error, got {outcome:?}");
            }
        },
    }
}
