use crate::blob;

// A hierarchical deterministic (HD) Sapling spending key with derivation information.
//
// `SaplingExtendedSpendingKey` extends the core spending key functionality by adding the
// necessary components for hierarchical deterministic (HD) key derivation according to
// ZIP-32 (Zcash's equivalent of BIP-32). This enables the creation of structured wallet
// hierarchies with parent-child key relationships.
//
// This key is encoded as defined in https://zips.z.cash/zip-0032#sapling-extended-spending-keys
blob!(
    SaplingExtendedSpendingKey,
    169,
    "A Sapling Extended Spending Key, encoded as specified in ZIP 32"
);
