use anyhow::{Context, Result};
use bc_envelope::prelude::*;

use crate::{MnemonicLanguage, NoQuotesDebugOption, SeedFingerprint};

#[derive(Clone, PartialEq)]
pub struct Bip39Mnemonic {
    mnemonic: String,
    language: Option<MnemonicLanguage>,
    fingerprint: Option<SeedFingerprint>,
}

impl std::fmt::Debug for Bip39Mnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MnemonicSeed")
            .field("language", &NoQuotesDebugOption(&self.language))
            .field("mnemonic", &self.mnemonic)
            .field(
                "fingerprint",
                &NoQuotesDebugOption(&self.fingerprint.map(|f| f.to_hex())),
            )
            .finish()
    }
}

impl Bip39Mnemonic {
    pub fn new(mnemonic: impl AsRef<str>, language: Option<MnemonicLanguage>) -> Self {
        Self {
            mnemonic: mnemonic.as_ref().to_string(),
            language,
            fingerprint: None,
        }
    }

    pub fn set_fingerprint(&mut self, fingerprint: SeedFingerprint) {
        self.fingerprint = Some(fingerprint);
    }

    pub fn mnemonic(&self) -> &String {
        &self.mnemonic
    }

    pub fn set_mnemonic(&mut self, mnemonic: String) {
        self.mnemonic = mnemonic;
    }

    pub fn language(&self) -> Option<&MnemonicLanguage> {
        self.language.as_ref()
    }

    pub fn fingerprint(&self) -> Option<&SeedFingerprint> {
        self.fingerprint.as_ref()
    }

    pub fn set_language(&mut self, language: MnemonicLanguage) {
        self.language = Some(language);
    }
}

impl From<Bip39Mnemonic> for Envelope {
    fn from(value: Bip39Mnemonic) -> Self {
        Envelope::new(value.mnemonic)
            .add_type("Bip39Mnemonic")
            .add_optional_assertion("language", value.language)
            .add_optional_assertion("fingerprint", value.fingerprint)
    }
}

impl TryFrom<Envelope> for Bip39Mnemonic {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope
            .check_type_envelope("Bip39Mnemonic")
            .context("Bip39Mnemonic")?;
        let mnemonic = envelope.extract_subject().context("mnemonic")?;
        let language = envelope
            .try_optional_object_for_predicate("language")
            .context("language")?;
        let fingerprint = envelope
            .try_optional_object_for_predicate("fingerprint")
            .context("fingerprint")?;
        Ok(Self {
            mnemonic,
            language,
            fingerprint,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{MnemonicLanguage, SeedFingerprint, test_envelope_roundtrip};

    use super::Bip39Mnemonic;

    impl crate::RandomInstance for Bip39Mnemonic {
        fn random() -> Self {
            Self {
                mnemonic: String::random(),
                language: MnemonicLanguage::opt_random(),
                fingerprint: SeedFingerprint::opt_random(),
            }
        }
    }

    test_envelope_roundtrip!(Bip39Mnemonic);
}
