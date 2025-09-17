use crate::error::{Error, Result};
use bc_components::{ARID, SymmetricKey};
use bc_crypto::pbkdf2_hmac_sha256;
use bc_envelope::prelude::*;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct ZewifEnvelope {
    id: ARID,
    envelope: Envelope,
}

impl ZewifEnvelope {
    pub fn new(envelope: Envelope) -> Result<Self> {
        if !envelope.has_type_envelope("Zewif") {
            return Err(Error::NotZewifEnvelope);
        }
        let id = envelope.extract_subject().map_err(|e| Error::Context {
            message: Cow::Borrowed("ID"),
            source: Box::new(e),
        })?;
        Ok(Self { id, envelope })
    }

    pub fn id(&self) -> ARID {
        self.id
    }

    pub fn digest(&self) -> Digest {
        self.envelope.digest().clone().into_owned()
    }

    pub fn envelope(&self) -> &Envelope {
        &self.envelope
    }

    pub fn obscured_content(&self) -> Option<Envelope> {
        self.envelope.object_for_predicate("content").ok()
    }

    pub fn is_obscured(&self) -> bool {
        !self.envelope.objects_for_predicate("content").is_empty()
    }

    pub fn is_compressed(&self) -> bool {
        self.obscured_content()
            .is_some_and(|content| content.is_compressed())
    }

    pub fn is_encrypted(&self) -> bool {
        self.obscured_content()
            .is_some_and(|content| content.is_encrypted())
    }

    pub fn can_compress(&self) -> bool {
        !self.is_obscured()
    }

    pub fn can_encrypt(&self) -> bool {
        !self.is_encrypted()
    }

    pub fn can_uncompress(&self) -> bool {
        self.is_compressed()
    }

    pub fn can_decrypt(&self) -> bool {
        self.is_encrypted()
    }

    pub fn compress(&mut self) -> Result<()> {
        if self.can_compress() {
            let content = self.envelope.wrap().compress()?;
            self.envelope = Envelope::new(self.id)
                .add_type("Zewif")
                .add_assertion("content", content);
        } else {
            return Err(Error::AlreadyCompressedOrEncrypted);
        }
        Ok(())
    }

    pub fn uncompress(&mut self) -> Result<()> {
        if self.can_uncompress() {
            self.envelope = self
                .envelope
                .object_for_predicate("content")?
                .uncompress()?
                .try_unwrap()?;
        } else {
            return Err(Error::NotCompressed);
        }
        Ok(())
    }

    pub fn derive_encryption_key(password: impl AsRef<str>) -> SymmetricKey {
        let key_bytes = pbkdf2_hmac_sha256(password.as_ref(), b"Zewif", 100_000, 32);
        SymmetricKey::from_data_ref(key_bytes).unwrap()
    }

    pub fn encrypt(&mut self, key: &SymmetricKey) -> Result<()> {
        if self.can_encrypt() {
            let content = self.envelope.encrypt(key);
            self.envelope = Envelope::new(self.id)
                .add_type("Zewif")
                .add_assertion("content", content);
        } else {
            return Err(Error::AlreadyEncrypted);
        }
        Ok(())
    }

