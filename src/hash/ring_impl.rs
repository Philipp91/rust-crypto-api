// NOTE: This module is for testing only. This code would normally live in the `ring` crate.

use ring::digest::{self, Context};
use ::hash::{Hasher};
use std::cmp::{PartialEq, Eq};

pub struct RingDigest(digest::Digest);

impl AsRef<[u8]> for RingDigest {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl PartialEq for RingDigest {
    fn eq(&self, other: &RingDigest) -> bool {
        self.as_ref() == other.as_ref() // TODO INSECURE, must be constant time
    }
}

impl Eq for RingDigest {}

macro_rules! implement_hash {
    ($name:ident, $digest_bytes:expr) => {
        pub struct $name(Context);
        impl Hasher for $name {
            type Digest = RingDigest;
            fn new() -> Self { $name(Context::new(&digest::$name)) }
            fn write<D>(&mut self, data: D) where D: AsRef<[u8]> + Sized {
                self.0.update(data.as_ref())
            }
            fn finish(self) -> Self::Digest {
                RingDigest(self.0.finish())
            }
        }
    }
}

implement_hash!(SHA1, 20);
implement_hash!(SHA512, 64);