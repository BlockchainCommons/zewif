use crate::{blob, blob_envelope};

// A hierarchical deterministic (HD) Sapling spending key with derivation information.
//
// `SaplingExtendedFullViewingKey` extends the core spending key functionality by adding the
// necessary components for hierarchical deterministic (HD) key derivation according to [ZIP 32].
// This enables the creation of structured wallet hierarchies with parent-child key relationships.
//
// This key is encoded as defined in https://zips.z.cash/zip-0032#sapling-extended-full-viewing-keys
//
// [ZIP 32]: https://zips.z.cash/zip-0032
blob!(
    SaplingExtendedFullViewingKey,
    73,
    "A Sapling Extended Full Viewing Key, encoded as specified in ZIP 32"
);

blob_envelope!(SaplingExtendedFullViewingKey);