    pub fn decrypt(&mut self, key: &SymmetricKey) -> Result<()> {
        if self.can_decrypt() {
            self.envelope = self
                .envelope
                .object_for_predicate("content")?
                .decrypt(key)?;
        } else {
            return Err(Error::NotEncrypted);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{RandomInstance, Zewif};

    use super::*;

    #[test]
    fn test_new_envelope() {
        // Create a random Zewif instance
        let zewif = Zewif::random();

        // Convert the Zewif instance to an Envelope
        let envelope = Envelope::from(zewif.clone());

        // Create a ZewifEnvelope instance from the Envelope
        let ze = ZewifEnvelope::new(envelope.clone()).unwrap();

        // Check the properties of the ZewifEnvelope instance
        assert_eq!(ze.id(), zewif.id());
        assert_eq!(ze.envelope(), &envelope);
        assert_eq!(ze.obscured_content(), None);
        assert!(!ze.is_obscured());
        assert!(!ze.is_compressed());
        assert!(!ze.is_encrypted());
        assert!(ze.can_compress());
        assert!(ze.can_encrypt());
        assert!(!ze.can_uncompress());
        assert!(!ze.can_decrypt());

        // Compress the ZewifEnvelope
        let mut ze_compressed = ze.clone();
        ze_compressed.compress().unwrap();
        //println!("{}", ze_compressed.envelope().format());
        // Check the properties of the compressed ZewifEnvelope
        assert!(ze_compressed.is_compressed());
        assert!(!ze_compressed.can_compress());
        assert!(ze_compressed.can_encrypt());
        assert!(ze_compressed.can_uncompress());
        assert!(!ze_compressed.can_decrypt());

        // Check the size of the compressed envelope
        let ze_size = ze.envelope().to_cbor_data().len();
        let ze_compressed_size = ze_compressed.envelope().to_cbor_data().len();
        let _percent_saved = 100.0 * (1.0 - (ze_compressed_size as f64 / ze_size as f64));
        // println!("Compressed:\n  Before: {}, After: {}, Savings:{:.2}%", ze_size, ze_compressed_size, _percent_saved);

        // Uncompress the ZewifEnvelope and make sure it matches the original
        let mut ze_uncompressed = ze_compressed.clone();
        ze_uncompressed.uncompress().unwrap();
        assert_eq!(ze.digest(), ze_uncompressed.digest());
        assert!(!ze_uncompressed.is_compressed());
        assert!(ze_uncompressed.can_compress());
        assert!(ze_uncompressed.can_encrypt());
        assert!(!ze_uncompressed.can_decrypt());

        // Encrypt the ZewifEnvelope
        let mut ze_encrypted = ze.clone();
        let key = ZewifEnvelope::derive_encryption_key("password");
        ze_encrypted.encrypt(&key).unwrap();
        //println!("{}", ze_encrypted.envelope().format());
        assert!(ze_encrypted.is_encrypted());
        assert!(!ze_encrypted.can_encrypt());
        assert!(ze_encrypted.can_decrypt());
        assert!(!ze_encrypted.can_compress());
        assert!(!ze_encrypted.can_uncompress());

        // Decrypt the ZewifEnvelope and make sure it matches the original
        let mut ze_decrypted = ze_encrypted.clone();
        ze_decrypted.decrypt(&key).unwrap();
        assert_eq!(ze.digest(), ze_decrypted.digest());
        assert!(!ze_decrypted.is_encrypted());
        assert!(ze_decrypted.can_encrypt());
        assert!(!ze_decrypted.can_decrypt());
        assert!(!ze_decrypted.is_compressed());
        assert!(ze_decrypted.can_compress());

        // Compress then encrypt the ZewifEnvelope
        //
        // You must always compress before encrypting, because the encryption
        // algorithm produces random data, which is not compressible.
        let mut ze_compressed_encrypted = ze.clone();
        ze_compressed_encrypted.compress().unwrap();
        ze_compressed_encrypted.encrypt(&key).unwrap();
        let ze_compressed_encrypted_size = ze_compressed_encrypted.envelope().to_cbor_data().len();
        let _percent_saved = 100.0 * (1.0 - (ze_compressed_encrypted_size as f64 / ze_size as f64));
        // println!("Encrypted and Compressed:\n  Before: {}, After: {}, Savings:{:.2}%", ze_size, ze_compressed_encrypted_size, _percent_saved);

        // Decompress then decrypt the ZewifEnvelope and make sure it matches the original
        let mut ze_decompressed_decrypted = ze_compressed_encrypted.clone();
        ze_decompressed_decrypted.decrypt(&key).unwrap();
        ze_decompressed_decrypted.uncompress().unwrap();
        assert_eq!(ze.digest(), ze_decompressed_decrypted.digest());

        // Reconstruct the Zewif instance from the decrypted envelope
        let zewif2 = Zewif::try_from(ze_decompressed_decrypted.envelope().clone()).unwrap();
        // Check that the reconstructed Zewif instance matches the original
        assert_eq!(zewif, zewif2);
    }
}
