// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashMap;

use libparsec_types::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum InvalidManifestHistoryError {
    #[error("Manifest `{entry_id}` lookup at `{at}` resolved as `Exists{{ version={manifest_version}, timestamp={manifest_timestamp} }}` breaks consistency: manifest timestamp is more recent than the lookup")]
    TooRecent {
        at: DateTime,
        entry_id: VlobID,
        manifest_version: VersionInt,
        manifest_timestamp: DateTime,
    },
    #[error("Manifest `{entry_id}` lookup at `{at}` resolved as `NoFound` breaks consistency: resolution is already known and should be `Exists{{ version={expected_manifest_version}, timestamp={expected_manifest_timestamp} }}`")]
    AlreadyKnownToExist {
        at: DateTime,
        entry_id: VlobID,
        expected_manifest_version: VersionInt,
        expected_manifest_timestamp: DateTime,
    },
    #[error("Manifest `{entry_id}` lookup at `{at}` resolved as `Exists{{ version={manifest_version}, timestamp={manifest_timestamp} }}` breaks consistency: resolution is already known to be not found at the time of the lookup")]
    AlreadyKnownToBeNotFound {
        at: DateTime,
        entry_id: VlobID,
        manifest_version: VersionInt,
        manifest_timestamp: DateTime,
    },
    #[error("Manifest `{entry_id}` lookup at `{at}` resolved as `Exists{{ version={manifest_version}, timestamp={manifest_timestamp} }}` breaks consistency: resolution is already known and should be `Exists{{ version={expected_manifest_version}, timestamp={expected_manifest_timestamp} }}`")]
    AlreadyKnownAndDiffers {
        at: DateTime,
        entry_id: VlobID,
        manifest_version: VersionInt,
        manifest_timestamp: DateTime,
        expected_manifest_version: VersionInt,
        expected_manifest_timestamp: DateTime,
    },
    #[error("Manifest `{entry_id}` lookup at `{at}` resolved as `Exists{{ version={manifest_version}, timestamp={manifest_timestamp} }}` breaks consistency: a more recent resolution is known to have a smaller version `Exists{{ version={known_manifest_version}, timestamp={known_manifest_timestamp} }}`")]
    AlreadyKnownToHaveMoreRecentSmallerVersion {
        at: DateTime,
        entry_id: VlobID,
        manifest_version: VersionInt,
        manifest_timestamp: DateTime,
        known_manifest_version: VersionInt,
        known_manifest_timestamp: DateTime,
    },
}

#[derive(Debug)]
pub(super) enum CacheResolvedEntry {
    Exists(ArcChildManifest),
    NotFound,
    CacheMiss,
}

#[derive(Debug)]
pub(super) enum CachePopulateManifestEntry {
    Exists(ArcChildManifest),
    NotFound(VlobID),
}

/// Stores the fact a manifest had a given version at a given time.
enum ManifestAtResolution {
    Exists {
        at: DateTime,
        version: VersionInt,
        timestamp: DateTime,
        manifest: ArcChildManifest,
    },
    NotFound {
        at: DateTime,
    },
}

