use bc_envelope::prelude::*;

use crate::{Data, NoQuotesDebugOption, SeedFingerprint};

#[derive(Clone, PartialEq)]
pub struct LegacySeed {
    seed_data: Data,
    fingerprint: Option<SeedFingerprint>,
}

impl std::fmt::Debug for LegacySeed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MnemonicSeed")
            .field("seed_data", &"<elided>".to_string())
            .field(
                "fingerprint",
                &NoQuotesDebugOption(&self.fingerprint.map(|f| f.to_hex())),
            )
            .finish()
    }
}

impl LegacySeed {
    pub fn new(seed_data: Data, fingerprint: Option<SeedFingerprint>) -> Self {
        Self { seed_data, fingerprint }
    }

    pub fn seed_data(&self) -> &Data {
        &self.seed_data
    }

    pub fn fingerprint(&self) -> Option<&SeedFingerprint> {
        self.fingerprint.as_ref()
    }
}

impl From<LegacySeed> for Envelope {
    fn from(value: LegacySeed) -> Self {
        Envelope::new(value.seed_data)
            .add_type("LegacySeed")
            .add_optional_assertion("fingerprint", value.fingerprint)
    }
}

impl TryFrom<Envelope> for LegacySeed {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type_envelope("LegacySeed")?;
        let seed_data = envelope.extract_subject()?;
        let fingerprint =
            envelope.try_optional_object_for_predicate("fingerprint")?;
        Ok(Self { seed_data, fingerprint })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Data, SeedFingerprint, test_envelope_roundtrip};

    use super::LegacySeed;

    impl crate::RandomInstance for LegacySeed {
        fn random() -> Self {
            Self {
                seed_data: Data::random(),
                fingerprint: SeedFingerprint::opt_random(),
            }
        }
    }

    test_envelope_roundtrip!(LegacySeed);
}
