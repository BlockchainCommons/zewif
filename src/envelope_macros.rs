#[macro_export]
macro_rules! blob_envelope {
    ($name:ident) => {
        impl From<$name> for bc_envelope::Envelope {
            fn from(value: $name) -> Self {
                let bytes: &[u8] = value.as_ref();
                let cbor = bc_envelope::prelude::CBOR::to_byte_string(bytes);
                bc_envelope::Envelope::new(cbor).add_type(stringify!($name))
            }
        }

        impl TryFrom<bc_envelope::Envelope> for $name {
            type Error = anyhow::Error;

            fn try_from(envelope: bc_envelope::Envelope) -> Result<Self, Self::Error> {
                envelope.check_type_envelope(stringify!($name))?;
                let bytes = envelope.subject().try_byte_string()?;
                $crate::parse!(buf = &bytes, $name, stringify!($name))
            }
        }

        #[cfg(test)]
        impl $crate::RandomInstance for $name {
            fn random() -> Self {
                Self($crate::Blob::random())
            }
        }

        $crate::test_envelope_roundtrip!($name);
    };
}
