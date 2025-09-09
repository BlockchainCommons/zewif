use bc_envelope::prelude::*;
use std::collections::{HashMap, HashSet};
use std::error::Error;

pub trait RandomInstance {
    fn random() -> Self;

    fn opt_random() -> Option<Self>
    where
        Self: Sized,
    {
        let mut rng = bc_rand::thread_rng();
        if bc_rand::rng_random_bool(&mut rng) {
            Some(Self::random())
        } else {
            None
        }
    }

    fn random_with_size(_size: usize) -> Self
    where
        Self: Sized,
    {
        panic!(
            "RandomInstance::random_with_size is not implemented for this type"
        );
    }

    fn opt_random_with_size(size: usize) -> Option<Self>
    where
        Self: Sized,
    {
        if bc_rand::rng_random_bool(&mut bc_rand::thread_rng()) {
            Some(Self::random_with_size(size))
        } else {
            None
        }
    }
}

impl RandomInstance for u8 {
    fn random() -> Self {
        let mut rng = bc_rand::thread_rng();
        u8::from_le_bytes(bc_rand::rng_random_array(&mut rng))
    }
}

impl RandomInstance for u32 {
    fn random() -> Self {
        let mut rng = bc_rand::thread_rng();
        u32::from_le_bytes(bc_rand::rng_random_array(&mut rng))
    }
}

impl RandomInstance for usize {
    fn random() -> Self {
        let mut rng = bc_rand::thread_rng();
        usize::from_le_bytes(bc_rand::rng_random_array(&mut rng))
    }
}

impl RandomInstance for String {
    fn random() -> Self {
        let mut rng = bc_rand::thread_rng();
        let len = rand::Rng::gen_range(&mut rng, 10..=100);
        let alphabet =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        let mut s = String::new();
        for _ in 0..len {
            let c = alphabet
                .chars()
                .nth(rand::Rng::gen_range(&mut rng, 0..alphabet.len()))
                .unwrap();
            s.push(c);
        }
        s
    }
}

impl<T> RandomInstance for Vec<T>
where
    T: RandomInstance,
{
    fn random() -> Self {
        let mut rng = bc_rand::thread_rng();
        let len = rand::Rng::gen_range(&mut rng, 1..=5);
        (0..len).map(|_| T::random()).collect()
    }
}

impl<K, V> RandomInstance for HashMap<K, V>
where
    K: RandomInstance + std::hash::Hash + Eq + Clone,
    V: RandomInstance + Clone,
{
    fn random() -> Self {
        let mut rng = bc_rand::thread_rng();
        let len = rand::Rng::gen_range(&mut rng, 1..=10);
        (0..len).map(|_| (K::random(), V::random())).collect()
    }
}

impl<T> RandomInstance for HashSet<T>
where
    T: RandomInstance + std::hash::Hash + Eq + Clone,
{
    fn random() -> Self {
        let mut rng = bc_rand::thread_rng();
        let len = rand::Rng::gen_range(&mut rng, 1..=10);
        (0..len).map(|_| T::random()).collect()
    }
}

pub fn test_cbor_roundtrip<T>(iterations: usize, print: bool)
where
    T: RandomInstance
        + Into<CBOR>
        + TryFrom<CBOR, Error = dcbor::Error>
        + Clone
        + std::fmt::Debug
        + PartialEq,
{
    if print {
        bc_envelope::register_tags();
    }
    for _ in 0..iterations {
        let i1 = T::random();
        let cbor: CBOR = i1.clone().into();
        if print {
            println!("{}", cbor.diagnostic_annotated());
        }
        let i3 = T::try_from(cbor).unwrap();
        assert_eq!(i1, i3);
    }
}

pub fn test_envelope_roundtrip<T>(iterations: usize, print: bool)
where
    T: RandomInstance
        + Into<Envelope>
        + TryFrom<Envelope, Error = bc_envelope::Error>
        + Clone
        + std::fmt::Debug
        + PartialEq,
{
    for _ in 0..iterations {
        let i1 = T::random();
        let envelope = i1.clone().into();
        if print {
            println!("{}", envelope.format());
        }
        for _ in 0..10 {
            let e2 = i1.clone().into();
            if envelope.digest() != e2.digest() {
                eprintln!("Determinism error: envelope digest mismatch");
                eprintln!("  Original: {}", envelope.format_flat());
                eprintln!("  Copy:     {}", e2.format_flat());
                panic!("Envelope digest mismatch");
            }
        }
        let i3 = match T::try_from(envelope) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Error converting envelope to type: {}", e);
                // Print the error context stack
                eprintln!("Error stack:");
                let mut source = e.source();
                while let Some(err) = source {
                    eprintln!("  caused by: {}", err);
                    source = err.source();
                }
                panic!("Failed to convert envelope: {}", e);
            }
        };
        assert_eq!(i1, i3);
    }
}

impl RandomInstance for Attachments {
    fn random() -> Self {
        let mut attachments = Attachments::new();
        let mut rng = bc_rand::thread_rng();
        let len = rand::Rng::gen_range(&mut rng, 0..=3);
        for _ in 0..len {
            attachments.add(
                String::random(),
                String::random(),
                String::opt_random(),
            );
        }
        attachments
    }
}
