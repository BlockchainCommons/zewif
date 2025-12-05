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
            type Error = bc_envelope::Error;

            fn try_from(
                envelope: bc_envelope::Envelope,
            ) -> bc_envelope::Result<Self> {
                envelope.check_type(stringify!($name))?;
                let bytes = envelope.subject().try_byte_string()?;
                Self::try_from(bytes).map_err(|_| {
                    bc_envelope::Error::General(format!(
                        "Invalid {} blob",
                        stringify!($name)
                    ))
                })
            }
        }

        #[cfg(test)]
        mod test_envelope {
            use $crate::test_envelope_roundtrip;

            use super::$name;

            test_envelope_roundtrip!($name);
        }
    };
}
