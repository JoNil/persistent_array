#![feature(plugin)]

#![plugin(clippy)]

#![warn(cast_possible_truncation, cast_possible_wrap, cast_precision_loss, cast_sign_loss,
        non_ascii_literal, shadow_same, string_add, string_add_assign, unicode_not_nfc)]

extern crate persistent_array;

use persistent_array::{PersistentArray, Error};
use std::default::Default;

#[derive(Debug, Copy, Clone)]
struct Pair {
    a: u32,
    b: u32,
}

impl Default for Pair {
    fn default() -> Pair {
        Pair {
            a: 0xddaaddaa,
            b: 0xffeeffee,
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Pair3 {
    a: u32,
    b: u32,
    c: u32,
}

#[derive(Debug, Default, Copy, Clone)]
struct OtherPair2 {
    a: u32,
    b: u32,
}

#[test]
fn test() {
    {
        let mut db = PersistentArray::<Pair>::new("pair.db", 1024).unwrap();

        db[0] = Pair { a: 1, b: 2 };

        assert_eq!(db[0].a, 1);
        assert_eq!(db[0].b, 2);

        assert_eq!(db[1023].a, 0xddaaddaa);
        assert_eq!(db[1023].b, 0xffeeffee);
    }
    {
        let db = PersistentArray::<Pair>::open("pair.db").unwrap();

        assert_eq!(db.len(), 1024);

        assert_eq!(db[0].a, 1);
        assert_eq!(db[0].b, 2);

        assert_eq!(db[1023].a, 0xddaaddaa);
        assert_eq!(db[1023].b, 0xffeeffee);
    }
    {
        let db = PersistentArray::<Pair>::open("README.md");

        match db {
            Err(Error::WrongMagicBytes) => (),
            _ => assert!(false),
        };
    }
    {
        let db = PersistentArray::<Pair3>::open("pair.db");

        match db {
            Err(Error::WrongFileSize) => (),
            _ => assert!(false),
        };
        
    }
    {
        let db = PersistentArray::<OtherPair2>::open("pair.db");

        match db {
            Err(Error::WrongTypeId) => (),
            _ => assert!(false),
        };
    }
}
