use std::hash as stdhash;
use std::marker::{Sized, PhantomData};
use byteorder::{ByteOrder, LittleEndian, BigEndian};

/**
 * The Hasher trait specifies an interface common to hash functions, such as SHA-1 and the SHA-2
 * family of hash functions.
 */
pub trait Hasher: Sized {

    // TODO Make this a fixed struct instead?
    type Digest: AsRef<[u8]> + Eq; // TODO More? Like PartialOrd, Ord, Copy, Clone, Encodable, Decodable, Index

    fn new() -> Self;

    // TODO Not happy with the function names yet
    fn write<D: AsRef<[u8]> + Sized>(&mut self, data: D); // TODO bikeshedding: "input"?
    fn write_le<D: stdhash::Hash>(&mut self, data: D) {
        data.hash(&mut HasherAdapter::<Self, LittleEndian>(self, PhantomData))
    }
    fn write_be<D: stdhash::Hash>(&mut self, data: D) {
        data.hash(&mut HasherAdapter::<Self, BigEndian>(self, PhantomData))
    }

    fn finish(self) -> Self::Digest; // TODO or &mut self? // TODO bikeshedding: "result"?

    fn hash<D: AsRef<[u8]> + Sized>(data: D) -> Self::Digest {
        let mut hasher = Self::new();
        hasher.write(data);
        hasher.finish()
    }
    fn hash_le<D: stdhash::Hash>(data: D) -> Self::Digest {
        let mut hasher = Self::new();
        hasher.write_le(data);
        hasher.finish()
    }
    fn hash_be<D: stdhash::Hash>(data: D) -> Self::Digest {
        let mut hasher = Self::new();
        hasher.write_be(data);
        hasher.finish()
    }

    //fn reset(&mut self);
    //fn output_bits(&self) -> usize;
    //fn output_bytes(&self) -> usize { (self.output_bits() + 7) / 8 }
    //fn block_size(&self) -> usize;
    //fn input_str(&mut self, input: &str) { self.input(input.as_bytes());}
}

#[allow(dead_code)]
struct HasherAdapter<'a, H: 'a + Hasher, B: ByteOrder>(&'a mut H, PhantomData<B>);

impl<'a, H: Hasher, B: ByteOrder> stdhash::Hasher for HasherAdapter<'a, H, B> {
    fn finish(&self) -> u64 { unreachable!() }
    fn write(&mut self, bytes: &[u8]) { self.0.write(bytes); }

    #[inline]
    fn write_u8(&mut self, i: u8) {
        self.write(&[i])
    }
    #[inline]
    fn write_u16(&mut self, i: u16) {
        let mut buf = [0u8; 2];
        B::write_u16(&mut buf, i);
        self.write(&buf);
    }
    #[inline]
    fn write_u32(&mut self, i: u32) {
        let mut buf = [0u8; 4];
        B::write_u32(&mut buf, i);
        self.write(&buf);
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        let mut buf = [0u8; 8];
        B::write_u64(&mut buf, i);
        self.write(&buf);
    }
    #[inline]
    fn write_usize(&mut self, i: usize) {
        // TODO Is that correct endian-wise?
        use std::mem;
        let nbytes = mem::size_of::<usize>();
        let mut buf = [0u8; 8];
        B::write_uint(&mut buf, i as u64, nbytes);
        self.write(&buf[..nbytes]);
    }
    #[inline]
    fn write_i8(&mut self, i: i8) {
        self.write_u8(i as u8)
    }
    #[inline]
    fn write_i16(&mut self, i: i16) {
        self.write_u16(i as u16)
    }
    #[inline]
    fn write_i32(&mut self, i: i32) {
        self.write_u32(i as u32)
    }
    #[inline]
    fn write_i64(&mut self, i: i64) {
        self.write_u64(i as u64)
    }
    #[inline]
    fn write_isize(&mut self, i: isize) {
        self.write_usize(i as usize)
    }
}

#[cfg(test)]
mod ring_tests;

#[cfg(test)]
mod ring_impl;