#[derive(Debug, thiserror::Error)]
pub(super) enum CachePopulateManifestAtError {
    #[error(transparent)]
    InvalidHistory(#[from] Box<InvalidManifestHistoryError>),
}

#[derive(Default)]
pub(super) struct WorkspaceHistoryStoreCache {
    /// Very simple cache for blocks: we consider a block is most of the time much bigger
    /// than a typical kernel read (512ko vs 4ko), so it's a big win to just keep the
    /// blocks currently being read in memory.
    ///
    /// This is especially important when the workspace history works in server-based access
    /// since any block cache miss means a round trip to the server.
    ///
    /// To approximate that, we just keep the last 128 blocks read in memory.
    ///
    /// More practical information in this issue: https://github.com/Scille/parsec-cloud/issues/7111
    blocks: RoundRobinCache<BlockID, Bytes, 128>,
    /// Stores all the informations we know about what version a manifest was at a given
    /// point in time.
    ///
    /// For instance lets consider the following subsequent resolutions for a given manifest:
    ///
    /// dawn of humanity  t0     t1       v1         t2     v2       t3    present time
    ///  |                |      |        |          |      |        |         |
    ///   <-- NotFound --> <--------------- ?? ------------------------------->
    ///   <------- NotFound ----> <--------------- ?? ------------------------>
    ///   <------- NotFound ----> <--------??--------------> <- v2 -> <-- ?? ->
    ///   <------- NotFound -------------> <-------- v1 ---> <- v2 -> <-- ?? ->
    ///
    /// There is four populates here:
    /// 1. A lookup at t0 lead to a NotFound.
    /// 2. A lookup at t1 also lead to a NotFound, hence replacing the previous one.
    /// 3. A lookup at t3 lead to discovering v2.
    /// 4. A lookup at t2 lead to discovering v1. Since this is the initial version,
    ///    we know any lookup before it should lead to a NotFound. And since we
    ///    also know v2, we can deduce v1 should also be used between t2 and the time
    ///    v2 was created.
    ///
    /// Notes:
    /// - The Resolutions are ordered by lookup time (i.e. `at` param in `ManifestAtResolution`).
    /// - Unlike for blocks, we never evict entries from the manifest cache.
    /// - We use a smallvec to store the different versions of a manifest.
    ///   This is because it is very likely the history will only be accessed for a
    ///   single point in time.
    ///   However a regular vector bets on the fact it will need to grow and hence
    ///   allocates much more memory than necessary (even if we use `Vec::with_capacity(1)`).
    manifests: HashMap<VlobID, smallvec::SmallVec<[ManifestAtResolution; 1]>>,
}

impl WorkspaceHistoryStoreCache {
    pub(super) fn get_block(&self, block_id: BlockID) -> Option<Bytes> {
        self.blocks.get(&block_id).cloned()
    }

    pub(super) fn populate_block(&mut self, block_id: BlockID, block_data: Bytes) {
        // Note in case of a concurrent populate of this block ID, we will end up
        // with two copies of the block in the cache !
        //
        // This is not a big deal since the cache is limited in size and both entries
        // should contain the same data (and on the other hand it makes insertion
        // faster).
        //
        // The only downside is we cannot guarantee that the block stays the same
        // (in case we use server-based access and the server is buggy/malicious).
        //
        // However the cache is of limited size, so best we could do is only to
        // provide the guarantee for the last N blocks, which is not very useful.
        //
        // On top of that, the block data has been validated against a block access
        // containing its hash. So to have a given block changing between two
        // concurrent populates, it must be two different manifests that are accessed
        // concurrently and which both reference the same block ID but with different
        // block hash (which is a purely theoretical case).
        self.blocks.push(block_id, block_data.clone());
    }

