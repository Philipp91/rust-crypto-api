use ::hash::Hasher;
use ::hash::ring_impl;
use rustc_serialize::hex::ToHex;

#[test]
fn it_works() {
    assert_eq!(ring_impl::SHA512::hash(&[1, 2, 3]).as_ref().to_hex(), "27864cc5219a951a7a6e52b8c8dddf6981d098da1658d96258c870b2c88dfbcb51841aea172a28bafa6a79731165584677066045c959ed0f9929688d04defc29");
    assert_eq!(ring_impl::SHA512::hash_le("test").as_ref().to_hex(), "b6119eefd0e3959848079c5158f53c538a9d05059661c6402bbea25b96fc4537084f6e8a21ed8544e65fa69d07492ef3697f724fec0049d33c708f30760247af");
}

#[test]
fn it_works_with_sha1() {
    assert_eq!(ring_impl::SHA1::hash(&[1, 2, 3]).as_ref().to_hex(), "7037807198c22a7d2b0807371d763779a84fdfcf");
}