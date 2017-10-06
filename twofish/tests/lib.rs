#![no_std]
extern crate twofish;
extern crate block_cipher_trait;
extern crate generic_array;

use block_cipher_trait::{BlockCipher, NewVarKey};
use generic_array::GenericArray;
use twofish::Twofish;

#[test]
fn encrypt_ecb128() {
    let mut key = GenericArray::default();
    let mut plain = GenericArray::default();
    let mut cipher;

    for i in 1..50 {
        let twofish = Twofish::new(&key).unwrap();

        let mut buf = plain.clone();
        twofish.encrypt_block(&mut buf);
        cipher = buf.clone();
        twofish.decrypt_block(&mut buf);
        assert_eq!(plain, buf);

        let correct = match i {
            1 => Some([
                0x9F, 0x58, 0x9F, 0x5C, 0xF6, 0x12, 0x2C, 0x32,
                0xB6, 0xBF, 0xEC, 0x2F, 0x2A, 0xE8, 0xC3, 0x5A,
            ]),
            2 => Some([
                0xD4, 0x91, 0xDB, 0x16, 0xE7, 0xB1, 0xC3, 0x9E,
                0x86, 0xCB, 0x08, 0x6B, 0x78, 0x9F, 0x54, 0x19,
            ]),
            3 => Some([
                0x01, 0x9F, 0x98, 0x09, 0xDE, 0x17, 0x11, 0x85,
                0x8F, 0xAA, 0xC3, 0xA3, 0xBA, 0x20, 0xFB, 0xC3,
            ]),
            4 => Some([
                0x63, 0x63, 0x97, 0x7D, 0xE8, 0x39, 0x48, 0x62,
                0x97, 0xE6, 0x61, 0xC6, 0xC9, 0xD6, 0x68, 0xEB,
            ]),
            5 => Some([
                0x81, 0x6D, 0x5B, 0xD0, 0xFA, 0xE3, 0x53, 0x42,
                0xBF, 0x2A, 0x74, 0x12, 0xC2, 0x46, 0xF7, 0x52,
            ]),
            48 => Some([
                0x6B, 0x45, 0x92, 0x86, 0xF3, 0xFF, 0xD2, 0x8D,
                0x49, 0xF1, 0x5B, 0x15, 0x81, 0xB0, 0x8E, 0x42,
            ]),
            _ => None,
        };

        correct.map(|v| assert_eq!(&cipher[..], v, "i = {}", i));
        key = plain;
        plain = cipher;
    }
}

#[test]
fn encrypt_ecb192() {
    let mut key = [0u8; 24];
    let mut plain = GenericArray::default();
    let mut cipher;

    for i in 1..50 {
        let twofish = Twofish::new(&key).unwrap();

        let mut buf = plain.clone();
        twofish.encrypt_block(&mut buf);
        cipher = buf.clone();
        twofish.decrypt_block(&mut buf);
        assert_eq!(plain, buf);

        let correct = match i {
            1 => Some([
                0xEF, 0xA7, 0x1F, 0x78, 0x89, 0x65, 0xBD, 0x44,
                0x53, 0xF8, 0x60, 0x17, 0x8F, 0xC1, 0x91, 0x01,
            ]),
            2 => Some([
                0x88, 0xB2, 0xB2, 0x70, 0x6B, 0x10, 0x5E, 0x36,
                0xB4, 0x46, 0xBB, 0x6D, 0x73, 0x1A, 0x1E, 0x88,
            ]),
            3 => Some([
                0x39, 0xDA, 0x69, 0xD6, 0xBA, 0x49, 0x97, 0xD5,
                0x85, 0xB6, 0xDC, 0x07, 0x3C, 0xA3, 0x41, 0xB2,
            ]),
            4 => Some([
                0x18, 0x2B, 0x02, 0xD8, 0x14, 0x97, 0xEA, 0x45,
                0xF9, 0xDA, 0xAC, 0xDC, 0x29, 0x19, 0x3A, 0x65,
            ]),
            5 => Some([
                0x7A, 0xFF, 0x7A, 0x70, 0xCA, 0x2F, 0xF2, 0x8A,
                0xC3, 0x1D, 0xD8, 0xAE, 0x5D, 0xAA, 0xAB, 0x63,
            ]),
            48 => Some([
                0xF0, 0xAB, 0x73, 0x30, 0x11, 0x25, 0xFA, 0x21,
                0xEF, 0x70, 0xBE, 0x53, 0x85, 0xFB, 0x76, 0xB6,
            ]),
            _ => None,
        };

        correct.map(|v| assert_eq!(&cipher[..], v, "i = {}", i));
        let (l, r) = key.split_at_mut(16);
        r.copy_from_slice(&l[..8]);
        l.copy_from_slice(&plain);
        plain = cipher;
    }
}

#[test]
fn encrypt_ecb256() {
    let mut key = [0u8; 32];
    let mut plain = GenericArray::default();
    let mut cipher;

    for i in 1..50 {
        let twofish = Twofish::new(&key).unwrap();

        let mut buf = plain.clone();
        twofish.encrypt_block(&mut buf);
        cipher = buf.clone();
        twofish.decrypt_block(&mut buf);
        assert_eq!(plain, buf);

        let correct = match i {
            1 => Some([
                0x57, 0xFF, 0x73, 0x9D, 0x4D, 0xC9, 0x2C, 0x1B,
                0xD7, 0xFC, 0x01, 0x70, 0x0C, 0xC8, 0x21, 0x6F,
            ]),
            2 => Some([
                0xD4, 0x3B, 0xB7, 0x55, 0x6E, 0xA3, 0x2E, 0x46,
                0xF2, 0xA2, 0x82, 0xB7, 0xD4, 0x5B, 0x4E, 0x0D,
            ]),
            3 => Some([
                0x90, 0xAF, 0xE9, 0x1B, 0xB2, 0x88, 0x54, 0x4F,
                0x2C, 0x32, 0xDC, 0x23, 0x9B, 0x26, 0x35, 0xE6,
            ]),
            4 => Some([
                0x6C, 0xB4, 0x56, 0x1C, 0x40, 0xBF, 0x0A, 0x97,
                0x05, 0x93, 0x1C, 0xB6, 0xD4, 0x08, 0xE7, 0xFA,
            ]),
            5 => Some([
                0x30, 0x59, 0xD6, 0xD6, 0x17, 0x53, 0xB9, 0x58,
                0xD9, 0x2F, 0x47, 0x81, 0xC8, 0x64, 0x0E, 0x58,
            ]),
            48 => Some([
                0x43, 0x10, 0x58, 0xF4, 0xDB, 0xC7, 0xF7, 0x34,
                0xDA, 0x4F, 0x02, 0xF0, 0x4C, 0xC4, 0xF4, 0x59,
            ]),
            _ => None,
        };

        correct.map(|v| assert_eq!(&cipher[..], v, "i = {}", i));
        let (l, r) = key.split_at_mut(16);
        r.copy_from_slice(&l[..16]);
        l.copy_from_slice(&plain);
        plain = cipher;
    }
}