    pub(super) fn populate_manifest_at(
        &mut self,
        at: DateTime,
        entry: CachePopulateManifestEntry,
    ) -> Result<(), CachePopulateManifestAtError> {
        match entry {
            // `NotFound` means the manifest didn't exist up to at least this point
            // in time, so having a `NotFound` after an `Exists` is not possible.
            // Hence here the possibilities are:
            // 1. The is no existing resolutions, so we can just add ours.
            // 2. The first existing resolution is already a `NotFound` and we want
            //   to merge them together.
            // 3. The first existing resolution is not a `NotFound`, and we should
            //   just insert our `NotFound` as first resolution.
            // 4. The first existing resolution is an `Exists` with version 1, this is
            //   equivalent to the first case and we don't need to update anything (since
            //   we already precisely know when the manifest has been created).
            CachePopulateManifestEntry::NotFound(entry_id) => {
                match self.manifests.entry(entry_id) {
                    std::collections::hash_map::Entry::Vacant(e) => {
                        // Case 1: The is no existing resolutions, so we can just add ours and call it a day ;-)
                        e.insert(smallvec::smallvec![ManifestAtResolution::NotFound { at }]);
                        Ok(())
                    }

                    std::collections::hash_map::Entry::Occupied(e) => {
                        let resolutions = e.into_mut();
                        // Sanity check since we never remove any resolution and the smallvec is
                        // always created with an initial item.
                        assert!(!resolutions.is_empty());

                        match &mut resolutions[0] {
                            ManifestAtResolution::NotFound {
                                at: not_found_until,
                            } => {
                                // Case 2: merge our `NotFound` with the existing one
                                *not_found_until = std::cmp::max(*not_found_until, at);
                                Ok(())
                            }
                            ManifestAtResolution::Exists {
                                version: resolution_version,
                                timestamp: resolution_timestamp,
                                ..
                            } => {
                                if *resolution_timestamp <= at {
                                    // This is unexpected: our new resolution states that
                                    // the manifest at least didn't exist until a time posterior
                                    // to when previous resolution tells us a manifest version
                                    // was created !
                                    return Err(CachePopulateManifestAtError::InvalidHistory(
                                        Box::new(
                                            InvalidManifestHistoryError::AlreadyKnownToExist {
                                                at,
                                                entry_id,
                                                expected_manifest_version: *resolution_version,
                                                expected_manifest_timestamp: *resolution_timestamp,
                                            },
                                        ),
                                    ));
                                }
                                if *resolution_version != 1 {
                                    // Case 3: No `NotFound` exists, we can  just add ours !
                                    resolutions.insert(0, ManifestAtResolution::NotFound { at });
                                } else {
                                    // Case 4: Special case if we already know version 1 of the manifest
                                }
                                Ok(())
                            }
                        }
                    }
                }
            }

            // Populate an actual version of the manifest
            // For this we must iterate over the existing resolutions until we
            // find the right spot to insert ourselves:
            // 1. There is no existing resolutions, so we can just add ours.
            // 2. An existing resolution occurred after our own. We should insert our
            //    own right before...
            // 3. ...or merge it we the previous resolution if they both refers to
            //    the same manifest version.
            // 4. All existing resolutions occurred at times before our own resolution.
            //    So we can just insert ours last...
            // 5. ...or merge it we the previous resolution if they both refers to
            //    the same manifest version.
            CachePopulateManifestEntry::Exists(manifest) => {
                let (id, version, timestamp) = match &manifest {
                    ArcChildManifest::File(manifest) => {
                        (manifest.id, manifest.version, manifest.timestamp)
                    }
                    ArcChildManifest::Folder(manifest) => {
                        (manifest.id, manifest.version, manifest.timestamp)
                    }
                };

                if timestamp > at {
                    return Err(CachePopulateManifestAtError::InvalidHistory(Box::new(
                        InvalidManifestHistoryError::TooRecent {
                            at,
                            entry_id: id,
                            manifest_version: version,
                            manifest_timestamp: timestamp,
                        },
                    )));
                }

                let resolutions = match self.manifests.entry(id) {
                    std::collections::hash_map::Entry::Vacant(e) => {
                        // Case 1: There is no existing resolutions, so we can just add ours and call it a day ;-)
                        e.insert(smallvec::smallvec![ManifestAtResolution::Exists {
                            at,
                            version,
                            timestamp,
                            manifest,
                        }]);
                        return Ok(());
                    }

                    std::collections::hash_map::Entry::Occupied(e) => {
                        let resolutions = e.into_mut();
                        // Sanity check since we never remove any resolution and the smallvec is
                        // always created with an initial item.
                        assert!(!resolutions.is_empty());
                        resolutions
                    }
                };

                for (i, resolution) in resolutions.iter_mut().enumerate() {
                    match resolution {
                        ManifestAtResolution::NotFound { at: resolution_at } => {
                            if *resolution_at >= timestamp {
                                // This is unexpected: our new resolution states that
                                // the manifest was created at a time anterior than
                                // when previous resolution tells us the manifest didn't
                                // exist !
                                return Err(CachePopulateManifestAtError::InvalidHistory(
                                    Box::new(
                                        InvalidManifestHistoryError::AlreadyKnownToBeNotFound {
                                            at,
                                            entry_id: id,
                                            manifest_version: version,
                                            manifest_timestamp: timestamp,
                                        },
                                    ),
                                ));
                            }
                            // Sanity check since we have already checked `timestamp <= at`
                            assert!(*resolution_at < at);
                        }

                        ManifestAtResolution::Exists {
                            at: resolution_at,
                            version: resolution_version,
                            timestamp: resolution_timestamp,
                            ..
                        } => {
                            if at < *resolution_at {
                                match version.cmp(resolution_version) {
                                    std::cmp::Ordering::Less => {
                                        // Case 2: An existing resolution occured after our own and refers to a different manifest version.

                                        if at >= *resolution_timestamp {
                                            return Err(CachePopulateManifestAtError::InvalidHistory(Box::new(
                                                InvalidManifestHistoryError::AlreadyKnownAndDiffers {
                                                    at,
                                                    entry_id: id,
                                                    manifest_version: version,
                                                    manifest_timestamp: timestamp,
                                                    expected_manifest_version: *resolution_version,
                                                    expected_manifest_timestamp: *resolution_timestamp,
                                                }
                                            )));
                                        }

                                        // Sanity check since we have already checked `timestamp <= at`
                                        assert!(timestamp < *resolution_timestamp);

                                        // Special case if our resolution's version strictly follow the
                                        // version of the next resolution: in this case we now perfectly
                                        // know the span of time our resolution was valid !
                                        let at = if version + 1 == *resolution_version {
                                            (*resolution_timestamp).add_us(-1)
                                        } else {
                                            at
                                        };

                                        resolutions.insert(
                                            i,
                                            ManifestAtResolution::Exists {
                                                at,
                                                version,
                                                timestamp,
                                                manifest,
                                            },
                                        );
                                    }
                                    std::cmp::Ordering::Equal => {
                                        // Case 3: An existing resolution occured after our own and refers to the same manifest version.
                                        // We can update the existing resolution to indicate the manifest was still using the same
                                        // version up until the time of our new resolution.

                                        if *resolution_timestamp != timestamp {
                                            return Err(CachePopulateManifestAtError::InvalidHistory(Box::new(
                                                InvalidManifestHistoryError::AlreadyKnownAndDiffers {
                                                    at,
                                                    entry_id: id,
                                                    manifest_version: version,
                                                    manifest_timestamp: timestamp,
                                                    expected_manifest_version: *resolution_version,
                                                    expected_manifest_timestamp: *resolution_timestamp,
                                                }
                                            )));
                                        }

                                        *resolution_at = std::cmp::max(*resolution_at, at);
                                    }
                                    std::cmp::Ordering::Greater => {
                                        return Err(CachePopulateManifestAtError::InvalidHistory(Box::new(
                                            InvalidManifestHistoryError::AlreadyKnownToHaveMoreRecentSmallerVersion {
                                                at,
                                                entry_id: id,
                                                manifest_version: version,
                                                manifest_timestamp: timestamp,
                                                known_manifest_version: *resolution_version,
                                                known_manifest_timestamp: *resolution_timestamp,
                                            }
                                        )));
                                    }
                                }
                                return Ok(());
                            }
                        }
                    }
                }

                // If we end up here, it means all existing resolutions occured at times
                // before our own resolution.

                let previous_resolution = resolutions.last_mut().expect("cannot be empty");

                match previous_resolution {
                    ManifestAtResolution::Exists {
                        at: resolution_at,
                        version: resolution_version,
                        timestamp: resolution_timestamp,
                        ..
                    } => {
                        match version.cmp(resolution_version) {
                            std::cmp::Ordering::Greater => {
                                // Case 4: Previous resolution is for a different version, just insert our own last

                                // Sanity check since the previous loop should have handled this case
                                assert!(at > *resolution_at);

                                // Ensure our resolution is about a manifest newer that the previous resolution one
                                if timestamp <= *resolution_timestamp {
                                    return Err(CachePopulateManifestAtError::InvalidHistory(Box::new(
                                        InvalidManifestHistoryError::AlreadyKnownToHaveMoreRecentSmallerVersion {
                                            at,
                                            entry_id: id,
                                            manifest_version: version,
                                            manifest_timestamp: timestamp,
                                            known_manifest_version: *resolution_version,
                                            known_manifest_timestamp: *resolution_timestamp,
                                        }
                                    )));
                                }

                                // Special case if our resolution's version strictly follow the
                                // version of the previous resolution: in this case we now perfectly
                                // know the span of time the previous resolution was valid !
                                if version == *resolution_version + 1 {
                                    assert!(timestamp > *resolution_at);
                                    *resolution_at = timestamp.add_us(-1);
                                }

                                resolutions.push(ManifestAtResolution::Exists {
                                    at,
                                    version,
                                    timestamp,
                                    manifest,
                                });
                            }
                            std::cmp::Ordering::Equal => {
                                // Case 5: Previous resolution is for the same version, update it

                                if *resolution_timestamp != timestamp {
                                    return Err(CachePopulateManifestAtError::InvalidHistory(
                                        Box::new(
                                            InvalidManifestHistoryError::AlreadyKnownAndDiffers {
                                                at,
                                                entry_id: id,
                                                manifest_version: version,
                                                manifest_timestamp: timestamp,
                                                expected_manifest_version: *resolution_version,
                                                expected_manifest_timestamp: *resolution_timestamp,
                                            },
                                        ),
                                    ));
                                }

                                *resolution_at = std::cmp::max(*resolution_at, at);
                            }
                            std::cmp::Ordering::Less => {
                                return Err(CachePopulateManifestAtError::InvalidHistory(Box::new(
                                    InvalidManifestHistoryError::AlreadyKnownToHaveMoreRecentSmallerVersion {
                                        at,
                                        entry_id: id,
                                        manifest_version: version,
                                        manifest_timestamp: timestamp,
                                        known_manifest_version: *resolution_version,
                                        known_manifest_timestamp: *resolution_timestamp,
                                    }
                                )));
                            }
                        }
                    }
                    ManifestAtResolution::NotFound { .. } => {
                        // Case 4: Previous resolution not for the same version, just insert our own last
                        resolutions.push(ManifestAtResolution::Exists {
                            at,
                            version,
                            timestamp,
                            manifest,
                        });
                    }
                }
                Ok(())
            }
        }
    }

    pub(super) fn resolve_manifest_at(&self, at: DateTime, id: VlobID) -> CacheResolvedEntry {
        let resolutions = match self.manifests.get(&id) {
            Some(resolutions) => resolutions,
            None => return CacheResolvedEntry::CacheMiss,
        };

        for resolution in resolutions {
            match *resolution {
                ManifestAtResolution::Exists {
                    at: resolution_at,
                    version: resolution_version,
                    timestamp: resolution_timestamp,
                    manifest: ref resolution_manifest,
                } if at <= resolution_at => {
                    if at >= resolution_timestamp {
                        return CacheResolvedEntry::Exists(resolution_manifest.to_owned());
                    }

                    // Version 1 is a special case since we know anything before its timestamp
                    // should lead to a NotFound.
                    if resolution_version == 1 {
                        return CacheResolvedEntry::NotFound;
                    }
                }
                ManifestAtResolution::NotFound { at: resolution_at } => {
                    if at <= resolution_at {
                        return CacheResolvedEntry::NotFound;
                    }
                }
                _ => (),
            }
        }

        CacheResolvedEntry::CacheMiss
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/workspace_history_store_cache.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
